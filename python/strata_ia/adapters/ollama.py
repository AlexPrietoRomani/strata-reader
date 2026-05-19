"""Ollama HTTP adapter.

Wraps the Ollama ``/api/generate`` endpoint with deterministic parameters
(``temperature=0``, ``seed=42``) and a Tenacity-driven retry policy.
The adapter is *async-only* — never block the FastAPI event loop.

References:
- Ollama API docs: <https://github.com/ollama/ollama/blob/main/docs/api.md>
- Plan Maestro §10.T5.2.
"""

from __future__ import annotations

import asyncio
import base64
import json
from dataclasses import dataclass
from typing import Any, AsyncIterator

import httpx
import structlog
from tenacity import (
    AsyncRetrying,
    RetryError,
    retry_if_exception_type,
    stop_after_attempt,
    wait_exponential,
)

logger = structlog.get_logger(__name__)


class OllamaError(Exception):
    """Raised when Ollama responds with a non-2xx HTTP status after all retries."""


class OllamaUnreachable(Exception):
    """Raised when the HTTP layer fails to even connect after all retries."""


@dataclass(frozen=True, slots=True)
class GenerateResult:
    text: str
    raw_response: dict[str, Any]
    model: str


class OllamaClient:
    """Async client around the Ollama HTTP API."""

    def __init__(
        self,
        endpoint: str = "http://localhost:11434",
        timeout_s: float = 60.0,
        retry_attempts: int = 3,
        seed: int = 42,
        temperature: float = 0.0,
        http_client: httpx.AsyncClient | None = None,
    ) -> None:
        self._endpoint = endpoint.rstrip("/")
        self._timeout = timeout_s
        self._retry_attempts = retry_attempts
        self._seed = seed
        self._temperature = temperature
        self._owns_client = http_client is None
        self._http = http_client or httpx.AsyncClient(
            timeout=timeout_s, base_url=self._endpoint
        )

    async def __aenter__(self) -> "OllamaClient":
        return self

    async def __aexit__(self, *_: object) -> None:
        await self.aclose()

    async def aclose(self) -> None:
        if self._owns_client:
            await self._http.aclose()

    async def list_models(self) -> list[str]:
        """Return the tags currently pulled on the server (cheap reachability probe)."""
        resp = await self._http.get("/api/tags")
        resp.raise_for_status()
        payload: dict[str, Any] = resp.json()
        return [m["name"] for m in payload.get("models", []) if "name" in m]

    async def generate(
        self,
        model: str,
        prompt: str,
        images: list[bytes] | None = None,
        *,
        format_json: bool = False,
    ) -> GenerateResult:
        """One-shot generation (``stream=false``) with retries.

        ``images`` carry raw bytes; the adapter base64-encodes them for the
        Ollama payload as required by the API. ``format_json`` forces the
        model to emit a single JSON object (Ollama's ``format=json`` mode).
        """
        payload: dict[str, Any] = {
            "model": model,
            "prompt": prompt,
            "stream": False,
            "options": {
                "temperature": self._temperature,
                "seed": self._seed,
                # Greedy sampling — the seed only matters when temperature > 0,
                # but we set both for forward-compat with model upgrades.
            },
        }
        if images:
            payload["images"] = [base64.b64encode(img).decode("ascii") for img in images]
        if format_json:
            payload["format"] = "json"

        retry = AsyncRetrying(
            stop=stop_after_attempt(self._retry_attempts),
            wait=wait_exponential(multiplier=0.5, min=0.5, max=5.0),
            retry=retry_if_exception_type((httpx.HTTPStatusError, httpx.ConnectError, httpx.ReadTimeout)),
            reraise=False,
        )

        try:
            async for attempt in retry:
                with attempt:
                    resp = await self._http.post("/api/generate", json=payload)
                    resp.raise_for_status()
                    body: dict[str, Any] = resp.json()
                    return GenerateResult(text=str(body.get("response", "")), raw_response=body, model=model)
        except RetryError as exc:
            cause = exc.last_attempt.exception()
            logger.warning("ollama_retry_exhausted", model=model, error=str(cause))
            if isinstance(cause, httpx.ConnectError):
                raise OllamaUnreachable(str(cause)) from cause
            raise OllamaError(str(cause)) from cause

        # `async for retry` always returns inside the loop; this branch is
        # unreachable but satisfies the type checker.
        raise OllamaError("unreachable")

    async def stream_generate(
        self, model: str, prompt: str, images: list[bytes] | None = None
    ) -> AsyncIterator[str]:
        """Stream incremental tokens. Used by long-running OCR jobs."""
        payload: dict[str, Any] = {
            "model": model,
            "prompt": prompt,
            "stream": True,
            "options": {"temperature": self._temperature, "seed": self._seed},
        }
        if images:
            payload["images"] = [base64.b64encode(img).decode("ascii") for img in images]

        async with self._http.stream("POST", "/api/generate", json=payload) as resp:
            resp.raise_for_status()
            async for line in resp.aiter_lines():
                if not line:
                    continue
                try:
                    chunk = json.loads(line)
                except json.JSONDecodeError:
                    continue
                if "response" in chunk:
                    yield str(chunk["response"])
                if chunk.get("done"):
                    return


# ---------------------------------------------------------------------------
# Sync wrapper for the few sites that don't run inside the FastAPI loop
# (e.g. the `strata doctor` smoke test).
# ---------------------------------------------------------------------------


def sync_generate(client: OllamaClient, model: str, prompt: str) -> str:
    """Blocking helper. Spins a fresh event loop — DO NOT call from inside
    one (FastAPI handlers MUST stay async)."""
    return asyncio.run(_collect(client, model, prompt))


async def _collect(client: OllamaClient, model: str, prompt: str) -> str:
    result = await client.generate(model=model, prompt=prompt)
    return result.text

"""Tests for the Ollama HTTP adapter using httpx.MockTransport.

We bypass `respx` in favour of `httpx.MockTransport`: respx's interaction
with `httpx.AsyncClient(base_url=…)` is brittle across httpx releases,
while MockTransport is the documented, version-stable test harness for
httpx clients.
"""

from __future__ import annotations

import base64
import json

import httpx
import pytest
from strata_ia.adapters.ollama import GenerateResult, OllamaClient, OllamaError, OllamaUnreachable

OLLAMA_URL = "http://test-ollama:11434"


def _client(handler, retry_attempts: int = 1) -> OllamaClient:  # type: ignore[no-untyped-def]
    transport = httpx.MockTransport(handler)
    http = httpx.AsyncClient(transport=transport, base_url=OLLAMA_URL)
    return OllamaClient(endpoint=OLLAMA_URL, retry_attempts=retry_attempts, http_client=http)


@pytest.mark.asyncio
async def test_generate_happy_path() -> None:
    captured: list[httpx.Request] = []

    def handler(request: httpx.Request) -> httpx.Response:
        captured.append(request)
        return httpx.Response(200, json={"response": "Hello from VLM", "done": True})

    async with _client(handler) as client:
        result = await client.generate(model="qwen2.5vl:7b", prompt="hi")

    assert isinstance(result, GenerateResult)
    assert result.text == "Hello from VLM"
    assert len(captured) == 1
    sent = json.loads(captured[0].content)
    assert sent["model"] == "qwen2.5vl:7b"
    assert sent["stream"] is False
    assert sent["options"]["temperature"] == 0.0
    assert sent["options"]["seed"] == 42


@pytest.mark.asyncio
async def test_generate_with_images_encodes_base64() -> None:
    captured: list[httpx.Request] = []

    def handler(request: httpx.Request) -> httpx.Response:
        captured.append(request)
        return httpx.Response(200, json={"response": "ok"})

    async with _client(handler) as client:
        await client.generate(model="m", prompt="p", images=[b"\x89PNG"])
    sent = json.loads(captured[0].content)
    assert sent["images"] == [base64.b64encode(b"\x89PNG").decode("ascii")]


@pytest.mark.asyncio
async def test_retries_on_500_then_succeeds() -> None:
    responses = iter(
        [
            httpx.Response(500, json={"error": "boom"}),
            httpx.Response(500, json={"error": "boom"}),
            httpx.Response(200, json={"response": "third time lucky"}),
        ]
    )
    call_count = {"n": 0}

    def handler(request: httpx.Request) -> httpx.Response:
        call_count["n"] += 1
        return next(responses)

    async with _client(handler, retry_attempts=3) as client:
        result = await client.generate(model="m", prompt="p")
    assert result.text == "third time lucky"
    assert call_count["n"] == 3


@pytest.mark.asyncio
async def test_raises_when_retries_exhausted() -> None:
    def handler(_: httpx.Request) -> httpx.Response:
        return httpx.Response(500, json={"error": "boom"})

    async with _client(handler, retry_attempts=2) as client:
        with pytest.raises(OllamaError):
            await client.generate(model="m", prompt="p")


@pytest.mark.asyncio
async def test_connect_error_raises_unreachable() -> None:
    def handler(_: httpx.Request) -> httpx.Response:
        raise httpx.ConnectError("nope")

    async with _client(handler, retry_attempts=1) as client:
        with pytest.raises(OllamaUnreachable):
            await client.generate(model="m", prompt="p")


@pytest.mark.asyncio
async def test_list_models() -> None:
    def handler(request: httpx.Request) -> httpx.Response:
        assert request.url.path == "/api/tags"
        return httpx.Response(
            200, json={"models": [{"name": "qwen2.5vl:7b"}, {"name": "minicpm-v:8b"}]}
        )

    async with _client(handler) as client:
        tags = await client.list_models()
    assert tags == ["qwen2.5vl:7b", "minicpm-v:8b"]


@pytest.mark.asyncio
async def test_format_json_sets_request_field() -> None:
    captured: list[httpx.Request] = []

    def handler(request: httpx.Request) -> httpx.Response:
        captured.append(request)
        return httpx.Response(200, json={"response": "{}"})

    async with _client(handler) as client:
        await client.generate(model="m", prompt="p", format_json=True)
    assert json.loads(captured[0].content)["format"] == "json"

"""FastAPI entry point for the IA microservice.

Run with::

    uvicorn strata_ia.main:app --host 0.0.0.0 --port 8081

Or, for production behind the strata-server (Plan Maestro §14)::

    python -m strata_ia.main

The gRPC channel — primary transport for the Rust bridge — will be wired
in F6.T6.2 once the ``strata_ia.v1`` proto contract lands. For now this
process speaks HTTP/REST only and the Rust side uses the HTTP fallback.
"""

from __future__ import annotations

from contextlib import asynccontextmanager
from typing import AsyncIterator

import structlog
import uvicorn
from fastapi import FastAPI

from strata_ia import __version__
from strata_ia.adapters.ollama import OllamaClient
from strata_ia.config import IaConfig, load_config
from strata_ia.routers import ocr, vlm_formula, vlm_image, vlm_table

logger = structlog.get_logger(__name__)


@asynccontextmanager
async def lifespan(app: FastAPI) -> AsyncIterator[None]:
    """Set up the singleton Ollama client and config; tear down on shutdown."""
    config = load_config()
    app.state.config = config
    app.state.ollama = OllamaClient(
        endpoint=config.ollama_endpoint,
        timeout_s=config.ollama_request_timeout_s,
        retry_attempts=config.ollama_retry_attempts,
        seed=config.seed,
        temperature=config.temperature,
    )
    logger.info(
        "strata_ia_startup",
        version=__version__,
        ollama=config.ollama_endpoint,
        port=config.http_port,
    )
    try:
        yield
    finally:
        await app.state.ollama.aclose()
        logger.info("strata_ia_shutdown")


def create_app() -> FastAPI:
    """App factory — used by Uvicorn and by the test client."""
    app = FastAPI(
        title="strata-ia",
        description="Strata-Reader IA microservice — OCR + VLM dispatch over Ollama.",
        version=__version__,
        lifespan=lifespan,
    )

    @app.get("/healthz", tags=["health"], summary="Liveness probe")
    async def healthz() -> dict[str, str]:
        """Returns ``{status: ok}`` whenever the process is up."""
        return {"status": "ok", "service": "strata-ia", "version": __version__}

    @app.get("/readyz", tags=["health"], summary="Readiness probe")
    async def readyz() -> dict[str, str]:
        """Returns ``{status: ready}`` once the Ollama endpoint is reachable.

        On failure the response is ``{status: starting}`` with HTTP 200 so
        kubelet treats the pod as still warming up rather than crashed.
        """
        try:
            await app.state.ollama.list_models()
            return {"status": "ready"}
        except Exception:  # noqa: BLE001 — readiness must never raise
            return {"status": "starting"}

    app.include_router(ocr.router)
    app.include_router(vlm_table.router)
    app.include_router(vlm_image.router)
    app.include_router(vlm_formula.router)
    return app


app = create_app()


def main() -> None:
    """Entry point for ``python -m strata_ia.main``."""
    cfg = load_config()
    uvicorn.run(
        "strata_ia.main:app",
        host=cfg.http_host,
        port=cfg.http_port,
        log_level=cfg.log_level.lower(),
    )


if __name__ == "__main__":
    main()

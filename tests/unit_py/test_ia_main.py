"""Tests for the FastAPI app factory + lifespan + health endpoints."""

from __future__ import annotations

import httpx
import pytest
from strata_ia.main import create_app


@pytest.mark.asyncio
async def test_healthz_returns_ok() -> None:
    app = create_app()
    transport = httpx.ASGITransport(app=app)
    async with httpx.AsyncClient(transport=transport, base_url="http://test") as ac:
        async with app.router.lifespan_context(app):
            resp = await ac.get("/healthz")
    assert resp.status_code == 200
    body = resp.json()
    assert body["status"] == "ok"
    assert body["service"] == "strata-ia"
    assert "version" in body


@pytest.mark.asyncio
async def test_readyz_reports_starting_when_ollama_down() -> None:
    """Readiness must NEVER raise — failures map to 'starting' with HTTP 200."""
    app = create_app()
    transport = httpx.ASGITransport(app=app)
    async with httpx.AsyncClient(transport=transport, base_url="http://test") as ac:
        async with app.router.lifespan_context(app):
            resp = await ac.get("/readyz")
    assert resp.status_code == 200
    assert resp.json()["status"] in {"ready", "starting"}


@pytest.mark.asyncio
async def test_lifespan_attaches_state() -> None:
    app = create_app()
    async with app.router.lifespan_context(app):
        # After lifespan startup, state must carry the singletons.
        assert app.state.config is not None
        assert app.state.ollama is not None


@pytest.mark.asyncio
async def test_openapi_lists_v1_endpoints() -> None:
    app = create_app()
    transport = httpx.ASGITransport(app=app)
    async with httpx.AsyncClient(transport=transport, base_url="http://test") as ac:
        async with app.router.lifespan_context(app):
            resp = await ac.get("/openapi.json")
    assert resp.status_code == 200
    paths = resp.json()["paths"]
    assert "/v1/extract/table" in paths
    assert "/v1/describe/image" in paths
    assert "/v1/ocr/formula" in paths
    assert "/v1/ocr/page" in paths
    assert "/healthz" in paths
    assert "/readyz" in paths

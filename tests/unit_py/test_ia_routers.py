"""Tests for the FastAPI routers using dependency overrides + MockTransport."""

from __future__ import annotations

import base64
import json

import httpx
import pytest
from fastapi import FastAPI
from strata_ia.adapters.ollama import OllamaClient
from strata_ia.config import IaConfig
from strata_ia.routers import ocr, vlm_formula, vlm_image, vlm_table


def _make_app(handler) -> FastAPI:  # type: ignore[no-untyped-def]
    """Construct a minimal FastAPI app with a mocked Ollama client."""
    app = FastAPI()
    transport = httpx.MockTransport(handler)
    http = httpx.AsyncClient(transport=transport, base_url="http://mock-ollama:11434")
    client = OllamaClient(endpoint="http://mock-ollama:11434", retry_attempts=1, http_client=http)
    app.state.ollama = client
    app.state.config = IaConfig(_env_file=None)  # type: ignore[call-arg]
    app.include_router(ocr.router)
    app.include_router(vlm_table.router)
    app.include_router(vlm_image.router)
    app.include_router(vlm_formula.router)
    return app


def _crop_payload() -> dict:
    return {
        "pngBytes": base64.b64encode(b"\x89PNG-fake").decode("ascii"),
        "dpi": 200,
        "pageNo": 1,
        "bbox": {"x0": 0.0, "y0": 0.0, "x1": 100.0, "y1": 50.0},
        "hint": "table-borderless",
    }


@pytest.mark.asyncio
async def test_extract_table_happy_path() -> None:
    vlm_payload = {
        "rows": [{"cells": [{"text": "a", "row": 0, "col": 0}, {"text": "b", "row": 0, "col": 1}]}],
        "confidence": 0.9,
        "cellCount": 2,
    }

    def handler(request: httpx.Request) -> httpx.Response:
        return httpx.Response(200, json={"response": json.dumps(vlm_payload)})

    app = _make_app(handler)
    async with httpx.AsyncClient(
        transport=httpx.ASGITransport(app=app), base_url="http://test"
    ) as ac:
        resp = await ac.post("/v1/extract/table", json=_crop_payload())
    assert resp.status_code == 200, resp.text
    body = resp.json()
    assert body["confidence"] == 0.9
    assert body["cellCount"] == 2
    assert body["provenance"]["backend"] == "ollama"
    assert body["provenance"]["modelId"] == "qwen2.5vl:7b"


@pytest.mark.asyncio
async def test_describe_image_happy_path() -> None:
    vlm_payload = {
        "caption": "a cat",
        "description": "A cute cat sitting on a chair.",
        "altText": "cat photo",
        "confidence": 0.85,
    }

    def handler(request: httpx.Request) -> httpx.Response:
        return httpx.Response(200, json={"response": json.dumps(vlm_payload)})

    app = _make_app(handler)
    async with httpx.AsyncClient(
        transport=httpx.ASGITransport(app=app), base_url="http://test"
    ) as ac:
        resp = await ac.post("/v1/describe/image", json=_crop_payload())
    assert resp.status_code == 200
    body = resp.json()
    assert body["caption"] == "a cat"
    assert body["altText"] == "cat photo"


@pytest.mark.asyncio
async def test_ocr_formula_happy_path() -> None:
    vlm_payload = {"latex": r"\frac{1}{2}", "mathml": None, "confidence": 0.92}

    def handler(request: httpx.Request) -> httpx.Response:
        return httpx.Response(200, json={"response": json.dumps(vlm_payload)})

    app = _make_app(handler)
    async with httpx.AsyncClient(
        transport=httpx.ASGITransport(app=app), base_url="http://test"
    ) as ac:
        resp = await ac.post("/v1/ocr/formula", json=_crop_payload())
    assert resp.status_code == 200
    body = resp.json()
    assert body["latex"] == r"\frac{1}{2}"
    assert body["mathml"] is None


@pytest.mark.asyncio
async def test_table_500_translated_to_502() -> None:
    def handler(request: httpx.Request) -> httpx.Response:
        return httpx.Response(500, json={"error": "boom"})

    app = _make_app(handler)
    async with httpx.AsyncClient(
        transport=httpx.ASGITransport(app=app), base_url="http://test"
    ) as ac:
        resp = await ac.post("/v1/extract/table", json=_crop_payload())
    assert resp.status_code == 502


@pytest.mark.asyncio
async def test_table_connect_error_translated_to_503() -> None:
    def handler(request: httpx.Request) -> httpx.Response:
        raise httpx.ConnectError("nope")

    app = _make_app(handler)
    async with httpx.AsyncClient(
        transport=httpx.ASGITransport(app=app), base_url="http://test"
    ) as ac:
        resp = await ac.post("/v1/extract/table", json=_crop_payload())
    assert resp.status_code == 503


@pytest.mark.asyncio
async def test_invalid_json_from_vlm_returns_502() -> None:
    def handler(request: httpx.Request) -> httpx.Response:
        return httpx.Response(200, json={"response": "<this is not json>"})

    app = _make_app(handler)
    async with httpx.AsyncClient(
        transport=httpx.ASGITransport(app=app), base_url="http://test"
    ) as ac:
        resp = await ac.post("/v1/extract/table", json=_crop_payload())
    assert resp.status_code == 502
    assert "malformed" in resp.json()["detail"].lower()


@pytest.mark.asyncio
async def test_ocr_page_falls_back_to_ollama_when_no_local_ocr(
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    # Force both Surya and Tesseract to report unavailable.
    from strata_ia.adapters import surya as surya_mod
    from strata_ia.adapters import tesseract as tess_mod

    monkeypatch.setattr(surya_mod, "is_available", lambda: False)
    monkeypatch.setattr(tess_mod, "is_available", lambda: False)

    vlm_payload = {"text": "Hello world", "words": [], "confidence": 0.8, "language": "en"}

    def handler(request: httpx.Request) -> httpx.Response:
        return httpx.Response(200, json={"response": json.dumps(vlm_payload)})

    app = _make_app(handler)
    async with httpx.AsyncClient(
        transport=httpx.ASGITransport(app=app), base_url="http://test"
    ) as ac:
        resp = await ac.post("/v1/ocr/page", json=_crop_payload())
    assert resp.status_code == 200
    body = resp.json()
    assert body["text"] == "Hello world"
    assert body["provenance"]["backend"] == "ollama"

"""POST /v1/ocr/page — full-page OCR.

Dispatches to Surya when available, falls back to Tesseract, and to
Ollama VLM as a last resort. The selection happens per request — if
Surya was disabled at startup we never even try.
"""

from __future__ import annotations

import time

import structlog
from fastapi import APIRouter, Depends, HTTPException, Request, status

from strata_ia.adapters import surya, tesseract
from strata_ia.adapters.ollama import OllamaClient, OllamaError, OllamaUnreachable
from strata_ia.config import IaConfig
from strata_ia.models import Crop, OcrResult, Provenance
from strata_ia.routers.prompts import OCR_PAGE_PROMPT

logger = structlog.get_logger(__name__)
router = APIRouter(prefix="/v1/ocr", tags=["ocr"])


class OcrResponse(OcrResult):
    provenance: Provenance


async def get_ollama(request: Request) -> OllamaClient:
    client: OllamaClient = request.app.state.ollama
    return client


async def get_config(request: Request) -> IaConfig:
    config: IaConfig = request.app.state.config
    return config


@router.post("/page", response_model=OcrResponse, summary="OCR a full page or large crop")
async def ocr_page(
    crop: Crop,
    ollama: OllamaClient = Depends(get_ollama),
    config: IaConfig = Depends(get_config),
) -> OcrResponse:
    png_bytes = bytes(crop.png_bytes)
    start = time.perf_counter()

    # 1) Try Surya (GPU).
    if surya.is_available():
        try:
            surya_res = surya.run_surya(png_bytes)
            latency_ms = int((time.perf_counter() - start) * 1000)
            return OcrResponse(
                text=surya_res.text,
                words=[],
                confidence=surya_res.confidence,
                language=surya_res.language,
                provenance=Provenance(model_id="surya", backend="surya", latency_ms=latency_ms),
            )
        except Exception as exc:
            logger.warning("surya_failed_falling_back", error=str(exc))

    # 2) Try Tesseract (CPU).
    if tesseract.is_available():
        try:
            tess_res = tesseract.run_tesseract(png_bytes)
            latency_ms = int((time.perf_counter() - start) * 1000)
            return OcrResponse(
                text=tess_res.text,
                words=[],
                confidence=tess_res.confidence,
                language="eng",
                provenance=Provenance(model_id="tesseract", backend="tesseract", latency_ms=latency_ms),
            )
        except Exception as exc:
            logger.warning("tesseract_failed_falling_back", error=str(exc))

    # 3) Last resort: Ollama VLM transcribes the image.
    try:
        result = await ollama.generate(
            model=config.model_ocr_fallback,
            prompt=OCR_PAGE_PROMPT,
            images=[png_bytes],
            format_json=True,
        )
    except OllamaUnreachable as exc:
        raise HTTPException(status.HTTP_503_SERVICE_UNAVAILABLE, detail=str(exc)) from exc
    except OllamaError as exc:
        raise HTTPException(status.HTTP_502_BAD_GATEWAY, detail=str(exc)) from exc

    try:
        parsed = OcrResult.model_validate_json(result.text)
    except Exception as exc:
        raise HTTPException(status.HTTP_502_BAD_GATEWAY, detail="OCR fallback returned malformed JSON") from exc

    latency_ms = int((time.perf_counter() - start) * 1000)
    return OcrResponse(
        text=parsed.text,
        words=parsed.words,
        confidence=parsed.confidence,
        language=parsed.language,
        provenance=Provenance(
            model_id=config.model_ocr_fallback,
            backend="ollama",
            latency_ms=latency_ms,
        ),
    )

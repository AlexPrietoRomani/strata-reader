"""POST /v1/ocr/page — full-page OCR.

Dispatches to Surya when available, falls back to Tesseract, and to
Ollama VLM as a last resort. The selection happens per request — if
Surya was disabled at startup we never even try.
"""

from __future__ import annotations

import time

import structlog
from fastapi import APIRouter, Depends, HTTPException, Request, status

from strata_ia.adapters.ollama import OllamaClient, OllamaError, OllamaUnreachable
from strata_ia.config import IaConfig
from strata_ia.models import Crop, OcrResult, Provenance
from strata_ia.services.ocr import run_ocr_page

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
    try:
        res = await run_ocr_page(png_bytes, ollama, config)
        return OcrResponse(
            text=res.text,
            words=res.words,
            confidence=res.confidence,
            language=res.language,
            provenance=res.provenance,
        )
    except OllamaUnreachable as exc:
        raise HTTPException(status.HTTP_503_SERVICE_UNAVAILABLE, detail=str(exc)) from exc
    except OllamaError as exc:
        raise HTTPException(status.HTTP_502_BAD_GATEWAY, detail=str(exc)) from exc
    except Exception as exc:
        raise HTTPException(status.HTTP_502_BAD_GATEWAY, detail=str(exc)) from exc

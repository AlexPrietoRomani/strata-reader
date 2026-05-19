"""POST /v1/ocr/formula — math formula → LaTeX via Ollama VLM."""

from __future__ import annotations

import time

import structlog
from fastapi import APIRouter, Depends, HTTPException, Request, status
from pydantic import ValidationError

from strata_ia.adapters.ollama import OllamaClient, OllamaError, OllamaUnreachable
from strata_ia.config import IaConfig
from strata_ia.models import Crop, FormulaResult, Provenance
from strata_ia.routers.prompts import OCR_FORMULA_PROMPT

logger = structlog.get_logger(__name__)
router = APIRouter(prefix="/v1/ocr", tags=["formula"])


class FormulaResponse(FormulaResult):
    provenance: Provenance


async def get_ollama(request: Request) -> OllamaClient:
    return request.app.state.ollama


async def get_config(request: Request) -> IaConfig:
    return request.app.state.config


@router.post("/formula", response_model=FormulaResponse, summary="OCR a math formula to LaTeX")
async def ocr_formula(
    crop: Crop,
    ollama: OllamaClient = Depends(get_ollama),
    config: IaConfig = Depends(get_config),
) -> FormulaResponse:
    start = time.perf_counter()
    try:
        result = await ollama.generate(
            model=config.model_formula,
            prompt=OCR_FORMULA_PROMPT,
            images=[bytes(crop.png_bytes)],
            format_json=True,
        )
    except OllamaUnreachable as exc:
        raise HTTPException(status.HTTP_503_SERVICE_UNAVAILABLE, detail=str(exc)) from exc
    except OllamaError as exc:
        raise HTTPException(status.HTTP_502_BAD_GATEWAY, detail=str(exc)) from exc

    try:
        parsed = FormulaResult.model_validate_json(result.text)
    except ValidationError as exc:
        logger.warning("vlm_formula_invalid_json", error=exc.errors())
        raise HTTPException(status.HTTP_502_BAD_GATEWAY, detail="VLM returned malformed JSON") from exc

    latency_ms = int((time.perf_counter() - start) * 1000)
    return FormulaResponse(
        latex=parsed.latex,
        mathml=parsed.mathml,
        confidence=parsed.confidence,
        provenance=Provenance(
            model_id=config.model_formula,
            backend="ollama",
            latency_ms=latency_ms,
        ),
    )

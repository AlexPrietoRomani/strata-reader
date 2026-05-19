"""POST /v1/extract/table — borderless table extraction via Ollama VLM."""

from __future__ import annotations

import json
import time

import structlog
from fastapi import APIRouter, Depends, HTTPException, Request, status
from pydantic import ValidationError

from strata_ia.adapters.ollama import OllamaClient, OllamaError, OllamaUnreachable
from strata_ia.config import IaConfig
from strata_ia.models import Crop, Provenance, TableResult
from strata_ia.routers.prompts import EXTRACT_TABLE_PROMPT

logger = structlog.get_logger(__name__)
router = APIRouter(prefix="/v1/extract", tags=["table"])


class TableResponse(TableResult):
    """Table result enveloped with provenance for end-to-end tracing."""

    provenance: Provenance


async def get_ollama(request: Request) -> OllamaClient:
    """Dependency: pull the singleton Ollama client out of app state."""
    return request.app.state.ollama


async def get_config(request: Request) -> IaConfig:
    return request.app.state.config


@router.post("/table", response_model=TableResponse, summary="Extract a borderless table")
async def extract_table(
    crop: Crop,
    ollama: OllamaClient = Depends(get_ollama),
    config: IaConfig = Depends(get_config),
) -> TableResponse:
    start = time.perf_counter()
    try:
        result = await ollama.generate(
            model=config.model_table,
            prompt=EXTRACT_TABLE_PROMPT,
            images=[bytes(crop.png_bytes)],
            format_json=True,
        )
    except OllamaUnreachable as exc:
        raise HTTPException(status.HTTP_503_SERVICE_UNAVAILABLE, detail=str(exc)) from exc
    except OllamaError as exc:
        raise HTTPException(status.HTTP_502_BAD_GATEWAY, detail=str(exc)) from exc

    try:
        parsed = TableResult.model_validate_json(result.text)
    except ValidationError as exc:
        logger.warning("vlm_table_invalid_json", error=exc.errors())
        raise HTTPException(status.HTTP_502_BAD_GATEWAY, detail="VLM returned malformed JSON") from exc
    except json.JSONDecodeError as exc:
        raise HTTPException(status.HTTP_502_BAD_GATEWAY, detail="VLM returned malformed JSON") from exc

    latency_ms = int((time.perf_counter() - start) * 1000)
    return TableResponse(
        rows=parsed.rows,
        confidence=parsed.confidence,
        cell_count=parsed.cell_count,
        provenance=Provenance(
            model_id=config.model_table,
            backend="ollama",
            latency_ms=latency_ms,
            retries=0,
            cache_hit=False,
        ),
    )

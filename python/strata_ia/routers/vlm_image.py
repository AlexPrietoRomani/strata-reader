"""POST /v1/describe/image — figure / chart description via Ollama VLM."""

from __future__ import annotations

import time

import structlog
from fastapi import APIRouter, Depends, HTTPException, Request, status
from pydantic import ValidationError

from strata_ia.adapters.ollama import OllamaClient, OllamaError, OllamaUnreachable
from strata_ia.config import IaConfig
from strata_ia.models import Crop, ImageDescription, Provenance
from strata_ia.routers.prompts import DESCRIBE_IMAGE_PROMPT

logger = structlog.get_logger(__name__)
router = APIRouter(prefix="/v1/describe", tags=["image"])


class ImageDescriptionResponse(ImageDescription):
    provenance: Provenance


async def get_ollama(request: Request) -> OllamaClient:
    client: OllamaClient = request.app.state.ollama
    return client


async def get_config(request: Request) -> IaConfig:
    config: IaConfig = request.app.state.config
    return config


@router.post(
    "/image", response_model=ImageDescriptionResponse, summary="Describe an embedded image"
)
async def describe_image(
    crop: Crop,
    ollama: OllamaClient = Depends(get_ollama),
    config: IaConfig = Depends(get_config),
) -> ImageDescriptionResponse:
    start = time.perf_counter()
    try:
        result = await ollama.generate(
            model=config.model_image,
            prompt=DESCRIBE_IMAGE_PROMPT,
            images=[bytes(crop.png_bytes)],
            format_json=True,
        )
    except OllamaUnreachable as exc:
        raise HTTPException(status.HTTP_503_SERVICE_UNAVAILABLE, detail=str(exc)) from exc
    except OllamaError as exc:
        raise HTTPException(status.HTTP_502_BAD_GATEWAY, detail=str(exc)) from exc

    try:
        parsed = ImageDescription.model_validate_json(result.text)
    except ValidationError as exc:
        logger.warning("vlm_image_invalid_json", error=exc.errors())
        raise HTTPException(
            status.HTTP_502_BAD_GATEWAY, detail="VLM returned malformed JSON"
        ) from exc

    latency_ms = int((time.perf_counter() - start) * 1000)
    return ImageDescriptionResponse(
        caption=parsed.caption,
        description=parsed.description,
        alt_text=parsed.alt_text,
        confidence=parsed.confidence,
        provenance=Provenance(
            model_id=config.model_image,
            backend="ollama",
            latency_ms=latency_ms,
        ),
    )

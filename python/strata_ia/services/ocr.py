"""
Archivo: ocr.py
Fecha de modificación: 30/06/2026
Autor: Antigravity

Descripción:
Servicio unificado para ejecutar OCR en páginas de PDF o recortes de imagen.
Aplica una estrategia de fallback secuencial: Surya -> Tesseract -> Ollama VLM.

Acciones Principales:
    - `run_ocr_page`: Ejecuta OCR en los bytes de una imagen PNG recibida.

Estructura Interna:
    - `run_ocr_page`: Función asíncrona de orquestación de OCR.

Entradas / Dependencias:
    - `png_bytes` (bytes)
    - `ollama` (OllamaClient)
    - `config` (IaConfig)

Ejemplo de Integración:
    from strata_ia.services.ocr import run_ocr_page
    res = await run_ocr_page(png_bytes, ollama, config)
"""

from __future__ import annotations

import time
import structlog

from strata_ia.adapters import surya, tesseract
from strata_ia.adapters.ollama import OllamaClient, OllamaError, OllamaUnreachable
from strata_ia.config import IaConfig
from strata_ia.models import OcrResult, Provenance
from strata_ia.routers.prompts import OCR_PAGE_PROMPT

logger = structlog.get_logger(__name__)


class OcrServiceResponse(OcrResult):
    """Representa la respuesta completa de OCR con información de trazabilidad."""
    provenance: Provenance


async def run_ocr_page(
    png_bytes: bytes,
    ollama: OllamaClient,
    config: IaConfig,
) -> OcrServiceResponse:
    """
    Ejecuta el pipeline de OCR secuencial sobre los bytes de una imagen PNG.

    Prueba primero Surya (GPU), luego Tesseract (CPU) y finalmente Ollama VLM.

    Args:
        png_bytes (bytes): Los bytes de la imagen PNG a procesar.
        ollama (OllamaClient): Cliente de Ollama para fallback.
        config (IaConfig): Configuración del servicio IA.

    Returns:
        OcrServiceResponse: El resultado del OCR conteniendo el texto extraído y provenance.

    Raises:
        OllamaUnreachable: Si Ollama no está disponible en la red local.
        OllamaError: Si Ollama retorna una respuesta de error inesperada.
        Exception: Si falla el procesamiento del JSON de fallback.
    """
    start = time.perf_counter()

    # 1) Intentar Surya (GPU).
    if surya.is_available():
        try:
            surya_res = surya.run_surya(png_bytes)
            latency_ms = int((time.perf_counter() - start) * 1000)
            return OcrServiceResponse(
                text=surya_res.text,
                words=[],
                confidence=surya_res.confidence,
                language=surya_res.language,
                provenance=Provenance(model_id="surya", backend="surya", latency_ms=latency_ms),
            )
        except Exception as exc:
            # Capturamos la excepción para degradar graciosamente al siguiente backend
            logger.warning("surya_failed_falling_back", error=str(exc))

    # 2) Intentar Tesseract (CPU).
    if tesseract.is_available():
        try:
            tess_res = tesseract.run_tesseract(png_bytes)
            latency_ms = int((time.perf_counter() - start) * 1000)
            return OcrServiceResponse(
                text=tess_res.text,
                words=[],
                confidence=tess_res.confidence,
                language="eng",
                provenance=Provenance(
                    model_id="tesseract", backend="tesseract", latency_ms=latency_ms
                ),
            )
        except Exception as exc:
            # Capturamos la excepción para degradar graciosamente al Ollama VLM
            logger.warning("tesseract_failed_falling_back", error=str(exc))

    # 3) Último recurso: Ollama VLM.
    result = await ollama.generate(
        model=config.model_ocr_fallback,
        prompt=OCR_PAGE_PROMPT,
        images=[png_bytes],
        format_json=True,
    )

    parsed = OcrResult.model_validate_json(result.text)
    latency_ms = int((time.perf_counter() - start) * 1000)

    return OcrServiceResponse(
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

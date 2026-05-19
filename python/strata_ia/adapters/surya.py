"""Surya-OCR adapter — primary GPU OCR backend.

Surya is a GPU-friendly OCR + layout model. We load it lazily so the
process can boot without consuming the ~2 GB of VRAM the model needs.

The dep itself is **optional** (``[project.optional-dependencies.ocr]``)
since it pulls in PyTorch + CUDA. Hosts without Surya fall back to
Tesseract via [`strata_ia.adapters.tesseract`].

This module exposes a thin singleton wrapper so multiple FastAPI workers
sharing the same process don't duplicate the model in VRAM.
"""

from __future__ import annotations

import io
import threading
from dataclasses import dataclass
from typing import Any

import structlog

logger = structlog.get_logger(__name__)


@dataclass(frozen=True, slots=True)
class SuryaResult:
    text: str
    confidence: float
    language: str | None


class SuryaUnavailable(RuntimeError):
    """Raised when Surya is not importable or model load failed."""


_lock = threading.Lock()
_loaded: Any | None = None
_load_error: str | None = None


def _load_models() -> Any:
    """Load Surya's detection and recognition models. Idempotent."""
    global _loaded, _load_error
    if _loaded is not None:
        return _loaded
    with _lock:
        if _loaded is not None:
            return _loaded
        try:
            # Imports are intentionally inside the function — package may be
            # absent on hosts without GPU.
            from PIL import Image  # noqa: F401 — used implicitly to validate dep
            from surya.detection import DetectionPredictor  # type: ignore[import-not-found]
            from surya.recognition import RecognitionPredictor  # type: ignore[import-not-found]

            detection = DetectionPredictor()
            recognition = RecognitionPredictor()
            _loaded = (detection, recognition)
            return _loaded
        except Exception as exc:
            _load_error = str(exc)
            logger.warning("surya_load_failed", error=_load_error)
            raise SuryaUnavailable(_load_error) from exc


def is_available() -> bool:
    """Probe without raising. Returns True iff Surya can be loaded."""
    try:
        _load_models()
        return True
    except SuryaUnavailable:
        return False


def run_surya(png_bytes: bytes, languages: list[str] | None = None) -> SuryaResult:
    """Run Surya on ``png_bytes`` and return text + average confidence."""
    detection, recognition = _load_models()
    from PIL import Image

    img = Image.open(io.BytesIO(png_bytes))
    langs = languages or ["en"]

    predictions = recognition([img], [langs], det_predictor=detection)
    if not predictions:
        return SuryaResult(text="", confidence=0.0, language=langs[0] if langs else None)

    pred = predictions[0]
    text_lines = [line.text for line in getattr(pred, "text_lines", [])]
    confs = [getattr(line, "confidence", 0.0) for line in getattr(pred, "text_lines", [])]
    text = "\n".join(text_lines).strip()
    confidence = sum(confs) / len(confs) if confs else 0.0
    return SuryaResult(text=text, confidence=float(confidence), language=langs[0])

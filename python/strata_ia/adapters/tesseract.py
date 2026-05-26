"""Tesseract OCR adapter — CPU fallback.

Plan Maestro §10.T5.3 calls Tesseract the "fallback when GPU is unavailable".
We keep the dep optional (``[project.optional-dependencies.ocr]``) so the
base wheel stays slim; the adapter raises a clear error at runtime when
the dep is missing rather than at import time.
"""

from __future__ import annotations

import io
from dataclasses import dataclass

import structlog

logger = structlog.get_logger(__name__)


@dataclass(frozen=True, slots=True)
class TesseractResult:
    text: str
    confidence: float
    words: list[tuple[str, tuple[int, int, int, int], float]]
    """Tuples of (word, (x, y, w, h) in pixels, confidence)."""


class TesseractUnavailable(RuntimeError):
    """Raised when the runtime can't load pytesseract or the system binary."""


def _try_import() -> tuple[object, object]:
    """Return (pytesseract module, PIL.Image module) or raise."""
    try:
        import pytesseract
        from PIL import Image
    except ImportError as exc:  # pragma: no cover - depends on host
        raise TesseractUnavailable(
            "pytesseract / Pillow not installed. "
            "Install with `pip install strata-reader[ocr]` and ensure the "
            "system `tesseract` binary is on PATH."
        ) from exc
    return pytesseract, Image


def is_available() -> bool:
    """Probe pytesseract + system binary presence without raising."""
    try:
        pytesseract, _ = _try_import()
        # Touch the binary; raises TesseractNotFoundError when missing.
        version = pytesseract.get_tesseract_version()  # type: ignore[attr-defined]
        return version is not None
    except Exception as exc:
        logger.debug("tesseract_unavailable", error=str(exc))
        return False


def run_tesseract(png_bytes: bytes, lang: str = "eng") -> TesseractResult:
    """OCR ``png_bytes`` and return text + per-word boxes."""
    pytesseract, Image = _try_import()
    img = Image.open(io.BytesIO(png_bytes))  # type: ignore[attr-defined]
    text = pytesseract.image_to_string(img, lang=lang).strip()  # type: ignore[attr-defined]

    data = pytesseract.image_to_data(  # type: ignore[attr-defined]
        img,
        lang=lang,
        output_type=pytesseract.Output.DICT,  # type: ignore[attr-defined]
    )
    words: list[tuple[str, tuple[int, int, int, int], float]] = []
    confidences: list[float] = []
    for i in range(len(data.get("text", []))):
        t = data["text"][i].strip()
        if not t:
            continue
        conf_raw = data["conf"][i]
        try:
            conf = float(conf_raw)
        except (TypeError, ValueError):
            continue
        if conf < 0:
            continue
        # Tesseract returns confidence in [0, 100].
        conf01 = conf / 100.0
        box = (
            int(data["left"][i]),
            int(data["top"][i]),
            int(data["width"][i]),
            int(data["height"][i]),
        )
        words.append((t, box, conf01))
        confidences.append(conf01)

    overall = sum(confidences) / len(confidences) if confidences else 0.0
    return TesseractResult(text=text, confidence=overall, words=words)

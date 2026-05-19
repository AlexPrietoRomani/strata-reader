"""Pydantic request / response models for every IA endpoint.

These models will be mirrored by the gRPC ``.proto`` contract once Fase 6
lands (T6.1). Keeping them as Pydantic types now means the HTTP / FastAPI
surface is fully typed and self-documenting, and the proto generation can
be derived programmatically later.

Conventions (Plan Maestro §1):
- camelCase aliases on the wire (JSON-friendly for the Rust bridge).
- ``confidence ∈ [0.0, 1.0]`` enforced via constrained float fields.
- Bytes use Pydantic's ``Base64Bytes`` so HTTP/JSON payloads stay portable.
"""

from __future__ import annotations

from pydantic import BaseModel, ConfigDict, Field
from pydantic.types import Base64Bytes


def _model_config() -> ConfigDict:
    return ConfigDict(populate_by_name=True, str_strip_whitespace=True)


# ---------------------------------------------------------------------------
# Shared geometry
# ---------------------------------------------------------------------------


class BBox(BaseModel):
    """Axis-aligned rectangle in PDF user space (1/72 inch, origin bottom-left)."""

    model_config = _model_config()

    x0: float
    y0: float
    x1: float
    y1: float


# ---------------------------------------------------------------------------
# Request payloads
# ---------------------------------------------------------------------------


class Crop(BaseModel):
    """One image crop that the Triage Engine has decided to escalate to IA.

    The bytes are *always* PNG (see ``strata_pdf::extract_images``). DPI is
    the rendering resolution used by ``render_crop`` — useful when the
    backend (Surya, VLM) wants to know the original pixel density.
    """

    model_config = _model_config()

    png_bytes: Base64Bytes = Field(alias="pngBytes")
    dpi: int = Field(default=200, ge=72, le=1200)
    page_no: int = Field(alias="pageNo", ge=1)
    bbox: BBox
    # Free-form hint that the Rust triage emits (e.g. ``"table-borderless"``,
    # ``"figure"``, ``"formula"``). The IA layer can use it to short-circuit
    # the prompt template.
    hint: str = ""


# ---------------------------------------------------------------------------
# Response payloads — one per Triage route
# ---------------------------------------------------------------------------


class WordBox(BaseModel):
    model_config = _model_config()

    text: str
    bbox: BBox
    confidence: float = Field(ge=0.0, le=1.0)


class OcrResult(BaseModel):
    """Native-language transcription of a crop. Used by ``OcrPage`` and
    ``OcrFormula`` (the latter returns LaTeX as plain text)."""

    model_config = _model_config()

    text: str
    words: list[WordBox] = Field(default_factory=list)
    confidence: float = Field(ge=0.0, le=1.0)
    language: str | None = None
    """ISO 639-1 / -3 code if the backend can detect it."""


class TableCell(BaseModel):
    model_config = _model_config()

    text: str
    row: int = Field(ge=0)
    col: int = Field(ge=0)
    row_span: int = Field(default=1, ge=1, alias="rowSpan")
    col_span: int = Field(default=1, ge=1, alias="colSpan")


class TableRow(BaseModel):
    model_config = _model_config()

    cells: list[TableCell]


class TableResult(BaseModel):
    """Output of ``ExtractTable``. Cells carry explicit ``row``/``col`` so
    callers can rebuild the grid even when rows are returned out of order."""

    model_config = _model_config()

    rows: list[TableRow] = Field(default_factory=list)
    confidence: float = Field(ge=0.0, le=1.0)
    cell_count: int = Field(default=0, ge=0, alias="cellCount")


class ImageDescription(BaseModel):
    """Output of ``DescribeImage``. A short caption + optional longer
    description + an ALT-text suitable for embedding in Markdown."""

    model_config = _model_config()

    caption: str
    description: str = ""
    alt_text: str = Field(default="", alias="altText")
    confidence: float = Field(ge=0.0, le=1.0)


class FormulaResult(BaseModel):
    """Output of ``OcrFormula``. ``latex`` is the canonical form; ``mathml``
    is optional and only populated when the backend supports it."""

    model_config = _model_config()

    latex: str
    mathml: str | None = None
    confidence: float = Field(ge=0.0, le=1.0)


# ---------------------------------------------------------------------------
# Provenance / metrics envelope returned with every response
# ---------------------------------------------------------------------------


class Provenance(BaseModel):
    """Metadata the Rust side embeds in the resulting ``Block.provenance``
    field for PRISMA traceability (Plan Maestro §1, §10.T5.6)."""

    model_config = _model_config()

    model_id: str = Field(alias="modelId")
    backend: str
    """Free-form backend tag: ``"ollama"``, ``"surya"``, ``"tesseract"``, …."""
    latency_ms: int = Field(ge=0, alias="latencyMs")
    retries: int = Field(default=0, ge=0)
    cache_hit: bool = Field(default=False, alias="cacheHit")

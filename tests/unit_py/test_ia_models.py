"""Pydantic model round-trip tests for the IA microservice."""

from __future__ import annotations

import base64

import pytest
from pydantic import ValidationError

from strata_ia.models import (
    BBox,
    Crop,
    FormulaResult,
    ImageDescription,
    OcrResult,
    Provenance,
    TableCell,
    TableResult,
    TableRow,
    WordBox,
)


def _crop() -> Crop:
    return Crop.model_validate(
        {
            "pngBytes": base64.b64encode(b"\x89PNG\r\n\x1a\n"),
            "dpi": 200,
            "pageNo": 1,
            "bbox": {"x0": 0.0, "y0": 0.0, "x1": 10.0, "y1": 10.0},
            "hint": "table-borderless",
        }
    )


def test_crop_round_trips_with_aliases() -> None:
    crop = _crop()
    payload = crop.model_dump(by_alias=True)
    back = Crop.model_validate(payload)
    assert back == crop


def test_crop_rejects_dpi_out_of_range() -> None:
    with pytest.raises(ValidationError):
        Crop.model_validate(
            {
                "pngBytes": base64.b64encode(b"x"),
                "dpi": 30,
                "pageNo": 1,
                "bbox": {"x0": 0.0, "y0": 0.0, "x1": 1.0, "y1": 1.0},
            }
        )


def test_ocr_result_rejects_confidence_above_one() -> None:
    with pytest.raises(ValidationError):
        OcrResult.model_validate({"text": "x", "confidence": 1.5})


def test_table_result_round_trip() -> None:
    r = TableResult(
        rows=[
            TableRow(
                cells=[
                    TableCell(text="a", row=0, col=0),
                    TableCell(text="b", row=0, col=1, col_span=2),
                ]
            )
        ],
        confidence=0.9,
        cell_count=2,
    )
    j = r.model_dump_json(by_alias=True)
    assert '"cellCount":2' in j
    assert '"colSpan":2' in j


def test_image_description_alt_text_aliased() -> None:
    img = ImageDescription(caption="cat", description="A cute cat", alt_text="cat photo", confidence=0.85)
    j = img.model_dump_json(by_alias=True)
    assert '"altText":"cat photo"' in j


def test_formula_result_accepts_mathml_null() -> None:
    f = FormulaResult(latex=r"\frac{1}{2}", confidence=0.95)
    assert f.mathml is None


def test_provenance_round_trip() -> None:
    p = Provenance(model_id="qwen2.5vl:7b", backend="ollama", latency_ms=230, retries=1, cache_hit=False)
    payload = p.model_dump(by_alias=True)
    assert payload["modelId"] == "qwen2.5vl:7b"
    assert payload["cacheHit"] is False
    back = Provenance.model_validate(payload)
    assert back == p


def test_wordbox_constructs_with_required_fields() -> None:
    w = WordBox(text="hello", bbox=BBox(x0=0, y0=0, x1=10, y1=10), confidence=1.0)
    assert w.confidence == 1.0

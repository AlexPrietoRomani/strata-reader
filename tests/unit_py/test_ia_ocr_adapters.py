"""Tests for the OCR adapters.

Both Surya and Tesseract are *optional* deps — without them the adapters
must fail gracefully with typed errors, never crash the process at import.
"""

from __future__ import annotations

import pytest

from strata_ia.adapters import surya, tesseract
from strata_ia.adapters.tesseract import TesseractUnavailable


def test_tesseract_is_available_returns_bool() -> None:
    # Either the system has tesseract installed or it doesn't — both are valid.
    result = tesseract.is_available()
    assert isinstance(result, bool)


def test_run_tesseract_raises_when_dep_missing(monkeypatch: pytest.MonkeyPatch) -> None:
    def fake_import() -> None:
        raise TesseractUnavailable("simulated absence")

    monkeypatch.setattr(tesseract, "_try_import", fake_import)
    with pytest.raises(TesseractUnavailable):
        tesseract.run_tesseract(b"\x89PNG", lang="eng")


def test_surya_is_available_returns_bool() -> None:
    result = surya.is_available()
    assert isinstance(result, bool)


def test_surya_unavailable_class_is_runtime_error() -> None:
    err = surya.SuryaUnavailable("no GPU")
    assert isinstance(err, RuntimeError)
    assert "no GPU" in str(err)


def test_tesseract_result_dataclass_immutable() -> None:
    from strata_ia.adapters.tesseract import TesseractResult

    r = TesseractResult(text="hi", confidence=0.9, words=[])
    with pytest.raises(Exception):
        r.text = "changed"  # type: ignore[misc] — frozen dataclass

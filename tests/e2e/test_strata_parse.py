"""End-to-end tests for `strata parse` against the 8 golden fixtures.

Plan Maestro §15.T10.1 — each PDF is processed via the real CLI with a
live Ollama backend, the resulting Markdown + JSON are compared against
the committed goldens (modulo timestamps).

Every test depends on ``require_ollama`` and ``strata_bin``; both skip
cleanly when the prerequisite is missing. Run on a host with Ollama up
plus a built binary with::

    cargo build -p strata-cli --release
    export PATH="$PWD/target/release:$PATH"
    uv run python -m pytest tests/e2e -m "ollama" -v
"""

from __future__ import annotations

import json
import subprocess
from pathlib import Path

import pytest

from tests.e2e.conftest import expected_for, fixture_paths

# Mark the whole module so users can opt-in/out with `-m "ollama"`.
pytestmark = pytest.mark.ollama


def _strip_volatile(payload: dict) -> dict:
    """Remove fields that legitimately differ between runs so JSON
    comparisons stay deterministic."""
    if "meta" in payload and isinstance(payload["meta"], dict):
        for key in ("generated_at", "elapsed_ms", "host"):
            payload["meta"].pop(key, None)
    return payload


@pytest.mark.parametrize("pdf", fixture_paths(), ids=lambda p: p.name)
def test_parse_matches_golden_json(
    require_ollama: None,
    strata_bin: str,
    pdf: Path,
    tmp_path: Path,
    expected_dir: Path,
) -> None:
    """Run `strata parse` and diff the JSON output against the committed golden."""
    golden = expected_for(pdf, ".json")
    if not golden.exists():
        pytest.skip(f"no golden yet for {pdf.name} — run scripts/regen_goldens.py")

    out = tmp_path / "out"
    out.mkdir()
    result = subprocess.run(
        [
            strata_bin,
            "parse",
            "--input",
            str(pdf),
            "--output",
            str(out),
            "--format",
            "json",
            "--profile",
            "scientific",
        ],
        capture_output=True,
        text=True,
        timeout=300,
    )
    assert result.returncode == 0, f"strata parse failed: {result.stderr}"

    actual_path = out / f"{pdf.stem}.json"
    assert actual_path.exists(), f"strata did not produce {actual_path}"

    actual = _strip_volatile(json.loads(actual_path.read_text(encoding="utf-8")))
    expected = _strip_volatile(json.loads(golden.read_text(encoding="utf-8")))
    assert actual == expected, f"output drifted from golden for {pdf.name}"


@pytest.mark.parametrize("pdf", fixture_paths(), ids=lambda p: p.name)
def test_parse_matches_golden_md(
    require_ollama: None,
    strata_bin: str,
    pdf: Path,
    tmp_path: Path,
    expected_dir: Path,
) -> None:
    """Same flow but for the Markdown output."""
    golden = expected_for(pdf, ".md")
    if not golden.exists():
        pytest.skip(f"no .md golden for {pdf.name}")

    out = tmp_path / "out"
    out.mkdir()
    subprocess.run(
        [
            strata_bin,
            "parse",
            "--input",
            str(pdf),
            "--output",
            str(out),
            "--format",
            "md",
            "--profile",
            "scientific",
        ],
        check=True,
        capture_output=True,
        text=True,
        timeout=300,
    )
    actual = (out / f"{pdf.stem}.md").read_text(encoding="utf-8")
    expected = golden.read_text(encoding="utf-8")
    assert actual.strip() == expected.strip(), f"Markdown drift for {pdf.name}"


def test_no_ia_mode_runs_without_ollama(strata_bin: str, tmp_path: Path) -> None:
    """`--no-ia` (T9.5) must succeed even with Ollama down — exercises the
    pure-native path. We pick `native_simple.pdf` because it has no
    tables / images / formulas that would force IA escalation."""
    pdfs = fixture_paths()
    native = next((p for p in pdfs if p.name == "native_simple.pdf"), None)
    if native is None:
        pytest.skip("native_simple.pdf fixture missing")
    out = tmp_path / "out"
    out.mkdir()
    result = subprocess.run(
        [strata_bin, "parse", "--input", str(native), "--output", str(out), "--no-ia"],
        capture_output=True,
        text=True,
        timeout=60,
    )
    assert result.returncode == 0, f"no-ia parse failed: {result.stderr}"

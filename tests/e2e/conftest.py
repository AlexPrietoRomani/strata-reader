"""Shared fixtures for the E2E suite.

The whole tree is gated behind the ``@pytest.mark.ollama`` marker plus
runtime probes — calling the marker doesn't yet *guarantee* an Ollama
instance is reachable. ``require_ollama`` does. Similarly ``strata_bin``
yields the path to a built ``strata`` CLI binary or skips the test.
"""

from __future__ import annotations

import os
import shutil
import socket
from pathlib import Path

import httpx
import pytest

REPO_ROOT = Path(__file__).resolve().parents[2]
FIXTURES_PDFS = REPO_ROOT / "tests" / "fixtures" / "pdfs"
EXPECTED_DIR = REPO_ROOT / "tests" / "fixtures" / "expected"


@pytest.fixture(scope="session")
def repo_root() -> Path:
    return REPO_ROOT


@pytest.fixture(scope="session")
def fixtures_dir() -> Path:
    return FIXTURES_PDFS


@pytest.fixture(scope="session")
def expected_dir() -> Path:
    return EXPECTED_DIR


@pytest.fixture(scope="session")
def ollama_endpoint() -> str:
    return os.environ.get("STRATA_OLLAMA_URL", "http://localhost:11434")


@pytest.fixture
def require_ollama(ollama_endpoint: str) -> None:
    """Skip the test when Ollama is unreachable on the configured endpoint."""
    try:
        resp = httpx.get(f"{ollama_endpoint}/api/tags", timeout=2.0)
        if resp.status_code != 200:
            pytest.skip(f"Ollama returned {resp.status_code} at {ollama_endpoint}")
    except (httpx.HTTPError, socket.gaierror) as exc:
        pytest.skip(f"Ollama unreachable at {ollama_endpoint}: {exc}")


@pytest.fixture(scope="session")
def strata_bin() -> str:
    """Resolve the `strata` CLI binary. Skips when not on PATH."""
    bin_path = shutil.which("strata")
    if bin_path is None:
        # Try the dev target directory.
        target = os.environ.get("CARGO_TARGET_DIR")
        if target:
            candidate = Path(target) / "release" / ("strata.exe" if os.name == "nt" else "strata")
            if candidate.exists():
                return str(candidate)
        pytest.skip("`strata` binary not on PATH; build with `cargo build -p strata-cli --release`")
    return bin_path


def fixture_paths() -> list[Path]:
    """Return every golden PDF under tests/fixtures/pdfs/."""
    if not FIXTURES_PDFS.exists():
        return []
    return sorted(p for p in FIXTURES_PDFS.glob("*.pdf"))


def expected_for(pdf: Path, suffix: str) -> Path:
    """`two_column_paper.pdf` + `.golden.json` → expected dir path."""
    return EXPECTED_DIR / f"{pdf.stem}.golden{suffix}"

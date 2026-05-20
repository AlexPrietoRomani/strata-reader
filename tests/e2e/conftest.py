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
import subprocess
from pathlib import Path

import httpx
import pytest

REPO_ROOT = Path(__file__).resolve().parents[2]
FIXTURES_PDFS = REPO_ROOT / "tests" / "fixtures" / "pdfs"
EXPECTED_DIR = REPO_ROOT / "tests" / "fixtures" / "expected"

# Tiempo máximo (segundos) para el escaneo EDR corporativo en primer arranque.
_EDR_WARMUP_TIMEOUT_S: int = 120


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
    """
    Resuelve el binario CLI `strata` y lo precalienta para el EDR corporativo.

    En entornos con AppLocker/antivirus (e.g. EMPRESA), el EDR escanea cada
    binario nuevo durante ~50s en su primera ejecución. Al ejecutar
    `strata --version` aquí (scope=session), el escaneo ocurre UNA SOLA VEZ
    al inicio de la sesión pytest, antes de que los tests individuales
    invoquen `strata parse` con timeouts más ajustados.

    Returns:
        str: Ruta absoluta al binario `strata` listo para ser invocado.

    Raises:
        pytest.skip.Exception: Si el binario no se encuentra en ninguna
            ubicación conocida o si el EDR no lo aprueba en tiempo.
    """
    bin_path = shutil.which("strata")
    if bin_path is None:
        # Fallback al directorio target/ de Cargo cuando strata no está en PATH.
        target = os.environ.get("CARGO_TARGET_DIR")
        if target:
            candidate = Path(target) / "release" / ("strata.exe" if os.name == "nt" else "strata")
            if candidate.exists():
                bin_path = str(candidate)
    if bin_path is None:
        pytest.skip("`strata` binary not on PATH; build with `cargo build -p strata-cli --release`")

    # Warmup EDR: ejecutar strata --version con timeout largo para que el
    # antivirus corporativo escanee el binario ANTES de los tests con timeouts
    # cortos. La segunda ejecución es instantánea (cache del EDR).
    # Ver AGENTS.md §2 y docs/usage/IT_request.md.
    try:
        subprocess.run(
            [bin_path, "--version"],
            capture_output=True,
            text=True,
            timeout=_EDR_WARMUP_TIMEOUT_S,
            check=False,
        )
    except subprocess.TimeoutExpired:
        pytest.skip(
            f"strata binary timed out during EDR warmup (>{_EDR_WARMUP_TIMEOUT_S}s): {bin_path}"
        )

    return bin_path


def fixture_paths() -> list[Path]:
    """Return every golden PDF under tests/fixtures/pdfs/."""
    if not FIXTURES_PDFS.exists():
        return []
    return sorted(p for p in FIXTURES_PDFS.glob("*.pdf"))


def expected_for(pdf: Path, suffix: str) -> Path:
    """`two_column_paper.pdf` + `.golden.json` → expected dir path."""
    return EXPECTED_DIR / f"{pdf.stem}.golden{suffix}"

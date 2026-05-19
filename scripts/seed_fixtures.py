"""Reproducible fixture management for strata-reader.

Usage::

    uv run python scripts/seed_fixtures.py --verify
    uv run python scripts/seed_fixtures.py --download
    uv run python scripts/seed_fixtures.py --build
    uv run python scripts/seed_fixtures.py --regen-expected

See ``docs/task/tareas.md`` T0.7 and ``tests/fixtures/README.md`` for the
provenance contract.
"""

from __future__ import annotations

import argparse
import hashlib
import logging
import shutil
import subprocess
import sys
from collections.abc import Iterable
from dataclasses import dataclass
from pathlib import Path

import httpx

ROOT = Path(__file__).resolve().parents[1]
PDF_DIR = ROOT / "tests" / "fixtures" / "pdfs"
SOURCE_DIR = ROOT / "tests" / "fixtures" / "sources"
EXPECTED_DIR = ROOT / "tests" / "fixtures" / "expected"
CHECKSUM_FILE = PDF_DIR / "CHECKSUMS.sha256"

logger = logging.getLogger("seed_fixtures")


@dataclass(frozen=True)
class RemoteFixture:
    name: str
    url: str
    sha256: str


REMOTE_FIXTURES: tuple[RemoteFixture, ...] = (
    RemoteFixture(
        name="two_column_paper.pdf",
        url="https://arxiv.org/pdf/1706.03762",
        sha256="bdfaa68d8984f0dc02beaca527b76f207d99b666d31d1da728ee0728182df697",
    ),
)

# Fixtures that are reconstructed from .tex sources via latexmk. The tex files
# live in ``tests/fixtures/sources/`` and are versioned.
LATEX_FIXTURES: tuple[str, ...] = (
    "native_simple",
    "cid_corrupted",
    "borderless_table",
    "equation_heavy",
    "figure_with_caption",
    "mixed_lang_arabic",
)


def _sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as fh:
        for chunk in iter(lambda: fh.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def _read_checksums() -> dict[str, str]:
    if not CHECKSUM_FILE.exists():
        return {}
    out: dict[str, str] = {}
    for line in CHECKSUM_FILE.read_text(encoding="utf-8").splitlines():
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        digest, _, name = line.partition("  ")
        if digest and name:
            out[name.strip()] = digest.strip()
    return out


def _write_checksums(table: dict[str, str]) -> None:
    PDF_DIR.mkdir(parents=True, exist_ok=True)
    lines = [f"{digest}  {name}" for name, digest in sorted(table.items())]
    CHECKSUM_FILE.write_text("\n".join(lines) + "\n", encoding="utf-8")


def cmd_verify() -> int:
    """Validate that on-disk PDFs match the recorded SHA-256."""
    table = _read_checksums()
    if not table:
        logger.error("CHECKSUMS.sha256 is empty or missing")
        return 1
    failures = 0
    for name, expected in table.items():
        path = PDF_DIR / name
        if not path.exists():
            logger.error("missing fixture: %s", name)
            failures += 1
            continue
        actual = _sha256(path)
        if actual != expected:
            logger.error("checksum mismatch for %s: expected %s, got %s", name, expected, actual)
            failures += 1
        else:
            logger.info("ok %s", name)
    return 0 if failures == 0 else 2


def cmd_download() -> int:
    """Download remote fixtures (arXiv etc.) and update CHECKSUMS."""
    PDF_DIR.mkdir(parents=True, exist_ok=True)
    table = _read_checksums()
    with httpx.Client(follow_redirects=True, timeout=60) as client:
        for fx in REMOTE_FIXTURES:
            dest = PDF_DIR / fx.name
            logger.info("downloading %s -> %s", fx.url, dest)
            resp = client.get(fx.url)
            resp.raise_for_status()
            dest.write_bytes(resp.content)
            digest = _sha256(dest)
            if digest != fx.sha256:
                logger.error("%s: hash drift %s != %s", fx.name, digest, fx.sha256)
                return 3
            table[fx.name] = digest
    _write_checksums(table)
    return 0


def _run_latex(name: str) -> Path:
    tex = SOURCE_DIR / f"{name}.tex"
    if not tex.exists():
        logger.warning("%s.tex not present yet — skipping", name)
        return PDF_DIR / f"{name}.pdf"
    build_dir = PDF_DIR / "_build"
    build_dir.mkdir(parents=True, exist_ok=True)
    cmd = [
        "latexmk",
        "-pdf",
        "-interaction=nonstopmode",
        "-halt-on-error",
        f"-output-directory={build_dir}",
        str(tex),
    ]
    logger.info("latexmk %s", tex.name)
    subprocess.run(cmd, check=True)
    out_pdf = build_dir / f"{name}.pdf"
    final = PDF_DIR / f"{name}.pdf"
    shutil.copyfile(out_pdf, final)
    return final


def cmd_build() -> int:
    """Compile LaTeX-based fixtures and refresh CHECKSUMS."""
    if shutil.which("latexmk") is None:
        logger.error("latexmk not found on PATH (install TeX Live).")
        return 4
    table = _read_checksums()
    for name in LATEX_FIXTURES:
        pdf = _run_latex(name)
        if pdf.exists():
            table[pdf.name] = _sha256(pdf)
    _write_checksums(table)
    return 0


def cmd_regen_expected() -> int:
    """Regenerate the *.golden.{json,md} outputs via the `strata` CLI.

    Phase 0 placeholder — the real implementation lands in T10.1.A10.1.2.
    """
    if shutil.which("strata") is None:
        logger.error("`strata` binary not found on PATH — build the Rust CLI first.")
        return 5
    if not PDF_DIR.exists():
        logger.error("PDF directory missing: %s", PDF_DIR)
        return 6
    EXPECTED_DIR.mkdir(parents=True, exist_ok=True)
    failures = 0
    for pdf in sorted(PDF_DIR.glob("*.pdf")):
        cmd = [
            "strata",
            "parse",
            "--input",
            str(pdf),
            "--output",
            str(EXPECTED_DIR),
            "--format",
            "md+json",
            "--profile",
            "scientific",
        ]
        logger.info("regen %s", pdf.name)
        result = subprocess.run(cmd, check=False)
        if result.returncode != 0:
            logger.error("strata parse failed for %s (exit %s)", pdf.name, result.returncode)
            failures += 1
    return 0 if failures == 0 else 7


def _build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--verify", action="store_true", help="verify SHA-256 of every fixture")
    group.add_argument("--download", action="store_true", help="(re)download remote fixtures (arXiv)")
    group.add_argument("--build", action="store_true", help="compile LaTeX-based fixtures")
    group.add_argument(
        "--regen-expected",
        action="store_true",
        help="regenerate golden .json/.md via the `strata` CLI",
    )
    parser.add_argument("-v", "--verbose", action="store_true")
    return parser


def main(argv: Iterable[str] | None = None) -> int:
    args = _build_parser().parse_args(argv)
    logging.basicConfig(
        level=logging.DEBUG if args.verbose else logging.INFO,
        format="%(asctime)s %(levelname)s %(message)s",
    )
    if args.verify:
        return cmd_verify()
    if args.download:
        return cmd_download()
    if args.build:
        return cmd_build()
    if args.regen_expected:
        return cmd_regen_expected()
    return 99  # unreachable


if __name__ == "__main__":
    sys.exit(main())

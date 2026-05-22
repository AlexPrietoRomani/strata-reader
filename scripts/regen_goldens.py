"""Regenerate the golden corpus under ``tests/fixtures/expected/``.

Plan Maestro §15.T10.1 / A10.1.2 — every PDF in
``tests/fixtures/pdfs/`` is fed to the real `strata` CLI and the
output is captured as the new golden. Operators review the diffs in a
PR and update ``tests/fixtures/expected/REVIEW.md`` with the
rationale before merging.

Usage::

    uv run python scripts/regen_goldens.py --check        # dry-run, exit 1 on drift
    uv run python scripts/regen_goldens.py                # overwrite + open REVIEW.md
    uv run python scripts/regen_goldens.py --profile fast
"""

from __future__ import annotations

import argparse
import logging
import shutil
import subprocess
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[1]
FIXTURES_PDFS = REPO_ROOT / "tests" / "fixtures" / "pdfs"
EXPECTED_DIR = REPO_ROOT / "tests" / "fixtures" / "expected"
REVIEW_FILE = EXPECTED_DIR / "REVIEW.md"

logger = logging.getLogger("regen_goldens")


def resolve_strata_bin() -> str | None:
    import os

    # Fallback 1: Check default target/release in REPO_ROOT (real compiled binary)
    candidate = REPO_ROOT / "target" / "release" / ("strata.exe" if os.name == "nt" else "strata")
    if candidate.exists():
        return str(candidate)

    # Fallback 2: Check target/debug in REPO_ROOT
    candidate = REPO_ROOT / "target" / "debug" / ("strata.exe" if os.name == "nt" else "strata")
    if candidate.exists():
        return str(candidate)

    # Fallback 3: check CARGO_TARGET_DIR
    target = os.environ.get("CARGO_TARGET_DIR")
    if target:
        candidate = Path(target) / "release" / ("strata.exe" if os.name == "nt" else "strata")
        if candidate.exists():
            return str(candidate)
        candidate = Path(target) / "debug" / ("strata.exe" if os.name == "nt" else "strata")
        if candidate.exists():
            return str(candidate)

    # Fallback 4: use shutil.which (might resolve to python venv CLI mock)
    bin_path = shutil.which("strata")
    if bin_path:
        return bin_path

    return None


def regen_one(strata_bin: str, pdf: Path, profile: str, out_dir: Path) -> tuple[str, str]:
    """Run `strata parse` and return (md, json) text contents."""
    sub_out = out_dir / pdf.stem
    sub_out.mkdir(parents=True, exist_ok=True)
    subprocess.run(
        [
            strata_bin,
            "parse",
            "--input",
            str(pdf),
            "--output",
            str(sub_out),
            "--format",
            "md+json",
            "--profile",
            profile,
        ],
        check=True,
        capture_output=True,
        text=True,
        timeout=600,
    )
    md = (sub_out / f"{pdf.stem}.md").read_text(encoding="utf-8")
    js = (sub_out / f"{pdf.stem}.json").read_text(encoding="utf-8")
    return md, js


def write_review_template(updated: list[str]) -> None:
    EXPECTED_DIR.mkdir(parents=True, exist_ok=True)
    lines = [
        "# Strata-Reader — Goldens Review",
        "",
        "Every entry here is a human acknowledgement that a regenerated",
        "golden was inspected and intentionally promoted. Plan Maestro",
        "§15.T10.1 mandates that no golden change merges without a",
        "matching block in this file.",
        "",
        "## Updated this run",
        "",
    ]
    if not updated:
        lines.append("_(no goldens changed)_")
    for name in updated:
        lines.append(f"- `{name}` — reviewed: ___ (initials), date: ___")
    REVIEW_FILE.write_text("\n".join(lines) + "\n", encoding="utf-8")
    logger.info("wrote %s", REVIEW_FILE.relative_to(REPO_ROOT))


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="dry-run; exit 1 on drift")
    parser.add_argument("--profile", default="scientific", choices=["fast", "balanced", "scientific"])
    parser.add_argument("-v", "--verbose", action="store_true")
    args = parser.parse_args(argv)

    logging.basicConfig(
        level=logging.DEBUG if args.verbose else logging.INFO,
        format="%(asctime)s %(levelname)s %(message)s",
    )

    strata_bin = resolve_strata_bin()
    if strata_bin is None:
        logger.error("`strata` binary not found on PATH; build with `cargo build -p strata-cli --release`")
        return 2

    pdfs = sorted(FIXTURES_PDFS.glob("*.pdf"))
    if not pdfs:
        logger.error("no PDFs under %s", FIXTURES_PDFS)
        return 3

    EXPECTED_DIR.mkdir(parents=True, exist_ok=True)
    out_dir = REPO_ROOT / "_regen_goldens"
    if out_dir.exists():
        shutil.rmtree(out_dir)
    out_dir.mkdir()

    drift: list[str] = []
    updated: list[str] = []

    try:
        for pdf in pdfs:
            logger.info("regenerating %s", pdf.name)
            md, js = regen_one(strata_bin, pdf, args.profile, out_dir)

            md_target = EXPECTED_DIR / f"{pdf.stem}.golden.md"
            js_target = EXPECTED_DIR / f"{pdf.stem}.golden.json"

            for target, content in [(md_target, md), (js_target, js)]:
                existing = target.read_text(encoding="utf-8") if target.exists() else None
                if existing == content:
                    continue
                if args.check:
                    drift.append(str(target.relative_to(REPO_ROOT)))
                else:
                    target.write_text(content, encoding="utf-8")
                    updated.append(target.name)
    finally:
        shutil.rmtree(out_dir, ignore_errors=True)

    if args.check:
        if drift:
            logger.error("goldens drifted (%d files): %s", len(drift), ", ".join(drift))
            return 1
        logger.info("all goldens up to date")
        return 0

    if updated:
        write_review_template(updated)
        logger.info("regenerated %d goldens; please review %s", len(updated), REVIEW_FILE.relative_to(REPO_ROOT))
    else:
        logger.info("no changes")
    return 0


if __name__ == "__main__":
    sys.exit(main())

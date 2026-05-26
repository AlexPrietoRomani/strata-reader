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
import contextlib
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
    """
    Compila un archivo LaTeX en formato PDF usando el motor más apropiado.

    Intenta usar latexmk si está disponible y funciona (con Perl). Como fallback,
    utiliza directamente pdflatex o xelatex dependiendo del contenido del archivo
    (se requiere xelatex para fuentes unicode complejas y multi-lenguaje).

    Args:
        name (str): Nombre base del fixture LaTeX (sin la extensión .tex).

    Returns:
        Path: Ruta absoluta al archivo PDF generado.

    Raises:
        RuntimeError: Si la compilación falla o no se encuentra ningún compilador compatible.
    """
    tex = SOURCE_DIR / f"{name}.tex"
    if not tex.exists():
        logger.warning("%s.tex not present yet — skipping", name)
        return PDF_DIR / f"{name}.pdf"

    build_dir = PDF_DIR / "_build"
    build_dir.mkdir(parents=True, exist_ok=True)

    # Decidir si compilar con xelatex o pdflatex
    # xeLaTeX es requerido para fuentes unicode complejas y multi-lenguaje (como árabe)
    engine = "pdflatex"
    try:
        content = tex.read_text(encoding="utf-8")
        if "xelatex" in content.lower() or "polyglossia" in content:
            engine = "xelatex"
    except Exception:
        pass

    # Si latexmk está disponible y funciona (con Perl), lo usamos como primera opción
    # de lo contrario, caemos en pdflatex/xelatex directo.
    use_latexmk = shutil.which("latexmk") is not None
    if use_latexmk:
        cmd = [
            "latexmk",
            "-pdfxe" if engine == "xelatex" else "-pdf",
            "-interaction=nonstopmode",
            "-halt-on-error",
            f"-output-directory={build_dir}",
            str(tex),
        ]
        logger.info("Intentando compilar con latexmk: %s", tex.name)
        try:
            # Capturamos la salida para verificar si falló MiKTeX/Perl
            subprocess.run(cmd, check=True, capture_output=True)
            out_pdf = build_dir / f"{name}.pdf"
            final = PDF_DIR / f"{name}.pdf"
            shutil.copyfile(out_pdf, final)
            return final
        except Exception as e:
            logger.warning(
                "latexmk falló o no tiene Perl instalado. Usando motor directo %s. Error: %s",
                engine,
                e,
            )

    # Compilación directa con pdflatex o xelatex
    if shutil.which(engine) is None:
        # Fallback alternativo al otro si el seleccionado no está
        alternative = "pdflatex" if engine == "xelatex" else "xelatex"
        if shutil.which(alternative) is not None:
            logger.warning("Motor %s no encontrado, usando fallback %s", engine, alternative)
            engine = alternative
        else:
            raise RuntimeError(
                f"No se encontró ningún compilador LaTeX compatible ({engine}/{alternative})"
            )

    # Ejecutar compilación directa. A veces se necesitan 2 pasadas para referencias cruzadas,
    # pero para estos fixtures simples con 1 pasada suele ser suficiente. Hacemos 2 por robustez.
    cmd = [
        engine,
        "-interaction=nonstopmode",
        "-halt-on-error",
        f"-output-directory={build_dir}",
        str(tex),
    ]
    logger.info("Compilando directamente con %s (pasada 1): %s", engine, tex.name)
    subprocess.run(cmd, check=True)

    logger.info("Compilando directamente con %s (pasada 2): %s", engine, tex.name)
    subprocess.run(cmd, check=True)

    out_pdf = build_dir / f"{name}.pdf"
    final = PDF_DIR / f"{name}.pdf"
    shutil.copyfile(out_pdf, final)
    return final


def _build_scanned_paper() -> Path:
    """
    Genera un documento PDF escaneado sintético a partir del paper real de arXiv.

    Utiliza pdftoppm para rasterizar las primeras 6 páginas de two_column_paper.pdf
    a 300 dpi (generando imágenes PNG) y luego las recombina en un único PDF
    usando la librería img2pdf para simular un documento escaneado.

    Returns:
        Path: Ruta absoluta al archivo scanned_paper.pdf generado.

    Raises:
        FileNotFoundError: Si no se encuentra el archivo de entrada two_column_paper.pdf.
        RuntimeError: Si la rasterización o recombinación fallan.
    """
    src = PDF_DIR / "two_column_paper.pdf"
    if not src.exists():
        raise FileNotFoundError(f"Se requiere {src} para generar scanned_paper.pdf")

    build_dir = PDF_DIR / "_build"
    build_dir.mkdir(parents=True, exist_ok=True)

    # Rasterizar las primeras 6 páginas a 300 dpi
    # pdftoppm -png -r 300 -f 1 -l 6 <src> <build_dir>/page
    logger.info("Rasterizando %s con pdftoppm...", src.name)
    prefix = build_dir / "page"
    cmd = [
        "pdftoppm",
        "-png",
        "-r",
        "300",
        "-f",
        "1",
        "-l",
        "6",
        str(src),
        str(prefix),
    ]
    subprocess.run(cmd, check=True)

    # Buscar las imágenes generadas
    images = sorted(build_dir.glob("page-*.png"))
    if not images:
        raise RuntimeError("No se generaron imágenes con pdftoppm")

    # Recombinar con img2pdf
    final = PDF_DIR / "scanned_paper.pdf"
    logger.info("Recombinando imágenes rasterizadas con img2pdf -> %s", final.name)

    import img2pdf

    pdf_bytes = img2pdf.convert([str(img) for img in images])
    final.write_bytes(pdf_bytes)

    # Limpiar imágenes temporales
    for img in images:
        with contextlib.suppress(Exception):
            img.unlink()

    return final


def cmd_build() -> int:
    """Compile LaTeX-based fixtures and refresh CHECKSUMS."""
    has_latexmk = shutil.which("latexmk") is not None
    has_pdflatex = shutil.which("pdflatex") is not None
    has_xelatex = shutil.which("xelatex") is not None

    if not (has_latexmk or has_pdflatex or has_xelatex):
        logger.error("No LaTeX compiler found (install TeX Live or MiKTeX with pdflatex/xelatex).")
        return 4

    table = _read_checksums()

    # 1. Compilar los fixtures de LaTeX
    for name in LATEX_FIXTURES:
        try:
            pdf = _run_latex(name)
            if pdf.exists():
                table[pdf.name] = _sha256(pdf)
        except Exception as e:
            logger.error("Failed to build fixture %s: %s", name, e)
            return 8

    # 2. Generar scanned_paper.pdf rasterizando el paper real
    try:
        pdf = _build_scanned_paper()
        if pdf.exists():
            table[pdf.name] = _sha256(pdf)
    except Exception as e:
        logger.error("Failed to build scanned_paper: %s", e)
        return 9

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
    group.add_argument(
        "--download", action="store_true", help="(re)download remote fixtures (arXiv)"
    )
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

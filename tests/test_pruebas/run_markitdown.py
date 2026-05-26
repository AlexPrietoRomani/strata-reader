"""
Archivo: run_markitdown.py
Fecha de modificación: 26/05/2026
Autor: Alex Prieto

Descripción:
Ejecuta de forma aislada el benchmark de conversión de PDFs científicos
utilizando la librería MarkItDown de Microsoft. Mide y calcula los tiempos de
procesamiento y la velocidad de extracción promedio por página de forma empírica.

Acciones Principales:
    - Convierte una lista de archivos PDF científicos de prueba usando `MarkItDown`.
    - Genera las salidas en formato Markdown.
    - Mide el tiempo total y la velocidad empírica de extracción por página.

Estructura Interna:
    - `run_markitdown_benchmark()`: Función principal exportable para ejecutar la suite.

Entradas / Dependencias:
    - Archivos PDF de artículos científicos en `tests/fixtures/pdfs/articles`.
    - Librería `markitdown` de Microsoft.
    - Librería `strata_reader` (para conteo de páginas).

Salidas / Efectos:
    - Archivos Markdown guardados en `tests/fixtures/salidas/markitdown-pdf`.

Ejecución:
    python tests/test_pruebas/run_markitdown.py
"""

from __future__ import annotations

import time
from pathlib import Path
from typing import Any

import strata_reader
from markitdown import MarkItDown


def run_markitdown_benchmark(pdf_files: list[Path], output_dir: Path) -> dict[str, Any]:
    """
    Ejecuta el procesamiento de PDFs usando Microsoft MarkItDown y mide su velocidad.

    Args:
        pdf_files (List[Path]): Lista de objetos Path correspondientes a los archivos PDF.
        output_dir (Path): Directorio de destino para las salidas en formato Markdown.

    Returns:
        Dict[str, Any]: Métricas calculadas que incluyen 'elapsed_time',
        'total_pages' y 'speed' (segundos por página).
    """
    output_dir.mkdir(parents=True, exist_ok=True)
    markitdown = MarkItDown()

    print(f"[MarkItDown] Iniciando conversion de {len(pdf_files)} PDFs...")
    start_time = time.time()

    for pdf_path in pdf_files:
        try:
            # Ejecutar conversión usando la API de MarkItDown
            result = markitdown.convert(str(pdf_path.absolute()))
            md_content = result.text_content

            # Guardar el Markdown extraído
            out_file = output_dir / f"{pdf_path.stem}.md"
            out_file.write_text(md_content, encoding="utf-8")
        except Exception as e:
            print(f"[MarkItDown - ERROR] Fallo al procesar {pdf_path.name}: {e}")

    elapsed_time = time.time() - start_time

    # Calcular el número total de páginas utilizando la lectura nativa de Strata-Reader
    # para garantizar un conteo exacto e imparcial entre todos los frameworks
    total_pages = 0
    for doc_path in pdf_files:
        try:
            parsed_doc = strata_reader.parse(str(doc_path.absolute()))
            total_pages += len(parsed_doc)
        except Exception:
            total_pages += 1  # Fallback mínimo de 1 página por archivo si ocurre algún error

    speed = elapsed_time / total_pages if total_pages > 0 else 0.0
    print(
        f"[MarkItDown] Procesadas {total_pages} paginas en {elapsed_time:.3f} s ({speed:.4f} s/pagina)."
    )

    return {
        "elapsed_time": elapsed_time,
        "total_pages": total_pages,
        "speed": speed,
    }


def main() -> None:
    """
    Orquestación para la ejecución aislada de este benchmark desde la terminal.
    """
    pdf_dir = Path("tests/fixtures/pdfs/articles")
    pdf_files = sorted(list(pdf_dir.glob("*.pdf")))

    if not pdf_files:
        print(f"[ERROR] No se encontraron PDFs de prueba en {pdf_dir.absolute()}")
        return

    output_path = Path("tests/fixtures/salidas/markitdown-pdf")
    metrics = run_markitdown_benchmark(pdf_files, output_path)
    print(f"Resultado Exitoso: {metrics}")


if __name__ == "__main__":
    main()

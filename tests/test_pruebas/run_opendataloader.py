"""
Archivo: run_opendataloader.py
Fecha de modificación: 26/05/2026
Autor: Alex Prieto

Descripción:
Ejecuta de forma aislada el benchmark de conversión de PDFs científicos
utilizando la librería OpenDataLoader-PDF. Mide y calcula los tiempos de
procesamiento y la velocidad de extracción promedio por página.

Acciones Principales:
    - Convierte una lista de archivos PDF científicos de prueba usando `opendataloader_pdf`.
    - Genera las salidas de tipo Markdown y JSON.
    - Calcula el tiempo total y la velocidad empírica de extracción por página.

Estructura Interna:
    - `run_opendataloader_benchmark()`: Función principal exportable para ejecutar la suite.

Entradas / Dependencias:
    - Archivos PDF de artículos científicos en `tests/fixtures/pdfs/articles`.
    - Librería `opendataloader_pdf` y `strata_reader` (para conteo de páginas).

Salidas / Efectos:
    - Archivos Markdown y JSON guardados en `tests/fixtures/salidas/opendataloader-pdf`.

Ejecución:
    python tests/test_pruebas/run_opendataloader.py
"""

from __future__ import annotations

import time
from pathlib import Path
from typing import Any

import opendataloader_pdf
import strata_reader


def run_opendataloader_benchmark(pdf_files: list[Path], output_dir: Path) -> dict[str, Any]:
    """
    Ejecuta el procesamiento de PDFs usando OpenDataLoader y mide su velocidad.

    Args:
        pdf_files (List[Path]): Lista de objetos Path correspondientes a los archivos PDF.
        output_dir (Path): Directorio de destino para las salidas en formato Markdown y JSON.

    Returns:
        Dict[str, Any]: Métricas calculadas que incluyen 'elapsed_time',
        'total_pages' y 'speed' (segundos por página).
    """
    output_dir.mkdir(parents=True, exist_ok=True)
    pdf_strs = [str(p.absolute()) for p in pdf_files]

    print(f"[OpenDataLoader] Iniciando conversion de {len(pdf_files)} PDFs...")
    start_time = time.time()

    opendataloader_pdf.convert(
        input_path=pdf_strs, output_dir=str(output_dir.absolute()), format="markdown,json"
    )

    elapsed_time = time.time() - start_time

    # Calcular el número total de páginas utilizando la lectura nativa de Strata-Reader
    # para garantizar un conteo exacto e imparcial entre ambos frameworks
    total_pages = 0
    for doc_path in pdf_files:
        try:
            parsed_doc = strata_reader.parse(str(doc_path.absolute()))
            total_pages += len(parsed_doc)
        except Exception:
            total_pages += 1  # Fallback mínimo de 1 página por archivo si ocurre algún error

    speed = elapsed_time / total_pages if total_pages > 0 else 0.0
    print(
        f"[OpenDataLoader] Procesadas {total_pages} paginas en {elapsed_time:.3f} s ({speed:.4f} s/pagina)."
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

    output_path = Path("tests/fixtures/salidas/opendataloader-pdf")
    metrics = run_opendataloader_benchmark(pdf_files, output_path)
    print(f"Resultado Exitoso: {metrics}")


if __name__ == "__main__":
    main()

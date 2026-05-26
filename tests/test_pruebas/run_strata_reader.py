"""
Archivo: run_strata_reader.py
Fecha de modificación: 26/05/2026
Autor: Strata-Reader Contributors

Descripción:
Ejecuta de forma aislada el benchmark de conversión de PDFs científicos
utilizando la librería Strata-Reader. Mide y calcula los tiempos de
procesamiento y la velocidad de extracción promedio por página de forma empírica.

Acciones Principales:
    - Configura dinámicamente la ruta a la biblioteca nativa de pdfium.
    - Convierte una lista de archivos PDF científicos de prueba usando `strata_reader`.
    - Genera las salidas en formato Markdown y JSON.
    - Calcula el tiempo total y la velocidad empírica de extracción por página.

Estructura Interna:
    - `run_strata_benchmark()`: Función principal exportable para ejecutar la suite.

Entradas / Dependencias:
    - Archivos PDF de artículos científicos en `tests/fixtures/pdfs/articles`.
    - Módulo de inicialización de `strata_reader`.

Salidas / Efectos:
    - Archivos Markdown y JSON guardados en `tests/fixtures/salidas/strata-reader-output`.

Ejecución:
    python tests/test_pruebas/run_strata_reader.py
"""

from __future__ import annotations

import os
from pathlib import Path
import time
from typing import Dict, List, Any

# Configurar ruta a la biblioteca pdfium en Windows de forma dinamica si no existe en el entorno
if "STRATA_PDFIUM_LIB_PATH" not in os.environ:
    local_pdfium = Path.home() / "AppData" / "Local" / "pdfium" / "bin"
    if local_pdfium.is_dir():
        os.environ["STRATA_PDFIUM_LIB_PATH"] = str(local_pdfium.absolute())

import strata_reader


def run_strata_benchmark(
    pdf_files: List[Path], output_dir: Path
) -> Dict[str, Any]:
    """
    Ejecuta el procesamiento de PDFs usando Strata-Reader y mide su velocidad.

    Args:
        pdf_files (List[Path]): Lista de objetos Path correspondientes a los archivos PDF.
        output_dir (Path): Directorio de destino para las salidas en formato md+json.

    Returns:
        Dict[str, Any]: Métricas calculadas que incluyen 'elapsed_time',
        'total_pages' y 'speed' (segundos por página).
    """
    output_dir.mkdir(parents=True, exist_ok=True)

    print(f"[Strata-Reader] Iniciando conversion de {len(pdf_files)} PDFs...")
    start_time = time.time()
    
    results = strata_reader.convert(
        input_path=pdf_files,
        output_dir=output_dir,
        format="md+json",
        use_ia=False,
        show_progress=False,
    )
    
    elapsed_time = time.time() - start_time

    # Calcular el número total de páginas procesadas exitosamente
    success_docs = [path for path, status in results.items() if status == "success"]
    total_pages = 0
    for doc_path in success_docs:
        try:
            parsed_doc = strata_reader.parse(doc_path)
            total_pages += len(parsed_doc)
        except Exception:
            total_pages += 1  # Fallback si falla la lectura de páginas
            
    speed = elapsed_time / total_pages if total_pages > 0 else 0.0
    print(f"[Strata-Reader] Procesadas {total_pages} paginas en {elapsed_time:.3f} s ({speed:.4f} s/pagina).")

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

    output_path = Path("tests/fixtures/salidas/strata-reader-output")
    metrics = run_strata_benchmark(pdf_files, output_path)
    print(f"Resultado Exitoso: {metrics}")


if __name__ == "__main__":
    main()

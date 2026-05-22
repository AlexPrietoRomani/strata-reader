"""
Archivo: run_strata_reader.py
Fecha de modificación: 22/05/2026
Autor: Strata-Reader Contributors

Descripción:
Mide y compara de forma empírica la velocidad y el rendimiento del procesamiento de PDFs
entre Strata-Reader y OpenDataLoader-PDF sobre el corpus de artículos científicos de prueba.

Sustentación Científica:
Permite obtener una evaluación empírica de velocidad de conversión por página (s/page)
para validar el rendimiento del motor nativo en Rust de Strata-Reader frente al baseline
de OpenDataLoader.

Acciones Principales:
    - Ejecuta `strata_reader.convert()` sobre la carpeta de artículos de prueba.
    - Ejecuta `opendataloader_pdf.convert()` sobre la misma carpeta de artículos.
    - Calcula la velocidad empírica de procesamiento por página para ambos motores.
    - Almacena las métricas resultantes en un archivo JSON unificado.

Estructura Interna:
    - `main()`: Orquesta la conversión y calcula el benchmark empírico.

Entradas / Dependencias:
    - PDFs de artículos científicos en `tests/fixtures/pdfs/articles`.
    - Módulo `strata_reader` y `opendataloader_pdf`.

Salidas / Efectos:
    - Salidas de Markdown y JSON de Strata-Reader en `tests/fixtures/salidas/strata-reader-output`.
    - Salidas de Markdown y JSON de OpenDataLoader en `tests/fixtures/salidas/opendataloader-pdf`.
    - Archivo de métricas consolidadas en `tests/fixtures/salidas/strata_real_metrics.json`.

Ejecución:
    python tests/test_pruebas/run_strata_reader.py

Ejemplo de Uso:
    python tests/test_pruebas/run_strata_reader.py
"""

from __future__ import annotations

import json
import os
from pathlib import Path
import time

# Configurar ruta a la biblioteca pdfium en Windows de forma dinamica
if "STRATA_PDFIUM_LIB_PATH" not in os.environ:
    local_pdfium = Path.home() / "AppData" / "Local" / "pdfium" / "bin"
    if local_pdfium.is_dir():
        os.environ["STRATA_PDFIUM_LIB_PATH"] = str(local_pdfium.absolute())

import strata_reader
import opendataloader_pdf


def run_strata_benchmark(pdf_files: list[Path], out_dir: Path) -> float:
    """
    Ejecuta el benchmark para el SDK de Strata-Reader y calcula la velocidad por página.

    Args:
        pdf_files (list[Path]): Lista de objetos Path correspondientes a los archivos PDF.
        out_dir (Path): Directorio de salida para guardar los resultados del procesamiento.

    Returns:
        float: Velocidad promedio de procesamiento expresada en segundos por página.
    """
    out_dir.mkdir(parents=True, exist_ok=True)
    
    # 1. Medir tiempo de ejecución del SDK de Strata-Reader
    start_time = time.time()
    results = strata_reader.convert(
        input_path=pdf_files,
        output_dir=out_dir,
        format="md+json",
        use_ia=False,
        show_progress=False,
    )
    elapsed_time = time.time() - start_time
    
    # 2. Calcular total de páginas procesadas exitosamente
    success_docs = [path for path, status in results.items() if status == "success"]
    total_pages = 0
    for doc_path in success_docs:
        try:
            # parse() retorna el Document nativo, del cual podemos extraer la cantidad de páginas
            parsed_doc = strata_reader.parse(doc_path)
            total_pages += len(parsed_doc)
        except Exception:
            total_pages += 1  # Fallback si falla la lectura de páginas
            
    speed = elapsed_time / total_pages if total_pages > 0 else 0.0
    print(f"[Strata-Reader] Procesadas {total_pages} paginas en {elapsed_time:.3f} s ({speed:.3f} s/pagina).")
    return speed


def run_opendataloader_benchmark(pdf_files: list[Path], out_dir: Path) -> float:
    """
    Ejecuta el benchmark para OpenDataLoader y calcula la velocidad por página.

    Args:
        pdf_files (list[Path]): Lista de objetos Path correspondientes a los archivos PDF.
        out_dir (Path): Directorio de salida para guardar los resultados del procesamiento.

    Returns:
        float: Velocidad promedio de procesamiento expresada en segundos por página.
    """
    out_dir.mkdir(parents=True, exist_ok=True)
    pdf_strs = [str(p.absolute()) for p in pdf_files]
    
    # 1. Medir tiempo de ejecución del SDK de OpenDataLoader
    start_time = time.time()
    opendataloader_pdf.convert(
        input_path=pdf_strs,
        output_dir=str(out_dir.absolute()),
        format="markdown,json"
    )
    elapsed_time = time.time() - start_time
    
    # 2. Calcular total de páginas (usando la lectura nativa de Strata-Reader como referencia estable)
    total_pages = 0
    for doc_path in pdf_files:
        try:
            parsed_doc = strata_reader.parse(str(doc_path.absolute()))
            total_pages += len(parsed_doc)
        except Exception:
            total_pages += 1
            
    speed = elapsed_time / total_pages if total_pages > 0 else 0.0
    print(f"[OpenDataLoader] Procesadas {total_pages} paginas en {elapsed_time:.3f} s ({speed:.3f} s/pagina).")
    return speed


def main() -> None:
    """
    Orquesta el benchmark comparativo entre ambos motores de extracción de PDFs.
    """
    pdf_dir = Path("tests/fixtures/pdfs/articles")
    pdf_files = sorted(list(pdf_dir.glob("*.pdf")))
    
    if not pdf_files:
        print(f"[ERROR] No se encontraron archivos PDF en {pdf_dir}")
        return
        
    strata_out_dir = Path("tests/fixtures/salidas/strata-reader-output")
    odl_out_dir = Path("tests/fixtures/salidas/opendataloader-pdf")
    
    print(f"Iniciando benchmark de velocidad con {len(pdf_files)} articulos...")
    
    # Ejecutar benchmarks y obtener velocidades
    strata_speed = run_strata_benchmark(pdf_files, strata_out_dir)
    odl_speed = run_opendataloader_benchmark(pdf_files, odl_out_dir)
    
    # Almacenar métricas en formato JSON consolidado
    metrics = {
        "strata_speed": strata_speed,
        "opendataloader_speed": odl_speed
    }
    
    metrics_path = Path("tests/fixtures/salidas/strata_real_metrics.json")
    metrics_path.parent.mkdir(parents=True, exist_ok=True)
    metrics_path.write_text(json.dumps(metrics, indent=2))
    print(f"Metricas de velocidad consolidadas y guardadas en: {metrics_path.absolute()}")


if __name__ == "__main__":
    main()

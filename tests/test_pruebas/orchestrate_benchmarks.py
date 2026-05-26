"""
Archivo: orchestrate_benchmarks.py
Fecha de modificación: 26/05/2026
Autor: Strata-Reader Contributors

Descripción:
Orquestador maestro de la suite de benchmarking comparativo. Coordina
secuencialmente la conversión de los PDFs de prueba con cada motor,
evalúa la precisión de diseño y formateo del Markdown generado mediante
SCE-Accuracy, consolida las métricas reales y desencadena la generación
del gráfico comparativo.

Acciones Principales:
    - Descubre y carga los artículos PDF científicos de prueba.
    - Ejecuta secuencialmente `run_strata_reader.py` y `run_opendataloader.py`.
    - Llama a `quality_benchmark.py` para calcular el Accuracy estructural.
    - Une y escribe las métricas a `tests/fixtures/salidas/strata_real_metrics.json`.
    - Invoca a `plot_benchmark.py` para actualizar el gráfico comparativo PNG.

Estructura Interna:
    - `main()`: Orquestación secuencial completa del flujo de benchmarking.

Entradas / Dependencias:
    - Directorio de PDFs en `tests/fixtures/pdfs/articles`.
    - Módulos locales `run_strata_reader`, `run_opendataloader`,
      `quality_benchmark` y `plot_benchmark`.

Salidas / Efectos:
    - Directorios de salidas Markdown y JSON para cada librería.
    - Métrica consolidada en `tests/fixtures/salidas/strata_real_metrics.json`.
    - Gráfico premium en `tests/fixtures/salidas/benchmark_comparison.png`.

Ejecución:
    python tests/test_pruebas/orchestrate_benchmarks.py
"""

from __future__ import annotations

import json
from pathlib import Path
import sys
from typing import Dict, List, Any

# Agregar el directorio de pruebas al path de Python para evitar fallos de importacion
CURRENT_DIR = Path(__file__).parent.absolute()
if str(CURRENT_DIR) not in sys.path:
    sys.path.append(str(CURRENT_DIR))

# Importaciones locales de los modulos del benchmark
from run_opendataloader import run_opendataloader_benchmark
from run_strata_reader import run_strata_benchmark
from quality_benchmark import compute_extraction_accuracy
from plot_benchmark import generate_benchmark_plot


def main() -> None:
    """
    Orquesta de forma secuencial todo el flujo de benchmarking comparativo.
    """
    print("\n" + "="*80)
    print(" INICIANDO PIPELINE DE BENCHMARKING DE PARSEO MULTI-MOTOR")
    print("="*80)

    # 1. Definir rutas y buscar PDFs de artículos de prueba
    pdf_dir = Path("tests/fixtures/pdfs/articles")
    pdf_files = sorted(list(pdf_dir.glob("*.pdf")))
    
    if not pdf_files:
        print(f"[ERROR] No se encontraron archivos PDF científicos de prueba en: {pdf_dir.absolute()}")
        return

    print(f"Se encontraron {len(pdf_files)} articulos PDF cientificos en el corpus de prueba.")
    print(f"Ruta de fixtures: {pdf_dir.absolute()}\n")

    strata_out = Path("tests/fixtures/salidas/strata-reader-output")
    odl_out = Path("tests/fixtures/salidas/opendataloader-pdf")

    # 2. Ejecutar Benchmark de velocidad para Strata-Reader (Rust Native)
    print("-"*80)
    print("PASO 1: Ejecutando conversion y benchmark de velocidad para Strata-Reader...")
    print("-"*80)
    strata_metrics = run_strata_benchmark(pdf_files, strata_out)

    # 3. Ejecutar Benchmark de velocidad para OpenDataLoader (Baseline Python)
    print("\n" + "-"*80)
    print("PASO 2: Ejecutando conversion y benchmark de velocidad para OpenDataLoader...")
    print("-"*80)
    odl_metrics = run_opendataloader_benchmark(pdf_files, odl_out)

    # 4. Calcular Accuracy de formateo y jerarquía Markdown (SCE-Accuracy)
    print("\n" + "-"*80)
    print("PASO 3: Analizando calidad y calculando SCE-Accuracy estructural...")
    print("-"*80)
    accuracy_metrics = compute_extraction_accuracy(strata_out, odl_out)

    # 5. Consolidar todas las métricas reales
    print("\n" + "-"*80)
    print("PASO 4: Consolidando metricas de velocidad y precision en JSON...")
    print("-"*80)
    
    consolidated_metrics = {
        "strata_speed": strata_metrics["speed"],
        "opendataloader_speed": odl_metrics["speed"],
        "strata_accuracy": accuracy_metrics["strata_accuracy"],
        "opendataloader_accuracy": accuracy_metrics["opendataloader_accuracy"],
        "total_pdfs_processed": len(pdf_files),
        "total_pages_strata": strata_metrics["total_pages"],
        "total_pages_opendataloader": odl_metrics["total_pages"],
        "elapsed_time_strata_seconds": strata_metrics["elapsed_time"],
        "elapsed_time_opendataloader_seconds": odl_metrics["elapsed_time"]
    }

    metrics_json_path = Path("tests/fixtures/salidas/strata_real_metrics.json")
    metrics_json_path.parent.mkdir(parents=True, exist_ok=True)
    metrics_json_path.write_text(json.dumps(consolidated_metrics, indent=2), encoding="utf-8")
    print(f"Archivo de metricas reales escrito exitosamente en: {metrics_json_path.absolute()}")

    # 6. Generar el gráfico comparativo PNG
    print("\n" + "-"*80)
    print("PASO 5: Generando grafico de barras comparativo premium...")
    print("-"*80)
    try:
        generate_benchmark_plot()
    except Exception as e:
        print(f"[ERROR] No se pudo generar el grafico comparativo: {e}")
        return

    print("\n" + "="*80)
    print(" EJECUCIÓN DEL PIPELINE DE BENCHMARK COMPLETADO EXITOSAMENTE")
    print("="*80)
    print(f"Resumen de Metricas Consolidadas:")
    print(f"  - Strata-Reader:")
    print(f"      * Velocidad: {consolidated_metrics['strata_speed']:.4f} s/pagina")
    print(f"      * Accuracy (SCE-Accuracy): {consolidated_metrics['strata_accuracy']*100:.2f}%")
    print(f"  - OpenDataLoader:")
    print(f"      * Velocidad: {consolidated_metrics['opendataloader_speed']:.4f} s/pagina")
    print(f"      * Accuracy (SCE-Accuracy): {consolidated_metrics['opendataloader_accuracy']*100:.2f}%")
    print("="*80 + "\n")


if __name__ == "__main__":
    main()

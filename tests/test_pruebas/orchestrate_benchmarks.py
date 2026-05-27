"""
Archivo: orchestrate_benchmarks.py
Fecha de modificación: 26/05/2026
Autor: Alex Prieto

Descripción:
Orquestador maestro de la suite de benchmarking comparativo multimotor.
Coordina secuencialmente la conversión de los PDFs de prueba con cada motor
(Strata-Reader, OpenDataLoader y Microsoft MarkItDown), evalúa la precisión
de diseño y formateo del Markdown generado mediante la métrica SCE-Accuracy,
consolida las métricas reales y desencadena la generación del gráfico comparativo.

Acciones Principales:
    - Descubre y carga los artículos PDF científicos de prueba.
    - Ejecuta secuencialmente `run_strata_reader.py`, `run_opendataloader.py` y `run_markitdown.py`.
    - Llama a `quality_benchmark.py` para calcular el SCE-Accuracy de todos los motores.
    - Une y escribe las métricas a `tests/fixtures/salidas/strata_real_metrics.json`.
    - Invoca a `plot_benchmark.py` para actualizar el gráfico comparativo PNG.

Estructura Interna:
    - `main()`: Orquestación secuencial completa de la suite de benchmarking de tres motores.

Entradas / Dependencias:
    - Directorio de PDFs en `tests/fixtures/pdfs/articles`.
    - Módulos locales `run_strata_reader`, `run_opendataloader`,
      `run_markitdown`, `quality_benchmark` y `plot_benchmark`.

Salidas / Efectos:
    - Directorios de salidas Markdown y JSON correspondientes a cada librería.
    - Métrica consolidada en `tests/fixtures/salidas/strata_real_metrics.json`.
    - Gráfico premium en `tests/fixtures/salidas/benchmark_comparison.png`.

Ejecución:
    python tests/test_pruebas/orchestrate_benchmarks.py
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

# Agregar el directorio de pruebas al path de Python para evitar fallos de importación
CURRENT_DIR = Path(__file__).parent.absolute()
if str(CURRENT_DIR) not in sys.path:
    sys.path.append(str(CURRENT_DIR))

# Importaciones locales de los módulos del benchmark
from plot_benchmark import generate_benchmark_plot  # noqa: E402
from quality_benchmark import compute_extraction_accuracy  # noqa: E402
from run_markitdown import run_markitdown_benchmark  # noqa: E402
from run_opendataloader import run_opendataloader_benchmark  # noqa: E402
from run_strata_reader import run_strata_benchmark  # noqa: E402


def main() -> None:
    """
    Orquesta de forma secuencial todo el flujo de benchmarking comparativo.
    """
    print("\n" + "=" * 80)
    print(" INICIANDO PIPELINE DE BENCHMARKING DE PARSEO MULTI-MOTOR (3 MOTORES)")
    print("=" * 80)

    # 1. Definir rutas y buscar PDFs de artículos de prueba
    pdf_dir = Path("tests/fixtures/pdfs/articles")
    pdf_files = sorted(list(pdf_dir.glob("*.pdf")))

    if not pdf_files:
        print(
            f"[ERROR] No se encontraron archivos PDF científicos de prueba en: {pdf_dir.absolute()}"
        )
        return

    print(f"Se encontraron {len(pdf_files)} artículos PDF científicos en el corpus de prueba.")
    print(f"Ruta de fixtures: {pdf_dir.absolute()}\n")

    strata_out = Path("tests/fixtures/salidas/strata-reader-output")
    odl_out = Path("tests/fixtures/salidas/opendataloader-pdf")
    markitdown_out = Path("tests/fixtures/salidas/markitdown-pdf")

    # 2. Ejecutar Benchmark de velocidad para Strata-Reader (Rust Native)
    print("-" * 80)
    print("PASO 1: Ejecutando conversión y benchmark de velocidad para Strata-Reader...")
    print("-" * 80)
    strata_metrics = run_strata_benchmark(pdf_files, strata_out)

    # 3. Ejecutar Benchmark de velocidad para OpenDataLoader (Baseline Python)
    print("\n" + "-" * 80)
    print("PASO 2: Ejecutando conversión y benchmark de velocidad para OpenDataLoader...")
    print("-" * 80)
    odl_metrics = run_opendataloader_benchmark(pdf_files, odl_out)

    # 4. Ejecutar Benchmark de velocidad para Microsoft MarkItDown
    print("\n" + "-" * 80)
    print("PASO 3: Ejecutando conversión y benchmark de velocidad para Microsoft MarkItDown...")
    print("-" * 80)
    markitdown_metrics = run_markitdown_benchmark(pdf_files, markitdown_out)

    # 5. Calcular Accuracy de formateo y jerarquía Markdown (SCE-Accuracy)
    print("\n" + "-" * 80)
    print("PASO 4: Analizando calidad y calculando SCE-Accuracy estructural...")
    print("-" * 80)
    engine_dirs = {"strata": strata_out, "opendataloader": odl_out, "markitdown": markitdown_out}
    accuracy_metrics = compute_extraction_accuracy(engine_dirs)

    # 6. Consolidar todas las métricas reales
    print("\n" + "-" * 80)
    print("PASO 5: Consolidando métricas de velocidad y precisión en JSON...")
    print("-" * 80)

    consolidated_metrics = {
        "strata_speed": strata_metrics["speed"],
        "opendataloader_speed": odl_metrics["speed"],
        "markitdown_speed": markitdown_metrics["speed"],
        "total_pdfs_processed": len(pdf_files),
        "total_pages_strata": strata_metrics["total_pages"],
        "total_pages_opendataloader": odl_metrics["total_pages"],
        "total_pages_markitdown": markitdown_metrics["total_pages"],
        "elapsed_time_strata_seconds": strata_metrics["elapsed_time"],
        "elapsed_time_opendataloader_seconds": odl_metrics["elapsed_time"],
        "elapsed_time_markitdown_seconds": markitdown_metrics["elapsed_time"],
    }
    consolidated_metrics.update(accuracy_metrics)

    metrics_json_path = Path("tests/fixtures/salidas/strata_real_metrics.json")
    metrics_json_path.parent.mkdir(parents=True, exist_ok=True)
    metrics_json_path.write_text(json.dumps(consolidated_metrics, indent=2), encoding="utf-8")
    print(f"Archivo de métricas reales escrito exitosamente en: {metrics_json_path.absolute()}")

    # 7. Generar el gráfico comparativo PNG
    print("\n" + "-" * 80)
    print("PASO 6: Generando gráfico de barras comparativo premium (3 motores)...")
    print("-" * 80)
    try:
        generate_benchmark_plot()
    except Exception as e:
        print(f"[ERROR] No se pudo generar el gráfico comparativo: {e}")
        return

    print("\n" + "=" * 80)
    print(" EJECUCIÓN DEL PIPELINE DE BENCHMARK COMPLETADO EXITOSAMENTE")
    print("=" * 80)
    print("Resumen de Métricas Consolidadas:")
    print("  - Strata-Reader:")
    print(f"      * Velocidad: {consolidated_metrics['strata_speed']:.4f} s/página")
    print(f"      * SCE-Accuracy: {consolidated_metrics['strata_sce_accuracy'] * 100:.2f}%")
    print(f"      * TEDS (Tablas): {consolidated_metrics['strata_teds_score'] * 100:.2f}%")
    print(f"      * ANLS (Texto): {consolidated_metrics['strata_anls_score'] * 100:.2f}%")
    print(f"      * IoU (Figuras): {consolidated_metrics['strata_iou_figures'] * 100:.2f}%")
    print("  - OpenDataLoader:")
    print(f"      * Velocidad: {consolidated_metrics['opendataloader_speed']:.4f} s/página")
    print(f"      * SCE-Accuracy: {consolidated_metrics['opendataloader_sce_accuracy'] * 100:.2f}%")
    print(f"      * TEDS (Tablas): {consolidated_metrics['opendataloader_teds_score'] * 100:.2f}%")
    print(f"      * ANLS (Texto): {consolidated_metrics['opendataloader_anls_score'] * 100:.2f}%")
    print(f"      * IoU (Figuras): {consolidated_metrics['opendataloader_iou_figures'] * 100:.2f}%")
    print("  - Microsoft MarkItDown:")
    print(f"      * Velocidad: {consolidated_metrics['markitdown_speed']:.4f} s/página")
    print(f"      * SCE-Accuracy: {consolidated_metrics['markitdown_sce_accuracy'] * 100:.2f}%")
    print(f"      * TEDS (Tablas): {consolidated_metrics['markitdown_teds_score'] * 100:.2f}%")
    print(f"      * ANLS (Texto): {consolidated_metrics['markitdown_anls_score'] * 100:.2f}%")
    print(f"      * IoU (Figuras): {consolidated_metrics['markitdown_iou_figures'] * 100:.2f}%")
    print("=" * 80 + "\n")


if __name__ == "__main__":
    main()

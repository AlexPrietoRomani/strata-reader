"""
Archivo: plot_benchmark.py
Fecha de modificación: 26/05/2026
Autor: Strata-Reader Contributors

Descripción:
Genera un gráfico comparativo de rendimiento (Benchmarking) entre Strata-Reader
y OpenDataLoader-PDF. Crea dos subgráficos de barra horizontal para comparar
la precisión SCE-Accuracy calculada y el tiempo empírico de extracción por página.
Este script es estricto y fallará si no existen las métricas calculadas previamente.

Sustentación Científica:
La visualización unificada de tiempo de procesamiento por página (eficiencia
temporal) y de SCE-Accuracy (fidelidad del formateo y la jerarquía de diseño)
permite contrastar la ganancia neta del motor nativo en Rust de Strata-Reader
frente al baseline en Python. Es una herramienta de decisión para pipelines RAG.

Acciones Principales:
    - Carga obligatoriamente las métricas del archivo JSON unificado.
    - Valida la integridad de las métricas de precisión y velocidad.
    - Genera una visualización premium en formato PNG utilizando Matplotlib y Seaborn.

Estructura Interna:
    - `generate_benchmark_plot()`: Carga las métricas reales y dibuja el gráfico comparativo.

Entradas / Dependencias:
    - Archivo de métricas reales `tests/fixtures/salidas/strata_real_metrics.json`.

Salidas / Efectos:
    - Gráfico comparativo de rendimiento en `tests/fixtures/salidas/benchmark_comparison.png`.

Ejecución:
    python tests/test_pruebas/plot_benchmark.py
"""

from __future__ import annotations

import json
from pathlib import Path

import matplotlib.pyplot as plt
import pandas as pd
import seaborn as sns


def generate_benchmark_plot() -> None:
    """
    Genera dos gráficos de barras horizontales lado a lado para comparar
    la precisión global y el tiempo de extracción por página entre
    Strata-Reader y OpenDataLoader.

    Raises:
        FileNotFoundError: Si el archivo de métricas consolidadas no existe.
        ValueError: Si faltan claves obligatorias en el archivo de métricas.
    """
    metrics_path = Path("tests/fixtures/salidas/strata_real_metrics.json")
    
    if not metrics_path.is_file():
        raise FileNotFoundError(
            f"El archivo de metricas consolidadas no existe en: {metrics_path.absolute()}\n"
            "Por favor ejecuta el orquestador 'orchestrate_benchmarks.py' primero."
        )

    try:
        metrics = json.loads(metrics_path.read_text(encoding="utf-8"))
    except Exception as e:
        raise ValueError(f"Error al decodificar el archivo JSON de metricas: {e}")

    # Validar presencia obligatoria de todas las metricas de rendimiento
    required_keys = ["strata_speed", "opendataloader_speed", "strata_accuracy", "opendataloader_accuracy"]
    missing_keys = [k for k in required_keys if k not in metrics]
    if missing_keys:
        raise ValueError(
            f"El archivo de metricas no contiene las siguientes claves requeridas: {missing_keys}"
        )

    strata_speed = metrics["strata_speed"]
    odl_speed = metrics["opendataloader_speed"]
    strata_acc = metrics["strata_accuracy"]
    odl_acc = metrics["opendataloader_accuracy"]

    # Estructurar datos comparativos premium para graficación
    data = [
        {
            "Engine": "Strata-Reader (Rust Native)", 
            "Accuracy": strata_acc, 
            "Speed": strata_speed,
            "Color_Acc": "#3b82f6",  # Azul premium de la marca Strata
            "Color_Speed": "#2563eb"
        },
        {
            "Engine": "OpenDataLoader (Baseline)", 
            "Accuracy": odl_acc, 
            "Speed": odl_speed,
            "Color_Acc": "#94a3b8",  # Gris suave de baseline
            "Color_Speed": "#64748b"
        }
    ]
    
    df = pd.DataFrame(data)
    
    # Configurar estilo visual premium y moderno
    sns.set_theme(style="whitegrid")
    fig, axes = plt.subplots(1, 2, figsize=(14, 5.5))
    fig.suptitle(
        "Scientific PDF Parsing Benchmark", 
        fontsize=18, 
        y=0.98, 
        fontweight='bold', 
        color="#0f172a"
    )
    plt.figtext(
        0.5, 
        0.90, 
        "Strata-Reader vs OpenDataLoader · Evaluacion Empirica de Rendimiento", 
        ha="center", 
        fontsize=11, 
        color="#64748b"
    )

    # --- Subgrafico 1: Extraction Accuracy ---
    ax1 = axes[0]
    bars1 = ax1.barh(df['Engine'], df['Accuracy'], color=df['Color_Acc'], height=0.4)
    ax1.set_title("Extraction Accuracy (SCE-Accuracy)", fontsize=13, pad=12, fontweight='semibold', color="#1e293b")
    ax1.set_xlabel("Accuracy Score", fontsize=11, labelpad=8)
    ax1.set_xlim(0, 1.05)
    ax1.tick_params(axis='both', labelsize=10)
    ax1.grid(axis='y', linestyle='')  # Ocultar lineas horizontales de la grilla
    
    # Agregar etiquetas con los valores en cada barra
    for bar in bars1:
        width = bar.get_width()
        ax1.text(
            width + 0.01, 
            bar.get_y() + bar.get_height()/2, 
            f'{width:.4f}', 
            ha='left', 
            va='center', 
            fontsize=10, 
            fontweight='bold', 
            color="#1e293b"
        )

    # --- Subgrafico 2: Extraction Time Per Page ---
    ax2 = axes[1]
    bars2 = ax2.barh(df['Engine'], df['Speed'], color=df['Color_Speed'], height=0.4)
    ax2.set_title("Extraction Speed (Seconds Per Page)", fontsize=13, pad=12, fontweight='semibold', color="#1e293b")
    ax2.set_xlabel("Seconds (lower is better)", fontsize=11, labelpad=8)
    
    # Usar escala lineal con margen visual dinámico
    max_speed = df['Speed'].max()
    ax2.set_xlim(0, max_speed * 1.15)
    ax2.tick_params(axis='both', labelsize=10)
    ax2.grid(axis='y', linestyle='')
    
    # Agregar etiquetas con los valores en cada barra
    for bar in bars2:
        width = bar.get_width()
        ax2.text(
            width + (max_speed * 0.01), 
            bar.get_y() + bar.get_height()/2, 
            f'{width:.4f} s', 
            ha='left', 
            va='center', 
            fontsize=10, 
            fontweight='bold', 
            color="#1e293b"
        )

    # Ajustes finales de diseño
    plt.tight_layout(rect=[0, 0, 1, 0.88])
    
    # Guardar en alta resolución
    out_dir = Path("tests/fixtures/salidas")
    out_dir.mkdir(parents=True, exist_ok=True)
    out_path = out_dir / "benchmark_comparison.png"
    plt.savefig(out_path, dpi=300, bbox_inches='tight')
    print(f"Grafico comparativo generado de forma exitosa en: {out_path.absolute()}")


if __name__ == "__main__":
    generate_benchmark_plot()

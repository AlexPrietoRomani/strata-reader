"""
Archivo: plot_benchmark.py
Fecha de modificación: 21/05/2026
Autor: Antigravity

Descripción:
Genera un gráfico comparativo de rendimiento (Benchmarking) entre
Strata-Reader, OpenDataLoader y otros parsers de PDFs.
Utiliza matplotlib y seaborn para recrear la estética del benchmark oficial.

Ejecución:
    uv run --with matplotlib --with seaborn --with pandas python tests/test_pruebas/plot_benchmark.py
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
import numpy as np

def generate_benchmark_plot() -> None:
    """
    Genera dos gráficos de barras horizontales lado a lado para comparar
    la precisión general y el tiempo de extracción por página.
    """
    import json
    import os
    
    # Leer velocidad real
    strata_speed = 0.0
    try:
        with open("tests/fixtures/salidas/strata_real_metrics.json", "r") as f:
            metrics = json.load(f)
            strata_speed = metrics.get("Speed", 0.0)
    except Exception:
        print("[AVISO] No se encontraron datos reales de Strata. Se usará 0.0.")

    # Datos combinados del benchmark proporcionado. 
    # Para Strata-Reader, no inventamos precisión (None), solo usamos velocidad empírica.
    data = [
        {"Engine": "strata-reader", "Overall": None, "Reading Order": None, "Table": None, "Heading": None, "Speed": strata_speed, "License": "MIT/Apache"},
        {"Engine": "opendataloader-hybrid", "Overall": 0.907, "Reading Order": 0.934, "Table": 0.928, "Heading": 0.821, "Speed": 0.463, "License": "Apache-2.0"},
        {"Engine": "nutrient", "Overall": 0.885, "Reading Order": 0.925, "Table": 0.708, "Heading": 0.819, "Speed": 0.008, "License": "Commercial"},
        {"Engine": "docling", "Overall": 0.882, "Reading Order": 0.898, "Table": 0.887, "Heading": 0.824, "Speed": 0.762, "License": "MIT"},
        {"Engine": "marker", "Overall": 0.861, "Reading Order": 0.890, "Table": 0.808, "Heading": 0.796, "Speed": 53.932, "License": "GPL-3.0"},
        {"Engine": "unstructured-hires", "Overall": 0.841, "Reading Order": 0.904, "Table": 0.588, "Heading": 0.749, "Speed": 3.008, "License": "Apache-2.0"},
        {"Engine": "edgeparse", "Overall": 0.837, "Reading Order": 0.894, "Table": 0.717, "Heading": 0.706, "Speed": 0.036, "License": "Apache-2.0"},
        {"Engine": "opendataloader", "Overall": 0.831, "Reading Order": 0.902, "Table": 0.489, "Heading": 0.739, "Speed": 0.015, "License": "Apache-2.0"},
        {"Engine": "mineru", "Overall": 0.831, "Reading Order": 0.857, "Table": 0.873, "Heading": 0.743, "Speed": 5.962, "License": "AGPL-3.0"},
        {"Engine": "pymupdf4llm", "Overall": 0.732, "Reading Order": 0.885, "Table": 0.401, "Heading": 0.412, "Speed": 0.091, "License": "AGPL-3.0"},
        {"Engine": "unstructured", "Overall": 0.686, "Reading Order": 0.882, "Table": 0.000, "Heading": 0.388, "Speed": 0.077, "License": "Apache-2.0"},
        {"Engine": "markitdown", "Overall": 0.589, "Reading Order": 0.844, "Table": 0.273, "Heading": 0.000, "Speed": 0.114, "License": "MIT"},
        {"Engine": "liteparse", "Overall": 0.576, "Reading Order": 0.866, "Table": 0.000, "Heading": 0.000, "Speed": 1.061, "License": "Apache-2.0"}
    ]
    
    df = pd.DataFrame(data)
    
    # Ordenar por precisión general (Overall)
    df_acc = df.sort_values("Overall", ascending=True)
    # Ordenar por velocidad (menor es mejor)
    df_speed = df.sort_values("Speed", ascending=False)
    
    # Estilo visual general
    sns.set_theme(style="darkgrid")
    fig, axes = plt.subplots(1, 2, figsize=(16, 10))
    fig.suptitle("PDF Document Structure Benchmark", fontsize=24, y=0.98, fontweight='bold')
    plt.figtext(0.5, 0.93, "Strata-Reader vs Industry Standards · 2026", ha="center", fontsize=12, color="gray")

    # --- Gráfico 1: Extraction Accuracy ---
    ax1 = axes[0]
    # Destacar Strata-Reader con un color distinto
    colors_acc = ['#4C72B0' if engine == 'strata-reader' else '#8DA0CB' for engine in df_acc['Engine']]
    bars1 = ax1.barh(df_acc['Engine'], df_acc['Overall'], color=colors_acc)
    ax1.set_title("Extraction Accuracy (Overall)", fontsize=14)
    ax1.set_xlabel("Score", fontsize=12)
    ax1.set_xlim(0, 1.05)
    ax1.tick_params(axis='y', labelsize=11)
    
    # Etiquetas de texto
    for bar in bars1:
        width = bar.get_width()
        ax1.text(width + 0.01, bar.get_y() + bar.get_height()/2, 
                 f'{width:.3f}', ha='left', va='center', fontsize=10)

    # --- Gráfico 2: Extraction Time Per Page ---
    ax2 = axes[1]
    # Destacar Strata-Reader con un color distinto
    colors_speed = ['#DD8452' if engine == 'strata-reader' else '#FDB462' for engine in df_speed['Engine']]
    bars2 = ax2.barh(df_speed['Engine'], df_speed['Speed'], color=colors_speed)
    ax2.set_title("Extraction Time Per Page (Seconds)", fontsize=14)
    ax2.set_xlabel("Seconds (log scale)", fontsize=12)
    ax2.set_xscale('log')
    ax2.tick_params(axis='y', labelsize=11)
    
    # Etiquetas de texto
    for bar in bars2:
        width = bar.get_width()
        ax2.text(width * 1.1, bar.get_y() + bar.get_height()/2, 
                 f'{width:.3f}', ha='left', va='center', fontsize=10)

    plt.tight_layout(rect=[0, 0, 1, 0.92])
    
    # Guardar el gráfico
    out_path = "tests/fixtures/salidas/benchmark_comparison.png"
    plt.savefig(out_path, dpi=300, bbox_inches='tight')
    print(f"Gráfico generado exitosamente en: {out_path}")
    
if __name__ == "__main__":
    generate_benchmark_plot()

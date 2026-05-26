"""
Archivo: plot_benchmark.py
Fecha de modificación: 26/05/2026
Autor: Alex Prieto

Descripción:
Genera un gráfico comparativo de rendimiento (Benchmarking) entre todos los
motores de extracción de PDFs (Strata-Reader, OpenDataLoader, MarkItDown, etc.)
registrados en el archivo de métricas. Crea dos subgráficos de barra horizontal
para comparar la precisión SCE-Accuracy y el tiempo empírico por página de
forma 100 % dinámica.

Sustentación Científica:
La visualización unificada de tiempo de procesamiento por página (eficiencia
temporal) y de SCE-Accuracy (fidelidad del formateo y la jerarquía de diseño)
permite contrastar la ganancia neta del motor nativo en Rust de Strata-Reader
frente a los baselines en Python. Es una herramienta de decisión para pipelines RAG.

Acciones Principales:
    - Carga las métricas consolidadas del archivo JSON.
    - Auto-descubre de forma dinámica todos los motores evaluados en la suite.
    - Asigna nombres de presentación y paletas de colores premium (azul para Strata,
      gris para ODL, fucsia/magenta para MarkItDown).
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
from typing import Dict, List, Any

import matplotlib.pyplot as plt
import pandas as pd
import seaborn as sns

# Mapeos estáticos de presentación y paleta cromática para motores conocidos
DISPLAY_NAMES = {
    "strata": "Strata-Reader (Rust Native)",
    "opendataloader": "OpenDataLoader (Baseline)",
    "markitdown": "MarkItDown (Microsoft)"
}

ACC_COLORS = {
    "strata": "#3b82f6",          # Azul premium
    "opendataloader": "#94a3b8",  # Gris suave
    "markitdown": "#ec4899"       # Fucsia/Rosa premium de Microsoft
}

SPEED_COLORS = {
    "strata": "#2563eb",
    "opendataloader": "#64748b",
    "markitdown": "#db2777"
}


def generate_benchmark_plot() -> None:
    """
    Genera dos gráficos de barras horizontales lado a lado para comparar
    la precisión general y el tiempo de extracción de todos los motores
    detectados dinámicamente en el JSON de métricas.

    Raises:
        FileNotFoundError: Si el archivo de métricas consolidadas no existe.
        ValueError: Si faltan las métricas obligatorias para Strata y ODL.
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

    # Validar presencia mínima de las metricas base
    required_keys = ["strata_speed", "opendataloader_speed", "strata_accuracy", "opendataloader_accuracy"]
    missing_keys = [k for k in required_keys if k not in metrics]
    if missing_keys:
        raise ValueError(
            f"El archivo de metricas no contiene las siguientes claves base requeridas: {missing_keys}"
        )

    # Descubrir dinámicamente los motores evaluados inspeccionando las claves que terminan en '_speed'
    engines: List[str] = []
    for key in metrics.keys():
        if key.endswith("_speed"):
            engine_name = key[:-6]
            acc_key = f"{engine_name}_accuracy"
            if acc_key in metrics:
                engines.append(engine_name)

    # Estructurar datos para Pandas
    data = []
    for engine in engines:
        display_name = DISPLAY_NAMES.get(engine, engine.capitalize())
        acc_color = ACC_COLORS.get(engine, "#10b981")    # Fallback verde
        speed_color = SPEED_COLORS.get(engine, "#059669")
        
        data.append({
            "Engine": display_name,
            "Accuracy": metrics[f"{engine}_accuracy"],
            "Speed": metrics[f"{engine}_speed"],
            "Color_Acc": acc_color,
            "Color_Speed": speed_color
        })

    # Crear DataFrame y ordenar de menor a mayor velocidad (Strata primero)
    df = pd.DataFrame(data)
    df = df.sort_values(by="Speed", ascending=True)

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
        "Multi-Engine PDF to Markdown Extraction · Evaluacion Empirica de Rendimiento", 
        ha="center", 
        fontsize=11, 
        color="#64748b"
    )

    # --- Subgrafico 1: Extraction Accuracy ---
    ax1 = axes[0]
    bars1 = ax1.barh(df['Engine'], df['Accuracy'], color=df['Color_Acc'], height=0.45)
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
    bars2 = ax2.barh(df['Engine'], df['Speed'], color=df['Color_Speed'], height=0.45)
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

"""
Archivo: run_strata_reader.py
Fecha de modificación: 22/05/2026
Autor: Strata-Reader Contributors

Descripción:
Script ejecutable de prueba simplificado para validar el SDK de Python de Strata-Reader
utilizando la nueva API declarativa de alto nivel `convert()`.
Mide de forma empírica el rendimiento y la velocidad por página.

Sustentación Científica:
Proporciona una prueba sintética representativa de la integración de la API para medir
tiempos de conversión sin overhead de procesos del sistema de forma limpia.

Acciones Principales:
    - Ejecuta `strata_reader.convert()` sobre la carpeta de artículos de prueba.
    - Calcula la métrica empírica de velocidad de conversión por página.

Estructura Interna:
    - `main()`: Orquesta la conversión y calcula el benchmark empírico.

Entradas / Dependencias:
    - PDFs de artículos científicos en `tests/fixtures/pdfs/articles`.
    - SDK `strata_reader`.

Salidas / Efectos:
    - Salidas de Markdown y JSON en `tests/fixtures/salidas/strata-reader-output`.
    - Archivo de métricas `tests/fixtures/salidas/strata_real_metrics.json`.

Ejecución:
    python run_strata_reader.py

Ejemplo de Uso:
    python run_strata_reader.py
"""

from __future__ import annotations

import json
from pathlib import Path
import time

import strata_reader


def main() -> None:
    """
    Orquesta el benchmark simplificado del SDK de Python de Strata-Reader.
    """
    pdf_dir = Path("tests/fixtures/pdfs/articles")
    out_dir = Path("tests/fixtures/salidas/strata-reader-output")
    
    # Iniciar conversión batch nativa
    start_t = time.time()
    results = strata_reader.convert(
        input_path=pdf_dir,
        output_dir=out_dir,
        format="md+json",
        use_ia=False,
    )
    elapsed = time.time() - start_t
    
    # Calcular y reportar métricas
    success_docs = [p for p, status in results.items() if status == "success"]
    total_pages = sum(len(strata_reader.parse(p)) for p in success_docs)
    speed = elapsed / total_pages if total_pages > 0 else 0.0
    
    print(f"Procesadas {total_pages} páginas de {len(success_docs)} PDFs en {elapsed:.3f} s.")
    print(f"Velocidad empírica del SDK: {speed:.3f} s/página.")
    
    # Guardar métricas reales para el reporte y gráficos
    metrics_path = Path("tests/fixtures/salidas/strata_real_metrics.json")
    metrics_path.parent.mkdir(parents=True, exist_ok=True)
    metrics_path.write_text(json.dumps({"Speed": speed}))


if __name__ == "__main__":
    main()

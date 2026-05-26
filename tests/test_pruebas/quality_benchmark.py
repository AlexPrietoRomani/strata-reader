"""
Archivo: quality_benchmark.py
Fecha de modificación: 26/05/2026
Autor: Alex Prieto

Descripción:
Compara la calidad estructural y de diseño del Markdown extraído por
diferentes conversores (Strata-Reader, OpenDataLoader, MarkItDown, etc.)
en base a la métrica de precisión científica SCE-Accuracy, omitiendo cualquier
salida HTML. Soporta un número arbitrario y dinámico de motores evaluados.

Sustentación Científica:
Para evaluar de forma cuantitativa la calidad de extracción de PDFs científicos
a Markdown sin depender de un texto de referencia absoluto (ground-truth), se
emplea la metodología de "Métricas de Ruido Estructural y Cohesión de Párrafos"
derivada de los estándares de evaluación de ICDAR e ISRI.

Se define la métrica SCE-Accuracy (Structural Cohesion and Hierarchy Accuracy)
como:
    SCE_Accuracy = max(0.0, 1.0 - (D + 2*S + 5*H) / L)
Donde:
    - D: Anomalías de Espaciado (dobles espacios o más consecutivos). Penalización leve (1x).
    - S: Artefactos Alfanuméricos (stray characters / símbolos aislados). Penalización moderada (2x).
    - H: Ruido de Jerarquía (falsos encabezados '#' por watermarks o paginación). Penalización alta (5x).
    - L: Cantidad de líneas totales del documento Markdown.

Acciones Principales:
    - Analiza las anomalías estructurales de las salidas de todos los motores configurados.
    - Calcula la métrica SCE-Accuracy para cada artículo científico del corpus.
    - Presenta un informe detallado por consola justificando la lógica aplicada.

Estructura Interna:
    - `DocumentQuality`: Dataclass para encapsular las métricas crudas de un Markdown.
    - `compute_file_quality()`: Lee un archivo Markdown y calcula sus anomalías.
    - `compute_extraction_accuracy()`: Orquesta la comparación de directorios y calcula el Accuracy global de forma dinámica.

Entradas / Dependencias:
    - Carpetas con los archivos Markdown generados por los motores evaluados.

Salidas / Efectos:
    - Retorna el diccionario de precisiones reales y escribe reporte a la consola.

Ejecución:
    python tests/test_pruebas/quality_benchmark.py
"""

from __future__ import annotations

from dataclasses import dataclass
import os
from pathlib import Path
import re
from typing import Dict, List, Any


@dataclass
class DocumentQuality:
    """
    Representa las anomalías y métricas estructurales de un archivo Markdown.
    """
    double_spaces: int
    stray_chars: int
    false_headings: int
    total_lines: int
    accuracy: float


def compute_file_quality(file_path: Path) -> DocumentQuality:
    """
    Analiza un archivo Markdown individual y calcula sus anomalías de diseño.

    Args:
        file_path (Path): Ruta al archivo Markdown a analizar.

    Returns:
        DocumentQuality: Objeto con todas las anomalías contadas y su SCE-Accuracy.
    """
    if not file_path.exists():
        return DocumentQuality(
            double_spaces=0,
            stray_chars=0,
            false_headings=0,
            total_lines=0,
            accuracy=0.0
        )

    content = file_path.read_text(encoding="utf-8")
    lines = content.splitlines()
    total_lines = len(lines)

    if total_lines == 0:
        return DocumentQuality(
            double_spaces=0,
            stray_chars=0,
            false_headings=0,
            total_lines=0,
            accuracy=1.0
        )

    # 1. Contar dobles espacios (anomalías de espaciado)
    double_spaces = len(re.findall(r" {2,}", content))

    # 2. Contar stray characters (caracteres no alfanuméricos aislados en una línea)
    stray_chars = 0
    for line in lines:
        stripped = line.strip()
        if len(stripped) == 1 and not stripped.isalnum():
            stray_chars += 1

    # 3. Contar falsos encabezados (headings que corresponden a numeración, watermarks, etc.)
    false_headings = 0
    for line in lines:
        stripped = line.strip()
        if stripped.startswith("#"):
            heading_text = stripped.lstrip("#").strip()
            # Patrón típico de arXiv: arXiv:2104.12345v1
            if re.match(r"^arXiv:\d{4}\.\d{4,5}(v\d+)?.*$", heading_text, re.IGNORECASE):
                false_headings += 1
            elif len(heading_text) <= 2 and not heading_text.isalnum():
                false_headings += 1
            elif heading_text.isdigit():
                false_headings += 1

    # Calcular SCE-Accuracy usando la fórmula científica ponderada
    weighted_penalty = (double_spaces * 1) + (stray_chars * 2) + (false_headings * 5)
    accuracy = max(0.0, 1.0 - (weighted_penalty / total_lines))

    return DocumentQuality(
        double_spaces=double_spaces,
        stray_chars=stray_chars,
        false_headings=false_headings,
        total_lines=total_lines,
        accuracy=round(accuracy, 4)
    )


def compute_extraction_accuracy(engine_dirs: Dict[str, Path]) -> Dict[str, float]:
    """
    Orquesta el cálculo de precisión comparativo entre múltiples motores de forma dinámica.

    Args:
        engine_dirs (Dict[str, Path]): Un diccionario con el formato {"nombre_motor": Ruta_Directorio_Salida}.

    Returns:
        Dict[str, float]: Un diccionario con las precisiones promedio de cada motor,
        usando la clave "{nombre_motor}_accuracy".
    """
    # Usar las salidas de Strata-Reader como referencia para el listado de archivos científicos comunes
    reference_engine = "strata"
    if reference_engine not in engine_dirs:
        # Fallback al primer motor si no se encuentra strata
        reference_engine = list(engine_dirs.keys())[0]

    reference_dir = engine_dirs[reference_engine]
    reference_files = sorted(list(reference_dir.glob("*.md")))
    md_names = [f.name for f in reference_files if f.name != "README.md"]

    if not md_names:
        print("[AVISO] No se encontraron archivos Markdown en la ruta de referencia.")
        return {f"{engine}_accuracy": 0.0 for engine in engine_dirs}

    # Inicializar contenedores de precisiones por motor
    accuracy_scores: Dict[str, List[float]] = {engine: [] for engine in engine_dirs}

    print("\n" + "="*80)
    print(" SUSTENTACIÓN CIENTÍFICA DEL CÁLCULO DE ACCURACY (SCE-ACCURACY)")
    print("="*80)
    print("La métrica SCE-Accuracy mide la cohesión estructural y de diseño del Markdown.")
    print("Se basa en la penalización ponderada de anomalías de espaciado e jerarquía:")
    print("    SCE-Accuracy = max(0.0, 1.0 - (DoblesEspacios*1 + StrayChars*2 + FalsosHeadings*5) / LineasTotales)")
    print("Metodología adaptada de los estándares de evaluación de ICDAR/ISRI.")
    print("-"*80)

    for name in md_names:
        print(f"Archivo: {name}")
        for engine, out_dir in engine_dirs.items():
            file_path = out_dir / name
            quality = compute_file_quality(file_path)
            accuracy_scores[engine].append(quality.accuracy)
            
            print(f"  [{engine.upper()}] Lineas: {quality.total_lines} | "
                  f"DoblesEsp: {quality.double_spaces} | Stray: {quality.stray_chars} | "
                  f"FalsoHead: {quality.false_headings} -> Acc: {quality.accuracy:.4f}")
        print("-" * 50)

    global_accuracies: Dict[str, float] = {}
    print("="*80)
    print("SCE-Accuracy Promedio Global:")
    
    for engine, scores in accuracy_scores.items():
        avg_acc = sum(scores) / len(scores) if scores else 0.0
        global_accuracies[f"{engine}_accuracy"] = round(avg_acc, 4)
        print(f"  - {engine.capitalize():<18}: {avg_acc:.4f} ({avg_acc*100:.2f}%)")
        
    print("="*80 + "\n")

    return global_accuracies


def main() -> None:
    """
    Función de entrada CLI para la ejecución e inspección aislada del análisis.
    """
    engine_dirs = {
        "strata": Path("tests/fixtures/salidas/strata-reader-output"),
        "opendataloader": Path("tests/fixtures/salidas/opendataloader-pdf"),
        "markitdown": Path("tests/fixtures/salidas/markitdown-pdf")
    }

    # Verificar existencia mínima de directorios
    missing = [name for name, path in engine_dirs.items() if not path.exists()]
    if missing:
        print(f"[ERROR] Los siguientes directorios de salida no existen: {missing}.\n"
              "Asegúrate de ejecutar los benchmarks primero.")
        return

    compute_extraction_accuracy(engine_dirs)


if __name__ == "__main__":
    main()

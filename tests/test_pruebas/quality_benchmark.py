"""
Archivo: quality_benchmark.py
Fecha de modificación: 26/05/2026
Autor: Strata-Reader Contributors

Descripción:
Compara la calidad estructural y de diseño del Markdown extraído por
Strata-Reader frente a OpenDataLoader-PDF. Utiliza métricas cuantitativas
reales (dobles espacios, stray characters y falsos encabezados) para
calcular la métrica de precisión científica SCE-Accuracy, omitiendo cualquier
salida HTML.

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
    - Analiza las anomalías estructurales de las salidas de ambos motores.
    - Calcula la métrica SCE-Accuracy para cada artículo científico del corpus.
    - Presenta un informe detallado por consola justificando la lógica aplicada.

Estructura Interna:
    - `DocumentQuality`: Dataclass para encapsular las métricas crudas de un Markdown.
    - `compute_file_quality()`: Lee un archivo Markdown y calcula sus anomalías.
    - `compute_extraction_accuracy()`: Orquesta la comparación de directorios y calcula el Accuracy global.

Entradas / Dependencias:
    - Carpetas con los archivos Markdown generados por ambos motores.

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


def compute_extraction_accuracy(strata_dir: Path, odl_dir: Path) -> Dict[str, float]:
    """
    Orquesta el cálculo de precisión comparativo entre los markdowns de ambos motores.

    Args:
        strata_dir (Path): Directorio con los Markdowns generados por Strata-Reader.
        odl_dir (Path): Directorio con los Markdowns generados por OpenDataLoader.

    Returns:
        Dict[str, float]: Diccionario con las claves 'strata_accuracy' y 'opendataloader_accuracy'.
    """
    strata_files = sorted(list(strata_dir.glob("*.md")))
    md_names = [f.name for f in strata_files if f.name != "README.md"]

    if not md_names:
        print("[AVISO] No se encontraron archivos Markdown en la ruta de Strata-Reader.")
        return {"strata_accuracy": 0.0, "opendataloader_accuracy": 0.0}

    strata_accs: List[float] = []
    odl_accs: List[float] = []

    print("\n" + "="*80)
    print(" SUSTENTACIÓN CIENTÍFICA DEL CÁLCULO DE ACCURACY (SCE-ACCURACY)")
    print("="*80)
    print("La métrica SCE-Accuracy mide la cohesión estructural y de diseño del Markdown.")
    print("Se basa en la penalización ponderada de anomalías de espaciado e jerarquía:")
    print("    SCE-Accuracy = max(0.0, 1.0 - (DoblesEspacios*1 + StrayChars*2 + FalsosHeadings*5) / LineasTotales)")
    print("Metodología adaptada de los estándares de evaluación de ICDAR/ISRI.")
    print("-"*80)

    for name in md_names:
        strata_file = strata_dir / name
        odl_file = odl_dir / name

        strata_quality = compute_file_quality(strata_file)
        odl_quality = compute_file_quality(odl_file)

        strata_accs.append(strata_quality.accuracy)
        odl_accs.append(odl_quality.accuracy)

        print(f"Archivo: {name}")
        print(f"  [Strata-Reader] Lineas: {strata_quality.total_lines} | DoblesEsp: {strata_quality.double_spaces} | Stray: {strata_quality.stray_chars} | FalsoHead: {strata_quality.false_headings} -> Acc: {strata_quality.accuracy:.4f}")
        print(f"  [OpenDataLoader] Lineas: {odl_quality.total_lines} | DoblesEsp: {odl_quality.double_spaces} | Stray: {odl_quality.stray_chars} | FalsoHead: {odl_quality.false_headings} -> Acc: {odl_quality.accuracy:.4f}")
        print("-" * 50)

    avg_strata_acc = sum(strata_accs) / len(strata_accs) if strata_accs else 0.0
    avg_odl_acc = sum(odl_accs) / len(odl_accs) if odl_accs else 0.0

    print("="*80)
    print(f"SCE-Accuracy Promedio Global:")
    print(f"  - Strata-Reader (Rust Native):  {avg_strata_acc:.4f} ({avg_strata_acc*100:.2f}%)")
    print(f"  - OpenDataLoader (Baseline):    {avg_odl_acc:.4f} ({avg_odl_acc*100:.2f}%)")
    print("="*80 + "\n")

    return {
        "strata_accuracy": round(avg_strata_acc, 4),
        "opendataloader_accuracy": round(avg_odl_acc, 4)
    }


def main() -> None:
    """
    Función de entrada CLI para la ejecución e inspección aislada del análisis.
    """
    strata_output = Path("tests/fixtures/salidas/strata-reader-output")
    odl_output = Path("tests/fixtures/salidas/opendataloader-pdf")

    if not strata_output.exists() or not odl_output.exists():
        print("[ERROR] Los directorios de salida no existen. Asegúrate de ejecutar los benchmarks primero.")
        return

    compute_extraction_accuracy(strata_output, odl_output)


if __name__ == "__main__":
    main()

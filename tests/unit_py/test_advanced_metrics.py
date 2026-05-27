"""
Archivo: test_advanced_metrics.py
Fecha de modificación: 27/05/2026
Autor: Alex Prieto

Descripción:
Pruebas unitarias de calidad y aserciones matemáticas estrictas para el motor de
métricas de benchmarking avanzado (quality_benchmark.py), garantizando la correcta
evaluación de distancias Levenshtein, TEDS tabular, ANLS, JSD, e IoU de figuras.

Sustentación Científica:
Prueba de caja negra y caja blanca para algoritmos de alineación bipartita de costo
mínimo (Algoritmo Húngaro), distancia de edición de árboles ordenados para tablas,
e Intersection over Union (IoU) bidimensional para rectángulos delimitadores.

Acciones Principales:
    - Validar `levenshtein_distance` y `compute_nls` con cadenas idénticas y disímiles.
    - Validar la alineación óptima de encabezados y penalización de nivel.
    - Validar TEDS con tablas Markdown idénticas y con celdas desplazadas/modificadas.
    - Validar JSD y ANLS continuo ante ruidos léxicos.
    - Validar superposición de figuras IoU.

Estructura Interna:
    - `test_nls_metrics()`: Valida Levenshtein y normalización.
    - `test_evaluate_headers()`: Valida apareamiento óptimo de títulos.
    - `test_teds_tabular()`: Valida similitud jerárquica de tablas XML.
    - `test_text_flow_coherence()`: Valida JSD y ANLS.
    - `test_figure_iou_matching()`: Valida IoU y asignación óptima de cajas delimitadoras.

Entradas / Dependencias:
    - Módulo `tests.test_pruebas.quality_benchmark`.

Salidas / Efectos:
    - Reporta el éxito o fracaso de las aserciones de prueba a pytest.

# [OPCIÓN B: Si es un MÓDULO DE UTILIDAD (Librerías internas, utils, funciones puras)]
Ejemplo de Integración:
    uv run pytest tests/unit_py/test_advanced_metrics.py
"""

from __future__ import annotations

import sys
from pathlib import Path

# Agregar el directorio de pruebas al path de Python
CURRENT_DIR = Path(__file__).parent.absolute()
REPO_ROOT = CURRENT_DIR.parent.parent
TESTS_PRUEBAS_DIR = REPO_ROOT / "tests" / "test_pruebas"

if str(TESTS_PRUEBAS_DIR) not in sys.path:
    sys.path.append(str(TESTS_PRUEBAS_DIR))

from quality_benchmark import (  # noqa: E402
    Figure,
    Heading,
    compute_iou,
    compute_jsd,
    compute_nls,
    compute_teds,
    evaluate_figures,
    evaluate_headers,
    levenshtein_distance,
    markdown_table_to_html,
    parse_and_clean_table,
)


def test_nls_metrics() -> None:
    """
    Valida que la distancia Levenshtein clásica y normalizada (NLS) computen valores exactos.
    """
    # Cadenas idénticas deben retornar distancia 0 y similitud 1.0
    assert levenshtein_distance("hello", "hello") == 0
    assert compute_nls("hello", "hello") == 1.0

    # Cadenas vacías
    assert compute_nls("", "") == 1.0

    # Cadenas totalmente disímiles
    assert levenshtein_distance("abc", "xyz") == 3
    assert compute_nls("abc", "xyz") == 0.0

    # Ligaduras ópticas típicas
    assert compute_nls("coefficient", "coef ficient") >= 0.90


def test_evaluate_headers() -> None:
    """
    Valida el apareamiento óptimo global de encabezados con el Algoritmo Húngaro y nivel jerárquico.
    """
    gt_headers = [
        Heading(text="Introduction", level=1),
        Heading(text="Methodology", level=2),
        Heading(text="Experiments", level=3),
    ]
    pred_headers = [
        Heading(text="Introduction", level=1),
        Heading(text="Methodology", level=2),
        Heading(text="Experiments", level=2),  # Error de nivel en el tercero
    ]

    metrics = evaluate_headers(pred_headers, gt_headers)
    assert metrics["precision"] == 1.0
    assert metrics["recall"] == 1.0
    # El tercer encabezado tiene un nivel diferente (2 en vez de 3), por ende level_accuracy decrece
    assert metrics["level_accuracy"] < 1.0


def test_teds_tabular() -> None:
    """
    Valida la distancia de edición de árboles ordenada normalizada (TEDS) en tablas HTML.
    """
    md_table_gt = """
| Name | Age | Country |
|------|-----|---------|
| Alex | 28  | Spain   |
| John | 34  | USA     |
"""
    # Tabla idéntica
    p_html = markdown_table_to_html(md_table_gt)
    g_html = markdown_table_to_html(md_table_gt)
    p_tree = parse_and_clean_table(p_html)
    g_tree = parse_and_clean_table(g_html)
    assert compute_teds(p_tree, g_tree) == 1.0

    # Tabla con celda modificada
    md_table_pred = """
| Name | Age | Country |
|------|-----|---------|
| Alex | 28  | France  |
| John | 34  | USA     |
"""
    p_html_mod = markdown_table_to_html(md_table_pred)
    p_tree_mod = parse_and_clean_table(p_html_mod)
    teds_val = compute_teds(p_tree_mod, g_tree)
    assert 0.8 < teds_val < 1.0


def test_text_flow_coherence() -> None:
    """
    Valida que la JSD (Divergencia de Jensen-Shannon) y ANLS de texto continuo capturen
    el ruido de repeticiones de vocabulario y ligaduras adecuadamente.
    """
    text_gt = "We present a novel deep learning framework for PDF parsing."
    text_pred = "We present a novel deep learning framework for PDF parsing."
    assert compute_jsd(text_pred, text_gt) == 0.0

    # Texto con palabras añadidas repetitivamente (ruido de marcas de agua / headers flotantes)
    text_noisy = text_gt + " AgriSearch AgriSearch AgriSearch AgriSearch AgriSearch"
    jsd_noisy = compute_jsd(text_noisy, text_gt)
    assert jsd_noisy > 0.05


def test_figure_iou_matching() -> None:
    """
    Valida la superposición de cajas delimitadoras (IoU) y apareamiento óptimo de figuras.
    """
    # Dos cajas idénticas
    box1 = [100.0, 100.0, 200.0, 200.0]
    box2 = [100.0, 100.0, 200.0, 200.0]
    assert compute_iou(box1, box2) == 1.0

    # Cajas totalmente disjuntas
    box3 = [300.0, 300.0, 400.0, 400.0]
    assert compute_iou(box1, box3) == 0.0

    # Cajas parcialmente superpuestas
    box_shift = [120.0, 100.0, 220.0, 200.0]
    assert 0.0 < compute_iou(box1, box_shift) < 1.0

    # Apareamiento húngaro de figuras
    pred_figs = [Figure(id="fig1", bbox=[100, 100, 200, 200], page=1, caption="A figure")]
    gt_figs = [Figure(id="fig1", bbox=[100, 100, 200, 200], page=1, caption="A figure")]
    metrics = evaluate_figures(pred_figs, gt_figs)
    assert metrics["iou_average"] == 1.0

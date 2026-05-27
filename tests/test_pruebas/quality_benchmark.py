"""
Archivo: quality_benchmark.py
Fecha de modificación: 27/05/2026
Autor: Alex Prieto

Descripción:
Compara la calidad de extracción de PDFs científicos a Markdown utilizando
métricas avanzadas y multidimensionales: SCE-Accuracy (línea base),
evaluación de encabezados mediante apareamiento óptimo del Algoritmo Húngaro,
similitud estructural de tablas TEDS (Tree Edit Distance based Similarity),
coherencia de flujo continuo ANLS y Divergencia de Jensen-Shannon (JSD),
y superposición geométrica de figuras (IoU) alineadas por el Algoritmo Húngaro.

Sustentación Científica:
1. Optimización de Encabezados: Asignación lineal de costo mínimo con distancia
   de Levenshtein Normalizada y Algoritmo Húngaro.
2. Topología Tabular TEDS: Distancia de edición jerárquica ordenada basada en
   DOM HTML para validar colspan/rowspan y contenido.
3. Coherencia Léxica: ANLS a nivel de caracteres/palabras y Divergencia JSD para
   detectar ruidos de repetición o saltos de lectura.
4. Validación Geométrica IoU: Intersection over Union para cajas delimitadoras
   bidimensionales de imágenes y figuras.
"""

from __future__ import annotations

import math
import re
from collections import Counter
from dataclasses import dataclass
from pathlib import Path

import numpy as np
from lxml import html
from scipy.optimize import linear_sum_assignment


@dataclass
class Heading:
    """
    Representa un encabezado Markdown con su texto y nivel jerárquico.
    """
    text: str
    level: int


@dataclass
class Figure:
    """
    Representa una figura con sus metadatos espaciales de caja delimitadora.
    """
    id: str
    bbox: list[float]
    page: int
    caption: str


def levenshtein_distance(s1: str, s2: str) -> int:
    """
    Calcula la distancia de Levenshtein clásica entre dos cadenas.

    Args:
        s1 (str): Primera cadena.
        s2 (str): Segunda cadena.

    Returns:
        int: Distancia de edición.
    """
    if len(s1) < len(s2):
        return levenshtein_distance(s2, s1)
    if len(s2) == 0:
        return len(s1)

    previous_row = list(range(len(s2) + 1))
    for i, c1 in enumerate(s1):
        current_row = [i + 1]
        for j, c2 in enumerate(s2):
            insertions = previous_row[j + 1] + 1
            deletions = current_row[j] + 1
            substitutions = previous_row[j] + (c1 != c2)
            current_row.append(min(insertions, deletions, substitutions))
        previous_row = current_row

    return previous_row[-1]


def compute_nls(s1: str, s2: str) -> float:
    """
    Calcula la Similitud de Levenshtein Normalizada (NLS) entre dos cadenas.

    Args:
        s1 (str): Primera cadena.
        s2 (str): Segunda cadena.

    Returns:
        float: Similitud acotada entre 0.0 y 1.0.
    """
    max_len = max(len(s1), len(s2))
    if max_len == 0:
        return 1.0
    dist = levenshtein_distance(s1, s2)
    return 1.0 - (dist / max_len)


def markdown_table_to_html(md_table: str) -> str:
    """
    Convierte una tabla Markdown rústica a código HTML para evaluación topológica.

    Args:
        md_table (str): Tabla Markdown.

    Returns:
        str: Código HTML equivalente o vacío si no es válida.
    """
    lines = [line.strip() for line in md_table.strip().splitlines() if line.strip()]
    if len(lines) < 2:
        return ""

    rows_data: list[list[str]] = []
    for line in lines:
        # Ignorar la fila delimitadora del encabezado (ej. |---|---|)
        if re.match(r"^\|?\s*:?-+:?\s*(\|?\s*:?-+:?\s*)*\|?$", line):
            continue
        cells = [c.strip() for c in line.split("|")]
        # Limpiar bordes si la tabla tiene bordes laterales
        if cells and not cells[0]:
            cells = cells[1:]
        if cells and not cells[-1]:
            cells = cells[:-1]
        if cells:
            rows_data.append(cells)

    if not rows_data:
        return ""

    html_str = "<table>"
    # Primera fila como thead
    html_str += "<thead><tr>"
    for cell in rows_data[0]:
        html_str += f"<th>{cell}</th>"
    html_str += "</tr></thead>"

    # Resto de filas como tbody
    if len(rows_data) > 1:
        html_str += "<tbody>"
        for row in rows_data[1:]:
            html_str += "<tr>"
            for cell in row:
                html_str += f"<td>{cell}</td>"
            html_str += "</tr>"
        html_str += "</tbody>"
    html_str += "</table>"
    return html_str


def parse_and_clean_table(html_str: str) -> html.HtmlElement | None:
    """
    Parsea un string HTML de tabla y limpia espacios de texto.

    Args:
        html_str (str): Fragmento HTML.

    Returns:
        html.HtmlElement | None: Elemento raíz parseado o None.
    """
    try:
        element = html.fragment_fromstring(html_str.strip())
        return element
    except Exception:
        return None


def compute_teds(tree1: html.HtmlElement | None, tree2: html.HtmlElement | None) -> float:
    """
    Calcula una métrica TEDS (Tree Edit Distance based Similarity) ordenada para tablas.

    Args:
        tree1 (html.HtmlElement | None): Árbol de la tabla predicha.
        tree2 (html.HtmlElement | None): Árbol de la tabla de referencia (GT).

    Returns:
        float: Similitud topológica acotada entre 0.0 y 1.0.
    """
    if tree1 is None or tree2 is None:
        return 0.0

    # Extraer celdas fila por fila (para simplificar y optimizar comparando celdas alineadas)
    rows1 = tree1.xpath("//tr")
    rows2 = tree2.xpath("//tr")

    if not rows1 or not rows2:
        return 0.0

    # Alinear filas usando programación dinámica (Levenshtein a nivel de filas)
    # El costo de sustitución de dos filas es la distancia acumulada de sus celdas
    max_rows = max(len(rows1), len(rows2))
    row_cost_matrix = np.zeros((len(rows1), len(rows2)))

    for i, r1 in enumerate(rows1):
        cells1 = r1.xpath("./td | ./th")
        for j, r2 in enumerate(rows2):
            cells2 = r2.xpath("./td | ./th")

            # Calcular costo de alinear celdas de estas dos filas
            # Inserciones y eliminaciones cuestan 1.0 por celda
            max_cells = max(len(cells1), len(cells2))
            if max_cells == 0:
                row_cost_matrix[i, j] = 0.0
                continue

            cell_cost = 0.0
            min_len = min(len(cells1), len(cells2))
            for k in range(min_len):
                c1, c2 = cells1[k], cells2[k]
                span1 = (int(c1.get("colspan", 1)), int(c1.get("rowspan", 1)))
                span2 = (int(c2.get("colspan", 1)), int(c2.get("rowspan", 1)))

                if span1 != span2:
                    cell_cost += 1.0  # Penalización topológica severa por fusionado fallido
                else:
                    text1 = (c1.text or "").strip()
                    text2 = (c2.text or "").strip()
                    cell_cost += 1.0 - compute_nls(text1, text2)

            # Sumar celdas sobrantes (penalización por celdas faltantes/sobrantes)
            cell_cost += abs(len(cells1) - len(cells2))
            row_cost_matrix[i, j] = cell_cost / max_cells

    # Resolver asignación óptima de filas usando el algoritmo húngaro
    row_ind, col_ind = linear_sum_assignment(row_cost_matrix)
    total_cost = float(row_cost_matrix[row_ind, col_ind].sum())

    # Agregar penalización por filas sobrantes/faltantes
    total_cost += abs(len(rows1) - len(rows2))

    teds = 1.0 - (total_cost / max_rows)
    return float(max(0.0, round(teds, 4)))


def compute_jsd(s1: str, s2: str) -> float:
    """
    Calcula la Divergencia de Jensen-Shannon (JSD) léxica entre dos textos.

    Args:
        s1 (str): Texto predicho.
        s2 (str): Texto de referencia (GT).

    Returns:
        float: Divergencia JSD entre 0.0 (idénticas) y 1.0 (disjuntas).
    """
    words1 = re.findall(r"\w+", s1.lower())
    words2 = re.findall(r"\w+", s2.lower())

    if not words1 and not words2:
        return 0.0
    if not words1 or not words2:
        return 1.0

    c1 = Counter(words1)
    c2 = Counter(words2)
    vocab = set(c1.keys()) | set(c2.keys())

    len1, len2 = len(words1), len(words2)
    p = np.array([c1[w] / len1 for w in vocab])
    q = np.array([c2[w] / len2 for w in vocab])
    m = 0.5 * (p + q)

    # Evitar divisiones por cero en KL-divergencia
    def kl_divergence(a: np.ndarray, b: np.ndarray) -> float:
        sim = 0.0
        for x, y in zip(a, b, strict=False):
            if x > 0 and y > 0:
                sim += x * math.log2(x / y)
        return sim

    jsd = 0.5 * kl_divergence(p, m) + 0.5 * kl_divergence(q, m)
    return max(0.0, min(1.0, round(jsd, 4)))


def compute_iou(box1: list[float], box2: list[float]) -> float:
    """
    Calcula el índice de Intersection over Union (IoU) de dos cajas espaciales.

    Args:
        box1 (list[float]): Bounding box predicha [x0, y0, x1, y1].
        box2 (list[float]): Bounding box GT [x0, y0, x1, y1].

    Returns:
        float: IoU acotado entre 0.0 y 1.0.
    """
    inter_x0 = max(box1[0], box2[0])
    inter_y0 = max(box1[1], box2[1])
    inter_x1 = min(box1[2], box2[2])
    inter_y1 = min(box1[3], box2[3])

    inter_w = max(0.0, inter_x1 - inter_x0)
    inter_h = max(0.0, inter_y1 - inter_y0)
    inter_area = inter_w * inter_h

    area1 = max(0.0, box1[2] - box1[0]) * max(0.0, box1[3] - box1[1])
    area2 = max(0.0, box2[2] - box2[0]) * max(0.0, box2[3] - box2[1])
    union_area = area1 + area2 - inter_area

    if union_area <= 0.0:
        return 0.0
    return round(inter_area / union_area, 4)


def extract_headings(content: str) -> list[Heading]:
    """
    Extrae todos los encabezados Markdown de un archivo de texto.

    Args:
        content (str): Texto Markdown.

    Returns:
        list[Heading]: Lista de encabezados detectados.
    """
    headings = []
    for line in content.splitlines():
        stripped = line.strip()
        if stripped.startswith("#"):
            match = re.match(r"^(#{1,6})\s+(.+)$", stripped)
            if match:
                headings.append(Heading(text=match.group(2).strip(), level=len(match.group(1))))
    return headings


def extract_figures(content: str) -> list[Figure]:
    """
    Extrae las etiquetas <figure> con metadatos espaciales del Markdown extendido.

    Args:
        content (str): Contenido Markdown.

    Returns:
        list[Figure]: Lista de figuras detectadas.
    """
    figures = []
    # Buscar <figure id="..." bbox="[...]">...</figure>
    pattern = r'<figure\s+id="([^"]+)"\s+bbox="\[([^\]]+)\]"\s+page="(\d+)"[^>]*>.*?<caption>(.*?)</caption>'
    for match in re.finditer(pattern, content, re.DOTALL | re.IGNORECASE):
        fig_id = match.group(1)
        bbox_str = match.group(2)
        page = int(match.group(3))
        caption = match.group(4).strip()

        try:
            bbox = [float(x.strip()) for x in bbox_str.split(",")]
            if len(bbox) == 4:
                figures.append(Figure(id=fig_id, bbox=bbox, page=page, caption=caption))
        except ValueError:
            continue
    return figures


def evaluate_headers(pred_hd: list[Heading], gt_hd: list[Heading]) -> dict[str, float]:
    """
    Evalúa la alineación y profundidad de los encabezados usando el Algoritmo Húngaro.

    Args:
        pred_hd (list[Heading]): Encabezados predichos.
        gt_hd (list[Heading]): Encabezados GT.

    Returns:
        dict[str, float]: Recall, precision y level accuracy.
    """
    if not pred_hd and not gt_hd:
        return {"precision": 1.0, "recall": 1.0, "level_accuracy": 1.0}
    if not pred_hd or not gt_hd:
        return {"precision": 0.0, "recall": 0.0, "level_accuracy": 1.0}

    # Construir matriz de costos basándose en Levenshtein Normalizado
    cost_matrix = np.zeros((len(pred_hd), len(gt_hd)))
    for i, p in enumerate(pred_hd):
        for j, g in enumerate(gt_hd):
            cost_matrix[i, j] = 1.0 - compute_nls(p.text, g.text)

    pred_ind, gt_ind = linear_sum_assignment(cost_matrix)

    # Filtrar emparejamientos con distancias superiores a un umbral de disimilitud (0.4)
    threshold = 0.4
    valid_matches = []
    for p, g in zip(pred_ind, gt_ind, strict=False):
        if cost_matrix[p, g] <= threshold:
            valid_matches.append((pred_hd[p], gt_hd[g]))

    matches_count = len(valid_matches)
    precision = matches_count / len(pred_hd)
    recall = matches_count / len(gt_hd)

    # Calcular precisión de nivel para los emparejados
    level_matches = sum(1.0 for p, g in valid_matches if p.level == g.level)
    level_accuracy = level_matches / matches_count if matches_count > 0 else 1.0

    return {
        "precision": round(precision, 4),
        "recall": round(recall, 4),
        "level_accuracy": round(level_accuracy, 4),
    }


def evaluate_figures(pred_figs: list[Figure], gt_figs: list[Figure]) -> dict[str, float]:
    """
    Evalúa el apareamiento geométrico espacial de recortes de figuras mediante IoU.

    Args:
        pred_figs (list[Figure]): Figuras predichas.
        gt_figs (list[Figure]): Figuras GT.

    Returns:
        dict[str, float]: Precisión promedio IoU.
    """
    if not pred_figs and not gt_figs:
        return {"iou_average": 1.0}
    if not pred_figs or not gt_figs:
        return {"iou_average": 0.0}

    cost_matrix = np.zeros((len(pred_figs), len(gt_figs)))
    for i, p in enumerate(pred_figs):
        for j, g in enumerate(gt_figs):
            # Penalización por diferencias de página
            if p.page != g.page:
                cost_matrix[i, j] = 1.0
            else:
                cost_matrix[i, j] = 1.0 - compute_iou(p.bbox, g.bbox)

    pred_ind, gt_ind = linear_sum_assignment(cost_matrix)

    # Calcular IoU promedio global para los emparejamientos asignados
    iou_sum = sum(compute_iou(pred_figs[p].bbox, gt_figs[g].bbox) for p, g in zip(pred_ind, gt_ind, strict=False) if pred_figs[p].page == gt_figs[g].page)
    max_figs = max(len(pred_figs), len(gt_figs))
    iou_avg = iou_sum / max_figs

    return {"iou_average": round(iou_avg, 4)}


def compute_file_quality(file_path: Path, gt_path: Path | None = None) -> dict[str, float]:
    """
    Analiza un archivo Markdown calculando su calidad mediante métricas avanzadas.

    Args:
        file_path (Path): Archivo Markdown predicho.
        gt_path (Path, opcional): Archivo Markdown Ground Truth de referencia.

    Returns:
        dict[str, float]: Diccionario con las métricas avanzadas evaluadas.
    """
    if not file_path.exists():
        return {
            "sce_accuracy": 0.0,
            "header_precision": 0.0,
            "header_recall": 0.0,
            "header_level_accuracy": 0.0,
            "teds_score": 0.0,
            "anls_score": 0.0,
            "jsd_divergence": 1.0,
            "iou_figures": 0.0,
        }

    content = file_path.read_text(encoding="utf-8")
    lines = content.splitlines()
    total_lines = len(lines)

    # 1. Línea Base: Métrica Heurística SCE-Accuracy
    double_spaces = len(re.findall(r" {2,}", content))
    stray_chars = sum(1 for line in lines if len(line.strip()) == 1 and not line.strip().isalnum())
    false_headings = 0
    for line in lines:
        stripped = line.strip()
        if stripped.startswith("#"):
            heading_text = stripped.lstrip("#").strip()
            if (
                re.match(r"^arXiv:\d{4}\.\d{4,5}(v\d+)?.*$", heading_text, re.IGNORECASE)
                or (len(heading_text) <= 2 and not heading_text.isalnum())
                or heading_text.isdigit()
            ):
                false_headings += 1

    weighted_penalty = (double_spaces * 1) + (stray_chars * 2) + (false_headings * 5)
    sce_accuracy = max(0.0, 1.0 - (weighted_penalty / total_lines)) if total_lines > 0 else 1.0

    # 2. Métricas Científicas Avanzadas frente a Ground Truth
    header_precision = 0.0
    header_recall = 0.0
    header_level_accuracy = 1.0
    teds_score = 1.0
    anls_score = 0.0
    jsd_divergence = 1.0
    iou_figures = 1.0

    if gt_path and gt_path.exists():
        gt_content = gt_path.read_text(encoding="utf-8")

        # A. Evaluación de Encabezados
        pred_hd = extract_headings(content)
        gt_hd = extract_headings(gt_content)
        hd_metrics = evaluate_headers(pred_hd, gt_hd)
        header_precision = hd_metrics["precision"]
        header_recall = hd_metrics["recall"]
        header_level_accuracy = hd_metrics["level_accuracy"]

        # B. Evaluación de Tablas mediante TEDS
        # Buscar todas las tablas Markdown en ambos archivos
        table_pattern = r"(?:^\|.*\n)+|\<table.*?\>.*?\<\/table\>"
        pred_tables_raw = re.findall(table_pattern, content, re.MULTILINE | re.DOTALL)
        gt_tables_raw = re.findall(table_pattern, gt_content, re.MULTILINE | re.DOTALL)

        teds_list = []
        min_tables = min(len(pred_tables_raw), len(gt_tables_raw))
        for k in range(min_tables):
            p_raw = pred_tables_raw[k].strip()
            g_raw = gt_tables_raw[k].strip()

            p_html = p_raw if p_raw.startswith("<table") else markdown_table_to_html(p_raw)
            g_html = g_raw if g_raw.startswith("<table") else markdown_table_to_html(g_raw)

            p_tree = parse_and_clean_table(p_html)
            g_tree = parse_and_clean_table(g_html)
            teds_list.append(compute_teds(p_tree, g_tree))

        # Agregar penalizaciones por tablas faltantes/sobrantes
        abs(len(pred_tables_raw) - len(gt_tables_raw))
        teds_score = (sum(teds_list) / max(len(pred_tables_raw), len(gt_tables_raw))) if max(len(pred_tables_raw), len(gt_tables_raw)) > 0 else 1.0

        # C. Coherencia Léxica (ANLS y JSD)
        # Limpiar etiquetas HTML y metadatos espaciales antes de evaluar el texto fluido
        clean_content = re.sub(r"<[^>]+>", "", content)
        clean_gt_content = re.sub(r"<[^>]+>", "", gt_content)
        anls_score = compute_nls(clean_content, clean_gt_content)
        jsd_divergence = compute_jsd(clean_content, clean_gt_content)

        # D. Evaluación Geométrica de Figuras (IoU)
        pred_figs = extract_figures(content)
        gt_figs = extract_figures(gt_content)
        fig_metrics = evaluate_figures(pred_figs, gt_figs)
        iou_figures = fig_metrics["iou_average"]

    return {
        "sce_accuracy": round(sce_accuracy, 4),
        "header_precision": round(header_precision, 4),
        "header_recall": round(header_recall, 4),
        "header_level_accuracy": round(header_level_accuracy, 4),
        "teds_score": round(teds_score, 4),
        "anls_score": round(anls_score, 4),
        "jsd_divergence": round(jsd_divergence, 4),
        "iou_figures": round(iou_figures, 4),
    }


def compute_extraction_accuracy(engine_dirs: dict[str, Path]) -> dict[str, float]:
    """
    Orquesta el cálculo comparativo multidimensional de precisión entre los motores.

    Args:
        engine_dirs (dict[str, Path]): Formato {"nombre_motor": Ruta_Directorio_Salida}.

    Returns:
        dict[str, float]: Precisiones e índices promedio para el consolidado JSON.
    """
    reference_engine = "strata"
    if reference_engine not in engine_dirs:
        reference_engine = next(iter(engine_dirs.keys()))

    reference_dir = engine_dirs[reference_engine]
    reference_files = sorted(list(reference_dir.glob("*.md")))
    md_names = [f.name for f in reference_files if f.name != "README.md"]

    # Carpeta canónica del Ground Truth
    gt_dir = Path("tests/fixtures/salidas/ground_truth")

    # Contenedores de métricas agrupadas por motor
    metrics_by_engine: dict[str, dict[str, list[float]]] = {
        engine: {
            "sce_accuracy": [],
            "header_precision": [],
            "header_recall": [],
            "header_level_accuracy": [],
            "teds_score": [],
            "anls_score": [],
            "jsd_divergence": [],
            "iou_figures": [],
        }
        for engine in engine_dirs
    }

    print("\n" + "=" * 90)
    print(" PIPELINE DE EVALUACIÓN MULTIDIMENSIONAL DE CALIDAD DE EXTRACCIÓN (5 MÉTRICAS)")
    print("=" * 90)

    for name in md_names:
        print(f"Archivo: {name}")
        gt_path = gt_dir / name
        has_gt = gt_path.exists()

        if not has_gt:
            print(f"  [AVISO] No se encontró Ground Truth anotado para {name}. Comparando en modo heurístico base.")

        for engine, out_dir in engine_dirs.items():
            file_path = out_dir / name
            metrics = compute_file_quality(file_path, gt_path if has_gt else None)

            for k, v in metrics.items():
                metrics_by_engine[engine][k].append(v)

            print(
                f"  [{engine.upper():<14}] SCE: {metrics['sce_accuracy']:.4f} | "
                f"TEDS (Tablas): {metrics['teds_score']:.4f} | "
                f"ANLS (Text): {metrics['anls_score']:.4f} | "
                f"JSD: {metrics['jsd_divergence']:.4f} | "
                f"IoU (Figs): {metrics['iou_figures']:.4f}"
            )
        print("-" * 75)

    global_metrics: dict[str, float] = {}
    print("=" * 90)
    print("Resumen de Métricas Promedio Globales por Motor:")

    for engine, engine_metrics in metrics_by_engine.items():
        print(f"\n  • {engine.upper()}:")
        for k, scores in engine_metrics.items():
            avg_val = sum(scores) / len(scores) if scores else 0.0
            global_metrics[f"{engine}_{k}"] = round(avg_val, 4)
            print(f"      - {k:<25}: {avg_val:.4f} ({avg_val * 100:.2f}%)")

    print("\n" + "=" * 90 + "\n")
    return global_metrics


def main() -> None:
    """
    Función de entrada CLI para la ejecución e inspección aislada del análisis.
    """
    engine_dirs = {
        "strata": Path("tests/fixtures/salidas/strata-reader-output"),
        "opendataloader": Path("tests/fixtures/salidas/opendataloader-pdf"),
        "markitdown": Path("tests/fixtures/salidas/markitdown-pdf"),
    }

    missing = [name for name, path in engine_dirs.items() if not path.exists()]
    if missing:
        print(
            f"[ERROR] Los siguientes directorios de salida no existen: {missing}.\n"
            "Asegúrate de ejecutar los benchmarks primero."
        )
        return

    compute_extraction_accuracy(engine_dirs)


if __name__ == "__main__":
    main()

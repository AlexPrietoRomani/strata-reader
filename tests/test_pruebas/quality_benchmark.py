"""
Archivo: quality_benchmark.py
Fecha de modificación: 22/05/2026
Autor: Antigravity

Descripción:
Compara la calidad del Markdown extraído por Strata-Reader frente a
OpenDataLoader en base a métricas cuantitativas reales: dobles espacios,
stray characters, falsos encabezados y densidad de párrafos.

Acciones Principales:
    - Carga los markdowns de ambos motores para los 9 fixtures del corpus.
    - Analiza las métricas críticas para medir las mejoras cuantitativas.
    - Genera un reporte HTML estilizado en la carpeta de salidas.

Estructura Interna:
    - `QualityMetrics`: Dataclass para encapsular las métricas de un archivo.
    - `QualityAnalyzer`: Clase orquestadora encargada de procesar los archivos.
    - `main`: Función de entrada del CLI.

Entradas / Dependencias:
    - Archivos .md generados por Strata-Reader y OpenDataLoader.

Salidas / Efectos:
    - Archivo HTML `tests/fixtures/salidas/quality_report.html`.

Ejecución:
    python tests/test_pruebas/quality_benchmark.py
"""

import re
import os
from pathlib import Path
from dataclasses import dataclass
from typing import Dict, List, Any

@dataclass
class QualityMetrics:
    """
    Representa el conjunto de métricas de calidad medidas sobre un archivo Markdown.
    """
    double_spaces: int
    stray_chars: int
    false_headings: int
    total_lines: int
    image_refs: int
    file_size_kb: float

class QualityAnalyzer:
    """
    Clase encargada de leer los Markdowns de salida y computar las métricas de calidad.
    """

    def __init__(self, strata_dir: Path, odl_dir: Path) -> None:
        """
        Inicializa el analizador con las rutas de los directorios de salida.

        Args:
            strata_dir (Path): Directorio con las salidas de Strata-Reader.
            odl_dir (Path): Directorio con las salidas de OpenDataLoader.
        """
        self.strata_dir = strata_dir
        self.odl_dir = odl_dir

    def analyze_file(self, file_path: Path) -> QualityMetrics:
        """
        Analiza un archivo Markdown individual y calcula sus métricas de calidad.

        Args:
            file_path (Path): Ruta del archivo Markdown a analizar.

        Returns:
            QualityMetrics: Las métricas calculadas.
        """
        if not file_path.exists():
            return QualityMetrics(0, 0, 0, 0, 0, 0.0)

        content = file_path.read_text(encoding="utf-8")
        lines = content.splitlines()

        # 1. Contar dobles espacios (o más consecutivos)
        double_spaces = len(re.findall(r" {2,}", content))

        # 2. Contar stray characters (caracteres no alfanuméricos sueltos o solos en una línea)
        stray_chars = 0
        for line in lines:
            stripped = line.strip()
            if len(stripped) == 1 and not stripped.isalnum():
                stray_chars += 1

        # 3. Contar falsos encabezados (headings que son solo números o ruido de watermarks)
        false_headings = 0
        for line in lines:
            stripped = line.strip()
            if stripped.startswith("#"):
                # Remover los '#' y limpiar espacios
                heading_text = stripped.lstrip("#").strip()
                # Si es un número de página o versión de arXiv
                if re.match(r"^arXiv:\d{4}\.\d{4,5}(v\d+)?.*$", heading_text, re.IGNORECASE):
                    false_headings += 1
                elif len(heading_text) <= 2 and not heading_text.isalnum():
                    false_headings += 1
                elif heading_text.isdigit():
                    false_headings += 1

        # 4. Referencias a imágenes
        image_refs = len(re.findall(r"!\[.*?\]\(.*?\)", content))

        # 5. Tamaño de archivo en KB
        file_size_kb = file_path.stat().st_size / 1024.0

        return QualityMetrics(
            double_spaces=double_spaces,
            stray_chars=stray_chars,
            false_headings=false_headings,
            total_lines=len(lines),
            image_refs=image_refs,
            file_size_kb=round(file_size_kb, 2)
        )

    def run(self) -> Dict[str, Dict[str, QualityMetrics]]:
        """
        Ejecuta el análisis comparativo para todos los fixtures encontrados.

        Returns:
            Dict[str, Dict[str, QualityMetrics]]: Un diccionario con los resultados.
        """
        results: Dict[str, Dict[str, QualityMetrics]] = {}
        # Obtener los archivos .md comunes en el directorio de strata-reader
        md_files = sorted(list(self.strata_dir.glob("*.md")))
        
        for strata_file in md_files:
            if strata_file.name == "README.md":
                continue
            name = strata_file.name
            odl_file = self.odl_dir / name
            
            strata_metrics = self.analyze_file(strata_file)
            odl_metrics = self.analyze_file(odl_file)
            
            results[name] = {
                "strata": strata_metrics,
                "opendataloader": odl_metrics
            }
            
        return results

def generate_html_report(results: Dict[str, Dict[str, QualityMetrics]], output_path: Path) -> None:
    """
    Genera un reporte HTML interactivo y responsivo comparando las métricas de calidad.

    Args:
        results (Dict[str, Dict[str, QualityMetrics]]): Resultados del análisis.
        output_path (Path): Ruta donde guardar el archivo HTML resultante.
    """
    # Totales acumulados para calcular mejoras promedio
    totales = {
        "strata": {"double_spaces": 0, "stray_chars": 0, "false_headings": 0, "total_lines": 0, "image_refs": 0, "size": 0.0},
        "odl": {"double_spaces": 0, "stray_chars": 0, "false_headings": 0, "total_lines": 0, "image_refs": 0, "size": 0.0}
    }

    for name, engines in results.items():
        strata = engines["strata"]
        odl = engines["opendataloader"]

        totales["strata"]["double_spaces"] += strata.double_spaces
        totales["strata"]["stray_chars"] += strata.stray_chars
        totales["strata"]["false_headings"] += strata.false_headings
        totales["strata"]["total_lines"] += strata.total_lines
        totales["strata"]["image_refs"] += strata.image_refs
        totales["strata"]["size"] += strata.file_size_kb

        totales["odl"]["double_spaces"] += odl.double_spaces
        totales["odl"]["stray_chars"] += odl.stray_chars
        totales["odl"]["false_headings"] += odl.false_headings
        totales["odl"]["total_lines"] += odl.total_lines
        totales["odl"]["image_refs"] += odl.image_refs
        totales["odl"]["size"] += odl.file_size_kb

    # Calcular porcentajes de reducción/mejora
    def get_improvement(strata_val: float, odl_val: float, reverse: bool = False) -> str:
        """
        Calcula la mejora relativa en porcentaje.
        """
        if odl_val == 0:
            return "0.0%"
        diff = odl_val - strata_val if not reverse else strata_val - odl_val
        pct = (diff / odl_val) * 100
        return f"{pct:+.1f}%"

    rows_html = []
    for name, engines in sorted(results.items()):
        strata = engines["strata"]
        odl = engines["opendataloader"]
        
        row = f"""
        <tr>
            <td class="font-semibold">{name}</td>
            <td class="text-center">{odl.double_spaces} / <span class="badge strata">{strata.double_spaces}</span> <span class="text-xs text-green-600">({get_improvement(strata.double_spaces, odl.double_spaces)})</span></td>
            <td class="text-center">{odl.stray_chars} / <span class="badge strata">{strata.stray_chars}</span> <span class="text-xs text-green-600">({get_improvement(strata.stray_chars, odl.stray_chars)})</span></td>
            <td class="text-center">{odl.false_headings} / <span class="badge strata">{strata.false_headings}</span> <span class="text-xs text-green-600">({get_improvement(strata.false_headings, odl.false_headings)})</span></td>
            <td class="text-center">{odl.total_lines} / <span class="badge strata">{strata.total_lines}</span> <span class="text-xs text-green-600">({get_improvement(strata.total_lines, odl.total_lines)})</span></td>
            <td class="text-center">{odl.image_refs} / <span class="badge strata">{strata.image_refs}</span></td>
            <td class="text-center">{odl.file_size_kb} KB / <span class="badge strata">{strata.file_size_kb} KB</span> <span class="text-xs text-green-600">({get_improvement(strata.file_size_kb, odl.file_size_kb)})</span></td>
        </tr>
        """
        rows_html.append(row)

    # Mejoras agregadas
    double_spaces_pct = get_improvement(totales["strata"]["double_spaces"], totales["odl"]["double_spaces"])
    stray_chars_pct = get_improvement(totales["strata"]["stray_chars"], totales["odl"]["stray_chars"])
    false_headings_pct = get_improvement(totales["strata"]["false_headings"], totales["odl"]["false_headings"])
    lines_pct = get_improvement(totales["strata"]["total_lines"], totales["odl"]["total_lines"])
    size_pct = get_improvement(totales["strata"]["size"], totales["odl"]["size"])

    html_content = f"""<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Reporte de Calidad Markdown: Strata-Reader vs OpenDataLoader</title>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&display=swap" rel="stylesheet">
    <style>
        :root {{
            --bg-color: #0b0f19;
            --card-bg: #151c2c;
            --text-color: #f3f4f6;
            --text-muted: #9ca3af;
            --primary: #3b82f6;
            --green: #10b981;
            --border: #1e293b;
        }}
        body {{
            font-family: 'Inter', sans-serif;
            background-color: var(--bg-color);
            color: var(--text-color);
            margin: 0;
            padding: 2rem;
            line-height: 1.5;
        }}
        .container {{
            max-width: 1200px;
            margin: 0 auto;
        }}
        header {{
            text-align: center;
            margin-bottom: 3rem;
        }}
        h1 {{
            font-size: 2.5rem;
            font-weight: 700;
            margin-bottom: 0.5rem;
            background: linear-gradient(135deg, #60a5fa, #3b82f6);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }}
        .subtitle {{
            color: var(--text-muted);
            font-size: 1.1rem;
        }}
        .grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 1.5rem;
            margin-bottom: 3rem;
        }}
        .card {{
            background-color: var(--card-bg);
            border: 1px solid var(--border);
            border-radius: 12px;
            padding: 1.5rem;
            text-align: center;
            transition: transform 0.2s;
        }}
        .card:hover {{
            transform: translateY(-4px);
        }}
        .card .value {{
            font-size: 2rem;
            font-weight: 700;
            color: var(--green);
            margin: 0.5rem 0;
        }}
        .card .label {{
            font-size: 0.875rem;
            color: var(--text-muted);
            font-weight: 500;
        }}
        table {{
            width: 100%;
            border-collapse: collapse;
            background-color: var(--card-bg);
            border: 1px solid var(--border);
            border-radius: 12px;
            overflow: hidden;
            margin-bottom: 2rem;
        }}
        th, td {{
            padding: 1rem 1.5rem;
            text-align: left;
            border-bottom: 1px solid var(--border);
        }}
        th {{
            background-color: #1e293b;
            font-weight: 600;
            font-size: 0.875rem;
            color: var(--text-muted);
            text-transform: uppercase;
        }}
        tr:last-child td {{
            border-bottom: none;
        }}
        .text-center {{
            text-align: center;
        }}
        .font-semibold {{
            font-weight: 600;
        }}
        .badge {{
            display: inline-block;
            padding: 0.25rem 0.5rem;
            border-radius: 6px;
            font-size: 0.875rem;
            font-weight: 600;
        }}
        .badge.strata {{
            background-color: rgba(59, 130, 246, 0.2);
            color: #60a5fa;
            border: 1px solid rgba(59, 130, 246, 0.3);
        }}
        footer {{
            text-align: center;
            margin-top: 3rem;
            color: var(--text-muted);
            font-size: 0.875rem;
        }}
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>Reporte de Calidad de Salida Markdown</h1>
            <p class="subtitle">Strata-Reader frente al Baseline de OpenDataLoader · Análisis Cuantitativo Empírico</p>
        </header>

        <div class="grid">
            <div class="card">
                <div class="label">Reducción de Dobles Espacios</div>
                <div class="value">{double_spaces_pct}</div>
                <p style="margin: 0; font-size: 0.875rem; color: var(--text-muted);">de {totales["odl"]["double_spaces"]} a {totales["strata"]["double_spaces"]}</p>
            </div>
            <div class="card">
                <div class="label">Eliminación de Stray Chars</div>
                <div class="value">{stray_chars_pct}</div>
                <p style="margin: 0; font-size: 0.875rem; color: var(--text-muted);">de {totales["odl"]["stray_chars"]} a {totales["strata"]["stray_chars"]}</p>
            </div>
            <div class="card">
                <div class="label">Falsos Encabezados Filtrados</div>
                <div class="value">{false_headings_pct}</div>
                <p style="margin: 0; font-size: 0.875rem; color: var(--text-muted);">de {totales["odl"]["false_headings"]} a {totales["strata"]["false_headings"]}</p>
            </div>
            <div class="card">
                <div class="label">Compactación de Líneas (Fluidez)</div>
                <div class="value">{lines_pct}</div>
                <p style="margin: 0; font-size: 0.875rem; color: var(--text-muted);">de {totales["odl"]["total_lines"]} a {totales["strata"]["total_lines"]}</p>
            </div>
        </div>

        <table>
            <thead>
                <tr>
                    <th>Fixture (Artículo PDF)</th>
                    <th class="text-center">Dobles Espacios (ODL / Strata)</th>
                    <th class="text-center">Stray Chars (ODL / Strata)</th>
                    <th class="text-center">Falsos Headings (ODL / Strata)</th>
                    <th class="text-center">Total Líneas (ODL / Strata)</th>
                    <th class="text-center">Imágenes Guardadas</th>
                    <th class="text-center">Tamaño de Archivo (ODL / Strata)</th>
                </tr>
            </thead>
            <tbody>
                {"".join(rows_html)}
            </tbody>
        </table>

        <footer>
            <p>Strata-Reader v0.1.0 · Motor de alto rendimiento escrito en Rust puro con bindings de Python</p>
        </footer>
    </div>
</body>
</html>
"""
    output_path.write_text(html_content, encoding="utf-8")
    print(f"Reporte HTML de calidad generado exitosamente en: {output_path.absolute()}")

def main() -> None:
    """
    Función principal para orquestar la ejecución del benchmark.
    """
    strata_dir = Path("tests/fixtures/salidas/strata-reader-output")
    # Si por algún motivo está en strata-reader-md, usar ese como fallback
    if not strata_dir.exists() or len(list(strata_dir.glob("*.md"))) == 0:
        strata_dir = Path("tests/fixtures/salidas/strata-reader-md")
        
    odl_dir = Path("tests/fixtures/salidas/opendataloader-pdf")

    print(f"Leyendo salidas de Strata-Reader desde: {strata_dir}")
    print(f"Leyendo salidas de OpenDataLoader desde: {odl_dir}")

    analyzer = QualityAnalyzer(strata_dir, odl_dir)
    results = analyzer.run()

    report_path = Path("tests/fixtures/salidas/quality_report.html")
    generate_html_report(results, report_path)

if __name__ == "__main__":
    main()

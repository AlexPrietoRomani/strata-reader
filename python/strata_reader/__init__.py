"""
Archivo: __init__.py
Fecha de modificación: 22/05/2026
Autor: Strata-Reader Contributors

Descripción:
Punto de entrada principal del SDK de Python de Strata-Reader. Re-exporta los tipos
y funciones nativos compilados en Rust e implementa la función de conveniencia
de alto nivel `convert()` para el procesamiento por lotes (batch) de documentos PDF.

Sustentación Científica:
Diseñado como una interfaz declarativa de bajo acoplamiento para simplificar la ingesta
en sistemas RAG vectoriales y Graph-RAG, automatizando la resolución de rutas,
el manejo de directorios y la persistencia estructurada.

Acciones Principales:
    - Re-exporta `Document`, `ParseOptions`, `parse`, `parse_batch`, `version` desde Rust Core.
    - Implementa `convert()` para procesamiento batch tolerante a fallos.

Estructura Interna:
    - `convert()`: Función de conveniencia para procesamiento batch.

Entradas / Dependencias:
    - Módulo nativo compilado de Rust (`strata_reader._native`).
    - Dependencia opcional `tqdm` para barras de progreso interactivas.

Salidas / Efectos:
    - Genera archivos Markdown y/o JSON en la carpeta de destino especificada.

Ejemplo de Integración:
    from strata_reader import convert
    convert(input_path="papers/", output_dir="out/", format="md+json")
"""

from __future__ import annotations

import glob
import json

# --- Auto-descubrimiento de PDFium Embebido (Fase 13) ---
import os
import sys
from pathlib import Path

_PDFIUM_DIR = Path(__file__).parent / "_pdfium"
if _PDFIUM_DIR.exists():
    # En Linux/macOS la librería está en lib/, en Windows en bin/
    for subdir in ("lib", "bin"):
        candidate = _PDFIUM_DIR / subdir
        if candidate.exists():
            os.environ.setdefault("STRATA_PDFIUM_LIB_PATH", str(candidate))
            # En Windows, agregar la ruta al DLL search path para Python 3.8+
            if sys.platform == "win32":
                os.add_dll_directory(str(candidate))
            break

try:
    from ._native import (
        Document,
        ParseOptions,
        parse,
        parse_batch,
        version,
    )
except ImportError as exc:  # pragma: no cover — native module missing
    msg = (
        "strata_reader._native is not available. Run `uv run maturin develop` "
        "to build the Rust extension, or `pip install strata-reader` once "
        "wheels are published."
    )
    raise ImportError(msg) from exc


def convert(
    input_path: str | Path | list[str | Path],
    output_dir: str | Path,
    format: str = "md+json",
    profile: str = "balanced",
    use_ia: bool = True,
    max_concurrent_pages: int | None = None,
    media_dir: str | Path | None = None,
    save_images: bool = False,
    ollama_endpoint: str = "http://localhost:11434",
    show_progress: bool = True,
) -> dict[str, str]:
    """
    Convierte uno o más archivos PDF a Markdown semántico y/o JSON estructurado para Graph-RAG.

    Busca de manera recursiva o directa los archivos PDF indicados, crea las carpetas de destino,
    procesa cada documento tolerando errores individuales y guarda las salidas deseadas.

    Args:
        input_path (Union[str, Path, List[Union[str, Path]]]): Ruta a un PDF, directorio, glob o lista de estos.
        output_dir (Union[str, Path]): Carpeta de destino donde se guardarán los archivos resultantes.
        format (str, opcional): Formatos de salida separados por '+'. Puede ser 'md', 'json' o 'md+json'. Por defecto es 'md+json'.
        profile (str, opcional): Perfil de triage. Puede ser 'fast', 'balanced' o 'scientific'. Por defecto es 'balanced'.
        use_ia (bool, opcional): Indica si se habilita el motor híbrido con IA local (Ollama). Por defecto es True.
        max_concurrent_pages (Optional[int], opcional): Límite de páginas procesadas en paralelo. Por defecto es None.
        media_dir (Optional[Union[str, Path]], opcional): Carpeta alternativa para recortes de imágenes. Por defecto es None.
        save_images (bool, opcional): Si es True y media_dir no está definido, guarda las figuras en una subcarpeta. Por defecto es False.
        ollama_endpoint (str, opcional): URL del endpoint local de Ollama. Por defecto es 'http://localhost:11434'.
        show_progress (bool, opcional): Si es True, muestra una barra de progreso interactiva (requiere tqdm). Por defecto es True.

    Returns:
        Dict[str, str]: Un diccionario que mapea la ruta de cada PDF a su estado de procesamiento ('success' o un mensaje de error).

    Raises:
        ValueError: Si el formato especificado no es válido.
    """
    # 1. Validar formato
    format_clean = format.lower().strip()
    if format_clean not in ["md", "json", "md+json", "json+md"]:
        raise ValueError(f"Formato no válido: {format}. Use 'md', 'json' o 'md+json'.")

    # 2. Normalizar directorios
    out_path = Path(output_dir)
    out_path.mkdir(parents=True, exist_ok=True)

    # 3. Resolver lista de PDFs a procesar
    pdf_files: list[Path] = []

    # Asegurar que input_path sea una lista
    paths_to_process = input_path if isinstance(input_path, list) else [input_path]

    for item in paths_to_process:
        item_str = str(item)
        # Comprobar si es un patrón de glob (contiene * o ?)
        if "*" in item_str or "?" in item_str:
            for match in glob.glob(item_str, recursive=True):
                match_path = Path(match)
                if match_path.is_file() and match_path.suffix.lower() == ".pdf":
                    pdf_files.append(match_path)
        else:
            p = Path(item)
            if p.is_file() and p.suffix.lower() == ".pdf":
                pdf_files.append(p)
            elif p.is_dir():
                # Búsqueda recursiva de PDFs
                pdf_files.extend(p.rglob("*.pdf"))

    # Eliminar duplicados manteniendo orden
    seen = set()
    unique_pdf_files = []
    for x in pdf_files:
        if x not in seen:
            seen.add(x)
            unique_pdf_files.append(x)
    pdf_files = unique_pdf_files

    if not pdf_files:
        return {}

    # 4. Configurar opciones comunes para ParseOptions
    media_dir_str = str(media_dir) if media_dir else None

    # Preparar el iterador con progreso
    iterator = pdf_files
    if show_progress:
        try:
            from tqdm import tqdm

            iterator = tqdm(pdf_files, desc="Procesando PDFs", unit="doc")
        except ImportError:
            # tqdm no instalado, ignorar silenciosamente
            pass

    results: dict[str, str] = {}

    for pdf in iterator:
        pdf_str = str(pdf)
        stem = pdf.stem

        # Determinar el directorio de medios dinámico por archivo si no se especifica y save_images es True
        current_media_dir = media_dir_str
        if save_images and not current_media_dir:
            current_media_dir = str(out_path / f"{stem}_images")

        try:
            # Instanciar las opciones para este parseo
            opts = ParseOptions(
                profile=profile,
                use_ia=use_ia,
                max_concurrent_pages=max_concurrent_pages,
                media_dir=current_media_dir,
                ollama_endpoint=ollama_endpoint,
            )

            # Ejecutar el análisis nativo (Rust core)
            doc = parse(pdf_str, opts)

            # Guardar Markdown
            if "md" in format_clean:
                md_content = doc.to_markdown()
                md_file = out_path / f"{stem}.md"
                md_file.write_text(md_content, encoding="utf-8")

            # Guardar JSON estructurado
            if "json" in format_clean:
                graph_dict = doc.to_graph_json()
                json_content = json.dumps(graph_dict, indent=2, ensure_ascii=False)
                json_file = out_path / f"{stem}.json"
                json_file.write_text(json_content, encoding="utf-8")

            results[pdf_str] = "success"
        except Exception as e:
            results[pdf_str] = f"error: {e!s}"

    return results


__all__ = ["Document", "ParseOptions", "convert", "parse", "parse_batch", "version"]

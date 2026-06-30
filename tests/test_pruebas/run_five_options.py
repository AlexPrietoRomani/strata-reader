"""
Archivo: run_five_options.py
Fecha de modificación: 28/05/2026
Autor: Alex Prieto

Descripción:
Este script ejecuta de forma exclusiva y demostrativa las 5 opciones de conversión
documental documentadas en el README.md de la librería Strata-Reader. Utiliza 5 PDFs
científicos seleccionados de la colección de fixtures y guarda los archivos resultantes
(Markdown y JSON estructurado) en una subcarpeta dedicada.

Sustentación Científica:
Alineado con los estándares del Plan Maestro para la evaluación y demostración empírica
de pipelines híbridos con escalamiento VLM (Ollama) y OCR.

Acciones Principales:
    - Configura dinámicamente la ruta a la biblioteca pdfium.dll local en Windows.
    - Selecciona 5 PDFs específicos para mapear cada una de las 5 opciones del README.
    - Ejecuta cada opción con sus configuraciones de perfil, IA y salida respectivas.
    - Documenta de forma técnica las diferencias del flujo y la relación con la Fase 14.

Estructura Interna:
    - `run_five_options_demo()`: Orquesta la ejecución secuencial de las 5 configuraciones.
    - `main()`: Punto de entrada del script ejecutable.

Entradas / Dependencias:
    - 5 Archivos PDF de la carpeta `tests/fixtures/pdfs/articles/`.
    - Variable de entorno `STRATA_PDFIUM_LIB_PATH` configurada para resolver la carga nativa.

Salidas / Efectos:
    - Archivos Markdown, JSON y recortes de imágenes guardados en `tests/fixtures/salidas/five_options/`.

Ejecución:
    uv run python tests/test_pruebas/run_five_options.py
"""

from __future__ import annotations

import os
import time
from pathlib import Path

# Configuración dinámica del path a pdfium.dll local en AppData para el backend nativo
if "STRATA_PDFIUM_LIB_PATH" not in os.environ:
    local_pdfium = Path.home() / "AppData" / "Local" / "pdfium" / "bin"
    if local_pdfium.is_dir():
        os.environ["STRATA_PDFIUM_LIB_PATH"] = str(local_pdfium.absolute())
        print(f"[CONFIG] STRATA_PDFIUM_LIB_PATH configurada en: {local_pdfium}")

import strata_reader


def run_five_options_demo() -> None:
    """
    Ejecuta las 5 opciones recomendadas en el README.md en 5 PDFs de artículos científicos.
    """
    articles_dir = Path("tests/fixtures/pdfs/articles")
    base_output_dir = Path("tests/fixtures/salidas/five_options")
    base_output_dir.mkdir(parents=True, exist_ok=True)

    # 1. Mapeo de PDFs para cada opción
    configurations = [
        {
            "id": "Option_1_Standard_Digital",
            "pdf_name": "10.48550_arXiv.2012.14005.pdf",  # Paper sobre Neural Document Expansion
            "description": "PDF digital estándar (Nativo por defecto, Balanced)",
            "use_ia": False,
            "profile": "balanced",
            "save_images": False,
            "format": "md+json",
        },
        {
            "id": "Option_2_Complex_Borderless_Tables",
            "pdf_name": "10.48550_arXiv.1409.0473.pdf",  # Paper sobre Neural Machine Translation
            "description": "Tablas complejas / sin bordes (Híbrido IA, Scientific)",
            "use_ia": True,
            "profile": "scientific",
            "save_images": False,
            "format": "md+json",
        },
        {
            "id": "Option_3_Scanned_Or_Image_Based",
            "pdf_name": "10.48550_arXiv.1512.03385.pdf",  # Paper clásico de ResNet (imagen/tablas)
            "description": "PDF escaneado / basado en imágenes (IA + OCR, Balanced)",
            "use_ia": True,
            "profile": "balanced",
            "save_images": False,
            "format": "md+json",
        },
        {
            "id": "Option_4_Complex_Math_Formulas",
            "pdf_name": "10.48550_arXiv.1706.03762.pdf",  # Paper de Transformers (Attention Is All You Need)
            "description": "Fórmulas matemáticas complejas (Nativo por defecto, Balanced)",
            "use_ia": False,
            "profile": "balanced",
            "save_images": False,
            "format": "md",  # Solo Markdown para visualización de ecuaciones
        },
        {
            "id": "Option_5_Images_With_Descriptions",
            "pdf_name": "10.48550_arXiv.1810.04805.pdf",  # Paper de BERT (incluye diagramas de arquitectura)
            "description": "Imágenes e ilustraciones con descripción (Híbrido IA, Balanced, Guardado de imágenes)",
            "use_ia": True,
            "profile": "balanced",
            "save_images": True,
            "format": "md+json",
        },
    ]

    print("=" * 80)
    print("INICIANDO DEMOSTRACIÓN DE LAS 5 OPCIONES DE STRATA-READER")
    print("=" * 80)

    for config in configurations:
        pdf_path = articles_dir / config["pdf_name"]
        if not pdf_path.exists():
            print(
                f"[ADVERTENCIA] No se encontró el PDF {config['pdf_name']} en {articles_dir}. Se omitirá."
            )
            continue

        output_subfolder = base_output_dir / config["id"]
        output_subfolder.mkdir(parents=True, exist_ok=True)

        print(f"\n[EJECUCIÓN] {config['id']}: {config['description']}")
        print(f"  Archivo: {pdf_path.name}")
        print(
            f"  Configuración: use_ia={config['use_ia']}, profile={config['profile']}, format={config['format']}"
        )

        start_time = time.time()

        # Invocar al SDK de Strata-Reader
        results = strata_reader.convert(
            input_path=pdf_path,
            output_dir=output_subfolder,
            format=config["format"],
            profile=config["profile"],
            use_ia=config["use_ia"],
            save_images=config["save_images"],
            ollama_endpoint="http://localhost:11434",
            show_progress=False,
        )

        elapsed = time.time() - start_time
        status = results.get(str(pdf_path), "unknown")

        print(f"  Resultado: {status} (Procesado en {elapsed:.3f} segundos)")

        output_subfolder_res = output_subfolder.resolve()
        cwd_resolved = Path.cwd().resolve()
        try:
            relative_display = output_subfolder_res.relative_to(cwd_resolved)
        except ValueError:
            relative_display = output_subfolder_res

        print(f"  Salidas generadas en: {relative_display}")

        # Mensaje educativo sobre el estado de la integración de IA
        if config["use_ia"]:
            print(
                "  [NOTA ARQUITECTÓNICA] Aunque use_ia=True está configurado, la inferencia híbrida "
                "gRPC se encuentra desacoplada en el Core nativo actual. Esta opción correrá de forma "
                "nativa en Rust hasta que la Fase 14 sea implementada, momento en el cual se conectará "
                "con el microservicio FastAPI/gRPC local."
            )

    print("\n" + "=" * 80)
    print("PROCESAMIENTO DE LAS 5 OPCIONES COMPLETADO CON ÉXITO")
    print("=" * 80)


def main() -> None:
    """
    Punto de entrada ejecutable para el script.
    """
    run_five_options_demo()


if __name__ == "__main__":
    main()

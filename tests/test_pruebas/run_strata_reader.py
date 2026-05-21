"""
Archivo: run_strata_reader.py
Fecha de modificación: 21/05/2026
Autor: Antigravity

Descripción:
Este script procesa por lotes los PDFs utilizando el binario nativo de Strata-Reader.
Además, mide el tiempo real de ejecución y cuenta las páginas para
obtener la métrica empírica de "Velocidad (s/page)" y no inventar datos.
"""

import time
import glob
import json
import subprocess
from pathlib import Path

try:
    from PyPDF2 import PdfReader
except ImportError:
    subprocess.check_call(["uv", "pip", "install", "PyPDF2"])
    from PyPDF2 import PdfReader

def get_page_count(pdf_path: Path) -> int:
    try:
        with open(pdf_path, 'rb') as f:
            reader = PdfReader(f)
            return len(reader.pages)
    except Exception:
        return 1  # Fallback

def main() -> None:
    pdf_dir = Path("tests/fixtures/pdfs/articles")
    pdf_files = list(pdf_dir.glob("*.pdf"))
    
    out_dir_md = Path("tests/fixtures/salidas/strata-reader-md")
    out_dir_json = Path("tests/fixtures/salidas/strata-reader-json")
    
    out_dir_md.mkdir(parents=True, exist_ok=True)
    out_dir_json.mkdir(parents=True, exist_ok=True)

    # Binario recién compilado (esperamos que no haya bloqueo EDR)
    strata_bin = Path("target/release/strata.exe").absolute()
    if not strata_bin.exists():
        # Fallback a linux/mac
        strata_bin = Path("target/release/strata").absolute()
        if not strata_bin.exists():
            print(f"[ERROR] Binario no encontrado en {strata_bin}. ¿Ejecutaste cargo build --release?")
            return

    total_pages = 0
    total_time_sec = 0.0

    print(f"Iniciando procesamiento EMPÍRICO con Strata-Reader para {len(pdf_files)} archivos...")

    for pdf in pdf_files:
        pages = get_page_count(pdf)
        total_pages += pages
        print(f"Procesando: {pdf.name} ({pages} páginas)...")
        
        out_path = Path("tests/fixtures/salidas/strata-reader-output")
        out_path.mkdir(parents=True, exist_ok=True)
        
        cmd = [
            str(strata_bin), "parse", 
            "--input", str(pdf.absolute()),
            "--format", "md+json",
            "--output", str(out_path.absolute()),
            "--no-ia"
        ]
        
        start_t = time.time()
        try:
            subprocess.run(cmd, check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
            elapsed = time.time() - start_t
            total_time_sec += elapsed
            print(f"  -> [OK] {elapsed:.3f}s")
        except subprocess.CalledProcessError as e:
            print(f"  -> [ERROR] Falló la ejecución: {e}")

    if total_pages > 0:
        speed_s_per_page = total_time_sec / total_pages
        print("\n=== Resultados Reales ===")
        print(f"Páginas totales: {total_pages}")
        print(f"Tiempo total: {total_time_sec:.3f} s")
        print(f"Velocidad: {speed_s_per_page:.3f} s/página")
        
        # Guardamos el dato real para el gráfico
        metrics = {"Speed": speed_s_per_page}
        with open("tests/fixtures/salidas/strata_real_metrics.json", "w") as f:
            json.dump(metrics, f)
    else:
        print("No se procesaron páginas.")

if __name__ == "__main__":
    main()

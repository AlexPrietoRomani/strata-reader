"""
Archivo: cli.py
Fecha de modificación: 22/05/2026
Autor: Strata-Reader Contributors

Descripción:
Punto de entrada de consola (CLI) para la herramienta `strata`. Busca el binario
nativo compilado en Rust (`strata`) mediante una cascada inteligente de fallbacks y
lo ejecuta delegando todos los argumentos de consola recibidos.

Sustentación Científica:
Implementa un resolvedor de rutas de ejecución determinista para aislar los entornos
de desarrollo de los de producción, resolviendo colisiones de nombres y de wrappers
de entornos virtuales de Python.

Acciones Principales:
    - Resuelve y localiza el ejecutable nativo de Rust real en cascada.
    - Invoca el binario correspondiente mediante un subproceso con los mismos argumentos.

Estructura Interna:
    - `find_strata_binary()`: Resuelve la ruta al ejecutable nativo.
    - `main()`: Orquesta la búsqueda y la llamada al binario.

Entradas / Dependencias:
    - Argumentos del sistema (`sys.argv`).
    - Binario nativo compilado en target/ o empaquetado por maturin en site-packages.

Salidas / Efectos:
    - Retorna el código de salida devuelto por el binario real de Rust.

Ejecución:
    python cli.py [--argumentos]

Ejemplo de Uso:
    python cli.py parse --input paper.pdf --output out/

Argumentos:
    - Argumentos dinámicos pasados directamente a strata-cli.
"""

from __future__ import annotations

import os
from pathlib import Path
import shutil
import subprocess
import sys
from typing import Optional


def find_strata_binary() -> Optional[str]:
    """
    Busca el binario nativo compilado de Rust 'strata' en cascada de fallbacks.

    Prioriza el directorio del paquete Python (wheel), los directorios de
    compilación de Rust locales (workspace) y finalmente el PATH del sistema
    evitando wrappers circulares de entornos virtuales (.venv).

    Returns:
        Optional[str]: La ruta absoluta al binario real, o None si no se encuentra.
    """
    package_dir = Path(__file__).parent
    ext = ".exe" if os.name == "nt" else ""
    
    # 1. Buscar en el directorio del paquete Python (empaquetado del wheel)
    package_bin = package_dir / f"strata{ext}"
    if package_bin.is_file():
        return str(package_bin)
        
    # 2. Buscar en los directorios de compilación de Rust locales (Workspace)
    current = package_dir.resolve()
    for _ in range(5):
        target_release = current / "target" / "release" / f"strata{ext}"
        if target_release.is_file():
            return str(target_release)
        target_debug = current / "target" / "debug" / f"strata{ext}"
        if target_debug.is_file():
            return str(target_debug)
        if current.parent == current:
            break
        current = current.parent

    # 3. Buscar en el PATH del sistema, evitando directorios de entornos virtuales (.venv)
    env_path = os.environ.get("PATH", "")
    for path_dir in env_path.split(os.pathsep):
        if not path_dir:
            continue
        candidate = Path(path_dir) / f"strata{ext}"
        if candidate.is_file():
            cand_str = str(candidate).lower()
            # Omitimos wrappers de entorno virtual local para prevenir llamadas infinitas circulares
            if ".venv" in cand_str or "site-packages" in cand_str:
                continue
            return str(candidate)

    # 4. Fallback final al resolvedor general de PATH de la librería estándar
    path_bin = shutil.which("strata")
    if path_bin:
        return path_bin

    return None


def main() -> int:
    """
    Invoca el binario native 'strata', cayendo en cascada o mostrando error instructivo.

    Returns:
        int: Código de retorno del subproceso o 127 si el binario no fue encontrado.
    """
    bin_path = find_strata_binary()
    if bin_path is None:
        msg = (
            "Error: El binario nativo de Rust 'strata' no fue encontrado.\n"
            "Ejecute 'cargo build -p strata-cli --release' para compilarlo en el workspace,\n"
            "o reinstale el paquete usando 'pip install -U strata-reader'."
        )
        print(msg, file=sys.stderr)
        return 127
    return subprocess.call([bin_path, *sys.argv[1:]], env=os.environ.copy())


if __name__ == "__main__":
    raise SystemExit(main())

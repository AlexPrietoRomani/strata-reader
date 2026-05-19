"""Regenerate the Python gRPC stubs from `strata_ia.proto`.

Run with::

    uv run python scripts/gen_proto.py

Idempotent. Post-processes the generated ``*_pb2_grpc.py`` to use a
package-relative import (`from . import strata_ia_pb2`) — grpcio-tools
emits a top-level `import strata_ia_pb2` which breaks when the stubs
live inside a package (well-known issue:
https://github.com/grpc/grpc/issues/29459).
"""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[1]
PROTO_FILE = REPO_ROOT / "crates" / "strata-ia-bridge" / "proto" / "strata_ia.proto"
OUT_DIR = REPO_ROOT / "python" / "strata_ia" / "proto"


def regenerate() -> int:
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    (OUT_DIR / "__init__.py").touch()

    cmd = [
        sys.executable,
        "-m",
        "grpc_tools.protoc",
        f"-I{PROTO_FILE.parent}",
        f"--python_out={OUT_DIR}",
        f"--grpc_python_out={OUT_DIR}",
        f"--pyi_out={OUT_DIR}",
        str(PROTO_FILE),
    ]
    print("[gen_proto]", " ".join(cmd))
    result = subprocess.run(cmd, check=False)
    if result.returncode != 0:
        return result.returncode

    # Post-process: rewrite `import strata_ia_pb2 as strata__ia__pb2` to
    # `from . import strata_ia_pb2 as strata__ia__pb2`.
    grpc_file = OUT_DIR / "strata_ia_pb2_grpc.py"
    text = grpc_file.read_text(encoding="utf-8")
    fixed = text.replace(
        "import strata_ia_pb2 as strata__ia__pb2",
        "from . import strata_ia_pb2 as strata__ia__pb2",
    )
    if fixed != text:
        grpc_file.write_text(fixed, encoding="utf-8")
        print(f"[gen_proto] fixed relative import in {grpc_file.name}")
    return 0


if __name__ == "__main__":
    raise SystemExit(regenerate())

"""Python entry-point for `strata` invoked via the `[project.scripts]` table.

Phase 0 only forwards to the native binary if it is on PATH. The real CLI lives
in Rust (`crates/strata-cli`).
"""

from __future__ import annotations

import os
import shutil
import subprocess
import sys


def main() -> int:
    """Invoke the bundled `strata` binary, falling back to a helpful error."""
    bin_path = shutil.which("strata")
    if bin_path is None:
        msg = (
            "The native `strata` binary is not on PATH. Build it with "
            "`cargo build -p strata-cli --release` or install the wheel."
        )
        print(msg, file=sys.stderr)
        return 127
    return subprocess.call([bin_path, *sys.argv[1:]], env=os.environ.copy())


if __name__ == "__main__":
    raise SystemExit(main())

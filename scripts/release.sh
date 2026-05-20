#!/usr/bin/env bash
# Strata-Reader release helper.
# Plan Maestro §15.T10.6 — bump version, tag, push.
#
# Usage:
#     ./scripts/release.sh 0.1.0
#
# The script:
#  1. Validates the working tree is clean.
#  2. Updates every version reference (workspace + python package).
#  3. Runs the local sanity gates (cargo check, ruff, mypy, pytest).
#  4. Creates the annotated tag v<version>.
#  5. Prints the next manual steps (push + watch CI).

set -euo pipefail

if [ $# -ne 1 ]; then
    echo "usage: $0 <semver>" >&2
    exit 64
fi
VERSION="$1"

if ! [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[A-Za-z0-9.+-]+)?$ ]]; then
    echo "error: '$VERSION' is not a valid semver" >&2
    exit 65
fi

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

if [ -n "$(git status --porcelain)" ]; then
    echo "error: working tree has uncommitted changes" >&2
    git status --short
    exit 66
fi

if git tag --list "v$VERSION" | grep -q "v$VERSION"; then
    echo "error: tag v$VERSION already exists" >&2
    exit 67
fi

CURRENT="$(grep -m1 '^version' Cargo.toml | sed -E 's/.*"([0-9.]+)".*/\1/')"
echo "current workspace version: $CURRENT"
echo "target version:            $VERSION"

# Update Cargo workspace version.
python3 - "$VERSION" <<'PY'
import re, sys
from pathlib import Path
version = sys.argv[1]
text = Path("Cargo.toml").read_text(encoding="utf-8")
text = re.sub(r'(\[workspace\.package\][^\[]*?version\s*=\s*)"[^"]+"', rf'\1"{version}"', text, count=1)
Path("Cargo.toml").write_text(text, encoding="utf-8")
PY

# Update pyproject project version.
python3 - "$VERSION" <<'PY'
import re, sys
from pathlib import Path
version = sys.argv[1]
text = Path("pyproject.toml").read_text(encoding="utf-8")
text = re.sub(r'(\[project\][^\[]*?\nversion\s*=\s*)"[^"]+"', rf'\1"{version}"', text, count=1)
Path("pyproject.toml").write_text(text, encoding="utf-8")
PY

# Update python/strata_reader/__init__.py version constant if present.
INIT="python/strata_reader/__init__.py"
if [ -f "$INIT" ] && grep -q "__version__" "$INIT"; then
    python3 - "$VERSION" "$INIT" <<'PY'
import re, sys
from pathlib import Path
version, target = sys.argv[1], sys.argv[2]
p = Path(target)
text = p.read_text(encoding="utf-8")
text = re.sub(r'__version__\s*=\s*"[^"]+"', f'__version__ = "{version}"', text, count=1)
p.write_text(text, encoding="utf-8")
PY
fi

# Gate: lint + typecheck + pytest. We DON'T run `cargo` here because
# the EDR-blocked dev machines couldn't release otherwise (Plan Maestro
# Apéndice B). The release CI workflow runs the full cargo gate.
echo "--- ruff ---"
uv run --no-sync ruff check python tests benches scripts

echo "--- mypy ---"
uv run --no-sync python -m mypy python

echo "--- pytest ---"
uv run --no-sync python -m pytest tests/unit_py -q

# Commit version bump.
git add Cargo.toml pyproject.toml "$INIT"
git commit -m "chore(release): bump to v$VERSION"

# Tag.
git tag -a "v$VERSION" -m "Strata-Reader v$VERSION"

cat <<EOM

Release v$VERSION prepared. Next steps:

  git push origin master
  git push origin v$VERSION

CI will then build the wheels via .github/workflows/release-wheels.yml
and a release-publish workflow can promote them to PyPI.
EOM

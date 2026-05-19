#!/usr/bin/env bash
# Strata-Reader — local dev orchestrator (Linux/macOS).
# Replaces `docker compose up` for environments without Docker.
# Idempotent: safe to run multiple times.
# See docs/task/tareas.md T0.5.A0.5.3.
#
# Usage:
#   ./scripts/dev_up.sh                    # start Ollama + pull models
#   ./scripts/dev_up.sh --with-server      # also launch strata-server in bg
#   ./scripts/dev_up.sh --skip-models      # skip model pull (fast restart)

set -euo pipefail

WITH_SERVER=0
SKIP_MODELS=0
for arg in "$@"; do
    case "$arg" in
        --with-server) WITH_SERVER=1 ;;
        --skip-models) SKIP_MODELS=1 ;;
        -h|--help)
            sed -n '1,15p' "$0"
            exit 0
            ;;
    esac
done

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
STATE_DIR="${XDG_STATE_HOME:-$HOME/.local/state}/strata"
PID_FILE="$STATE_DIR/dev.pid"
OLLAMA_ENDPOINT="${STRATA_OLLAMA_URL:-http://localhost:11434}"
mkdir -p "$STATE_DIR"

section() { printf "\n==> %s\n" "$1"; }
ok()      { printf "    ok  %s\n" "$1"; }
warn()    { printf "    !!  %s\n" "$1" >&2; }

require_ollama() {
    if ! command -v ollama >/dev/null 2>&1; then
        cat >&2 <<'EOF'
ollama CLI not found on PATH.
Install (user-scope, no admin required):
  - macOS:   brew install ollama
  - Linux:   curl -fsSL https://ollama.com/install.sh | sh
After installing, re-run this script.
EOF
        exit 127
    fi
    ok "ollama $(ollama --version 2>&1 | head -n1)"
}

is_reachable() {
    curl -fsS "$OLLAMA_ENDPOINT/api/tags" >/dev/null 2>&1
}

start_ollama_if_needed() {
    if is_reachable; then
        ok "Ollama already reachable at $OLLAMA_ENDPOINT"
        return
    fi
    section "Starting 'ollama serve' in background"
    nohup ollama serve >"$STATE_DIR/ollama.log" 2>&1 &
    echo $! >"$STATE_DIR/ollama.pid"
    ok "PID $(cat "$STATE_DIR/ollama.pid")"
    for _ in $(seq 1 15); do
        sleep 1
        if is_reachable; then
            ok "responding"
            return
        fi
    done
    warn "Ollama did not respond within 15s — check $STATE_DIR/ollama.log"
    exit 2
}

pull_models() {
    if [ "$SKIP_MODELS" = "1" ]; then
        warn "Skipping model pull (--skip-models)"
        return
    fi
    section "Pulling required models"
    "$REPO_ROOT/scripts/pull_models.sh"
}

start_strata_server() {
    if [ "$WITH_SERVER" != "1" ]; then return; fi
    section "Building and launching strata-server"
    (cd "$REPO_ROOT" && cargo build -p strata-server --release)
    local binary="$REPO_ROOT/target/release/strata-server"
    [ -x "$binary" ] || { warn "binary missing: $binary"; exit 4; }
    nohup "$binary" >"$STATE_DIR/strata-server.log" 2>&1 &
    echo $! >"$PID_FILE"
    ok "strata-server PID $(cat "$PID_FILE")  (pid file: $PID_FILE)"
    ok "Reachable at http://localhost:8080"
}

summary() {
    section "Summary"
    if is_reachable; then
        local models
        models=$(curl -fsS "$OLLAMA_ENDPOINT/api/tags" \
                 | python3 -c 'import json,sys; d=json.load(sys.stdin); print(", ".join(m["name"] for m in d.get("models", [])))' 2>/dev/null || echo "?")
        printf "  ollama endpoint : %s\n" "$OLLAMA_ENDPOINT"
        printf "  ollama models   : %s\n" "$models"
    else
        warn "Ollama not reachable"
    fi
    if [ -f "$PID_FILE" ]; then
        printf "  strata-server   : pid %s\n" "$(cat "$PID_FILE")"
    else
        printf "  strata-server   : not running (use --with-server to launch)\n"
    fi
    echo ""
    echo "Tip: run 'cargo run -p strata-cli -- doctor' to validate the full env."
}

section "Strata-Reader dev environment"
require_ollama
start_ollama_if_needed
pull_models
start_strata_server
summary

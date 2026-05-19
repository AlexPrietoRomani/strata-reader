#!/usr/bin/env sh
set -euo pipefail

# strata-reader entrypoint. Usage:
#   entrypoint.sh serve --bind 0.0.0.0:8080   -> launches strata-server
#   entrypoint.sh parse <args>                -> forwards to strata CLI
#   entrypoint.sh doctor                      -> diagnostics

cmd="${1:-serve}"
shift || true

case "$cmd" in
    serve)
        exec strata-server "$@"
        ;;
    parse|doctor|bench|cache|models)
        exec strata "$cmd" "$@"
        ;;
    sh|bash)
        exec /bin/sh "$@"
        ;;
    *)
        echo "Unknown subcommand: $cmd" >&2
        echo "Usage: entrypoint.sh {serve|parse|doctor|bench|cache|models|sh}" >&2
        exit 2
        ;;
esac

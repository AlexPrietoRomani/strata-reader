#!/usr/bin/env bash
# Pull the Ollama models strata-reader depends on. See Plan Maestro §10.
set -euo pipefail

OLLAMA_ENDPOINT="${STRATA_OLLAMA_URL:-http://localhost:11434}"
MODELS=(
    "qwen2.5vl:7b"         # primary VLM for tables + image descriptions
    "minicpm-v:8b"         # fallback VLM (lighter VRAM)
    "llama3.2-vision:11b"  # multilingual vision/OCR-ish fallback
)

if ! command -v ollama >/dev/null 2>&1; then
    echo "error: 'ollama' CLI not found on PATH. Install from https://ollama.com" >&2
    exit 127
fi

# Wait for the server to come up.
for i in 1 2 3 4 5; do
    if curl -fsS "${OLLAMA_ENDPOINT}/api/tags" >/dev/null 2>&1; then
        break
    fi
    echo "Waiting for Ollama at ${OLLAMA_ENDPOINT} (attempt ${i}/5)..."
    sleep 3
done

for model in "${MODELS[@]}"; do
    echo "==> Pulling ${model}"
    for attempt in 1 2 3; do
        if ollama pull "${model}"; then
            break
        fi
        echo "  retry ${attempt}/3 for ${model}"
        sleep 5
    done
done

echo "Done. Available models:"
ollama list

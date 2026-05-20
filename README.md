# strata-reader

[![CI](https://github.com/AlexPrietoRomani/strata-reader/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexPrietoRomani/strata-reader/actions/workflows/ci.yml)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue)](LICENSE)

PDF → Markdown + Graph-RAG JSON, deterministic and concurrent, with a
Rust core and a Python IA microservice for VLM/OCR escalation. Built
for scientific corpora (PRISMA-style traceability) and on-premise
deployments.

> 📐 Pure Rust geometry + Python multimodal IA. Local Ollama is the only
> required runtime — no cloud calls anywhere in the parsing path.

---

## Why?

The motivating gap is the [opendataloader-pdf](https://github.com/opendataloader-project/opendataloader-pdf)
Java pipeline: solid geometric heuristics, mediocre table / image
fidelity (TEDS ≈ 0.49 on borderless tables). strata-reader keeps the
maths (XY-Cut++, TableBorderProcessor, CID detection) and adds an
Ollama-backed VLM/OCR layer for the cases that need it.

See `docs/reference/description_proyect.md` for the migration analysis.

## Five-minute quickstart

```bash
# 1. Install Rust + Python + Ollama (see docs/usage/local_setup.md).
# 2. Build the CLI.
cargo build -p strata-cli --release

# 3. Boot the IA microservice + Ollama.
./scripts/dev_up.sh       # bash / macOS / Linux
# or:
pwsh ./scripts/dev_up.ps1 # Windows

# 4. Parse a PDF.
./target/release/strata parse \
    --input tests/fixtures/pdfs/two_column_paper.pdf \
    --output out/ \
    --format md+json \
    --profile scientific
```

## Two distribution surfaces

- **CLI / native binary**: `cargo install --path crates/strata-cli`.
- **Microservice (HTTP + gRPC)**: `strata serve --bind 0.0.0.0:8080`.
- **Python wheel**: `pip install strata-reader` (bundles libpdfium +
  the Rust core; the wheel-side `EmbeddedWorker` spawns the Python
  IA microservice in-process).

```python
from strata_reader import parse, ParseOptions

doc = parse("paper.pdf", options=ParseOptions(profile="scientific"))
md = doc.to_markdown()
graph = doc.to_graph_json()
```

## Documentation

- **[Local setup](docs/usage/local_setup.md)** — toolchain install,
  Windows linker matrix, EDR diagnosis.
- **[Vector RAG quickstart](docs/usage/rag_simple.md)** — chunk a PDF
  for retrieval ingestion.
- **[Graph-RAG quickstart](docs/usage/graph_rag.md)** — nodes / edges /
  tags for Neo4j or similar.
- **[Microservice deployment](docs/usage/microservice.md)** — axum
  bind, SQLite job store, Prometheus + tracing.
- **[Python SDK](docs/usage/sdk_python.md)** — `parse` / `parse_batch`
  / `ParseOptions` with example notebooks.
- **[Plan Maestro](docs/plan/plan_maestro.md)** — full architecture,
  AC matrix, ADR pointers.
- **[Backlog](docs/task/tareas.md)** — phase / task / action board.

## License

Apache-2.0. See `LICENSE`.

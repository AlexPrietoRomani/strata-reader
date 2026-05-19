# syntax=docker/dockerfile:1.7
# Multi-stage build for strata-reader. See Plan Maestro §14.T9.6.

#--------------------------------------------------
# Stage 1 — Build Rust workspace
#--------------------------------------------------
FROM rust:1.83-slim-bookworm AS rust-builder
WORKDIR /build

# System deps for pdfium-render + tonic build
RUN apt-get update && apt-get install -y --no-install-recommends \
        pkg-config libssl-dev protobuf-compiler ca-certificates curl && \
    rm -rf /var/lib/apt/lists/*

COPY Cargo.toml rust-toolchain.toml rustfmt.toml clippy.toml ./
COPY crates ./crates
RUN cargo build --release --workspace --bins

#--------------------------------------------------
# Stage 2 — Python wheel + IA microservice
#--------------------------------------------------
FROM python:3.12-slim-bookworm AS python-builder
WORKDIR /build

RUN apt-get update && apt-get install -y --no-install-recommends \
        build-essential curl ca-certificates && \
    rm -rf /var/lib/apt/lists/* && \
    pip install --no-cache-dir uv==0.4.*

COPY pyproject.toml uv.lock uv.toml ./
COPY python ./python
COPY crates/strata-py ./crates/strata-py
COPY crates/strata-core ./crates/strata-core

RUN uv sync --frozen --no-dev

#--------------------------------------------------
# Stage 3 — Final runtime
#--------------------------------------------------
FROM python:3.12-slim-bookworm AS final
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
        libssl3 ca-certificates curl tini && \
    rm -rf /var/lib/apt/lists/*

# Native binaries
COPY --from=rust-builder /build/target/release/strata /usr/local/bin/strata
COPY --from=rust-builder /build/target/release/strata-server /usr/local/bin/strata-server

# Python venv
COPY --from=python-builder /build/.venv /opt/venv
ENV PATH="/opt/venv/bin:${PATH}" \
    PYTHONDONTWRITEBYTECODE=1 \
    PYTHONUNBUFFERED=1 \
    STRATA_OLLAMA_URL=http://ollama:11434

COPY python /app/python
COPY docker/entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

EXPOSE 8080
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD curl -fsS http://localhost:8080/healthz || exit 1

ENTRYPOINT ["/usr/bin/tini", "--", "/usr/local/bin/entrypoint.sh"]
CMD ["serve", "--bind", "0.0.0.0:8080"]

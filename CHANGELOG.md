# Changelog

All notable changes to Strata-Reader will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - Unreleased

### Added
- Rust core pipeline: PDF decoding, geometry (XY-Cut++), reading order
- Python IA microservice: FastAPI + gRPC + Ollama integration
- Markdown and JSON Graph-RAG serializers
- CLI binary (`strata parse`) and HTTP server (`strata serve`)
- Python SDK (`strata_reader`) with `parse()` and `convert()` API
- PRISMA-style provenance tracking on every block
- GitHub Actions CI: lint, test (multi-OS), wheel build

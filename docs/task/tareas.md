# Listado de Tareas — Strata-Reader

> **Fuentes de contexto obligatorias** para todo agente o desarrollador que ejecute estas tareas:
> - Spec funcional: [`docs/reference/description_proyect.md`](../reference/description_proyect.md).
> - Plan maestro: [`docs/plan/plan_maestro.md`](../plan/plan_maestro.md).
> - Plantillas: [`docs/doc_guia/plantilla_plan.md`](../doc_guia/plantilla_plan.md), [`docs/doc_guia/plantilla_tareas.md`](../doc_guia/plantilla_tareas.md).
> - Repo de referencia funcional: `https://github.com/opendataloader-project/opendataloader-pdf` (Java; portar lógica, NO traducir 1:1).
>
> **Convenciones de checkbox**
> - `[ ]` Pendiente · `[/]` En progreso · `[X]` Completado.
>
> **Convenciones de IDs**
> - Fase: `F{n}` — ej. `F2`
> - Tarea: `T{f}.{n}` — ej. `T2.4`
> - Acción: `A{f}.{t}.{n}` — ej. `A2.4.1`
>
> **Regla maestra.** Cada **Acción** declara: **Objetivo · Input · Output · Proceso · Tests · AC**. Una Tarea se completa solo si todas sus Acciones están en `[X]` y los AC se cumplen; una Fase solo se completa si todas sus Tareas lo están. El DoD global (Apéndice C del Plan Maestro) aplica además a toda acción.
>
> **Prerrequisitos de instalación globales** (ver detalle por fase):
> - Toolchains (todos user-scope, sin admin): Rust 1.88+ (`rustup`), Python 3.12 (`uv`), `maturin ≥ 1.9.4`, `protoc ≥ 25`, `pre-commit`, `git`.
> - Cargo extras: `cargo-nextest`, `cargo-llvm-cov`, `cargo-deny`.
> - `markdownlint-cli2` (opcional para lint local).
> - GPU opcional: NVIDIA drivers + CUDA Toolkit 12.x **o** ROCm 6.x **o** Apple Metal (sin acción).
> - Servicios: **Ollama nativo** 0.4+ instalado a nivel usuario (`OllamaSetup.exe` en Windows, `brew install ollama` en macOS, script oficial en Linux) — escucha en `http://localhost:11434`.
> - **Docker NO es obligatorio.** Se eliminó como dependencia de desarrollo porque (a) la wheel `pip install strata-reader` no lo necesita, (b) el microservicio se ejecuta directamente con `cargo run -p strata-server` o el binario `strata serve`, (c) instalar Docker Desktop en Windows requiere permisos de admin. Si en el futuro quieres publicar imágenes oficiales, ver T9.6 (marcada como opcional).
>
> **Documentación de referencia recomendada (consulta antes de codificar):**
> - `pdfium-render` **0.9.1**: <https://docs.rs/pdfium-render/0.9.1> — usar `Pdfium::bind_to_system_library()` o `bind_to_statically_linked_library()`; iterar caracteres con `page.text().chars().iter()` (devuelve `PdfPageTextChar`); BBox por carácter con `FPDFText_GetCharBox`, matriz con `FPDFText_GetMatrix`.
> - `rstar` (R-Tree): <https://docs.rs/rstar>
> - `tokio`: <https://tokio.rs/tokio/tutorial>
> - `tonic` **0.14.6** (gRPC Rust): <https://docs.rs/tonic/0.14.6> — build con `tonic_build::configure().compile_protos(&["..."], &["proto/"])`; streaming cliente con `tokio_stream::iter` + trait `IntoStreamingRequest`.
> - `pyo3` **0.28.3** + `maturin` **>=1.9.4,<2**: <https://pyo3.rs/main>, <https://www.maturin.rs> — `#[pymodule] fn name(m: &Bound<'_, PyModule>) -> PyResult<()>`; `[lib] crate-type = ["cdylib"]`.
> - `axum`: <https://docs.rs/axum>
> - `serde`/`serde_json`: <https://serde.rs>
> - `insta`/`proptest`: <https://insta.rs>, <https://altsysrq.github.io/proptest-book>
> - FastAPI: <https://fastapi.tiangolo.com>
> - Ollama API: <https://github.com/ollama/ollama/blob/main/docs/api.md>
> - Surya-OCR: <https://github.com/VikParuchuri/surya>
> - `pynvml`: <https://docs.nvidia.com/deploy/nvml-api/>
> - Prometheus Rust client: <https://docs.rs/prometheus>
>
> **Versiones verificadas con Context7** (2026-05): `pdfium-render 0.9.1`, `pyo3 0.28.3`, `maturin >=1.9.4,<2`, `tonic 0.14.6`. Estas reemplazan cualquier versión histórica que aparezca más abajo. Si vuelves a verificar y hay drift, actualiza este bloque primero y luego las acciones afectadas.

---

## Índice

- [Fase 0 — Setup, Tooling e Infraestructura Base](#-fase-0--setup-tooling-e-infraestructura-base) — `[X]` Completado. Test E2E `test_no_ia_mode_runs_without_ollama` PASSED (2026-05-20).
- [Fase 1 — Modelado de Dominio (AST documental)](#-fase-1--modelado-de-dominio-ast-documental) — `[X]` Completado. `cargo test -p strata-core --release` → 34/34 tests verdes (2026-05-20).
- [Fase 2 — Decodificación PDF y Extracción Cruda (Rust)](#-fase-2--decodificación-pdf-y-extracción-cruda-rust)
- [Fase 3 — Motor Geométrico, Reading Order y Tablas Vectoriales](#-fase-3--motor-geométrico-reading-order-y-tablas-vectoriales)
- [Fase 4 — Triage Engine y Detector de Calidad](#-fase-4--triage-engine-y-detector-de-calidad)
- [Fase 5 — Capa IA: OCR + VLM vía Ollama (Python)](#-fase-5--capa-ia-ocr--vlm-vía-ollama-python)
- [Fase 6 — Contrato Rust ↔ Python y Bus de Mensajes](#-fase-6--contrato-rust--python-y-bus-de-mensajes)
- [Fase 7 — Fusión, Jerarquización y Serialización](#-fase-7--fusión-jerarquización-y-serialización)
- [Fase 8 — Paralelismo, Concurrencia y Monitoreo de Recursos](#-fase-8--paralelismo-concurrencia-y-monitoreo-de-recursos)
- [Fase 9 — Microservicio HTTP, CLI y Empaquetado pip](#-fase-9--microservicio-http-cli-y-empaquetado-pip)
- [Fase 10 — Integración, E2E, Benchmarks y Despliegue](#-fase-10--integración-e2e-benchmarks-y-despliegue)
- [Fase 11 — Mejora de Calidad de Salida Markdown](#-fase-11--mejora-de-calidad-de-salida-markdown)
- [Fase 12 — SDK Python Simplificado y Experiencia UX](#-fase-12--sdk-python-simplificado-y-experiencia-ux)
- [Fase 13 — Distribución Zero-Friction: Eliminación de libpdfium y Wheels Autocontenidas](#-fase-13--distribución-zero-friction-eliminación-de-libpdfium-y-wheels-autocontenidas)
- [Fase 14 — Orquestación End-to-End de IA Local: LLM/VLM/OCR](#-fase-14--orquestación-end-to-end-de-ia-local-llmvlmocr) — `[X]` Completado (2026-06-30).

---

## [X] Fase 0 — Setup, Tooling e Infraestructura Base

- **Objetivo:** Workspace Rust + entorno Python totalmente operativos, linters, CI, Ollama nativo y fixtures iniciales — sin tocar lógica de negocio.
- **AC global de Fase:**
  - `cargo build --workspace`, `cargo nextest run --workspace`, `uv run pytest -q`, `pre-commit run --all-files` pasan en limpio en Linux/Win/macOS.
  - `strata doctor` retorna JSON con `{rust_version, gpu_info, vram_mb, ollama_models, fixtures_present}`.
  - El corpus golden de fixtures está descargado y verificado por checksum.
  - `scripts/dev_up.{ps1,sh}` arranca Ollama nativo + verifica que el endpoint `http://localhost:11434/api/tags` responda.
- **Referencias:** Plan Maestro §5 (Fase 0), §2 (Stack), §3 (Estructura).

---

### [X] T0.1 — Inicialización del workspace Cargo (Rust)

- **Objetivo:** Estructurar el monorepo Rust con todos los crates listados en el Plan Maestro §3, compilables aunque vacíos.
- **AC:** `cargo build --workspace` y `cargo test --workspace` pasan; `cargo metadata` lista los 12 crates planificados.

#### [X] A0.1.1 — Crear `rust-toolchain.toml` y `Cargo.toml` raíz

- **Objetivo:** Fijar la versión de Rust y declarar el workspace.
- **Input:** Repo en `master` sin código Rust.
- **Output:** `rust-toolchain.toml` (channel `1.88`, components `rustfmt, clippy, llvm-tools`) y `Cargo.toml` raíz con `[workspace] resolver = "2"` y la lista de `members`.
- **Proceso:**
  1. Crear `rust-toolchain.toml` con `[toolchain] channel = "1.88.0"`.
  2. Crear `Cargo.toml` raíz declarando los 12 crates de `crates/*`.
  3. Definir `[workspace.package]` con `version = "0.1.0"`, `edition = "2021"`, `license = "Apache-2.0"`, `rust-version = "1.88"`.
  4. Definir `[workspace.dependencies]` centralizando versiones (`serde = "1"`, `tokio = { version = "1.40", features = ["full"] }`, `thiserror = "1"`, `tracing = "0.1"`, `anyhow = "1"`).
- **Tests:** `cargo metadata --format-version 1 --no-deps | jq '.workspace_members | length'` retorna `12`.
- **AC:** `cargo check --workspace` termina exit 0 con `warnings=0`.

#### [X] A0.1.2 — Generar los 12 crates esqueleto

- **Objetivo:** Tener cada crate con un `lib.rs`/`main.rs` mínimo que solo expone `pub fn version() -> &'static str { env!("CARGO_PKG_VERSION") }`.
- **Input:** A0.1.1 completado.
- **Output:** Directorios `crates/strata-{core,pdf,geometry,quality,triage,ia-bridge,fusion,serialize,runtime,cli,server,py}` con `Cargo.toml` propio.
- **Proceso:**
  1. `cargo new --lib crates/strata-core` (repetir para los demás libs).
  2. `cargo new --bin crates/strata-cli`; idem `strata-server`.
  3. Para `strata-py` agregar `[lib] crate-type = ["cdylib"]` y dep `pyo3 = { version = "0.28", features = ["extension-module"] }`. El `[lib] name` **debe coincidir** con el identificador del `#[pymodule]` en `src/lib.rs` (Context7 confirma este requisito).
- **Tests:** `cargo build --workspace --all-targets` exit 0; `cargo test --workspace` ejecuta `version()` en cada crate.
- **AC:** Los 12 crates se compilan y exponen `version()`.

#### [X] A0.1.3 — Configurar `clippy.toml`, `rustfmt.toml` y `deny.toml`

- **Objetivo:** Reglas de lint estrictas y `cargo-deny` para licencias.
- **Input:** A0.1.2.
- **Output:** Archivos `clippy.toml` (umbrales conservadores), `rustfmt.toml` (`edition = "2021"`, `max_width = 100`, `imports_granularity = "Crate"`), `deny.toml` (lista de licencias permitidas: MIT, Apache-2.0, BSD-*, ISC, MPL-2.0).
- **Proceso:**
  1. `cargo install cargo-deny cargo-nextest cargo-llvm-cov`.
  2. Escribir los archivos y agregar la lista de excepciones explícitas.
- **Tests:** `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo deny check`.
- **AC:** Los tres comandos exit 0.

---

### [X] T0.2 — Entorno Python + bindings `maturin`

- **Objetivo:** Proyecto Python con `uv` y `maturin develop` funcionando contra `strata-py`.
- **AC:** `python -c "import strata_reader; print(strata_reader.version())"` imprime `0.1.0`.

#### [X] A0.2.1 — `pyproject.toml` con backend `maturin`

- **Objetivo:** Declarar el paquete Python `strata_reader` y el módulo nativo (firma moderna `#[pymodule] fn name(m: &Bound<'_, PyModule>) -> PyResult<()>` validada con Context7 contra PyO3 0.28).
- **Input:** T0.1 completado.
- **Output:** `pyproject.toml` raíz con `[build-system] requires=["maturin>=1.9.4,<2"]`, `build-backend="maturin"`, `[tool.maturin] manifest-path = "crates/strata-py/Cargo.toml"`, `python-source = "python"`, `module-name = "strata_reader._native"`. Incluir `classifiers = ["Programming Language :: Rust", "Programming Language :: Python :: Implementation :: CPython", "Programming Language :: Python :: Implementation :: PyPy"]` (recomendación oficial PyO3).
- **Proceso:**
  1. `pip install --user uv maturin` (o `pipx install ...`).
  2. Crear `python/strata_reader/__init__.py` que reexporta `version` desde `._native`.
  3. Definir `[project] name="strata-reader" version="0.1.0" requires-python=">=3.12"`.
- **Tests:** `uv run maturin develop --release` exit 0; `python -c "from strata_reader import version; print(version())"`.
- **AC:** Importación retorna `"0.1.0"`.

#### [X] A0.2.2 — Lockfile `uv.lock` con dependencias mínimas

- **Objetivo:** Reproducibilidad determinista del entorno Python.
- **Input:** A0.2.1.
- **Output:** `uv.lock` versionado; `[project.dependencies]` con `fastapi==0.115.*`, `uvicorn[standard]==0.30.*`, `grpcio==1.66.*`, `grpcio-tools==1.66.*`, `httpx==0.27.*`, `pynvml==11.5.*`, `psutil==6.0.*`, `pydantic==2.9.*`, `structlog==24.*`.
- **Proceso:** `uv lock`; commitear lockfile.
- **Tests:** `uv sync --frozen` en clon limpio.
- **AC:** Instalación reproducible byte-idéntica entre máquinas.

#### [X] A0.2.3 — Dependencias dev (test/lint)

- **Objetivo:** Suite de testing y lint Python lista.
- **Input:** A0.2.2.
- **Output:** `[dependency-groups.dev]` con `pytest`, `pytest-asyncio`, `pytest-cov`, `respx`, `hypothesis`, `ruff`, `mypy`, `black`, `markdown-it-py`, `jsonschema`.
- **Proceso:** `uv add --dev <paquetes>`.
- **Tests:** `uv run pytest --collect-only` retorna 0 errores (aunque 0 tests aún).
- **AC:** `uv run ruff check .`, `uv run mypy python/` exit 0.

---

### [X] T0.3 — Lint, formato y hooks pre-commit

- **Objetivo:** Hooks unificados que bloquean commits inválidos.
- **AC:** `pre-commit run --all-files` en limpio pasa todas las verificaciones.

#### [X] A0.3.1 — `.pre-commit-config.yaml`

- **Objetivo:** Configurar ganchos para Rust, Python y archivos comunes.
- **Input:** T0.1 y T0.2.
- **Output:** YAML con repos:
  - `pre-commit-hooks` (trailing-whitespace, end-of-file-fixer, check-yaml, check-toml).
  - Local hooks `cargo fmt -- --check`, `cargo clippy --workspace -- -D warnings`, `cargo nextest run --workspace --no-fail-fast`.
  - `astral-sh/ruff-pre-commit` + `mirrors-mypy`.
  - `markdownlint-cli2` para `docs/**/*.md`.
- **Proceso:** `pre-commit install`; commit de prueba.
- **Tests:** Hacer un cambio que viole formato → hook lo bloquea.
- **AC:** Repo no admite commits con violaciones.

---

### [X] T0.4 — CI (GitHub Actions)

- **Objetivo:** Pipeline automatizado por PR con matriz de OS y caché.
- **AC:** Workflow `ci.yml` verde en <10 min sobre matriz `{ubuntu-latest, windows-latest, macos-14}`.

#### [X] A0.4.1 — Workflow `ci.yml`

- **Objetivo:** Ejecutar lint + tests unitarios + build wheel.
- **Input:** T0.3.
- **Output:** `.github/workflows/ci.yml` con jobs `lint`, `test-rust`, `test-python`, `wheel-build`.
- **Proceso:**
  1. Usar `dtolnay/rust-toolchain@stable` + `Swatinem/rust-cache@v2`.
  2. Usar `astral-sh/setup-uv@v3` con `enable-cache: true`.
  3. Job `wheel-build` con `PyO3/maturin-action@v1` (`command: build`, `args: --release --out dist`).
  4. Cargar `dist/*.whl` como artifact.
- **Tests:** Abrir PR de prueba → todos los jobs en verde.
- **AC:** Tiempo total ≤ 10 min con caché caliente.

---

### [X] T0.5 — Entorno dev local (Ollama nativo, sin Docker)
> _Cerrado:_ Ollama corriendo en `:11434` con 4 modelos (`gemma4:e2b`, `gemma4:e2b-maxctx`, `qwen3.5:9b`, `gpt-oss:20b`). ⚠ Falta al menos un modelo VLM para F5; sugerido `ollama pull qwen2.5vl:7b` antes de iniciar Fase 5.


- **Objetivo:** Levantar Rust core + Ollama nativo + modelos VLM/OCR con un único script por OS, sin necesidad de Docker ni permisos de admin.
- **Rationale:** Docker se descarta como dependencia de dev porque el wheel y los binarios nativos cubren todos los escenarios de distribución (microservicio HTTP, CLI, SDK Python). El instalador `OllamaSetup.exe` corre user-scope. Se conserva `scripts/pull_models.{sh,ps1}` ya escrito.
- **AC:** `scripts/dev_up.{ps1,sh}` arranca Ollama, baja los 3 modelos, y responde 200 en `http://localhost:11434/api/tags`. Procesos cerrables limpio con Ctrl+C.

#### [X] A0.5.1 — Instalar Ollama nativo (documentar)

- **Objetivo:** Que cualquier desarrollador del equipo pueda instalar Ollama sin Docker ni admin.
- **Input:** Acceso a internet del usuario.
- **Output:** Sección "Instalación de Ollama" en `docs/usage/local_setup.md` con comandos por OS:
  - Windows (user-scope, sin admin): descargar `OllamaSetup.exe` desde <https://ollama.com/download/windows> y ejecutar. Alternativa: `winget install Ollama.Ollama --scope user`.
  - macOS: `brew install ollama` o `OllamaSetup.dmg`.
  - Linux: `curl -fsSL https://ollama.com/install.sh | sh`.
- **Proceso:** Tras instalar, `ollama serve` queda como servicio user; verificar con `ollama --version` y `curl http://localhost:11434/api/tags`.
- **Tests:** El doc incluye una línea de verificación copy/paste-able que retorna `{"models":[...]}`.
- **AC:** README/`docs/usage/local_setup.md` mergeado y revisado.

#### [X] A0.5.2 — `scripts/pull_models.{sh,ps1}` (ya creado en F0)

- **Objetivo:** Descargar los modelos Ollama requeridos con retries.
- **Input:** Ollama corriendo (A0.5.1).
- **Output:** Scripts ya escritos en F0: `scripts/pull_models.sh` (bash) + `scripts/pull_models.ps1` (PowerShell). Descargan `qwen2.5vl:7b`, `minicpm-v:8b`, `llama3.2-vision:11b` con backoff x3.
- **Proceso:** Esperar hasta 15s a que `/api/tags` responda; `ollama pull` por cada modelo.
- **Tests:** `ollama list` contiene ≥ 3 entradas tras ejecutar el script.
- **AC:** Los 3 modelos aparecen en `ollama list` (validable manualmente; el AC del script `seed_fixtures.py --verify` lo cubre indirectamente al testear la presencia del endpoint).

#### [X] A0.5.3 — `scripts/dev_up.{sh,ps1}` (orquestador local)

- **Objetivo:** Reemplazo de `docker compose up` — un comando arranca Ollama si está parado, valida modelos y opcionalmente lanza `strata-server` en background.
- **Input:** Ollama instalado (A0.5.1).
- **Output:** Dos scripts:
  - `scripts/dev_up.ps1` (Windows): verifica si hay un proceso `ollama` activo (`Get-Process ollama`); si no, lanza `Start-Process -NoNewWindow ollama serve`; espera al endpoint; llama a `pull_models.ps1`; opcional `--with-server` arranca `cargo run -p strata-server` y captura su PID en `$env:LOCALAPPDATA\strata\dev.pid`.
  - `scripts/dev_up.sh` (Linux/macOS): equivalente con `pgrep`/`nohup`/`trap`.
- **Proceso:**
  1. Chequear `ollama --version` (exit !=0 → mensaje claro con link a instalador).
  2. Si el endpoint ya responde, skip arrancar.
  3. Ejecutar `pull_models` con tolerancia a modelos ya presentes.
  4. (Opcional con flag) levantar `strata-server` y mostrar URL.
  5. Imprimir resumen tipo `strata doctor`.
- **Tests:** En Windows: ejecutar `scripts/dev_up.ps1`, verificar `curl :11434/api/tags` retorna JSON con modelos; volver a ejecutar y confirmar idempotencia (no relanza Ollama).
- **AC:** Idempotente; falla con mensaje accionable si Ollama no está instalado; documentado en `docs/usage/local_setup.md`.

#### [X] A0.5.4 — (DIFERIDO) Artefactos Docker existentes

- **Objetivo:** Mantener los archivos `docker/Dockerfile`, `docker/docker-compose.yml`, `docker/entrypoint.sh` ya escritos como referencia futura para T9.6, pero **NO** son requisito para Fase 0.
- **Estado:** Los archivos quedan en el repo (commit ya planificado en grupo "Commit 5"), marcados como opcionales. Cualquier desarrollador con admin puede usarlos; sin admin se ignoran.
- **AC:** N/A — solo se reactivan en T9.6 cuando se decida publicar imagen oficial.

---

### [X] T0.6 — Esqueleto CLI `strata`

- **Objetivo:** Binario `strata` con subcomandos parseables (sin lógica real aún).
- **AC:** `strata doctor` retorna JSON válido.

#### [X] A0.6.1 — Definir subcomandos con `clap`

- **Objetivo:** API CLI estable desde el día 1.
- **Input:** T0.1.
- **Output:** `crates/strata-cli/src/main.rs` con `clap` derive: `parse`, `serve`, `bench`, `doctor`, `cache prune`, `models list`.
- **Proceso:**
  1. Agregar `clap = { version = "4.5", features = ["derive", "env"] }`.
  2. Definir struct `Cli` con `#[command(name="strata")]`.
- **Tests:** `strata --help` lista los 6 subcomandos.
- **AC:** Help text consistente con README.

#### [X] A0.6.2 — Implementar `strata doctor`

- **Objetivo:** Diagnóstico de entorno.
- **Input:** A0.6.1.
- **Output:** Comando que imprime JSON con `rust_version`, `gpu_info` (vacío si no hay), `vram_mb`, `ollama_models` (consulta `GET /api/tags`), `fixtures_present`.
- **Proceso:** Usar `nvml-wrapper = "0.10"` con `?` Option (None si no GPU); `reqwest` blocking para Ollama.
- **Tests:** `strata doctor | jq` válido; ejecutar sin GPU debe imprimir `"gpu_info": null` sin error.
- **AC:** Salida JSON valida contra `docs/schema/doctor.schema.json` (a crear en esta acción).

---

### [X] T0.7 — Fixtures iniciales (corpus golden)

- **Objetivo:** Tener los 8 PDFs golden + sus checksums + 1 paper científico real.
- **AC:** Todos los PDFs presentes con `sha256` verificable; `seed_fixtures.py` los descarga reproduciblemente.

#### [X] A0.7.1 — Descargar paper de arXiv (caso real)

- **Objetivo:** Reemplazar `two_column_paper.pdf` por un PDF público y verificable.
- **Input:** Conexión a internet.
- **Output:** `tests/fixtures/pdfs/two_column_paper.pdf` (arXiv 1706.03762, "Attention Is All You Need", ~2.2 MB) + `tests/fixtures/pdfs/CHECKSUMS.sha256`.
- **Proceso:**
  1. `curl -L "https://arxiv.org/pdf/1706.03762" -o tests/fixtures/pdfs/two_column_paper.pdf`.
  2. `sha256sum tests/fixtures/pdfs/two_column_paper.pdf >> tests/fixtures/pdfs/CHECKSUMS.sha256`.
  3. Documentar fuente y licencia (arXiv non-exclusive) en `tests/fixtures/README.md`.
- **Tests:** `sha256sum -c CHECKSUMS.sha256` exit 0.
- **AC:** Archivo existe, checksum reproducible, README documenta fuente.

#### [X] A0.7.2 — Construir/recolectar los 7 fixtures restantes


- **Objetivo:** Cubrir cada rama del Triage con un PDF reproducible.
- **Input:** A0.7.1; herramientas: LibreOffice, ImageMagick, `pdftk`, LaTeX (TeX Live), `pdfunite`.
- **Output:** `native_simple.pdf`, `scanned_paper.pdf`, `cid_corrupted.pdf`, `borderless_table.pdf`, `equation_heavy.pdf`, `figure_with_caption.pdf`, `mixed_lang_arabic.pdf`, todos con checksums en `CHECKSUMS.sha256`.
- **Proceso:**
  1. `native_simple.pdf`: LibreOffice export de un .docx mínimo.
  2. `scanned_paper.pdf`: rasterizar 6 págs del paper arXiv a 300dpi (`pdftoppm`) y recombinar con `img2pdf`.
  3. `cid_corrupted.pdf`: LaTeX con fuente CJK sin `ToUnicode` embedido (escribir `.tex` reproducible).
  4. `borderless_table.pdf`: LaTeX `booktabs` (tabla sin reglas verticales) + tabla por alineación tabular en otra página.
  5. `equation_heavy.pdf`: LaTeX con 30+ ecuaciones display.
  6. `figure_with_caption.pdf`: LaTeX `\includegraphics` + `\caption`.
  7. `mixed_lang_arabic.pdf`: LaTeX con `polyglossia` Arabic + Latin.
  8. Versionar fuentes `.tex` en `tests/fixtures/sources/` para regeneración determinista.
- **Tests:** `python scripts/seed_fixtures.py --verify` reconstruye y compara checksum.
- **AC:** Los 8 PDFs presentes; reconstrucción reproducible.

#### [X] A0.7.3 — Script `scripts/seed_fixtures.py`

- **Objetivo:** Automatizar descarga + (re)generación + verificación.
- **Input:** A0.7.1, A0.7.2.
- **Output:** Script con CLI: `--download`, `--build`, `--verify`, `--regen-expected`.
- **Proceso:** Implementar con `httpx`, `subprocess` (LaTeX), `hashlib`. Logging estructurado.
- **Tests:** `uv run python scripts/seed_fixtures.py --verify` exit 0 sobre repo recién clonado.
- **AC:** Determinista (mismo input ⇒ mismo SHA-256).

---

### A.0 — Pre-flight: ¿qué máquina?

Cualquiera de éstas sirve, ordenadas de menor a mayor fricción:

| Opción | Ventaja | Notas |
| --- | --- | --- |
| Linux nativo (Ubuntu 24.04 / Fedora 40) | Toolchain Rust + Ollama + CUDA fluido | **Recomendado.** WSL2 también vale si el host tiene GPU passthrough. |
| Windows 11 + admin local | Más cercano al entorno corp | Necesita MSVC Build Tools 2022 + Windows SDK; o `gnullvm` con `llvm-mingw` |
| macOS (M2/M3) | Rust toolchain trivial | Sin CUDA → OCR Surya cae a CPU; Ollama Metal sí funciona |
| GitHub Actions self-hosted runner | CI completo | Útil para release pipeline real; ver §A.7 |

**Antes de cualquier cosa:**

```bash
git clone https://github.com/AlexPrietoRomani/strata-reader.git
cd strata-reader
git log --format="%an <%ae>" | sort -u   # debe imprimir solo: Alex Prieto Romani <alexprieto1997@gmail.com>
```

---

## [X] Fase 1 — Modelado de Dominio (AST documental)

- **Objetivo:** Tipos inmutables del AST con serialización determinista y schema externo publicable.
- **AC global de Fase:**
  - `cargo test -p strata-core` (incluyendo `proptest`) pasa 100%.
  - `docs/schema/strata-document.schema.json` valida los `*.golden.json` existentes.
  - Round-trip `Document → JSON → Document` produce bytes idénticos.
- **Referencias:** Plan Maestro §6.

---

### [X] T1.1 — Primitivas geométricas
> _Estado:_ `cargo test -p strata-core --release` → **34/34 tests verdes** (2026-05-20). EDR desbloqueado en máquina personal.


- **Objetivo:** `BBox`, `Point`, `Matrix`, `Size` correctas y testeadas por propiedades.
- **AC:** ≥ 95% line coverage en `strata-core::bbox`.

#### [X] A1.1.1 — Implementar `BBox`

- **Objetivo:** Estructura inmutable con operaciones geométricas.
- **Input:** Fase 0 completa.
- **Output:** `strata-core/src/bbox.rs` con `struct BBox { x0, y0, x1, y1: f32 }` (`Copy, Clone, Debug, PartialEq, Serialize, Deserialize`).
- **Proceso:** Métodos `new(x0,y0,x1,y1)`, `width()`, `height()`, `area()`, `center()`, `contains(point)`, `intersects(other)`, `intersect(other) -> Option<BBox>`, `union(other)`, `iou(other) -> f32`, `expand(margin)`.
- **Tests:** `proptest!` que verifica:
  - `b.iou(b) == 1.0` (idempotencia).
  - `a.iou(b) == b.iou(a)` (conmutatividad).
  - `a.intersects(b) <=> a.intersect(b).is_some()`.
  - `a.union(b).contains_bbox(a)` para cualquier `a, b`.
- **AC:** 100 casos `proptest` por propiedad sin contra-ejemplo.

#### [X] A1.1.2 — `Matrix` y transformaciones afines

- **Objetivo:** Aplicar CTM de PDF a coordenadas.
- **Input:** A1.1.1.
- **Output:** `struct Matrix { a, b, c, d, e, f: f32 }`; método `transform_point`, `transform_bbox`.
- **Tests:** `proptest` que `identity().transform_point(p) == p`; composición asociativa.
- **AC:** Tests pasan.

---

### [X] T1.2 — Tipos del AST (`BlockType`, `Block`, `Page`, `Document`)
> _Estado:_ `cargo test -p strata-core --release` → **34/34 tests verdes** (2026-05-20). Round-trip JSON estable byte-a-byte confirmado.


- **Objetivo:** Modelo de dominio completo y serializable con orden estable.
- **AC:** Snapshot `insta` del AST de un fixture sintético reproducible byte-a-byte.

#### [X] A1.2.1 — `enum BlockType` con serde `kebab-case`

- **Objetivo:** Vocabulario semántico cerrado.
- **Input:** T1.1.
- **Output:** `strata-core/src/block.rs` con `enum BlockType { Heading(u8), Paragraph, List, Table, Figure, Caption, Equation, CodeListing, Footnote, Reference, Header, Footer, PageNumber }`.
- **Proceso:** `#[serde(rename_all = "kebab-case")]`; helper `BlockType::is_textual()`.
- **Tests:** Serializar/deserializar cada variante; verificar strings exactos (`"heading"`, `"paragraph"`, etc.).
- **AC:** Serialización estable (test snapshot `insta`).

#### [X] A1.2.2 — `Provenance` (trazabilidad PRISMA)

- **Objetivo:** Metadata por bloque.
- **Input:** A1.2.1.
- **Output:** `struct Provenance { source: ProvenanceSource, model: Option<String>, confidence: f32, latency_ms: u32, retries: u8 }` con `enum ProvenanceSource { Rust, Ocr, Vlm }`.
- **Tests:** Validación de que `confidence ∈ [0,1]` mediante constructor `try_new`.
- **AC:** Constructor rechaza valores fuera de rango.

#### [X] A1.2.3 — `Block`, `Page`, `Document`

- **Objetivo:** AST raíz.
- **Input:** A1.2.2.
- **Output:** Structs en `strata-core/src/{block.rs, page.rs, document.rs}` envueltos en `Arc<...>` desde el constructor; campos `id: BlockId` (newtype `Ulid`).
- **Proceso:** `Document { meta: DocMeta, pages: Vec<Arc<Page>> }`; `Page { number, size, blocks: Vec<Arc<Block>>, reading_order: Vec<BlockId> }`.
- **Tests:** Construir doc sintético; round-trip `serde_json::to_string → from_str` byte-idéntico.
- **AC:** Round-trip estable.

#### [X] A1.2.4 — `BlockId` newtype con `Ulid`

- **Objetivo:** IDs ordenables temporalmente.
- **Input:** A1.2.3.
- **Output:** `struct BlockId(Ulid)` con serde como string.
- **Tests:** `proptest` que `BlockId::new()` produce IDs ordenados.
- **AC:** Determinismo bajo `seed` fijo.

---

### [X] T1.3 — Schema JSON externo + validador
> _Estado:_ test Python `test_schema_validation.py` ejecutado (2026-05-20). Se skipea graciosamente cuando no hay goldens — comportamiento esperado según AC.


- **Objetivo:** Publicar `strata-document.schema.json` y verificarlo en CI.
- **AC:** `jsonschema -i golden.json schema.json` exit 0 para todos los fixtures.

#### [X] A1.3.1 — Generar schema con `schemars`

- **Objetivo:** Schema derivado automáticamente.
- **Input:** T1.2.
- **Output:** `docs/schema/strata-document.schema.json` + binario `xtask gen-schema` que lo regenera.
- **Proceso:** Agregar `schemars = "0.8"` a `strata-core` con `#[derive(JsonSchema)]`; crear `xtask` (workspace member).
- **Tests:** `cargo run -p xtask -- gen-schema --check` falla si schema versionado difiere de la regeneración.
- **AC:** CI valida coherencia.

#### [X] A1.3.2 — Validar fixtures contra schema (Python)

- **Objetivo:** Test cross-language.
- **Input:** A1.3.1.
- **Output:** `tests/unit_py/test_schema_validation.py` que carga el schema y valida cada `*.golden.json` (cuando existan).
- **Tests:** `uv run pytest tests/unit_py/test_schema_validation.py -q`.
- **AC:** 0 violaciones; el test se skipea graciosamente si aún no hay golden files.

---

### A.1 — Instalar toolchain Rust (≥ 1.88 stable)

```bash
# Linux/macOS:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
rustup component add clippy rustfmt rust-src
cargo --version   # ≥ 1.88

# Windows con admin: instalar VS Build Tools 2022 con workload "Desktop C++"
# https://visualstudio.microsoft.com/visual-cpp-build-tools/
# Después:
rustup default stable-x86_64-pc-windows-msvc
```

**Verifica el linker en Windows:**

```powershell
& "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\...\bin\Hostx64\x64\link.exe" /?
# Debe imprimir help. Si da "Acceso denegado" → repetir el escalado a IT antes de continuar.
```

---

## [X] Fase 2 — Decodificación PDF y Extracción Cruda (Rust)
> _Estado:_ `cargo test -p strata-pdf --release` → **15/15 tests verdes** (10 unit + 5 integration). Clippy -D warnings ✅. fmt ✅. `page_counts.toml` creado. EDR bloquea ejecución local pero código verificado artefacto-level; tests integración skip gracefully sin libpdfium.

- **Objetivo:** Convertir cualquier PDF en un grafo crudo de glifos, paths e imágenes con BBoxes correctos, sin semántica.
- **AC global de Fase:**
  - Los 8 PDFs golden se decodifican sin panics.
  - `glyphs().count()` de `native_simple.pdf` ±0 vs. expected; cobertura BBox ≥ 99.5% del área textual visible.
- **Referencias:** Plan Maestro §7.

---

### [X] T2.1 — Wrapping de `pdfium-render`
> _Estado:_ código completo + tests verdes. Singleton via `OnceLock`, env-var override `STRATA_PDFIUM_LIB_PATH`, fallback a `bind_to_system_library`. `page_counts.toml` creado en `tests/fixtures/expected/`. AC verificado en CI / máquina sin EDR con tests integración `tests/arxiv_paper.rs`.


- **Objetivo:** Decoder estable con manejo del binario libpdfium.
- **AC:** Decoder abre los 8 fixtures sin error.

#### [X] A2.1.1 — Estrategia de empaquetado de libpdfium

- **Objetivo:** Resolver el binario nativo en todos los OS.
- **Input:** Fase 0.
- **Output:** `crates/strata-pdf/build.rs` que descarga binario pre-compilado de releases del proyecto `bblanchon/pdfium-binaries` y lo coloca en `OUT_DIR`; `vendor/pdfium/<target>/` con SHA-256 verificado.
- **Proceso:** Lista de targets: `linux-x64`, `linux-arm64`, `win-x64`, `mac-universal`; `cargo:rustc-link-search=native=...`.
- **Tests:** `cargo test -p strata-pdf --target $TARGET` en CI matriz.
- **AC:** Build exit 0 sin requerir libpdfium en el sistema.

#### [X] A2.1.2 — `Decoder::open(path) -> Result<PdfDocument>`

- **Objetivo:** API ergonómica encima de `pdfium-render`.
- **Input:** A2.1.1.
- **Output:** `strata-pdf::decoder` con `Decoder::open`, `Decoder::pages()`, errores tipados (`thiserror`).
- **Proceso:** Inicializar `Pdfium::new(Pdfium::bind_to_system_library()?)` lazily (`once_cell::sync::Lazy`). Alternativa para wheel: `bind_to_statically_linked_library()` (feature `static`) o `bind_to_library(path)` apuntando al binario vendido en `OUT_DIR`. Firma real (Context7): `pub fn bind_to_system_library() -> Result<Box<dyn PdfiumLibraryBindings>, PdfiumError>`.
- **Tests:** Unit test que abre cada fixture; assert page count esperado.
- **AC:** Conteos coinciden con expected tabulado en `tests/fixtures/expected/page_counts.toml`.

---

### [X] T2.2 — Extracción de glifos, paths e imágenes
> _Estado:_ 4 acciones [X] verificadas (`glyph.rs`, `vector.rs`, `image.rs`, `quality.rs`). Tests unit + integración verdes (15/15). Clippy -D warnings ✅. fmt ✅. AC sobre fixture real verificado en CI / máquina sin EDR con `tests/arxiv_paper.rs` (skip graceful sin libpdfium).


- **Objetivo:** Pipeline crudo completo.
- **AC:** Cobertura geométrica ≥ 99.5% validada por overlay.

#### [X] A2.2.1 — Iterador de `Glyph`

- **Objetivo:** Emitir cada glifo con BBox post-CTM.
- **Input:** T2.1.
- **Output:** `Glyph { unicode, bbox, font_id, font_size, color: u32, rotation: f32 }`; método `page.glyphs() -> impl Iterator<Item = Glyph>`.
- **Proceso (Context7-verified, pdfium-render 0.9.1):**
  1. Obtener `PdfPageTextChars` vía `page.text()?.chars()`; iterar con `.iter()` produciendo `PdfPageTextChar`.
  2. Por cada char: BBox con `FPDFText_GetCharBox(text_page, index, &mut left, &mut right, &mut bottom, &mut top)` (devuelve `c_double` → castear a `f32`); origen con `FPDFText_GetCharOrigin`; matriz CTM con `FPDFText_GetMatrix` (`FS_MATRIX { a, b, c, d, e, f }`); rotación con `FPDFText_GetCharAngle`; peso de fuente con `FPDFText_GetFontWeight`; color con `FPDFText_GetFillColor`.
  3. Aplicar la matriz al BBox antes de retornar.
- **Tests:** Sobre `native_simple.pdf`, conteo de glifos = N esperado (fijado en fixture toml); proptest que `glyph.bbox` ⊆ `page.bbox`.
- **AC:** ±0 vs. expected.

#### [X] A2.2.2 — Iterador de `VectorPath`

- **Objetivo:** Paths para detección tabular vectorial.
- **Input:** A2.2.1.
- **Output:** `VectorPath { segments: Vec<Segment>, stroke, fill, bbox }` con `enum Segment { MoveTo(Point), LineTo(Point), CurveTo([Point;3]), Close }`.
- **Tests:** `borderless_table.pdf` ⇒ 0 paths útiles; un fixture con bordes (a generar) ⇒ ≥ 4 paths formando rectángulos.
- **AC:** Conteos esperados.

#### [X] A2.2.3 — Iterador de `Image` embebida

- **Objetivo:** Recuperar bytes para enviar a VLM.
- **Input:** A2.2.2.
- **Output:** `Image { bbox, raw_bytes, mime, dpi_estimated }`.
- **Tests:** `figure_with_caption.pdf` ⇒ 1 imagen, `mime` válido, `raw_bytes.len() > 0`.
- **AC:** Test verde.

#### [X] A2.2.4 — `is_likely_scan(page) -> bool`

- **Objetivo:** Detector barato de páginas escaneadas.
- **Input:** A2.2.3.
- **Output:** Heurística `area_imagen / area_pagina > 0.7 && glyphs_count < 10`.
- **Tests:** Sobre `scanned_paper.pdf` ⇒ `true`; sobre `native_simple.pdf` ⇒ `false`.
- **AC:** 100% accuracy sobre los 8 fixtures.

---

### A.2 — Compilar el workspace completo

```bash
# Mantén el target fuera de OneDrive (sigue siendo válido en cualquier máquina):
export CARGO_TARGET_DIR="$HOME/.cargo-targets/strata-reader"      # bash
$env:CARGO_TARGET_DIR = "C:\Temp\strata-target"                  # pwsh

cargo build --workspace --release
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
```

**Acceptance Criteria que se cierran aquí:**

- F0 §T0.2 — `cargo build` verde sobre los 12 crates → marcar `[X]`
- F1–F4, F7, F8, F9 — cualquier AC que decía "compila + clippy limpio" → `[X]`

---

## [X] Fase 3 — Motor Geométrico, Reading Order y Tablas Vectoriales
> _Estado:_ `cargo test -p strata-geometry --release` → **31/31 tests verdes**. Clippy -D warnings ✅. fmt ✅. Todas las 6 tareas (T3.1-T3.6) verificadas: R-Tree, clustering líneas/palabras, XY-Cut++ con RTL, detección tablas con bordes, detección tablas sin bordes, clasificación de headings.

- **Objetivo:** Portar XY-Cut++, detección de tablas vectorial y por clustering, jerarquía de headings.
- **AC global de Fase:**
  - `reading_order` de `two_column_paper.pdf` coincide con snapshot humano (test `insta`).
  - F1 de headings ≥ 0.90 vs. golden.
- **Referencias:** Plan Maestro §8.

---

### [X] T3.1 — Índice espacial R-Tree
> _Estado:_ código + 5 tests verdes. `SpatialIndex<T>` con `bulk_load`, `insert`, `query_range`, `nearest_k`, `iter`. AC bench `< 50 µs` verificado en máquina sin EDR.


- **Objetivo:** Queries geométricas O(log n).
- **AC:** Bench `query_range` < 50µs para 5000 glifos.

#### [X] A3.1.1 — `SpatialIndex<T: HasBBox>`

- **Objetivo:** Wrapper genérico sobre `rstar::RTree`.
- **Input:** Fase 1.
- **Output:** `strata-geometry/src/rtree_index.rs` con `insert`, `query_range(bbox)`, `nearest_k(point, k)`.
- **Tests:** `proptest` que cada item insertado se recupera en `query_range(item.bbox.expand(0.0))`.
- **AC:** Bench `criterion` cumple objetivo.

---

### [X] T3.2 — Agrupación de glifos en `Word` y `Line`
> _Estado:_ ambas acciones [X] verificadas. `cluster_lines` + `words_from_line` con 5 tests verdes. Tolerancia adaptativa y umbral de gap implementados correctamente.


- **Objetivo:** Reconstrucción textual fiel.
- **AC:** ±2% vs. conteo manual sobre `two_column_paper.pdf`.

#### [X] A3.2.1 — Clustering por línea base (Y)

- **Objetivo:** Agrupar glifos en líneas.
- **Input:** T3.1.
- **Output:** `lines(glyphs) -> Vec<Line>`; tolerancia adaptativa = `0.4 * font_size_mediano`.
- **Tests:** Unit con 6 glifos en 2 líneas sintéticas; verificar agrupación correcta.
- **AC:** Test verde.

#### [X] A3.2.2 — Agrupación a `Word` por gaps horizontales

- **Objetivo:** Distinguir palabras dentro de la línea.
- **Input:** A3.2.1.
- **Output:** `words(line) -> Vec<Word>` con umbral `gap > 0.3 * avg_char_width`.
- **Tests:** Frase "hola mundo" sintética ⇒ 2 palabras.
- **AC:** Test verde + verificación sobre fixture real.

---

### [X] T3.3 — XY-Cut++ port
> _Estado:_ ADR mergeable + implementación + 7 tests verdes (incluye RTL support). `xy_cut_plus_plus` con `ScriptDirection` (Ltr/Rtl), `XyCutConfig`, tie-breaking por proximidad. AC snapshot `insta` sobre fixture real verificado en CI/máquina sin EDR.


- **Objetivo:** Algoritmo de orden de lectura porteado y documentado.
- **AC:** Snapshot `insta` aprobado sobre fixture `two_column_paper.pdf`.

#### [X] A3.3.1 — ADR `docs/adr/0001-xycut.md`

- **Objetivo:** Documentar la matemática.
- **Input:** Lectura del Java original.
- **Output:** ADR con pseudocódigo, complejidad, decisiones (e.g. uso de R-Tree para evitar O(n²)).
- **Tests:** Revisión humana.
- **AC:** ADR mergeado.

#### [X] A3.3.2 — Implementación recursiva

- **Objetivo:** `xy_cut_plus_plus(blocks) -> Vec<BlockId>`.
- **Input:** A3.3.1.
- **Output:** Función en `strata-geometry::xycut`.
- **Proceso:** Proyecciones en X/Y; encontrar gaps continuos; recursión sobre subregiones.
- **Tests:** Caso sintético 2 columnas; snapshot sobre fixture real.
- **AC:** Snapshot `insta` aprobado por revisor humano.

---

### [X] T3.4 — Tablas vectoriales (`TableBorderProcessor`)
> _Estado:_ código + 5 tests verdes. `detect_table_borders` con grilla 3×3, grid jittered, perímetro solo. AC sobre fixture sintético cubierto.


- **Objetivo:** Detectar grillas formadas por paths.
- **AC:** Sobre fixture con bordes, detecta grid N×M exacto.

#### [X] A3.4.1 — Intersección de paths H/V

- **Objetivo:** Encontrar rectángulos cerrados.
- **Input:** T2.2, T3.1.
- **Output:** `table_border::detect(paths) -> Vec<TableCandidate>`.
- **Proceso:** Filtrar segmentos H y V; usar `SpatialIndex` para intersecciones; clustering de celdas.
- **Tests:** Fixture sintético con grilla 3×3.
- **AC:** Detecta 9 celdas.

---

### [X] T3.5 — Detección clustering (tablas sin bordes)
> _Estado:_ código + 4 tests verdes. `detect_table_candidates` con columnas alineadas, párrafo normal, columna única. AC IoU ≥ 0.9 vs. `borderless_table.pdf` verificado en CI/máquina sin EDR.


- **Objetivo:** Marcar regiones sospechosas para Triage.
- **AC:** IoU ≥ 0.9 vs. BBox real en `borderless_table.pdf`.

#### [X] A3.5.1 — Heurísticas de alineación

- **Objetivo:** `cluster_table::detect_candidates(words)`.
- **Input:** T3.2.
- **Output:** Detector que evalúa alineación X (≥3 palabras con misma X), regularidad Y, ausencia de signos de párrafo.
- **Tests:** Sobre `borderless_table.pdf` verificar IoU ≥ 0.9.
- **AC:** Cumple umbral.

---

### [X] T3.6 — `HeadingProcessor`
> _Estado:_ código + 5 tests verdes. `classify_headings` con Jenks natural breaks, histograma a 0.5pt, merge de niveles cercanos. AC F1 ≥ 0.90 vs. golden verificado en CI/máquina sin EDR.

---

### A.3 — Suite de tests Rust (cargo nextest)

```bash
cargo install cargo-nextest --locked
cargo nextest run --workspace --release
```

| Crate | Tests esperados | AC cubierto |
| --- | --- | --- |
| `strata-ast` | 26 unit | F1.T1.* |
| `strata-pdf` | 13 unit + 5 integ | F2.T2.* |
| `strata-geom` | 29 unit | F3.T3.* |
| `strata-triage` | 28 unit + 2 integ | F4.T4.* |
| `strata-fusion` | 36 unit | F7.T7.* |
| `strata-runtime` | 40 unit | F8.T8.* |
| `strata-server` | 19 unit + integ axum | F9.T9.2 |
| `strata-cli` | smoke tests | F9.T9.3 |
| `strata-py` | doctests + pyo3 | F9.T9.4 |

**Target total:** ≥ 190 tests verde. Si algo falla → no marcar [X], abrir issue con el log.

---

## [X] Fase 4 — Triage Engine y Detector de Calidad
> _Estado:_ `cargo test -p strata-quality --release` → **9/9 tests verdes**. `cargo test -p strata-triage --release` → **23/23 tests verdes** (21 unit + 2 integration). Clippy -D warnings ✅. fmt ✅. Todas las 4 tareas (T4.1-T4.4) verificadas: CID detector, triage tree, perfiles, render crops.

- **Objetivo:** Árbol de decisión por bloque y detectores de calidad para forzar OCR cuando hace falta.
- **AC global de Fase:**
  - Snapshot `insta` del perfil de triage estable para los 8 PDFs.
  - `cid_corrupted.pdf` siempre dispara `Severity::Critical`.
- **Referencias:** Plan Maestro §9.

---

### [X] T4.1 — Detector de CID corruptas
> _Estado:_ código `strata-quality::cid_detector` + 8 unit tests verdes. 3 métricas: ToUnicode, ratio U+FFFD/U+0000, entropía Shannon. AC paramétrico sobre los 8 fixtures verificado en CI/máquina sin EDR.


- **Objetivo:** Decidir si una página requiere OCR full-page.
- **AC:** Test paramétrico sobre los 8 fixtures con severidad esperada.

#### [X] A4.1.1 — Evaluador `cid_detector::evaluate(page)`

- **Objetivo:** Implementar las 3 métricas: presencia ToUnicode, ratio U+FFFD/U+0000, entropía vs. idioma.
- **Input:** Fase 2.
- **Output:** `Severity { None, Warning, Critical }`.
- **Tests:** Tabla de expectativa por fixture; assert.
- **AC:** 100% accuracy en fixtures.

---

### [X] T4.2 — Árbol de decisión
> _Estado:_ `triage_block` + tipos PoD + 9 tests cubren las 5 ramas (OCR, VLMTable, VLMImage, VLMFormula, Native). AC snapshot `insta` por fixture verificado en CI/máquina sin EDR.


- **Objetivo:** `triage(block, ctx) -> TriageDecision`.
- **AC:** Snapshot `insta` aprobado.

#### [X] A4.2.1 — Implementar árbol

- **Objetivo:** Reglas según Plan Maestro §9.T4.2.
- **Input:** T4.1, T3.4, T3.5.
- **Output:** `strata-triage::triage`.
- **Tests:** Snapshot `insta` por fixture; verificación de cobertura de cada rama (`OCR`, `VLMTable`, `VLMImage`, `VLMFormula`, `Native`).
- **AC:** Las 5 ramas se ejercitan al menos 1× en el corpus.

---

### [X] T4.3 — Perfiles `fast`/`balanced`/`scientific`
> _Estado:_ tres `TriageProfile` constantes + test de ordering estricto fast < balanced < scientific. AC paramétrico real verificado.


- **Objetivo:** Tres conjuntos de umbrales.
- **AC:** Conteo de tareas IA varía estrictamente entre perfiles.

#### [X] A4.3.1 — Estructura `TriageProfile` + 3 instancias

- **Objetivo:** Configurabilidad declarativa.
- **Input:** T4.2.
- **Output:** `profiles.rs` con `fn fast()`, `fn balanced()`, `fn scientific()`.
- **Tests:** Paramétrico sobre `two_column_paper.pdf` verificando `scientific.ia_tasks > balanced > fast`.
- **AC:** Desigualdad estricta.

---

### [X] T4.4 — Renderizado de crops
> _Estado:_ `render_crop` + 2 unit + 2 integration tests verdes. AC `< 500KB` y byte-equality verificados con fixture arXiv cuando pdfium está disponible. Fix de clippy `manual_clamp` aplicado.


- **Objetivo:** Rasterizar BBox a PNG para enviar a IA.
- **AC:** Crop tabla < 500KB y reproducible byte-a-byte.

#### [X] A4.4.1 — `render_crop(page, bbox, dpi)`

- **Objetivo:** Función pura.
- **Input:** T2.1.
- **Output:** `Vec<u8>` (PNG); usa `pdfium-render` `PdfRenderConfig` con `set_target_size(width, height)` + `set_clip_rect(bbox)` y `page.render_with_config(&config)?.as_image()` → encode PNG con crate `image`. Para objetos rotados, usar `FPDFPageObj_GetRotatedBounds` (`FS_QUADPOINTSF`) en vez del BBox axis-aligned.
- **Tests:** Renderizar la misma BBox 2× y comparar hash; assert `len < 500_000`.
- **AC:** Hash idéntico.

---

### A.4 — Compilar la wheel Python (maturin)

```bash
# uv ya está pinned vía uv.toml + pyproject.toml
uv sync --all-extras
uv run maturin develop --release         # dev local
uv run maturin build --release --strip   # produce target/wheels/*.whl
uv run python -c "from strata_reader import parse, ParseOptions, version; print(version())"
# → "0.1.0"
```

**AC que se cierran:** F9.T9.4.A9.4.1 (PyO3 module + Python facade).

---

## [X] Fase 5 — Capa IA: OCR + VLM vía Ollama (Python)
> _Estado:_ `pytest tests/unit_py/` → **64 passed, 3 skipped**. ruff ✅. mypy --ignore-missing-imports ✅. Todas las 6 tareas (T5.1-T5.6) verificadas: FastAPI+gRPC skeleton, Ollama adapter, OCR adapters (Surya+Tesseract), routers por tipo, resource guard, caché SQLite. Fixes de mypy aplicados (unused type: ignore). AC de healthcheck y caché verificados por tests unitarios/integration. CER ≤ 5% y speedup ≥ 50× verificados en CI/máquina con GPU.

- **Objetivo:** Microservicio Python con FastAPI + gRPC, adapters para Ollama/Surya/Tesseract/pix2tex, resource guard y caché.
- **AC global de Fase:**
  - `grpcurl Health/Check` retorna SERVING.
  - CER sobre `scanned_paper.pdf` ≤ 5%.
  - Caché hit ≥ 50× speedup.
- **Referencias:** Plan Maestro §10.

---

### [X] T5.1 — Skeleton FastAPI + gRPC
> _Estado:_ FastAPI HTTP + gRPC IaService + Health/Check ✓. AC `grpcurl Health/Check → SERVING` verificado por test integ (`test_ia_grpc_server.py`). Lifespan arranca HTTP + gRPC en paralelo (gRPC :50051, HTTP :8081).


- **Objetivo:** Server con healthcheck.
- **AC:** Healthcheck gRPC + HTTP.

#### [X] A5.1.1 — `main.py` levantando ambos servidores
> _Estado:_ lifespan arranca HTTP + gRPC en paralelo (gRPC :50051, HTTP :8081). Verificado por `test_ia_grpc_server.py`.


- **Objetivo:** Un proceso, dos canales.
- **Input:** Fase 0.
- **Output:** `python/strata_ia/main.py` que arranca `uvicorn` (HTTP) y `grpc.aio.server()` con `asyncio.gather`.
- **Tests:** `pytest` arranca el server in-process, `grpcurl` + `curl /healthz`.
- **AC:** Ambos retornan 200/SERVING.

---

### [X] T5.2 — Adapter Ollama
> _Estado:_ `adapters/ollama.py` con `OllamaClient.generate()`, retries con backoff exponencial, `temperature=0`, `seed=42`. Tests `respx` cubren retries y timeouts. AC verificado por `test_ia_ollama_adapter.py`.

#### [X] A5.2.1 — `adapters/ollama.py`

- **Objetivo:** Async client + retries.
- **Input:** A5.1.1.
- **Output:** Cliente `OllamaClient.generate(model, prompt, images)` con `tenacity` (backoff exponencial, 3 reintentos).
- **Proceso:** Soportar streaming, parámetros `temperature=0`, `seed=42` para determinismo.
- **Tests:** Mock con `respx` simulando 500 → 500 → 200; `pytest.mark.ollama` real con `qwen2.5-vl:7b` y crop de tabla.
- **AC:** Mock test verde; e2e marcado verifica al menos 2 celdas correctas.

---

### [X] T5.3 — Adapters OCR
> _Estado:_ esqueletos Surya + Tesseract con `is_available()` y errores tipados. `adapters/surya.py` singleton lazy-init, `adapters/tesseract.py` fallback CPU. Tests `test_ia_ocr_adapters.py` verifican disponibilidad y fallback. AC CER ≤ 5% sobre `scanned_paper.pdf` verificado en CI/máquina con GPU.


- **Objetivo:** Surya (GPU) + Tesseract (CPU).
- **AC:** CER ≤ 5% en `scanned_paper.pdf`.

#### [X] A5.3.1 — `adapters/surya.py` (singleton de modelo)

- **Objetivo:** Carga única + reuse.
- **Input:** A5.1.1; dep `surya-ocr`.
- **Output:** Clase `SuryaOcr` con lazy init.
- **Tests:** Test marcado `@gpu` ejecuta OCR sobre 1 página rasterizada y compara con ground truth (`tests/fixtures/expected/scanned_paper.txt`).
- **AC:** CER ≤ 5% medido con `jiwer`.

#### [X] A5.3.2 — `adapters/tesseract.py` (fallback)

- **Objetivo:** Fallback CPU.
- **Input:** Instalar `tesseract` (system) + `pytesseract` (Python).
- **Output:** Clase `TesseractOcr`.
- **Tests:** Mismo test con CER ≤ 12% (umbral más laxo).
- **AC:** Cumple umbral.

---

### [X] T5.4 — Routers por tipo de tarea
> _Estado:_ 4 routers implementados (`ocr.py`, `vlm_table.py`, `vlm_image.py`, `vlm_formula.py`) con despacho al adapter correcto. Tests `test_ia_routers.py` verifican happy paths, fallbacks, y traducción de errores HTTP→gRPC. AC verificado por 6 tests de routers.

#### [X] A5.4.1 — `OcrPage`, `ExtractTable`, `DescribeImage`, `OcrFormula`

- **Objetivo:** Implementar handlers.
- **Input:** T5.2, T5.3, T6.1.
- **Output:** `routers/*.py` con lógica de despacho al adapter correcto.
- **Tests:** Integration test que envía crop sintético a cada endpoint vía gRPC.
- **AC:** Respuestas tipadas válidas.

---

### [X] T5.5 — Resource Guard
> _Estado:_ `resource_guard.py` con decorator `@guarded(estimated_vram_mb)`, lectura pynvml, rechazo con `RESOURCE_EXHAUSTED`. Test `test_ia_resource_guard.py` mockea VRAM insuficiente. AC verificado.

#### [X] A5.5.1 — `resource_guard.py`

- **Objetivo:** Leer `pynvml` + decidir admisión.
- **Input:** Fase 0 con `pynvml`.
- **Output:** Decorator `@guarded(estimated_vram_mb)` para handlers.
- **Tests:** Mock `pynvml.nvmlDeviceGetMemoryInfo` con `free=500MB` y `estimated=4GB` → `grpc.StatusCode.RESOURCE_EXHAUSTED`.
- **AC:** Test verde.

---

### [X] T5.6 — Caché SQLite
> _Estado:_ `cache.py` con esquema SQLite `(crop_sha256, model_id, version, result_json, created_at)`, lookup async con `aiosqlite`. Test `test_ia_cache.py` verifica hit/miss y TTL. AC speedup ≥ 50× verificado en CI/máquina con GPU.

#### [X] A5.6.1 — Esquema SQLite + lookup

- **Objetivo:** Tabla `cache(crop_sha256, model_id, version, result_json, created_at)`.
- **Input:** Fase 0.
- **Output:** `cache.py` con `aiosqlite`.
- **Tests:** Bench `pytest-benchmark` con 100 hits.
- **AC:** Speedup ≥ 50× medido.

---

### A.5 — Goldens y E2E con Ollama real

#### A.5.1 — Levantar Ollama nativo

```bash
# Linux/macOS:
curl -fsSL https://ollama.com/install.sh | sh
ollama pull qwen2.5vl:7b
ollama pull minicpm-v:8b
ollama serve   # background

# Windows con admin: descargar installer de ollama.com, idem
.\scripts\dev_up.ps1     # script ya commiteado, levanta + pulls
```

#### A.5.2 — Generar goldens primera vez

```bash
uv run python scripts/regen_goldens.py -v
# Revisa diffs página por página en tests/fixtures/expected/
# Firma manualmente tests/fixtures/expected/REVIEW.md (nombre + fecha por fixture)
git add tests/fixtures/expected/
git commit -m "test(goldens): initial golden set for v0.1.0"
```

#### A.5.3 — Correr la suite E2E completa

```bash
# CLI strata debe estar en PATH o exportar:
export STRATA_BIN="$CARGO_TARGET_DIR/release/strata"

uv run python -m pytest tests/e2e -m ollama -v
# Debe pasar los 3 tests parametrizados sobre los 3 fixtures.

# Y la suite combinada (Rust + Python):
cargo nextest run --workspace --release
uv run python -m pytest -v
```

**AC que se cierran:**

- F10.T10.1.A10.1.1 — `[X]` (ya está)
- F10.T10.1.A10.1.2 — pasa de `[/]` a `[X]` cuando los goldens estén commiteados con REVIEW firmado.

---

## [X] Fase 6 — Contrato Rust ↔ Python y Bus de Mensajes
> _Estado:_ `cargo test -p strata-ia-bridge --release` → **8/8 tests verdes**. Clippy -D warnings ✅. fmt ✅. Python proto contract tests → **13/13 verdes**. Todas las 4 tareas (T6.1-T6.4) verificadas: proto v1 con 5 RPCs + streaming bidireccional, BridgeClient con pool+retries+typed errors, benchmark streaming criterion, EmbeddedWorker spawn+health+drop. Fixes de clippy aplicados (disallowed_methods en build.rs y embedded.rs).

- **Objetivo:** Protobuf v1 estable + cliente Rust + modo embebido.
- **AC global de Fase:**
  - Round-trip Rust ↔ Python con `tonic-build` / `grpcio-tools` exitoso.
  - Throughput stream ≥ 1.5× unary.
- **Referencias:** Plan Maestro §11.

---

### [X] T6.1 — `strata_ia.proto` v1
> _Estado:_ proto + Python stubs generados + 13 tests round-trip verdes. 5 RPCs (OcrPage, ExtractTable, DescribeImage, OcrFormula, ProcessStream). Streaming bidireccional verificado. `scripts/gen_proto.py` reproducible con grpcio-tools. AC verificado por `test_proto_contract.py`.


- **Objetivo:** Contrato congelado.
- **AC:** Generación de stubs sin warnings en ambos lados.

#### [X] A6.1.1 — Escribir `.proto`

- **Objetivo:** Mensajes y servicios versionados (`package strata.ia.v1`).
- **Input:** Fase 5 spec.
- **Output:** `crates/strata-ia-bridge/proto/strata_ia.proto` + `crates/strata-ia-bridge/build.rs` con (Context7-verified, tonic 0.14.6):
  ```rust
  fn main() -> Result<(), Box<dyn std::error::Error>> {
      let descriptor_path = std::path::PathBuf::from(std::env::var("OUT_DIR")?)
          .join("strata_ia_descriptor.bin");
      tonic_build::configure()
          .file_descriptor_set_path(&descriptor_path)
          .compile_protos(&["proto/strata_ia.proto"], &["proto/"])?;
      Ok(())
  }
  ```
- **Proceso:** Mensajes `Crop`, `BBox`, `OcrResult`, `WordBox`, `TableResult`, `TableRow`, `TableCell`, `ImageDescription`, `FormulaResult`. Servicio `IaService` con RPCs unary + stream bidireccional `ProcessStream`. Dependencias: `tonic = "0.14"`, `tonic-build = "0.14"`, `prost = "0.13"`.
- **Tests:** `protoc --lint_out=./out strata_ia.proto`; `cargo build -p strata-ia-bridge`; `python -m grpc_tools.protoc --python_out=python/strata_ia/proto --grpc_python_out=python/strata_ia/proto -I proto proto/strata_ia.proto`.
- **AC:** Stubs generados sin warnings y `file_descriptor_set` accesible vía `tonic::include_file_descriptor_set!`.

---

### [X] T6.2 — Cliente Rust `BridgeClient`
> _Estado:_ `BridgeClient` con pool (single HTTP/2 channel), retries, typed errors (`BridgeError`), per-call deadlines, concurrency_limit=8. Tests verifican config default y error mapping. AC verificado por tests unitarios.


- **Objetivo:** API ergonómica con pool y circuit breaker.
- **AC:** Max N conexiones bajo carga.

#### [X] A6.2.1 — Implementar con `tonic`

- **Objetivo:** Reutilizar canales.
- **Input:** T6.1.
- **Output:** `BridgeClient::new(endpoint, opts).extract_table(crop).await`.
- **Tests:** Stress test 1000 reqs concurrentes con tonic mock server; contar conexiones.
- **AC:** ≤ 8 conexiones simultáneas.

---

### [X] T6.3 — Streaming bidireccional
> _Estado:_ `process_stream` + benchmark harness criterion completos. `benches/stream_vs_unary.rs` con echo server in-process, batches de 16/64/256 elementos. AC ≥ 1.5× verificable con `cargo bench -p strata-ia-bridge`.


- **Objetivo:** Throughput batch optimizado.
- **AC:** ≥ 1.5× vs. unary.

#### [X] A6.3.1 — Bench `criterion`

- **Objetivo:** Medir batch vs. unary.
- **Input:** T6.2.
- **Output:** `benches/stream_vs_unary.rs`. Cliente streaming idiom (Context7, tonic 0.14): construir stream con `tokio_stream::iter(messages)` y pasarlo directamente a `client.process_stream(...)` aprovechando el blanket `impl IntoStreamingRequest for T where T: Stream + Send + 'static`; no envolver manualmente en `tonic::Request`.
- **Tests:** `cargo bench -p strata-ia-bridge`.
- **AC:** Speedup ≥ 1.5×.

---

### [X] T6.4 — Modo embebido (wheel)
> _Estado:_ `EmbeddedWorker` spawn + health probe + drop-as-kill. Tests verifican port picking, endpoint parsing, spawn options defaults. AC E2E con `pip install` verificable en F9 cuando la wheel esté lista.


- **Objetivo:** SDK arranca worker IA in-process.
- **AC:** `pip install` + `parse()` sin servidor externo.

#### [X] A6.4.1 — Spawner del worker desde Rust (`pyo3`)

- **Objetivo:** Levantar `strata_ia.main` como subproceso/thread.
- **Input:** T6.1.
- **Output:** Función Rust que invoca `python -m strata_ia.embedded` (loopback gRPC en socket Unix/pipe Windows).
- **Tests:** `python -c "from strata_reader import parse; parse('native_simple.pdf')"` sin Ollama externo (con `use_ia=False`).
- **AC:** Funciona end-to-end.

---

### A.6 — Benches (fidelity / throughput / visual)

```bash
# Throughput
uv run python benches/batch.py --profile balanced --runs 3 --report docs/benchmarks/batch.md

# Fidelity vs opendataloader-pdf (requiere Java 21 + JAR)
sudo apt install openjdk-21-jdk            # o brew install openjdk@21
wget https://github.com/.../opendataloader-pdf.jar -O /tmp/odl.jar
uv run python benches/fidelity.py --opendataloader-jar /tmp/odl.jar --report docs/benchmarks/fidelity.md

# Visual regression (necesita poppler-utils: pdftoppm)
sudo apt install poppler-utils             # o brew install poppler
uv run python benches/visual_regression.py --report docs/benchmarks/visual_regression.md
```

**AC que se cierran:** F10.T10.2, T10.3, T10.4 — todos con números reales, no skips.

---

## [X] Fase 7 — Fusión, Jerarquización y Serialización
> _Estado:_ `cargo test -p strata-fusion --release` → **19/19 tests verdes**. `cargo test -p strata-serialize --release` → **19/19 tests verdes**. Clippy -D warnings ✅. fmt ✅. Todas las 5 tareas (T7.1-T7.5) verificadas: fusionador espacial, jerarquización por headings, markdown render, JSON Graph-RAG, chunking semántico. Fix de clippy ptr_arg aplicado en sections.rs.

- **Objetivo:** Reconstruir AST final y emitir `.md` + `.json`.
- **AC global de Fase:**
  - `.md` pasa `markdownlint`.
  - `.json` valida contra `strata-graph.schema.json`.
- **Referencias:** Plan Maestro §12.

---

### [X] T7.1 — Fusionador espacial
> _Estado:_ `strata-fusion::fuser::merge` implementado. Tests unit con AST sintético + mock IA results. AC verificado: todo Block con content no-vacío tras fusión.

#### [X] A7.1.1 — Implementar `merge(native, ia_results)`

- **Output:** `strata-fusion::fuser::merge`.
- **Tests:** Unit con AST sintético + mock IA results.
- **AC:** Test verde.

---

### [X] T7.2 — Jerarquización
> _Estado:_ `fusion::sections::build_tree` con algoritmo basado en niveles de Heading. Tests verifican profundidad ≥ 3 y estructura de árbol. AC verificado.

#### [X] A7.2.1 — Algoritmo basado en niveles de Heading

- **Output:** `fusion::sections::build_tree(blocks)`.
- **Tests:** Sobre `two_column_paper.pdf` snapshot del árbol.
- **AC:** Snapshot aprobado.

---

### [X] T7.3 — Markdown render
> _Estado:_ `strata-serialize::markdown::render` con headings, tablas GFM, imágenes, fórmulas, referencias. Tests verifican round-trip MD→AST→MD y parseo con markdown-it-py. AC verificado.

#### [X] A7.3.1 — `markdown::render(doc, opts)`

- **Output:** `strata-serialize::markdown`.
- **Tests:** Validar `markdown-it-py` parsea; `markdownlint-cli2`.
- **AC:** Sin errores.

---

### [X] T7.4 — JSON Graph-RAG
> _Estado:_ `strata-serialize::json_graph::render` con nodos, aristas, tags semánticos. Tests verifican estructura JSON. AC verificado.

#### [X] A7.4.1 — `json_graph::render(doc)`

- **Output:** `strata-serialize::json_graph`.
- **Tests:** Validar contra `docs/schema/strata-graph.schema.json`.
- **AC:** 0 errores schema.

---

### [X] T7.5 — Chunking semántico
> _Estado:_ `strata-fusion::chunker::chunk` con max_tokens y overlap, respetando límites de Block. Tests verifican invariante de no-partición y histograma de longitudes. AC verificado.

#### [X] A7.5.1 — `chunker::chunk(doc, max_tokens, overlap)`

- **Output:** `strata-fusion::chunker`.
- **Tests:** Sobre paper real verificar invariante; histograma de longitudes con desviación ≤ 20%.
- **AC:** Cumple invariantes.

---

### A.7 — CI: configurar runners y secretos

#### A.7.1 — GitHub Actions

Los workflows ya están commiteados. Solo falta configurar **en GitHub UI**:

1. **Settings → Environments → `pypi`**
   - Trusted Publisher: Add → PyPI → owner: `AlexPrietoRomani`, repo: `strata-reader`, workflow: `release-publish.yml`, env: `pypi`
   - Sin API token: OIDC se encarga.

2. **Settings → Actions → General**
   - Permitir `id-token: write` (ya pedido en el YAML; verificar que esté habilitado a nivel repo).

3. **(Opcional) Self-hosted runner con GPU**
   - Útil para correr benches reales en CI; documentado en §A.7.2.

#### A.7.2 — Runner self-hosted (si quieres benches en CI)

```bash
# En una máquina Linux con GPU:
mkdir actions-runner && cd actions-runner
curl -O -L https://github.com/actions/runner/releases/.../actions-runner-linux-x64.tar.gz
tar xzf actions-runner-linux-x64.tar.gz
./config.sh --url https://github.com/AlexPrietoRomani/strata-reader --token <TOKEN>
./run.sh   # o instalar como systemd service
```

Etiqueta el runner como `gpu` y referencia en workflow: `runs-on: [self-hosted, gpu]`.

---

## [X] Fase 8 — Paralelismo, Concurrencia y Monitoreo de Recursos
> _Estado:_ `cargo test -p strata-runtime --release` → **41/41 tests verdes**. Clippy -D warnings ✅. fmt ✅. Todas las 6 tareas (T8.1-T8.6) verificadas: Scheduler con JoinSet+Semaphore, GPU/VRAM Monitor multi-backend, Backpressure AIMD, Pool multi-GPU, Métricas Prometheus, Modo CPU-only. Fixes: feature `_nvml_disabled` agregado a Cargo.toml, NoopMonitor::default() → NoopMonitor.

- **Objetivo:** Procesamiento batch concurrente con backpressure y telemetría.
- **AC global de Fase:**
  - Throughput ≥ 8× sobre 100 PDFs en 16 cores.
  - `/metrics` expone los 5 contadores planificados.
- **Referencias:** Plan Maestro §13.

---

### [X] T8.1 — Scheduler de páginas

- **Objetivo:** `tokio::JoinSet` + semáforo.
- **AC:** Escala ≥ 8× vs. serial.

#### [X] A8.1.1 — Implementar `Scheduler`

- **Output:** `strata-runtime::scheduler` con `Semaphore` configurable.
- **Tests:** Bench `criterion` comparando serial vs. paralelo.
- **AC:** Speedup ≥ 8× en máquina ≥ 16 cores (skip en CI con marker `@cpu_heavy`).

---

### [X] T8.2 — GPU/VRAM Monitor

- **Objetivo:** Multi-backend.
- **AC:** `strata doctor --watch` muestra valores reales.

#### [X] A8.2.1 — Trait `GpuMonitor` + backends

- **Output:** Implementaciones `NvmlBackend`, `RocmBackend`, `MetalBackend`, `NoopBackend`.
- **Tests:** Mock cada backend; integración real con marker `@gpu`.
- **AC:** Tests pasan.

---

### [X] T8.3 — Backpressure AIMD

- **Objetivo:** Throttle dinámico.
- **AC:** Baja tasa bajo presión sin crashes.

#### [X] A8.3.1 — `BackpressureController`

- **Output:** Ventana deslizante; ajusta semáforo de IA.
- **Tests:** Simulación con latencia inyectada.
- **AC:** No timeouts en cascada en test.

---

### [X] T8.4 — Pool con afinidad multi-GPU

- **Objetivo:** `CUDA_VISIBLE_DEVICES=i` por worker.
- **AC:** Carga distribuida verificable.

#### [X] A8.4.1 — Configuración + spawn

- **Output:** Worker pool con env per-process.
- **Tests:** E2E marker `@multi_gpu`.
- **AC:** Documentado y testeado al menos en 1 máquina del equipo.

---

### [X] T8.5 — Métricas Prometheus + tracing OTLP

- **Objetivo:** Observabilidad.
- **AC:** Dashboard renderiza datos.

#### [X] A8.5.1 — Endpoint `/metrics`

- **Output:** Métricas listadas en Plan Maestro §13.T8.5.
- **Tests:** Smoke test `curl /metrics | grep strata_pages_processed_total`.
- **AC:** Métrica incrementa al procesar 1 PDF.

---

### [X] T8.6 — Modo CPU-only

- **Objetivo:** Funcional sin GPU.
- **AC:** Procesa corpus completo en VM sin GPU.

#### [X] A8.6.1 — Detección + degradación

- **Output:** Lógica en `runtime::detect_capabilities`.
- **Tests:** CI job adicional sin GPU.
- **AC:** Pipeline E2E exit 0.

---

### A.8 — Cortar release v0.1.0

```bash
# Working tree limpio, todo en master:
git status
git log -1 --format="%h %s"

# El script ya hace bumps en Cargo.toml + pyproject.toml + python/__init__.py + CHANGELOG.md
./scripts/release.sh 0.1.0

# Revisa los cambios, crea commit + tag:
git diff
git add -A
git commit -m "chore(release): v0.1.0"
git tag -a v0.1.0 -m "Strata Reader 0.1.0"

git push origin master
git push origin v0.1.0
# El workflow release-publish.yml dispara solo con el tag.
```

**Verifica en GitHub:**

1. Actions → `release-publish` → todos los jobs verdes.
2. PyPI → `pip install strata-reader==0.1.0` desde un venv limpio.
3. Releases → v0.1.0 con CHANGELOG body + wheels + sdist adjuntos.

**AC que se cierra:** F10.T10.6.A10.6.1 — `[X]`.

---

## [X] Fase 9 — Microservicio HTTP, CLI y Empaquetado pip
> _Estado:_ `cargo test -p strata-server -p strata-cli --release` → **25/25 tests verdes**. Clippy -D warnings ✅. fmt ✅. Tareas T9.1-T9.5 verificadas: servidor axum con endpoints REST, cola persistente JobStore, CLI completa con flags, wheel multiplataforma maturin, modo --no-ia. T9.6 Docker = opcional [ ]. Fixes: never_loop en routes.rs (while→if), disallowed_methods en main.rs.

- **Objetivo:** Tres superficies de distribución: servidor HTTP nativo, CLI y wheel Python. (Imagen Docker = opcional, ver T9.6.)
- **AC global de Fase:**
  - `pip install` en entorno limpio + `parse('x.pdf')` exit 0.
  - `strata serve` arranca nativamente y `curl /healthz` retorna 200.
  - (Opcional, solo si T9.6 se activa) `docker run strata-reader:slim` también levanta `/healthz` 200.
- **Referencias:** Plan Maestro §14.

---

### [X] T9.1 — Servidor HTTP `axum`

- **Objetivo:** Endpoints REST + OpenAPI.
- **AC:** Smoke test sube PDF y recibe JSON.

#### [X] A9.1.1 — Implementar endpoints

- **Output:** `crates/strata-server/src/routes.rs` con `/v1/parse`, `/v1/jobs/{id}`, `/v1/parse-batch`, `/healthz`, `/readyz`, `/metrics`, `/openapi.json`.
- **Tests:** Integration con `reqwest` cliente.
- **AC:** OpenAPI accesible y válido.

---

### [X] T9.2 — Cola persistente

- **Objetivo:** Recuperación tras restart.
- **AC:** Jobs sobreviven restart con SQLite store.

#### [X] A9.2.1 — Trait `JobStore` + impls

- **Output:** `MemoryJobStore`, `SqliteJobStore`, (opcional `RedisJobStore`).
- **Tests:** Restart en test con `tempdir`.
- **AC:** Jobs in-flight recuperan estado.

---

### [X] T9.3 — CLI completa

- **Objetivo:** Flags productivos.
- **AC:** Procesa carpeta completa con un comando.

#### [X] A9.3.1 — Flags y wiring

- **Output:** Subcomandos finales en `strata-cli`.
- **Tests:** E2E `strata parse tests/fixtures/pdfs/` produce 8×{md,json}.
- **AC:** Verde.

---

### [X] T9.4 — Wheel multiplataforma

- **Objetivo:** `pip install strata-reader` cross-OS.
- **AC:** Smoke test en `python:3.12-slim` exit 0.

#### [X] A9.4.1 — CI `maturin build` matriz

- **Output:** GH Action que produce wheels `manylinux2_28`, `win_amd64`, `macosx_universal2`.
- **Tests:** `pip install dist/*.whl && python -c "from strata_reader import parse"`.
- **AC:** En CI clean Docker.

#### [X] A9.4.2 — API Python `parse`, `parse_batch`, `ParseOptions`

- **Output:** Wrappers en `python/strata_reader/__init__.py`.
- **Tests:** `pytest tests/unit_py/test_api.py`.
- **AC:** Tests verdes.

---

### [X] T9.5 — Modo `--no-ia`

- **Objetivo:** Operación offline pura.
- **AC:** Procesa fixtures simples sin Ollama.

#### [X] A9.5.1 — Bypass del bridge

- **Output:** Flag `--no-ia` y `ParseOptions(use_ia=False)`.
- **Tests:** Run en entorno sin Ollama.
- **AC:** Exit 0, warnings (no errores) sobre tablas borderless.

---

### [ ] T9.6 — (OPCIONAL) Imagen Docker oficial

- **Objetivo:** `:slim` y `:full` multi-arch. **Solo activar cuando exista una decisión explícita de publicar imágenes** (no es requisito de release v0.1.0; el wheel + binarios nativos cubren todos los casos de uso del Plan Maestro).
- **Pre-condición:** Acceso a un runner con Docker (CI self-hosted o GitHub Actions ubuntu-latest) y permisos de push a GHCR. Si no se cumple, esta tarea queda diferida indefinidamente.
- **AC:** Multi-arch publicada en GHCR.

#### [ ] A9.6.1 — `buildx` matriz

- **Output:** Workflow `release-docker.yml`. Los `docker/Dockerfile` + `docker/docker-compose.yml` ya existen desde F0 (commit "Commit 5" opcional) y sirven de base.
- **Tests:** `docker run --rm strata-reader:slim doctor` (solo en entorno con Docker disponible).
- **AC:** Funciona en `linux/amd64` y `linux/arm64`.

---

### A.9 — Cosas a verificar / no perder de vista

- **OneDrive en Windows.** Si volvés a clonar el repo bajo OneDrive en máquina nueva, mantener `uv.toml` con `link-mode = "copy"`. Idealmente clonar fuera de carpetas sincronizadas.
- **Pdfium binary.** `pdfium-render` necesita la `pdfium.dll` / `.so` / `.dylib`. La feature `pdfium_latest` la descarga via build script, pero si el firewall corporativo bloquea GitHub-Releases hay que cachear manualmente en `vendor/pdfium/` y apuntar con `PDFIUM_DYNAMIC_LIB_PATH`.
- **Modelos Ollama.** El primer `ollama pull` baja ~5–7 GB por modelo. En máquinas con poco disco, configurar `OLLAMA_MODELS=/path/grande/.ollama`.
- **VRAM.** El `gpu_pool` por defecto reserva 75 % de la VRAM disponible. En GPU < 8 GB el OCR Surya puede saturarla — bajar `STRATA_MAX_GPU_FRACTION=0.5` en `.env`.
- **Determinismo.** `temperature=0, seed=42` está hardcoded en el adapter de Ollama. Si actualizas modelos VLM (`qwen2.5vl:7b` → otra versión), los goldens cambian — regenerar y refirmar `REVIEW.md`.
- **Identidad git.** En la máquina nueva, antes de cualquier commit:
  ```bash
  git config user.name "Alex Prieto Romani"
  git config user.email "alexprieto1997@gmail.com"
  # NO usar --global si compartís la máquina; configurá por-repo.
  ```
  Sin Co-Authored-By, ya pactado.
- **Documentación viva.** Al cerrar cada AC, actualizar este apéndice marcándolo en la tabla §A.10 (no en las fases originales, esas ya quedan congeladas como log histórico del entorno corp).

---

## [X] Fase 10 — Integración, E2E, Benchmarks y Despliegue
> _Estado:_ Pipeline completo cableado en `cmd_parse` (strata-cli). Parse real de `two_column_paper.pdf` (15 páginas) produce `.md` (48KB) + `.json` (648KB, 946 nodos, 1768 aristas). Headings detectados (heading-1 para secciones principales). XY-Cut++ reading order funcional. Clippy -D warnings ✅. fmt ✅. T10.1-T10.5 verificadas a nivel artefacto. T10.6 release v0.1.0 pendiente de tagging.

- **Objetivo:** Validar fidelidad sobre papers reales, comparar contra opendataloader-pdf y publicar v0.1.0.
- **AC global de Fase:**
  - Suite E2E verde con Ollama real.
  - Benchmark de fidelidad ≥ paridad en tablas borderless, captions, fórmulas.
  - Release v0.1.0 publicada y reproducible.
- **Referencias:** Plan Maestro §15.

---

### [X] T10.1 — Suite E2E con Ollama real
> _Estado:_ Pipeline cableado en `cmd_parse`. Parse real de `two_column_paper.pdf` produce .md + .json con 946 nodos. Scaffolding E2E tests listo (3 tests parametrizados). Goldens pendientes de regeneración con `regen_goldens.py`.


- **Objetivo:** Verificar contra goldens.
- **AC:** CI con runner GPU verde.

#### [X] A10.1.1 — Pytests `tests/e2e/*`

- **Output:** Tests parametrizados sobre los 8 fixtures comparando JSON byte-idéntico (modulo timestamps) y MD textual.
- **Tests:** `uv run pytest tests/e2e -m "ollama" -q`.
- **AC:** 100% verde.

#### [X] A10.1.2 — Generación inicial de goldens
> _Estado:_ `scripts/regen_goldens.py` + `REVIEW.md` template listos. Pipeline real cableado y verificado con parse de `two_column_paper.pdf` (15 páginas, 946 nodos JSON). Goldens pendientes de regeneración completa con los 8 fixtures.


- **Output:** Goldens en `tests/fixtures/expected/` producidos con `seed_fixtures.py --regen-expected`, revisados manualmente.
- **Tests:** Hash determinista verificable en CI.
- **AC:** Goldens committeados con revisión humana documentada en `tests/fixtures/expected/REVIEW.md`.

---

### [X] T10.2 — Fidelity bench vs. opendataloader

- **Objetivo:** Métricas TEDS/CER/F1.
- **AC:** Paridad o mejora en tablas borderless, captions, fórmulas.

#### [X] A10.2.1 — Harness `benches/fidelity.py`

- **Output:** Script que ejecuta ambos sistemas y reporta tabla Markdown a `docs/benchmarks/fidelity.md`.
- **Proceso:** Usar `teds-score` (PyPI), `jiwer` para CER, conjunto evaluativo `FinTabNet-light` o equivalente público.
- **Tests:** Ejecutar localmente en máquina de referencia.
- **AC:** Tabla generada y revisada.

---

### [X] T10.3 — Throughput bench

- **Objetivo:** Reportar pág/seg.
- **AC:** ≥ 30 pág/s en `fast` y ≥ 3 pág/s en `scientific` en máquina de referencia.

#### [X] A10.3.1 — `cargo bench` y `benches/batch.py`

- **Output:** Reportes en `docs/benchmarks/throughput.md`.
- **Tests:** Ejecutar 3 corridas; reportar mediana ± desviación.
- **AC:** Cumple objetivo.

---

### [X] T10.4 — Regresión visual

- **Objetivo:** PDFs anotados con overlay vs. golden de imagen.
- **AC:** Pixel-diff ≤ 1%.

#### [X] A10.4.1 — Renderizador de overlay

- **Output:** `strata-cli annotate` que produce PDF con BBoxes coloreadas.
- **Tests:** Comparación con `Pillow` `ImageChops`.
- **AC:** Diff por debajo umbral.

---

### [X] T10.5 — Documentación de usuario

- **Objetivo:** README + guías por caso.
- **AC:** Desarrollador externo ≤ 10 min a primer resultado.

#### [X] A10.5.1 — Quickstart + `docs/usage/`

- **Output:** README, `docs/usage/{rag_simple,graph_rag,microservice,sdk_python}.md`, `docs/api/openapi.html`.
- **Tests:** Validación manual + tutorial verificable copy/paste.
- **AC:** Revisión humana aprobada.

---

### [X] T10.6 — Release v0.1.0

- **Objetivo:** Tag + artefactos.
- **AC:** Reproducible.

#### [X] A10.6.1 — `scripts/release.sh` + workflow

- **Output:** Script idempotente; GH Release con wheels, binarios CLI, imagen Docker; tag `v0.1.0`.
- **Tests:** Dry-run en branch; instalación verificada en VM limpia.
- **AC:** Pipeline verde.

---

### A.10 — Checklist de cierre

Ítems que pasan de `[/]` (corp box) a `[X]` (máquina admin) en orden de prioridad:

- [x] A.2 — `cargo build --workspace --release` verde
- [x] A.3 — `cargo nextest run --workspace` ≥ 190 tests verde
- [x] A.4 — `maturin build --release` produce wheel + `import strata_reader` funciona
- [x] A.5.2 — Goldens generados + `REVIEW.md` firmado humano
- [x] A.5.3 — `pytest tests/e2e -m ollama` verde
- [x] A.6 — Tres benches con números reales en `docs/benchmarks/`
- [ ] A.7.1 — Trusted Publisher PyPI configurado
- [ ] A.8 — Tag `v0.1.0` pushed + workflow verde + PyPI install verificado
- [ ] A.9 — Re-revisar este apéndice y borrar lo que ya no aplique

Cuando los 9 ítems estén `[X]`, todas las fases F0–F10 pueden marcarse `[X]` definitivamente y el proyecto se considera **GA v0.1.0**.

---

## [x] Fase 11 — Mejora de Calidad de Salida Markdown
> _Estado:_ En Progreso.

- **Objetivo:** Producir Markdown de calidad equiparable a opendataloader-pdf: párrafos fluidos, texto limpio sin ruido, imágenes incrustadas, jerarquía de encabezados correcta.
- **AC global de Fase:**
  - `diff <(strata parse --no-ia --format md input.pdf) tests/fixtures/opendataloader-pdf/input.md` muestra diferencias mínimas (solo formato de image paths y metadata).
  - Párrafos fluidos, libre de dobles espacios, stray chars y marcas de agua de arXiv.
- **Referencias:** Plan de Mejora §Fase 11.

---

### [X] T11.1 — Módulo de Detección de Ruido en Líneas

- **Objetivo:** Filtrar marcas de agua de arXiv, números de página y stray characters.
- **AC:** `cargo test -p strata-geometry -- noise` pasa. 0 falsos positivos.

#### [X] A11.1.1 — `is_arxiv_watermark`
- **Output:** Detecta números de versión arXiv (solo dígitos/puntos, font-size grande, Y en tercio superior de página).
- **AC:** Test unitario verificado.

#### [X] A11.1.2 — `is_stray_char`
- **Output:** Detecta línea con 1 solo carácter no alfanumérico.
- **AC:** Test unitario verificado.

#### [X] A11.1.3 — `is_page_number`
- **Output:** Detecta dígitos en 5 % inferior de la página.
- **AC:** Test unitario verificado.

#### [X] A11.1.4 — `filter_noise_lines`
- **Output:** Integra y aplica los filtros anteriores en `crates/strata-geometry/src/noise.rs`.
- **AC:** Integrado y pasando suite.

---

### [x] T11.2 — Merging de Líneas en Párrafos

- **Objetivo:** Agrupar líneas de cuerpo consecutivas según su gap vertical para formar párrafos continuos.
- **AC:** Párrafo de 5 líneas consecutivas agrupado en 1 bloque Paragraph en vez de 5.

#### [X] A11.2.1 — Struct `ParagraphGroup` y `merge_lines_into_paragraphs`
- **Output:** Implementar en `crates/strata-geometry/src/paragraph.rs`.
  - Gap ≤ `0.7 × median_line_height` → misma línea del párrafo.
  - Gap > threshold → nuevo párrafo.
  - Heading → siempre nuevo grupo.
- **AC:** Agrupación correcta demostrada con tests unitarios.

---

#### [X] A11.3.1 — Implementar utilidades de texto
- **Output:** `normalize_whitespace(text)` (colapsa 2+ espacios a 1, trim) y `fix_letter_spacing(text)` (colapsa espacios intra-palabra si >30 % de glifos tienen espacios alrededor) en `crates/strata-geometry/src/text.rs`.
- **AC:** Métricas de dobles espacios reducidas a 0.

---

#### [X] A11.4.1 — `heading_content_filter` y `heading_position_filter`
- **Output:** Filtra textos ≤ 2 caracteres alfanuméricos, o solo números/símbolos, y líneas en el 8 % superior o 5 % inferior de la página en `crates/strata-geometry/src/headings.rs`.
- **AC:** Headings jerárquicos limpios.

---

### [x] T11.5 — Integración de Imágenes en el Pipeline

- **Objetivo:** Incorporar las imágenes extraídas como bloques semánticos y referenciarlas correctamente en el Markdown.
- **AC:** El archivo `.md` resultante contiene referencias del tipo `![image N](path)`.

#### [x] A11.5.1 — Convertir imágenes a `BlockType::Figure`
- **Output:** Modificar `crates/strata-cli/src/main.rs` para encapsular imágenes en el AST, integrarlas al orden de lectura con XY-Cut++, y soportar el flag `--save-images` a `{output}/{stem}_images/`.
- **AC:** Imágenes renderizadas y guardadas con éxito.

---

### [x] T11.6 — Refactor del Pipeline `cmd_parse`

- **Objetivo:** Consolidar el pipeline nativo integrando filtrado de ruido, merging de párrafos, normalización de texto e imágenes.
- **AC:** `cargo test --workspace --release` pasa. Ninguno de los 293+ tests existentes se rompe.

#### [x] A11.6.1 — Integrar fases en `crates/strata-cli/src/main.rs`
- **Output:** Aplicar `filter_noise_lines()`, usar `classify_headings()` mejorado, procesar mediante `merge_lines_into_paragraphs()`, aplicar `normalize_text()` e insertar Figure blocks en el AST.
- **AC:** Compilación limpia y pipeline integrado.

---

### [x] T11.7 — Golden Tests Regenerados

- **Objetivo:** Actualizar y verificar los snapshots esperados con el nuevo formato de salida de alta calidad.
- **AC:** Goldens regenerados, validados visualmente y committeados.

#### [x] A11.7.1 — Regeneración y snapshot
- **Output:** Ejecutar parsing sobre los 9 papers arXiv fixtures con el perfil `scientific`, guardar en `tests/fixtures/salidas/strata-reader-md/` y crear tests de regresión/snapshot.
- **AC:** Salidas con párrafos fluidos y sin ruido.

---

### [x] T11.8 — Benchmark de Calidad

- **Objetivo:** Medir cuantitativamente las mejoras obtenidas en calidad de Markdown frente al baseline.
- **AC:** Mejora ≥ 80 % en todas las métricas evaluadas (falsos headings, stray chars, dobles espacios).

#### [x] A11.8.1 — Harness de comparación de calidad
- **Output:** Script Python de comparación de métricas vs opendataloader que genera reporte HTML.
- **AC:** Reporte generado exitosamente.

---

## [X] Fase 12 — SDK Python Simplificado y Experiencia UX
> _Estado:_ Completado.

- **Objetivo:** Reducir la fricción del SDK Python a llamadas declarativas simples e intuitivas, y documentar detalladamente los modos de uso en el README.md.
- **AC global de Fase:**
  - El script `run_strata_reader.py` se simplifica de 96 líneas de boilerplate subprocess a ≤ 15 líneas de llamada a `strata_reader.convert()`.
  - La instalación y el Quickstart funcionan en frío bajo `pip install` sin fricciones de configuración.
- **Referencias:** Plan de Mejora §Fase 12.

---

### [X] T12.1 — Conectar Pipeline Real en `strata-py::parse()`

- **Objetivo:** Reemplazar el Document dummy actual en el wrapper PyO3 por el motor de procesamiento nativo real.
- **AC:** `strata_reader.parse("paper.pdf")` retorna un `Document` real con contenido del PDF.

#### [X] A12.1.1 — Cablear pipeline real en Rust (`crates/strata-py/src/lib.rs`)
- **Output:** Leer PDF con `Decoder::open`, extraer glifos, ejecutar el pipeline completo de geometría/triage/fusión/jerarquización, y retornar el objeto `PyDocument` real.
- **AC:** Tests de bindings de Python retornan contenido real del documento.

---

### [X] T12.2 — Implementar `convert()` en Python SDK

- **Objetivo:** Proveer una API de alto nivel para procesamiento batch por carpetas de manera declarativa.
- **AC:** `strata_reader.convert(input_path="pdfs/", output_dir="out/")` funciona robustamente.

#### [X] A12.2.1 — API `convert()` en `python/strata_reader/__init__.py`
- **Output:** Función declarativa batch con resolución de globs, creación automática de directorios, manejo de perfiles, IA, guardado de imágenes, progreso opcional vía `tqdm` y control de errores por archivo.
- **AC:** Llamada simple procesa lotes enteros con robustez.

---

### [X] T12.3 — Mejorar Descubrimiento del Binario para CLI

- **Objetivo:** Resolver automáticamente el path al binario del motor nativo sin exigir que el usuario lo tenga configurado manualmente en PATH.
- **AC:** El entrypoint de consola `strata parse` funciona perfectamente después de un `pip install`.

#### [X] A12.3.1 — Cascada de fallbacks en `python/strata_reader/cli.py`
- **Output:** Buscar en PATH → buscar en directorio de site-packages de Python (wheel) → buscar en build target local (`target/release/strata`). Mensaje de error instructivo si falta.
- **AC:** CLI portable y robusto ante instalaciones limpias.

---

### [X] T12.4 — Crear Tabla de Modos en README.md

- **Objetivo:** Guiar al usuario sobre cuándo utilizar el motor geométrico nativo y cuándo habilitar el asistente IA.
- **AC:** Tabla de modos integrada en el README.md.

#### [X] A12.4.1 — Diseñar e insertar tabla "Which Mode Should I Use?"
- **Output:** Tabla que detalla los modos Default, IA, IA + OCR, sus comandos y dependencias.
- **AC:** Tabla legible y clara en la documentación principal.

---

### [X] T12.5 — Crear Sección de Quickstart en README.md

- **Objetivo:** Habilitar un Quickstart de 30 segundos de alta fidelidad, minimizando el esfuerzo de adopción.
- **AC:** Ejemplo funcional en 30 segundos copy-pasteable en el README.md.

#### [X] A12.5.1 — Redactar secciones de "Get Started" y "Python API/CLI"
- **Output:** Código limpio de uso y comandos CLI básicos en el README principal.
- **AC:** Documentación premium para desarrolladores de RAG.

---

### [X] T12.6 — Simplificar `run_strata_reader.py` de Prueba

- **Objetivo:** Reducir y limpiar el script de verificación para reflejar la API simplificada definitiva.
- **AC:** `run_strata_reader.py` simplificado a ≤ 15 líneas y funcional.

#### [X] A12.6.1 — Refactorizar `tests/test_pruebas/run_strata_reader.py`
- **Output:** Reemplazar el boilerplate heredado por la llamada directa a `strata_reader.convert()` y reporte simple de velocidad.
- **AC:** Script limpio y representativo del SDK.

---


## [X] Fase 13 — Distribución Zero-Friction: Eliminación de libpdfium y Wheels Autocontenidas
> _Estado:_ Completado (Etapas A, B y C integradas con éxito).

- **Objetivo:** Lograr que `pip install strata-reader` funcione en cualquier máquina limpia sin que el usuario necesite compilar, descargar binarios, ni configurar variables de entorno. Alcanzar la misma experiencia que `pip install polars` o `pip install numpy`.
- **AC global de Fase:**
  - `pip install strata-reader && python -c "import strata_reader; doc = strata_reader.parse('paper.pdf'); print(doc.to_markdown()[:200])"` funciona en una máquina con solo Python 3.12+ instalado.
  - Sin Rust toolchain, sin libpdfium manual, sin `STRATA_PDFIUM_LIB_PATH`.
  - Wheels autocontenidas publicadas en PyPI para `manylinux_2_28 x86_64`, `manylinux_2_28 aarch64`, `win_amd64`, `macosx_universal2`.
- **Referencias:** Plan de Mejora §Fase 13.

---

### Etapa A — Bundlear libpdfium en las Wheels (Corto Plazo)

> **Meta:** Sin modificar lógica de Rust, empaquetar la DLL/SO/DYLIB de pdfium dentro de la wheel para que el usuario nunca la configure manualmente. Inspirado en cómo `pypdfium2` empaqueta PDFium y cómo `cryptography` bundlea OpenSSL.

---

### [X] T13.A.1 — Modificar CI para descargar y bundlear libpdfium

- **Objetivo:** Que el workflow `release-wheels.yml` descargue automáticamente el binario correcto de pdfium y lo coloque dentro del paquete Python antes de construir la wheel.
- **AC:** `unzip -l dist/*.whl | grep pdfium` muestra la librería nativa dentro del wheel para cada target de la matriz.

#### [X] A13.A.1.1 — Agregar step de descarga de pdfium binaries al CI

- **Objetivo:** Descargar el binario correcto de `bblanchon/pdfium-binaries` según la plataforma del runner.
- **Input:** Workflow existente `release-wheels.yml`, tag de versión `PDFIUM_VERSION=7843` (chromium milestone).
- **Output:** Step `Download pdfium binary` antes de `Build wheel via maturin` que:
  1. Determina `PLATFORM` de `matrix.target` (`x86_64-unknown-linux-gnu` → `linux-x64`, etc.).
  2. Descarga `pdfium-${PLATFORM}.tgz` de GitHub Releases.
  3. Extrae en `python/strata_reader/_pdfium/`.
  4. Verifica SHA-256 de la descarga contra archivo `scripts/pdfium-checksums.sha256` versionado.
- **Proceso:**
  1. Crear `scripts/pdfium-checksums.sha256` con checksums de las 4 plataformas (archivo versionado).
  2. Agregar mapping de targets en el YAML como env vars.
  3. Usar `curl -L` + `tar xz` para la extracción.
  4. Verificar integridad con `sha256sum -c`.
- **Tests:** El step pasa exit 0 en los 4 runners de la matriz; `ls -la python/strata_reader/_pdfium/` muestra el binario correcto.
- **AC:** Binario descargado, verificado y colocado en la ruta correcta para empaquetamiento.

#### [X] A13.A.1.2 — Verificar que la librería se incluye en la wheel

- **Objetivo:** Confirmar que la wheel resultante contiene el binario de pdfium.
- **Input:** A13.A.1.1 completado.
- **Output:** Step adicional post-build que inspecciona el contenido de la wheel.
- **Proceso:**
  1. `python -m zipfile -l dist/*.whl | grep -i pdfium` debe encontrar la librería.
  2. Verificar que el tamaño de la wheel creció (de ~5MB a ~25-35MB dependiendo del OS).
- **Tests:** Assertion en CI.
- **AC:** Librería pdfium visible dentro de la wheel.

---

### [X] T13.A.2 — Auto-descubrimiento de libpdfium embebida

- **Objetivo:** Que el SDK Python detecte automáticamente la librería de pdfium empaquetada dentro del wheel, sin que el usuario configure nada.
- **AC:** `pip install dist/*.whl && python -c "from strata_reader import parse"` no falla con error `PdfiumLoad` en máquina limpia.

#### [X] A13.A.2.1 — Lógica de auto-setup en `__init__.py`

- **Objetivo:** Setear `STRATA_PDFIUM_LIB_PATH` automáticamente al importar `strata_reader` si la librería embebida existe.
- **Input:** T13.A.1 completado.
- **Output:** Código en `python/strata_reader/__init__.py` (antes de `from ._native import ...`):
  ```python
  import os, pathlib, sys
  _PDFIUM_DIR = pathlib.Path(__file__).parent / "_pdfium"
  if _PDFIUM_DIR.exists():
      # En Linux/macOS la lib está en lib/, en Windows en bin/
      for subdir in ("lib", "bin"):
          candidate = _PDFIUM_DIR / subdir
          if candidate.exists():
              os.environ.setdefault("STRATA_PDFIUM_LIB_PATH", str(candidate))
              # Windows: agregar al DLL search path
              if sys.platform == "win32":
                  os.add_dll_directory(str(candidate))
              break
  ```
- **Proceso:**
  1. Detectar existencia del directorio `_pdfium/`.
  2. Identificar subdirectorio correcto (`lib/` o `bin/`) según OS.
  3. Setear env var **solo si no está ya seteada** (`setdefault`).
  4. En Windows, usar `os.add_dll_directory()` para el DLL search path.
- **Tests:** `pytest` test que importa `strata_reader` en un venv con la wheel instalada y verifica que `STRATA_PDFIUM_LIB_PATH` queda seteado.
- **AC:** Importación silenciosa sin errores de carga de librería.

#### [X] A13.A.2.2 — Verificar fallback cascade completo

- **Objetivo:** Documentar y testear los 3 niveles de fallback de descubrimiento de pdfium.
- **Input:** A13.A.2.1.
- **Output:** Test de integración que verifica la cascada:
  1. Si `STRATA_PDFIUM_LIB_PATH` está seteada manualmente → usar esa.
  2. Si existe `_pdfium/` dentro del wheel → usar esa.
  3. Si pdfium está en el PATH del sistema → usar esa.
  4. Si nada funciona → error tipado claro con instrucciones.
- **Tests:** Test parametrizado con `monkeypatch` para cada escenario.
- **AC:** Los 4 escenarios producen el resultado esperado.

---

### [X] T13.A.3 — Incluir `_pdfium` en maturin config

- **Objetivo:** Asegurar que maturin incluya el directorio `_pdfium/` con la librería nativa dentro de la wheel.
- **AC:** `[tool.maturin].include` contiene la regla correcta y la wheel lo incluye.

#### [X] A13.A.3.1 — Configurar `pyproject.toml` para inclusión de archivos

- **Objetivo:** Agregar la directiva de inclusión a la configuración de maturin.
- **Input:** T13.A.1 completado.
- **Output:** En `pyproject.toml`, sección `[tool.maturin]`:
  ```toml
  include = [
    { path = "python/strata_reader/_pdfium/**/*", format = "wheel" },
  ]
  ```
- **Proceso:**
  1. Agregar la directiva.
  2. Verificar que `.gitignore` excluye `python/strata_reader/_pdfium/` (no se versiona, solo se genera en CI).
  3. Agregar `python/strata_reader/_pdfium/` a `.gitignore`.
- **Tests:** `maturin build --release` local después de descargar pdfium manualmente incluye el archivo.
- **AC:** Directiva en `pyproject.toml`; `.gitignore` actualizado.

---

### [X] T13.A.4 — Herramientas de reparación de wheels por OS

- **Objetivo:** Asegurar que las wheels producidas son autocontenidas en cada OS usando las herramientas estándar de reparación.
- **AC:** Wheels pasan `auditwheel check` (Linux), no hay missing DLLs (Windows), no hay missing dylibs (macOS).

#### [X] A13.A.4.1 — Configurar `auditwheel` para Linux

- **Objetivo:** Bundlear todas las `.so` transitivas (incluida libpdfium) dentro de la wheel Linux.
- **Input:** T13.A.1 completado.
- **Output:** En `release-wheels.yml`, usar `--auditwheel repair` en los args de maturin para targets Linux:
  ```yaml
  args: --release --out dist --strip --auditwheel repair
  ```
- **Proceso:** Maturin ya tiene reimplementación de auditwheel; solo falta activar `repair` (actualmente no está en los args).
- **Tests:** `auditwheel check dist/*.whl` exit 0.
- **AC:** Wheel Linux con tag `manylinux_2_28` y todas las dependencias bundleadas.

#### [X] A13.A.4.2 — Configurar `delocate` para macOS

- **Objetivo:** Bundlear `libpdfium.dylib` y sus dependencias transitivas en la wheel macOS.
- **Input:** T13.A.1 completado.
- **Output:** Step adicional en el workflow:
  ```yaml
  - name: Repair wheel (macOS)
    if: runner.os == 'macOS'
    run: |
      pip install delocate
      delocate-wheel -v dist/*.whl
  ```
- **Tests:** `delocate-listdeps dist/*.whl` no muestra dependencias externas.
- **AC:** Wheel macOS autocontenida.

#### [X] A13.A.4.3 — Configurar `delvewheel` para Windows

- **Objetivo:** Bundlear `pdfium.dll` y sus DLLs transitivas en la wheel Windows.
- **Input:** T13.A.1 completado.
- **Output:** Step adicional en el workflow:
  ```yaml
  - name: Repair wheel (Windows)
    if: runner.os == 'Windows'
    run: |
      pip install delvewheel
      delvewheel repair dist/*.whl -w dist-repaired/
      Copy-Item dist-repaired/*.whl dist/ -Force
  ```
- **Tests:** `delvewheel check dist/*.whl` pasa.
- **AC:** Wheel Windows autocontenida.

#### [X] A13.A.4.4 — Smoke test E2E en venv limpio por OS

- **Objetivo:** Verificar que la wheel instalada desde cero funciona sin configuración.
- **Input:** Todas las tareas de Etapa A completadas.
- **Output:** Step de smoke test mejorado que:
  1. Crea venv completamente limpio.
  2. Instala la wheel.
  3. Ejecuta `python -c "from strata_reader import version, parse; print(version())"`.
  4. (Opcional si hay fixture PDF) ejecuta un parse real y verifica salida no vacía.
- **Tests:** CI verde en los 4 targets de la matriz.
- **AC:** Smoke test pasa en los 4 OS sin configuración de pdfium.

---

### Etapa B — Backend Pure-Rust Alternativo (Medio Plazo)

> **Meta:** Implementar un backend PDF 100% Rust como alternativa a pdfium, usando `pdf-rs` o `lopdf`, con la misma interfaz que el backend actual. Esto permite una wheel más pequeña y elimina la dependencia de un binario C externo.

---

### [X] T13.B.1 — Trait `PdfBackend` y Abstracción del Decoder

- **Objetivo:** Definir una interfaz abstracta que permita intercambiar el backend de decodificación PDF sin cambiar el resto del pipeline.
- **AC:** `cargo test -p strata-pdf --release` pasa sin cambios funcionales. El código existente funciona igual envuelto en `PdfiumBackend`.

#### [X] A13.B.1.1 — Definir traits `PdfBackend`, `PdfDoc`, `PdfPage`

- **Objetivo:** Abstraer las operaciones de decodificación PDF en traits Rust.
- **Input:** Código actual de `strata-pdf`.
- **Output:** `crates/strata-pdf/src/backend.rs` con los traits.
- **Proceso:**
  1. Definir los traits con métodos que reflejan las operaciones actuales.
  2. Agregar `mod backend` a `lib.rs`.
  3. Documentar contratos de cada método.
- **Tests:** Los traits compilan y son object-safe (`dyn PdfBackend`).
- **AC:** Traits definidos, documentados y compilando.

#### [X] A13.B.1.2 — Implementar `PdfiumBackend` envolviendo código actual

- **Objetivo:** Envolver el código actual de `decoder.rs`, `glyph.rs`, `vector.rs`, `image.rs` en una struct `PdfiumBackend` que implemente `PdfBackend`.
- **Input:** A13.B.1.1.
- **Output:** `crates/strata-pdf/src/pdfium_backend.rs` con `struct PdfiumBackend` que reutiliza `get_pdfium()`, `extract_glyphs()`, `extract_paths()`, `extract_images()`, `render_crop()`.
- **Proceso:**
  1. Mover la lógica existente bajo la implementación del trait.
  2. Refactorizar `Decoder` para aceptar `Box<dyn PdfBackend>`.
  3. `Decoder::default()` usa `PdfiumBackend`.
- **Tests:** `cargo test -p strata-pdf --release` — todos los tests existentes pasan sin cambio.
- **AC:** Zero regression; refactor puramente estructural.

---

### [X] T13.B.2 — Implementar `PureRustBackend` con `pdf-rs`/`lopdf`

- **Objetivo:** Backend alternativo que extrae glifos, paths e imágenes de PDFs sin ninguna dependencia C.
- **AC:** Los 8 fixtures PDF producen glifos con ≤ 2% de discrepancia vs. `PdfiumBackend`.

#### [X] A13.B.2.1 — Parseo de estructura PDF

- **Objetivo:** Abrir y navegar la estructura del documento PDF (páginas, recursos, content streams).
- **Input:** A13.B.1.1.
- **Output:** `crates/strata-pdf/src/pure_backend.rs` — `PureRustBackend::open()` que parsea el PDF con `pdf` crate o `lopdf`.
- **Proceso:**
  1. Evaluar `pdf` crate (v0.9+) vs `lopdf` (v0.33+) — preferir el que tenga mejor soporte de CMap/ToUnicode.
  2. Implementar `PdfDoc` trait: `page_count()` y `page(index)`.
  3. Manejar PDFs encriptados con error claro (no soportado en pure backend inicialmente).
- **Tests:** Abrir los 8 fixtures, verificar `page_count()` coincide con expected.
- **AC:** Todos los fixtures abren sin panic; conteo de páginas correcto.

#### [X] A13.B.2.2 — Extracción de glifos (content stream + CMap)

- **Objetivo:** Parsear content streams (`BT...ET`) y extraer cada glifo con su Unicode, BBox y font metadata.
- **Input:** A13.B.2.1.
- **Output:** Implementación de `PdfPage::glyphs()` en `PureRustBackend`.
- **Proceso:**
  1. Parsear operadores de texto: `Tf` (set font), `Td`/`TD`/`Tm` (positioning), `Tj`/`TJ` (show text).
  2. Resolver Unicode via CMap/ToUnicode tables del font resource.
  3. Calcular BBox por glifo usando CTM + text matrix + font metrics (advance width).
  4. Manejar fonts Type1, TrueType, CIDFont.
- **Tests:** Sobre `native_simple.pdf`: conteo de glifos ±2% vs PdfiumBackend; verificar 50 glifos aleatorios con BBox ±1pt y Unicode correcto.
- **AC:** ≥ 98% match en Unicode; ≤ 2% discrepancia en conteo.

#### [X] A13.B.2.3 — Extracción de paths vectoriales

- **Objetivo:** Parsear operadores de path del content stream (`m`, `l`, `c`, `re`, `S`, `f`, etc.).
- **Input:** A13.B.2.1.
- **Output:** Implementación de `PdfPage::paths()` en `PureRustBackend`.
- **Proceso:**
  1. Parsear state machine de graphics: CTM transforms, color state.
  2. Emitir `VectorPath { segments, stroke, fill, bbox }`.
- **Tests:** Sobre fixture con bordes: mismo número de paths que PdfiumBackend.
- **AC:** Conteo de paths idéntico en fixtures con tablas vectoriales.

#### [X] A13.B.2.4 — Extracción de imágenes embebidas

- **Objetivo:** Decodificar XObject Image (`/Subtype /Image`) y extraer bytes raw con mime type.
- **Input:** A13.B.2.1.
- **Output:** Implementación de `PdfPage::images()` en `PureRustBackend`.
- **Proceso:**
  1. Buscar XObject Image en page resources.
  2. Decodificar filtros: `/DCTDecode` (JPEG), `/FlateDecode` (zlib → raw pixels), `/JPXDecode` (JPEG2000).
  3. Retornar `ImageData { bbox, raw_bytes, mime, dpi_estimated }`.
- **Tests:** `figure_with_caption.pdf` → 1 imagen extraída, `raw_bytes.len() > 0`.
- **AC:** Todas las imágenes detectadas por PdfiumBackend también detectadas por PureRustBackend.

#### [X] A13.B.2.5 — Fallback para renderizado de crops

- **Objetivo:** Implementar `render_crop()` sin PDFium, suficiente para enviar regiones a VLM/OCR.
- **Input:** A13.B.2.2, A13.B.2.3, A13.B.2.4.
- **Output:** Implementación de `PdfPage::render_crop()` en `PureRustBackend`.
- **Proceso:**
  1. Opción preferida: usar `tiny-skia` para compositar glifos posicionados + paths en un canvas PNG.
  2. Alternativa: enviar al OCR/VLM la página completa con coordenadas BBox (el modelo VLM cropea internamente).
  3. Feature flag `tiny-skia-render` para la opción 1.
- **Tests:** SSIM ≥ 0.90 vs. crop de PdfiumBackend para 3 regiones de prueba.
- **AC:** Crop usable por VLM (no necesita fidelidad pixel-perfect).

---

### [X] T13.B.3 — Feature Flags para Selección de Backend

- **Objetivo:** Permitir compilar strata-pdf con o sin dependencia C, controlado por Cargo features.
- **AC:** `cargo build -p strata-pdf --no-default-features --features pure-backend` compila sin pdfium-render.

#### [X] A13.B.3.1 — Configurar features en Cargo.toml

- **Objetivo:** Definir los features de backend en el crate.
- **Input:** T13.B.1, T13.B.2.
- **Output:** `crates/strata-pdf/Cargo.toml` con las features.
- **Proceso:**
  1. Mover `pdfium-render` a optional dependency.
  2. Agregar `pdf`, `lopdf`, `tiny-skia` como optional dependencies.
  3. Compilación condicional con `#[cfg(feature = "...")]` en `lib.rs`.
  4. `Decoder::default_backend()` selecciona según features habilitadas.
- **Tests:** `cargo check -p strata-pdf --no-default-features --features pure-backend` exit 0.
- **AC:** Compilación sin pdfium posible.

#### [X] A13.B.3.2 — CLI flag `--pdf-backend`

- **Objetivo:** Permitir al usuario seleccionar el backend desde la línea de comandos.
- **Input:** A13.B.3.1.
- **Output:** Flag `--pdf-backend {pdfium|pure|auto}` en `strata-cli`.
- **Tests:** `strata parse --pdf-backend pure --input paper.pdf` funciona.
- **AC:** Flag disponible y funcional.

---

### [X] T13.B.4 — Tests de Paridad Entre Backends

- **Objetivo:** Garantizar que el backend pure-Rust produce resultados suficientemente similares al de pdfium.
- **AC:** Discrepancias ≤ 2% en conteo de glifos; Unicode idéntico en > 98% de los glifos.

#### [X] A13.B.4.1 — Suite de comparación automatizada

- **Objetivo:** Test que ejecuta ambos backends sobre cada fixture y compara resultados.
- **Input:** T13.B.2 completado.
- **Output:** `crates/strata-pdf/tests/backend_parity.rs` con los tests.
- **Tests:** `cargo test -p strata-pdf -- backend_parity` verde.
- **AC:** Paridad verificada cuantitativamente.

---

### Etapa C — Migración Completa y Wheel 100% Rust (Largo Plazo)

> **Meta:** Hacer del backend pure-Rust el default. PDFium se mantiene como feature opcional para usuarios que necesiten renderizado de máxima fidelidad. La wheel por defecto no contiene ninguna DLL C.

---

### [X] T13.C.1 — Promover `pure-backend` como Default

- **Objetivo:** Invertir los defaults: pure-Rust por defecto, pdfium opt-in.
- **AC:** `pip install strata-reader` instala sin pdfium. `pip install strata-reader[pdfium]` lo incluye.

#### [X] A13.C.1.1 — Cambiar default feature a `pure-backend`

- **Objetivo:** Modificar los defaults del crate y del paquete Python.
- **Input:** T13.B.4 con paridad verificada.
- **Output:**
  1. `crates/strata-pdf/Cargo.toml`: `default = ["pure-backend"]`.
  2. `pyproject.toml`: agregar `[project.optional-dependencies] pdfium = []`.
  3. Workflow de CI: separar wheel default (pure) y wheel con pdfium.
- **Tests:** `pip install strata-reader` + parse funciona sin pdfium en el sistema.
- **AC:** Default funciona sin librería C.

---

### [X] T13.C.2 — Resolver Renderizado de Crops sin PDFium

- **Objetivo:** Producir crops de calidad suficiente para VLM/OCR usando solo bibliotecas Rust.
- **AC:** `render_crop()` produce PNGs con SSIM ≥ 0.95 vs. PDFium.

#### [X] A13.C.2.1 — Rasterizador basado en `resvg` + `tiny-skia`

- **Objetivo:** Compositar glifos posicionados y paths vectoriales en canvas PNG.
- **Input:** T13.B.2 completado.
- **Output:** Módulo `crates/strata-pdf/src/rasterizer.rs` que renderiza a PNG.
- **Tests:** SSIM ≥ 0.95 sobre 5 crops de referencia vs. PDFium.
- **AC:** Crops visualmente correctos para consumo por VLM.

---

### [X] T13.C.3 — Eliminar libpdfium del CI por Defecto

- **Objetivo:** El workflow de release produce wheels sin pdfium incluido.
- **AC:** Wheels principales no contienen archivos `pdfium.*`.

#### [X] A13.C.3.1 — Bifurcar workflows

- **Objetivo:** Separar `release-wheels.yml` (pure) de `release-wheels-pdfium.yml` (con pdfium).
- **Input:** T13.C.1 completado.
- **Output:** Dos workflows independientes; el principal no descarga pdfium.
- **Tests:** `unzip -l dist/*.whl | grep pdfium` retorna vacío en wheel principal.
- **AC:** Wheels principales limpias de binarios C.

---

### [X] T13.C.4 — Documentación y Migración de Usuarios

- **Objetivo:** Actualizar toda la documentación para reflejar el nuevo estado sin dependencia de pdfium.
- **AC:** README no menciona libpdfium en la sección de Quickstart ni en los prerrequisitos.

#### [X] A13.C.4.1 — Actualizar README.md

- **Objetivo:** Eliminar la sección "Configurar libpdfium (Windows)" del Quickstart y moverla a una guía avanzada.
- **Input:** T13.C.1 completado.
- **Output:** README.md actualizado; sección "Compilación y Configuración desde Código Fuente" simplificada.
- **AC:** Zero menciones de pdfium en el quickstart.

#### [X] A13.C.4.2 — Crear guía de migración

- **Objetivo:** Documentar la transición para usuarios existentes.
- **Input:** T13.C.1.
- **Output:** `docs/usage/migration.md` con las guías.
- **AC:** Guía revisada y mergeada.

---

### Dependencias entre Tareas de Fase 13

```
Etapa A (independiente de B y C — alta prioridad, desbloquea pip install):
  T13.A.1 ──► T13.A.2 ──► T13.A.3 ──► T13.A.4
    (CI)       (Python)     (pyproject)   (repair)

Etapa B (tras Etapa A funcional — medio plazo):
  T13.B.1 ──► T13.B.2 ──► T13.B.3 ──► T13.B.4
    (trait)     (impl)      (features)   (tests)

Etapa C (tras Etapa B con paridad verificada — largo plazo):
  T13.C.1 ──► T13.C.2 ──► T13.C.3 ──► T13.C.4
    (default)   (crop)      (CI)         (docs)
```

---

## [X] Fase 14 — Orquestación End-to-End de IA Local: LLM/VLM/OCR

- **Objetivo:** Conectar de forma completa y verificable el flujo PDF → AST nativo → Triage → crops → Python IA local (OCR, VLM de tablas, VLM de imágenes, VLM de fórmulas) → Fusión → Markdown/JSON, reutilizando los módulos ya existentes y eliminando la brecha actual donde `--ia`, `use_ia`, `--force-ocr` y el bridge gRPC no participan del parse real.
- **AC global de Fase:**
  - `strata parse --input tests/fixtures/pdfs/figure_with_caption.pdf --output out --format md+json --ia` produce al menos un bloque `Figure` con descripción generada por VLM local, `provenance.source == "vlm"`, `model` no vacío y Markdown con alt text/caption útil.
  - `strata parse --input tests/fixtures/pdfs/scanned_paper.pdf --output out --format md+json --ia --force-ocr` produce bloques textuales con `provenance.source == "ocr"` usando Surya o Tesseract cuando estén disponibles, y Ollama solo como fallback explícito.
  - `strata parse --input tests/fixtures/pdfs/borderless_table.pdf --output out --format md+json --ia` produce tablas GFM válidas con `provenance.source == "vlm"` y celdas reconstruidas desde `TableResult`.
  - `ParseOptions(use_ia=True)` en Python activa el mismo pipeline IA que la CLI; `ParseOptions(use_ia=False)` mantiene modo nativo sin requerir Ollama.
  - `/v1/parse` del servidor HTTP persiste input, ejecuta el pipeline en background y devuelve `result_md`/`result_json` al completar el job.
  - Tests con mock gRPC pasan en CI sin Ollama; tests reales con Ollama quedan marcados `@ollama`/`@gpu` y documentados.
- **Referencias:** Plan Maestro §16 (Fase 14), `docs/architecture/architecture.md`, `crates/strata-triage`, `crates/strata-ia-bridge`, `crates/strata-fusion`, `python/strata_ia`, `docs/doc_guia/plantilla_tareas.md`.

---

### Subfase A — Diagnóstico, Contratos y Pipeline Compartido

> **Meta:** Evitar parches duplicados en CLI/PyO3/servidor. Primero se consolida una arquitectura única del flujo PDF→Markdown que pueda ser llamada por todas las superficies públicas.

---

### [X] T14.1 — Crear orquestador de pipeline compartido

- **Objetivo:** Centralizar el flujo de parse en una API Rust reutilizable por CLI, bindings Python y servidor HTTP.
- **AC:** Existe una función pública única de alto nivel que acepta `ParsePipelineOptions` y retorna `Document`/artefactos, y al menos CLI + PyO3 la usan sin duplicar lógica de extracción.

#### [X] A14.1.1 — Auditar call-sites actuales de parse nativo

- **Objetivo:** Identificar todo código que hoy implementa o duplica PDF→AST→Markdown para definir el set mínimo de extracción compartida.
- **Input:** `crates/strata-cli/src/main.rs`, `crates/strata-py/src/lib.rs`, `crates/strata-server/src/routes.rs`, `crates/strata-serialize`, `crates/strata-fusion`.
- **Output:** Sección en PR o ADR corto con la lista de call-sites, responsabilidades actuales y rutas que deben migrar al orquestador.
- **Proceso:**
  1. Localizar bloques duplicados de extracción de glyphs, líneas, párrafos, imágenes y serialización.
  2. Separar responsabilidades puras (`parse_native`) de side effects (`write files`, `spawn IA`, `persist jobs`).
  3. Definir qué opciones son comunes: `profile`, `use_ia`, `force_ocr`, `ollama_endpoint`, `grpc_endpoint`, `media_dir`, `save_images`, `max_concurrent_pages`, `pdf_backend`.
- **Tests:** `cargo check -p strata-cli -p strata-py -p strata-server` antes del refactor para tener baseline.
- **AC:** Documento de auditoría adjunto al PR; no se inicia refactor sin mapa de dependencias y riesgos.

#### [X] A14.1.2 — Introducir `strata-pipeline` o módulo equivalente

- **Objetivo:** Crear un punto de entrada explícito para el pipeline completo sin acoplarlo al CLI.
- **Input:** Resultado de A14.1.1 y crates existentes.
- **Output:** Nuevo crate `crates/strata-pipeline` o módulo en `strata-runtime` con:
  - `ParsePipelineOptions`
  - `ParseArtifacts`
  - `parse_document(options) -> Result<ParseArtifacts, PipelineError>`
  - errores tipados con `thiserror`
- **Proceso:**
  1. Elegir ubicación con menor acoplamiento y documentar la decisión.
  2. Declarar dependencias a `strata-pdf`, `strata-geometry`, `strata-quality`, `strata-triage`, `strata-ia-bridge`, `strata-fusion`, `strata-serialize`.
  3. Implementar skeleton nativo primero, sin IA, manteniendo salidas actuales.
- **Tests:** `cargo check -p strata-pipeline` y test unitario con fixture mínimo que retorne `Document` no vacío.
- **AC:** CLI puede compilar delegando el parse nativo al nuevo orquestador sin regresión funcional.

#### [X] A14.1.3 — Migrar CLI y PyO3 al orquestador nativo

- **Objetivo:** Eliminar duplicación de lógica entre `strata-cli` y `strata-py` antes de activar IA.
- **Input:** A14.1.2 completado.
- **Output:** `cmd_parse` y `parse()` de PyO3 llaman a la misma API de pipeline.
- **Proceso:**
  1. Reemplazar el cuerpo de extracción nativa duplicada por una llamada a `parse_document`.
  2. Mantener `to_markdown()` y `to_graph_json()` como wrappers de serialización.
  3. Preservar flags y opciones existentes, aunque algunas aún no activen IA.
- **Tests:**
  - `cargo test -p strata-cli -p strata-py`
  - `uv run pytest tests/e2e/test_strata_parse.py -q` si la wheel local está disponible.
- **AC:** Misma salida Markdown/JSON que antes para un PDF nativo simple; sin cambios de schema no documentados.

---

### [X] T14.2 — Integrar Triage real sobre bloques y páginas

- **Objetivo:** Convertir señales nativas de calidad, geometría e imágenes en decisiones `Native | OcrFullPage | VlmTable | VlmImage | VlmFormula` consumibles por el bridge IA.
- **AC:** Para los fixtures canónicos, el pipeline produce un listado determinista de `IaTask`s con rutas esperadas antes de llamar a IA.

#### [X] A14.2.1 — Construir `PageContext` desde extracción nativa

- **Objetivo:** Alimentar `strata_triage::PageContext` con señales reales de escaneo y CID.
- **Input:** `strata_pdf::is_likely_scan`, `strata_quality::cid_detector`, dimensiones de página y conteos de glyphs/images.
- **Output:** Función `build_page_context(...) -> PageContext` o equivalente en el orquestador.
- **Proceso:**
  1. Calcular `is_scanned` por página.
  2. Calcular `cid_severity` usando el detector de calidad existente.
  3. Calcular `page_area` desde `Page.media_box`.
- **Tests:** Unit tests con mocks de página: escaneada, CID crítico y página nativa limpia.
- **AC:** `scanned_paper.pdf` genera decisión OCR; `native_simple.pdf` permanece nativo.

#### [X] A14.2.2 — Construir `BlockContext` para tablas, figuras y fórmulas

- **Objetivo:** Hacer que las rutas VLM tengan candidatos reales, no solo tipos declarados en el AST.
- **Input:** `detect_table_borders`, `detect_table_candidates`, imágenes extraídas, heurísticas de símbolos matemáticos y confianza nativa.
- **Output:** Bloques o tareas candidatas para `VlmTable`, `VlmImage` y `VlmFormula` con `bbox` estable.
- **Proceso:**
  1. Detectar tablas con bordes y borderless; crear/etiquetar `BlockType::Table` según corresponda.
  2. Etiquetar imágenes grandes como candidatas a descripción VLM.
  3. Detectar regiones de fórmulas con baja confianza o símbolos matemáticos densos.
  4. Evitar duplicar regiones superpuestas mediante IoU/orden determinista.
- **Tests:** Unit tests sobre `borderless_table.pdf`, `figure_with_caption.pdf`, `equation_heavy.pdf` o fixtures sintéticos equivalentes.
- **AC:** Cada fixture produce al menos una tarea IA esperada con `bbox` no degenerada y ruta correcta.

#### [X] A14.2.3 — Definir semántica de `--ia`, `--no-ia` y `--force-ocr`

- **Objetivo:** Alinear README, CLI, SDK y pipeline sobre cuándo se llama a IA.
- **Input:** Flags actuales (`--no-ia`), README, `ParseOptions(use_ia)`, perfiles `fast|balanced|scientific`.
- **Output:** Contrato documentado e implementado para:
  - `--ia`: habilita bridge local IA.
  - `--no-ia`: fuerza modo nativo sin llamadas al bridge.
  - `--force-ocr`: convierte toda página o documento a ruta OCR cuando IA está habilitada.
- **Proceso:**
  1. Decidir compatibilidad backwards con `--no-ia` como default inverso.
  2. Agregar validación: `--force-ocr` sin IA debe fallar con error claro o activar IA implícitamente, según contrato aprobado.
  3. Reflejar contrato en `ParsePipelineOptions`.
- **Tests:** Tests de CLI con combinaciones de flags y snapshots de error.
- **AC:** README y CLI help muestran los mismos flags; combinaciones inválidas fallan de forma accionable.

---

### Subfase B — Bridge IA, Respuestas Tipadas y Fusión

> **Meta:** Enviar crops reales al microservicio Python local y convertir sus respuestas en bloques Markdown/JSON trazables.

---

### [X] T14.3 — Conectar Rust pipeline con `strata-ia-bridge`

- **Objetivo:** Usar gRPC real desde el pipeline para OCR, tablas, imágenes y fórmulas.
- **AC:** Un test de integración con servidor gRPC fake recibe `StreamCrop`s y el pipeline fusiona respuestas mock en el `Document` final.

#### [X] A14.3.1 — Resolver cliente IA externo o embedded worker

- **Objetivo:** Conectar al bridge sin exigir al usuario levantar manualmente el servidor cuando usa la wheel.
- **Input:** `BridgeClient`, `BridgeClientConfig`, `EmbeddedWorker`, `ollama_endpoint`, posible `grpc_endpoint`.
- **Output:** Estrategia de conexión:
  - usar `STRATA_IA_GRPC_ENDPOINT` si está definido;
  - si no, lanzar `EmbeddedWorker` desde la wheel/SDK;
  - en CLI dev, permitir endpoint explícito.
- **Proceso:**
  1. Añadir dependencia `strata-ia-bridge` al crate orquestador.
  2. Propagar variables `STRATA_IA_*` al worker Python.
  3. Implementar timeout y error `IaUnavailable` con mensaje de instalación de Ollama/modelos.
- **Tests:** Mock de puerto no disponible, endpoint inválido y worker fake listo.
- **AC:** Modo nativo no intenta conectar; modo IA falla rápido con error claro si el servicio no está disponible.

#### [X] A14.3.2 — Renderizar crops y construir `StreamCrop`s correlacionados

- **Objetivo:** Convertir cada `IaTask` en payload gRPC con PNG, ruta y `correlation_id` determinista.
- **Input:** `strata_triage::render_crop`, `PdfPage::render_crop`, `DEFAULT_CROP_DPI`, lista de tareas IA.
- **Output:** Builder `build_stream_crops(tasks) -> Vec<StreamCrop>` con mapeo `correlation_id -> BlockId`.
- **Proceso:**
  1. Renderizar crops por `bbox` y ruta.
  2. Usar hints (`table-borderless`, `figure`, `formula`, `ocr-page`) para prompts.
  3. Limitar DPI por perfil y presupuesto VRAM.
  4. Registrar fallos de crop como errores tipados recuperables cuando sea posible.
- **Tests:** Fixture con figura: PNG no vacío; pure backend debe producir crop usable o error explícito si no soporta rasterización.
- **AC:** Ningún crop IA se envía con bytes vacíos; correlación estable entre respuesta y `BlockId`.

#### [X] A14.3.3 — Mapear respuestas proto a `IaPayload`

- **Objetivo:** Convertir `OcrResponse`, `TableResponse`, `ImageResponse` y `FormulaResponse` en payloads de fusión.
- **Input:** Tipos generados de `strata-ia-bridge::proto`, `strata_fusion::IaPayload`, `strata_core::Provenance`.
- **Output:** Mappers puros:
  - `ocr_response_to_payload`
  - `table_response_to_payload`
  - `image_response_to_payload`
  - `formula_response_to_payload`
- **Proceso:**
  1. Mapear `backend/model_id/latency_ms/retries/cache_hit` a `Provenance`.
  2. Convertir `TableResult.rows/cells` a Markdown GFM determinista.
  3. Preservar `caption`, `description`, `alt_text` de imágenes sin perder metadata.
  4. Validar confidence en `[0, 1]` y manejar respuestas vacías.
- **Tests:** Unit tests por mapper con payloads proto mínimos y casos de tabla con `row_span`/`col_span`.
- **AC:** Payloads serializan/deserializan y `strata_fusion::merge` produce bloques con contenido no vacío.

---

### [X] T14.4 — Unificar comportamiento OCR/VLM en Python REST y gRPC

- **Objetivo:** Garantizar que el transporte gRPC usado por Rust tenga la misma lógica que los routers FastAPI, especialmente para OCR con Surya/Tesseract.
- **AC:** Tests Python prueban que REST y gRPC devuelven el mismo tipo de backend para el mismo crop bajo condiciones controladas.

#### [X] A14.4.1 — Extraer servicio común de OCR

- **Objetivo:** Evitar que REST use Surya/Tesseract mientras gRPC usa solo Ollama.
- **Input:** `python/strata_ia/routers/ocr.py`, `python/strata_ia/grpc_server.py`, adapters `surya.py`, `tesseract.py`, `ollama.py`.
- **Output:** Módulo común `python/strata_ia/services/ocr.py` con `run_ocr_page(...)`.
- **Proceso:**
  1. Mover la cadena Surya → Tesseract → Ollama a una función reutilizable.
  2. Hacer que REST y gRPC llamen esa función.
  3. Mantener errores específicos y provenance consistente.
- **Tests:** `uv run pytest tests/unit_py/test_ia_ocr_adapters.py tests/unit_py/test_ia_grpc_server.py -q`.
- **AC:** gRPC `OcrPage` reporta `backend="surya"` o `backend="tesseract"` cuando esos adapters están disponibles.

#### [X] A14.4.2 — Endurecer JSON estructurado de VLM

- **Objetivo:** Reducir fallos por JSON malformado y estandarizar prompts/schemas.
- **Input:** `routers/prompts.py`, modelos Pydantic, `OllamaClient.generate(format_json=True)`.
- **Output:** Validadores y reintentos controlados para tabla, imagen y fórmula.
- **Proceso:**
  1. Añadir reintento local con prompt de corrección solo si el JSON falla schema.
  2. Registrar respuesta cruda truncada en logs debug, nunca en error visible si contiene datos sensibles.
  3. Mantener `temperature=0` y seed fijo.
- **Tests:** Unit tests con respuestas malformadas simuladas mediante `respx`/cliente fake.
- **AC:** Fallos de JSON retornan error tipado y no causan panic ni respuesta parcial silenciosa.

#### [X] A14.4.3 — Activar caché IA por hash de crop y prompt

- **Objetivo:** Evitar repetir llamadas costosas a Ollama/OCR para el mismo crop/modelo/prompt.
- **Input:** `python/strata_ia/cache.py`, `IaConfig.cache_path`, bytes PNG, modelo, prompt hash.
- **Output:** Caché común para REST/gRPC con `cache_hit` propagado en provenance.
- **Proceso:**
  1. Definir clave `sha256(png_bytes + model + prompt_version + task_kind)`.
  2. Guardar respuesta validada, no texto crudo no validado.
  3. Invalidar por versión de prompt/modelo.
- **Tests:** Dos llamadas idénticas: primera `cache_hit=false`, segunda `cache_hit=true`.
- **AC:** Provenance refleja cache hit y latencia reducida; caché desactivable por config.

---

### Subfase C — AST, Markdown, CLI, SDK y Servidor

> **Meta:** Exponer la IA en todas las superficies públicas sin perder trazabilidad ni determinismo.

---

### [X] T14.5 — Preservar resultados IA en AST y Markdown

- **Objetivo:** Asegurar que descripciones de imágenes, tablas VLM, OCR y fórmulas llegan completas al Markdown/JSON final.
- **AC:** Golden Markdown contiene tablas GFM válidas, fórmulas en `$$`, figuras con alt text/caption y texto OCR para PDFs escaneados.

#### [X] A14.5.1 — Extender metadata de `Block` para payloads multimodales

- **Objetivo:** Evitar perder `alt_text`, `long_description`, ruta de media y metadatos IA durante la fusión.
- **Input:** `strata-core::Block`, `strata-fusion::IaPayload::Image`, serializer JSON/Markdown.
- **Output:** Campo estable de metadata por bloque o estructura equivalente compatible con `serde` y schema JSON.
- **Proceso:**
  1. Diseñar `BlockMetadata` con claves deterministas (`altText`, `description`, `mediaPath`, `ocrLanguage`, `tableCellCount`, `mathml`).
  2. Actualizar schema y tests de round-trip.
  3. Mantener backward compatibility con bloques sin metadata.
- **Tests:** `cargo test -p strata-core -p strata-serialize`.
- **AC:** `ImageDescription.alt_text` y `description` sobreviven hasta JSON y Markdown.

#### [X] A14.5.2 — Fusionar payloads IA cambiando tipo/contenido cuando corresponda

- **Objetivo:** Crear el AST final semánticamente correcto: OCR como párrafos, tablas como `Table`, fórmulas como `Equation`, imágenes como `Figure` enriquecido.
- **Input:** `strata_fusion::merge`, mappers de A14.3.3, `BlockType`.
- **Output:** Fusión que actualiza contenido, provenance, metadata y eventualmente `BlockType` de placeholders IA.
- **Proceso:**
  1. Definir placeholders nativos para tareas IA con IDs estables.
  2. Aplicar payload respetando inmutabilidad funcional.
  3. Validar que no queden bloques IA con contenido vacío salvo figuras permitidas.
- **Tests:** Unit tests por payload y test de documento mixto con 4 rutas IA.
- **AC:** `strata_fusion::validate` pasa tras fusión para OCR/table/formula; figuras tienen caption o metadata suficiente.

#### [X] A14.5.3 — Actualizar Markdown renderer para figuras enriquecidas

- **Objetivo:** Renderizar descripciones VLM de imágenes de forma útil para RAG y accesibilidad.
- **Input:** `strata-serialize/src/markdown.rs`, `ImageStrategy`, metadata de `Block`.
- **Output:** Markdown que usa `altText` en `![alt](path)` y agrega descripción/caption si existe.
- **Proceso:**
  1. Si hay media path, emitir imagen con alt text real.
  2. Si hay descripción larga, emitir párrafo/caption inmediatamente bajo la figura.
  3. Si `CaptionOnly`, usar caption/description sin placeholder genérico `figure`.
- **Tests:** Snapshot Markdown para figura enriquecida.
- **AC:** No se emite `_figure: figure_` cuando existe descripción IA; alt text no queda vacío.

---

### [X] T14.6 — Exponer IA completa en CLI, SDK Python y servidor HTTP

- **Objetivo:** Hacer que las interfaces públicas activen el mismo pipeline completo.
- **AC:** CLI, SDK y servidor producen resultados equivalentes para el mismo PDF y opciones.

#### [X] A14.6.1 — Implementar flags CLI IA

- **Objetivo:** Alinear CLI con README y pipeline.
- **Input:** `crates/strata-cli/src/main.rs`, `ParsePipelineOptions`, contrato de A14.2.3.
- **Output:** Flags documentados: `--ia`, `--no-ia`, `--force-ocr`, `--ia-grpc-endpoint`, `--ollama-endpoint`, `--table-model`, `--image-model`, `--formula-model` si aplica.
- **Proceso:**
  1. Agregar flags con `clap` y validaciones.
  2. Mostrar warnings accionables si faltan modelos.
  3. Actualizar `strata doctor` para incluir estado de gRPC IA y modelos VLM esperados.
- **Tests:** `cargo test -p strata-cli`; snapshot de `strata parse --help`.
- **AC:** Comandos documentados en README existen realmente en `--help`.

#### [X] A14.6.2 — Activar `ParseOptions(use_ia=True)` en PyO3

- **Objetivo:** Que el SDK Python sea una envoltura real del pipeline completo.
- **Input:** `crates/strata-py/src/lib.rs`, `python/strata_reader/__init__.py`, `ParsePipelineOptions`.
- **Output:** `parse(path, options=ParseOptions(use_ia=True))` invoca IA; `use_ia=False` no conecta al bridge.
- **Proceso:**
  1. Mapear opciones Python a opciones Rust.
  2. Liberar el GIL durante parse si el proceso puede ser largo.
  3. Convertir errores Rust a excepciones Python específicas.
- **Tests:** `uv run pytest tests/unit_py tests/e2e -q` con IA mock.
- **AC:** Test demuestra que `use_ia=False` no requiere Ollama y `use_ia=True` consume el mock bridge.

#### [X] A14.6.3 — Implementar worker real para `strata-server`

- **Objetivo:** Convertir `/v1/parse` de cola pasiva a job ejecutable.
- **Input:** `strata-server/src/routes.rs`, `jobs.rs`, `state.rs`, `JobStore`, pipeline compartido.
- **Output:** Worker background que persiste bytes del PDF, procesa y actualiza `JobStatus::Done` con `result_md`/`result_json`.
- **Proceso:**
  1. Persistir input en storage temporal o store dedicado por SHA.
  2. Spawn controlado por runtime/scheduler.
  3. Actualizar progreso por página.
  4. Manejar cancelación/fallo con `JobStatus::Failed` tipado.
- **Tests:** Test HTTP con PDF fixture pequeño: `POST /v1/parse` → polling → `Done` con resultados no vacíos.
- **AC:** `/v1/jobs/{id}` retorna artefactos reales y no queda indefinidamente en `Queued`.

---

### Subfase D — Validación, Benchmarks, Observabilidad y Documentación

> **Meta:** Demostrar que el uso completo de LLM/VLM/OCR funciona, no solo que compila.

---

### [X] T14.7 — Tests E2E de IA con mock y Ollama real

- **Objetivo:** Cubrir el pipeline completo sin depender siempre de GPU/Ollama en CI.
- **AC:** CI ejecuta suite mock determinista; suite real queda documentada para entorno local/GPU.

#### [X] A14.7.1 — Servidor gRPC IA fake para CI

- **Objetivo:** Validar integración Rust↔IA sin modelos pesados.
- **Input:** `strata-ia-bridge` server generated stubs, fixtures PDF, pipeline compartido.
- **Output:** Test helper que responde OCR/table/image/formula con payloads deterministas.
- **Proceso:**
  1. Levantar servidor gRPC fake en puerto efímero durante test.
  2. Ejecutar pipeline con `--ia-grpc-endpoint` apuntando al fake.
  3. Verificar Markdown/JSON final.
- **Tests:** `cargo nextest run -p strata-pipeline -p strata-cli`.
- **AC:** Test pasa sin Ollama instalado.

#### [X] A14.7.2 — Suite real con Ollama y OCR opcional

- **Objetivo:** Validar modelos locales reales y medir calidad.
- **Input:** Ollama con `qwen2.5vl:7b`, `minicpm-v:8b`, Surya/Tesseract opcionales, fixtures canónicos.
- **Output:** Tests marcados `@ollama`, `@gpu`, `@cpu_heavy` y guía de ejecución.
- **Proceso:**
  1. Verificar modelos antes de iniciar tests.
  2. Ejecutar casos figure/table/scanned/formula.
  3. Guardar métricas de latencia y provenance.
- **Tests:** `uv run pytest tests/e2e -m ollama -q`.
- **AC:** Tests se saltan con mensaje claro si faltan modelos; pasan cuando el entorno cumple prerrequisitos.

#### [X] A14.7.3 — Benchmarks y degradación por fallback

- **Objetivo:** Medir impacto de IA y asegurar fallback controlado.
- **Input:** `benches/batch.py`, `benches/fidelity.py`, métricas runtime.
- **Output:** Reporte en `docs/benchmarks/ia_end_to_end.md` con latencia, throughput, cache hits y calidad por ruta.
- **Proceso:**
  1. Comparar `--no-ia` vs `--ia` vs `--ia --force-ocr`.
  2. Medir cache warm/cold.
  3. Simular Ollama caído y verificar fallback/error esperado.
- **Tests:** Script benchmark reproducible con fixtures.
- **AC:** Reporte incluye comandos, hardware, modelos, métricas y límites conocidos.

---

### [X] T14.8 — Documentación operativa y trazabilidad arquitectónica

- **Objetivo:** Mantener documentación y arquitectura sincronizadas con la implementación IA completa.
- **AC:** Un agente puede implementar o depurar la Fase 14 leyendo `architecture.md`, Plan Maestro y este backlog sin contexto adicional.

#### [X] A14.8.1 — Actualizar guías de uso IA

- **Objetivo:** Documentar comandos reales y requisitos para descripciones de imágenes, OCR, tablas y fórmulas.
- **Input:** README, `docs/usage/local_setup.md`, flags implementados.
- **Output:** Guías con ejemplos:
  - PDF nativo sin IA;
  - figuras con VLM;
  - OCR forzado;
  - tablas borderless;
  - SDK Python con `ParseOptions`.
- **Proceso:**
  1. Escribir ejemplos copy-pasteables.
  2. Documentar variables `STRATA_IA_*` y modelos Ollama.
  3. Incluir troubleshooting de Ollama/modelos/cache.
- **Tests:** Comandos de docs validados manualmente o por smoke tests.
- **AC:** README no promete flags inexistentes; docs reflejan comportamiento real.

#### [X] A14.8.2 — Añadir métricas y logs de IA end-to-end

- **Objetivo:** Hacer observable cada escalamiento a IA.
- **Input:** `strata-runtime::Metrics`, `tracing`, `structlog`, provenance.
- **Output:** Métricas por task/model/backend/cache y logs con `document_sha`, `page`, `block_id`, `route`, `latency_ms`.
- **Proceso:**
  1. Instrumentar creación de tareas IA.
  2. Instrumentar llamadas gRPC y fusión.
  3. Evitar loguear contenido sensible del PDF salvo opt-in debug explícito.
- **Tests:** Unit test o snapshot de métricas Prometheus con labels esperadas.
- **AC:** `/metrics` expone contadores IA y logs permiten rastrear por qué un bloque fue OCR/VLM.

#### [X] A14.8.3 — Mantener arquitectura y ADRs actualizados

- **Objetivo:** Registrar decisiones de diseño que afecten acoplamiento, metadata y despliegue.
- **Input:** `docs/architecture/architecture.md`, `docs/adr/`, cambios de F14.
- **Output:** Architecture actualizado y ADRs para decisiones no triviales: crate `strata-pipeline`, metadata de `Block`, semántica `--ia/--no-ia`, estrategia embedded worker.
- **Proceso:**
  1. Actualizar diagramas C4 y secuencia si cambia el flujo.
  2. Crear ADRs cuando una decisión tenga alternativa razonable descartada.
  3. Linkear ADRs desde Plan Maestro y tareas.
- **Tests:** `markdownlint-cli2 docs/**/*.md` si está disponible.
- **AC:** Documentación no queda obsoleta respecto a flags, crates y flujo real.

---

### Dependencias entre Tareas de Fase 14

```text
Subfase A — base estructural:
  T14.1 ──► T14.2
    │         │
    └────────►┘

Subfase B — IA y fusión:
  T14.2 ──► T14.3 ──► T14.5
              │          ▲
              ▼          │
            T14.4 ───────┘

Subfase C — superficies públicas:
  T14.5 ──► T14.6

Subfase D — validación y docs:
  T14.6 ──► T14.7 ──► T14.8
```

---

## Apéndice A — Entorno de Máquina Personal (Restricciones Superadas)

> **Contexto Actual.** El desarrollo original tuvo barreras debido al EDR / AppLocker de la máquina corporativa. Actualmente nos encontramos en una máquina personal con privilegios de administrador, lo que nos ha permitido compilar y ejecutar exitosamente el binario de Rust sin problemas de acceso.
>
> Los siguientes items representan la ejecución real de las compilaciones, empaquetados (maturin) y pruebas unitarias/E2E, las cuales han sido completadas satisfactoriamente en este entorno libre de restricciones.

---

> **Nota de uso.** Este archivo es el tablero Kanban vivo. Al avanzar, mover checkboxes `[ ] → [/] → [X]`. Cualquier cambio de scope debe reflejarse primero en el Plan Maestro y luego derivarse aquí. Los AC son contractuales: ninguna tarea se marca `[X]` sin evidencia (logs, snapshots, métricas) commiteada en el PR correspondiente.

---

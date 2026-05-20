# Strata-Reader 📐

[![CI](https://github.com/AlexPrietoRomani/strata-reader/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexPrietoRomani/strata-reader/actions/workflows/ci.yml)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Rust: 1.88+](https://img.shields.io/badge/rust-1.88%2B-orange.svg)](Cargo.toml)
[![Python: 3.12+](https://img.shields.io/badge/python-3.12%2B-blue.svg)](pyproject.toml)
[![Local AI: Ollama](https://img.shields.io/badge/local%20AI-Ollama-purple.svg)](https://ollama.com)

**Strata-Reader** es un motor de extracción documental ultra-rápido, concurrente y fidedigno diseñado para transformar archivos PDF complejos en Markdown semántico (para Vector RAG) y grafos estructurados en JSON (para Graph-RAG). 

Construido con un **núcleo de geometría matemática en Rust** y una **capa de inteligencia artificial local en Python**, Strata-Reader escala el procesamiento de documentos densos (como papers científicos, informes técnicos y patentes) delegando de manera selectiva las regiones complejas a modelos de visión multimodal locales, sin enviar un solo byte a la nube.

> 📐 **Geometría en Rust + Inferencia Multimodal Local.** Diseñado para despliegues *on-premise* bajo rigor metodológico (trazabilidad de procedencia estilo PRISMA). Ollama local es el único motor de inferencia requerido.

---

## 🗺️ Arquitectura del Sistema

Strata-Reader divide el trabajo mediante un pipeline híbrido asíncrono. Los componentes nativos realizan el análisis geométrico inicial y el enrutamiento inteligente (Triage) hacia los modelos de lenguaje locales.

```mermaid
graph TD
    classDef rust fill:#dea584,stroke:#333,stroke-width:2px,color:#000;
    classDef python fill:#4b8bbe,stroke:#333,stroke-width:2px,color:#fff;
    classDef decision fill:#ffdfba,stroke:#333,stroke-width:2px,color:#000;
    classDef output fill:#74c476,stroke:#333,stroke-width:2px,color:#000;
    classDef file fill:#f9f9f9,stroke:#333,stroke-width:2px;

    Doc([Documento PDF / Lotes]) --> Ingesta
    
    subgraph RUST_CORE [Capa Geométrica y Triage — Rust Core]
        Ingesta[1. Decodificador PDF \n bindings pdfium-render]:::rust
        Ingesta --> Raw[2. Extracción Cruda \n Glifos, Vectores, Imágenes]:::rust
        Raw --> Quality[3. Detector de Calidad \n ¿Fuentes CID Corruptas o Escaneado?]:::rust
        Quality --> Topology[4. Análisis Topológico \n XY-Cut++ e Índice R-Tree]:::rust
        Topology --> StandardTables[5. Detección Vectorial \n Grillas con Bordes]:::rust
        StandardTables --> Triage{6. Motor de Triage \n Enrutamiento Inteligente}:::decision
    end

    subgraph PYTHON_IA [Capa IA Multimodal Local — Python gRPC]
        Triage -- "Página Escaneada o Fuentes Rotas" --> OCR[7a. Orquestador OCR \n Surya OCR / Tesseract]:::python
        Triage -- "Tabla Compleja sin Bordes" --> VLMTable[7b. Extracción Tabular VLM \n Qwen2.5-VL via Ollama]:::python
        Triage -- "Diagrama o Ilustración de Campo" --> VLMImage[7c. Visión Multimodal \n Descripciones Detalladas]:::python
        Triage -- "Ecuaciones Agronómicas / Físicas" --> VLMEq[7d. Conversor de Fórmulas \n Image-to-LaTeX]:::python
    end

    Triage -- "Texto Nativo Limpio y Tablas Simples" --> Fusion
    
    OCR --> Fusion
    VLMTable --> Fusion
    VLMImage --> Fusion
    VLMEq --> Fusion

    subgraph ENSAMBLAJE [Fusión semántica y salida]
        Fusion[8. Re-ensamblaje Espacial \n Fusión de BBoxes e IA]:::rust
        Fusion --> Structuring[9. Jerarquización Semántica \n Títulos, Párrafos, Encabezados]:::rust
        Structuring --> OutJSON[JSON Estructurado \n Listo para Graph-RAG]:::output
        Structuring --> OutMD[Markdown Semántico \n Chunking Lógico para RAG]:::output
    end
```

---

## 🔬 ¿Por qué Strata-Reader?

La motivación principal del proyecto nace al auditar la arquitectura monolítica de **[opendataloader-pdf](https://github.com/opendataloader-project/opendataloader-pdf)** en Java. Aunque sus heurísticas espaciales son sobresalientes, su fidelidad en estructuras complejas es deficiente (TEDS ≈ 0.49 en tablas sin bordes). 

**Strata-Reader** conserva el rigor matemático original y lo eleva mediante:
1. **Re-ingeniería en Rust Puro:** Re-escritura funcional del algoritmo **XY-Cut++** y del procesador geométrico optimizados mediante índices espaciales R-Tree (`rstar`) de O(log N).
2. **Inmutabilidad Absoluta:** Se erradica la mutación orientada a objetos in-place. Cada etapa de procesamiento genera un nuevo estado inmutable del AST documental (`Arc<Document>`).
3. **Escalabilidad Híbrida Inteligente (Triage Engine):** El motor no procesa todo con modelos de lenguaje caros. Clasifica bloque por bloque; el texto nativo se extrae a velocidad del metal, y solo las celdas densas sin bordes, diagramas o fórmulas se recortan y envían localmente a la IA.
4. **Trazabilidad PRISMA Completa:** Cada bloque retornado cuenta con metadatos de procedencia (`confidence`, `source` (`rust`/`ocr`/`vlm`), `model`, `latency_ms`), idóneo para entornos científicos.

---

## 🛠️ Estructura del Repositorio

El monorepo está diseñado de forma modular y desacoplada:

```text
strata-reader/
├── crates/                            # Workspace de Rust Core
│   ├── strata-core/                   # AST inmutable, BBoxes y tipos del dominio
│   ├── strata-pdf/                    # Decodificador de PDFium
│   ├── strata-geometry/               # XY-Cut++, R-Tree y detección de tablas
│   ├── strata-quality/                # Detector de fuentes CID rotas y escaneos
│   ├── strata-triage/                 # Árbol de decisiones y render de crops
│   ├── strata-ia-bridge/              # Cliente de comunicación gRPC (Tonic)
│   ├── strata-fusion/                 # Re-ensamblaje y jerarquización de contenidos
│   ├── strata-serialize/              # Generadores de Markdown y JSON Graph-RAG
│   ├── strata-runtime/                # Planificador Tokio, GPU monitor y backpressure
│   ├── strata-cli/                    # Binario ejecutable de consola `strata`
│   ├── strata-server/                 # Servidor microservicio HTTP (Axum)
│   └── strata-py/                     # Bindings nativos PyO3
├── python/                            # Capa de Inferencia y SDK de Python
│   ├── strata_ia/                     # FastAPI + Servidor gRPC de IA local
│   └── strata_reader/                 # Facada del SDK de Python (wheel)
└── tests/                             # Suites de testings cruzados y fixtures golden
```

---

## 🚀 Inicio Rápido en 5 Minutos

### 1. Prerrequisitos de la máquina
Asegúrate de contar con Rust 1.88+, Python 3.12+ (gestionado idealmente con `uv`) y Ollama corriendo localmente.

### 2. Compilar el CLI
```bash
# Compilar el binario nativo en modo optimizado
cargo build -p strata-cli --release
```

### 3. Iniciar el Microservicio de IA local
Arranca el orquestador local. Este script PowerShell o Bash encenderá Ollama y descargará automáticamente los 3 modelos necesarios (`qwen2.5vl:7b`, `minicpm-v:8b`, `llama3.2-vision:11b`):

```powershell
# En Windows (PowerShell)
.\scripts\dev_up.ps1 -WithServer
```
```bash
# En Linux / macOS
./scripts/dev_up.sh --with-server
```

### 4. Parsear un PDF científico
```bash
# Ejecutar el parseo completo
./target/release/strata parse \
    --input tests/fixtures/pdfs/two_column_paper.pdf \
    --output out/ \
    --format md+json \
    --profile scientific
```

---

## ⚠️ Advertencia de Entorno Corporativo (EDR / AppLocker)

Si estás trabajando en una máquina con directivas de seguridad corporativas estrictas (por ejemplo, `EMPRESA\usuario`), el compilador Rust o el linker (`ld.lld.exe`) pueden fallar inmediatamente arrojando **`Acceso denegado (os error 5)`** al intentar ejecutar binarios en directorios de usuario (`target/`).

Esto **no** es un problema de permisos de NTFS ni un error del código. Es un bloqueo del software de seguridad corporativo (AppLocker/CrowdStrike/Defender ATP). 

**Mitigación:**
- Consulta la solicitud formal enviada a IT en [IT_request.md](docs/usage/IT_request.md) para habilitar excepciones.
- Alternativamente, compila y ejecuta en una máquina sin estas restricciones (laptop personal, VM Hyper-V libre o un runner de CI en la nube) siguiendo los pasos del **Apéndice A** de [tareas.md](docs/task/tareas.md).

---

## 📦 Tres Superficies de Distribución

Strata-Reader se adapta a cualquier entorno de despliegue:

1. **Binario CLI Nativo:** Utilidad portable ultra-rápida. `cargo install --path crates/strata-cli`.
2. **Microservicio REST/gRPC en Red:** Listo para desplegar en K8s o contenedores en la nube. `strata serve --bind 0.0.0.0:8080`.
3. **Paquete Python pip-installable (Wheel):** Una rueda multiplataforma que embebe el núcleo de Rust y libpdfium. Puedes usarlo directamente en tus scripts de Python de la siguiente manera:

```python
from strata_reader import parse, ParseOptions

# Parsear usando el perfil científico de alta fidelidad
doc = parse("paper.pdf", options=ParseOptions(profile="scientific"))

# Generar salidas listas para ingesta de RAG
markdown_content = doc.to_markdown()
graph_json = doc.to_graph_json()
```

---

## 📖 Documentación Relacionada

- 📦 **[Guía de Instalación Local y EDR](docs/usage/local_setup.md)** — Configuración fina, compiladores de Windows y diagnóstico de seguridad.
- 💾 **[SDK de Python](docs/usage/sdk_python.md)** — Documentación de la API de Python, `parse_batch` y configuración de hilos.
- 🚀 **[Ingesta Vector RAG](docs/usage/rag_simple.md)** — Cómo trocear de forma lógica el Markdown generado para bases vectoriales.
- 🕸️ **[Ingesta Graph-RAG](docs/usage/graph_rag.md)** — Estructura del JSON con nodos y relaciones listo para Neo4j.
- 🗃️ **[Despliegue del Microservicio](docs/usage/microservice.md)** — Rutas, orquestación, telemetría de GPU y Prometheus.
- 📑 **[Plan Maestro de Arquitectura](docs/plan/plan_maestro.md)** — Especificación técnica contractual completa del monorepo.
- 📌 **[Backlog de Tareas (Tablero Kanban)](docs/task/tareas.md)** — Estado del desarrollo, pruebas unitarias y checklist de bring-up.

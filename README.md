# Strata-Reader 📐

**El conversor de PDF a Markdown más rápido y confiable para artículos científicos. Diseñado para RAG estándar y RAG de grafos de forma 100 % local y offline.**

[![CI](https://github.com/AlexPrietoRomani/strata-reader/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexPrietoRomani/strata-reader/actions/workflows/ci.yml)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Rust: 1.88+](https://img.shields.io/badge/rust-1.88%2B-orange.svg)](Cargo.toml)
[![Python: 3.12+](https://img.shields.io/badge/python-3.12%2B-blue.svg)](pyproject.toml)
[![Local AI: Ollama](https://img.shields.io/badge/local%20AI-Ollama-purple.svg)](https://ollama.com)

🔍 **Extractor documental ultra-rápido y fidedigno** — Transforma PDFs científicos complejos en Markdown semántico fluido (para Vector RAG) y grafos estructurados en JSON (para Graph-RAG) de forma 100 % local, garantizando la privacidad absoluta de tus datos.

- **¿Qué tan rápido y preciso es?** — Es la librería más rápida y confiable para artículos científicos. Su núcleo geométrico escrito en Rust procesa el texto nativo a la velocidad del metal (~0.009 segundos por página) y preserva el orden de lectura mediante el algoritmo **XY-Cut++** optimizado con índices espaciales R-Tree.
- **¿Es compatible con RAG estándar y RAG de grafos?** — Absolutamente. Está optimizado desde su arquitectura para alimentar pipelines RAG vectoriales con Markdown semántico limpio (evitando párrafos rotos y marcas de agua) y Graph-RAG con JSON estructurado rico en nodos de conocimiento, relaciones de citas y procedencia metodológica.
- **¿Procesa tablas, fórmulas e imágenes?** — Sí. Extrae tablas de forma híbrida (nativas con bordes y mediante IA local las sin bordes), fórmulas matemáticas representadas en LaTeX (`$$`), e imágenes nativas integradas directamente en el AST como `BlockType::Figure` con exportación automática a disco.
- **¿Es compatible con PDFs escaneados y OCR?** — Sí. A través de un motor de Triage híbrido e inteligente, evalúa automáticamente la calidad de las fuentes de cada página y orquesta OCR local de alta precisión (Surya/Tesseract) cuando es necesario.

---

## ⚡ Get Started in 30 Seconds

**Requirements:** Python 3.12+. No Rust toolchain required for standard use. No Java required. No Cloud APIs required.

```bash
pip install -U strata-reader
```

### Python API — Parse a single PDF (returns a structured Document)
```python
import strata_reader

# Parse a single PDF — returns a Document object
doc = strata_reader.parse("paper.pdf")

print(doc.to_markdown())     # Markdown ready for Vector RAG chunking (Chroma, FAISS)
print(doc.to_graph_json())   # Structured JSON for Graph-RAG ingestion (Neo4j)
```

### Python API — Batch convert folder or files to disk
```python
import strata_reader

strata_reader.convert(
    input_path=["file1.pdf", "file2.pdf", "papers/"],
    output_dir="output/",
    format="md+json"
)
# → Produces output/file1.md, output/file1.json, output/file2.md, ...
```

### CLI — Command Line Usage
```bash
# Single file
strata parse --input paper.pdf --output out/ --format md+json

# Batch folder recursive with scientific profile
strata parse --input papers/ --output out/ --format md+json --profile scientific
```

---

## 🎯 ¿Qué problemas resuelve Strata-Reader?

| Problema | Solución | Estado |
|:---|:---|:---:|
| **Pérdida de estructura en PDFs** — orden de lectura erróneo, párrafos fragmentados verticalmente, tablas rotas y sin coordenadas de elementos | Re-ingeniería en Rust con el algoritmo **XY-Cut++** e índices espaciales **R-Tree** para un orden de lectura determinista y fluido. | **Shipped** |
| **Inferencia costosa y lenta** — procesar páginas enteras con modelos de visión en la nube es caro, lento y compromete la privacidad de datos | **Triage Engine Híbrido** que extrae texto nativo a velocidad nativa y delega de forma selectiva solo regiones complejas (tablas sin bordes, figuras) a modelos de IA locales. | **Shipped** |
| **Baja fidelidad científica** — falta de procedencia y trazabilidad de los datos científicos requerida por rigor metodológico | **Trazabilidad PRISMA Completa**: Cada bloque de contenido extraído cuenta con metadatos de procedencia (fuente de origen, modelo de IA, confianza y latencia). | **Shipped** |
| **Integraciones complejas** — APIs engorrosas y scripts de automatización con docenas de líneas de código | **Python SDK simplificado** estilo `pandas` que permite realizar conversiones robustas con una sola línea de código o llamadas por lote. | **Shipped** |

---

## 📊 Matriz de Capacidades

| Capacidad | Soportada | Método de Ejecución |
|:---|:---:|:---|
| **Extracción de Texto** | **Yes** | Geométrico Nativo (Rust Core) |
| **Orden de Lectura Determinista** | **Yes** | Algoritmo XY-Cut++ con R-Tree |
| **Tablas con bordes (GFM)** | **Yes** | Geométrico Nativo (Rust Core) |
| **Tablas complejas/sin bordes** | **Yes** | Híbrido (IA local Qwen2.5-VL via Ollama) |
| **Fórmulas Matemáticas (LaTeX)** | **Yes** | Detección Nativa + Formato estándar `$$` |
| **Estructuración Jerárquica** | **Yes** | Clasificador Avanzado de Headings |
| **Extracción de Imágenes / Figuras** | **Yes** | Geométrico Nativo (Rust Core) |
| **Descripciones de Figuras (Alt text)**| **Yes** | Híbrido (IA local Qwen2.5-VL) |
| **OCR para PDFs escaneados** | **Yes** | Orquestador Local (Surya OCR / Tesseract) |
| **Metadata de Procedencia** | **Yes** | Trazabilidad PRISMA por bloque |
| **Offline 100 % Local** | **Yes** | Cero llamadas a APIs en la nube |

---

## 🗺️ Arquitectura del Sistema

Strata-Reader divide el trabajo mediante un pipeline híbrido asíncrono. Los componentes nativos en Rust realizan el análisis geométrico inicial y el enrutamiento inteligente (Triage) hacia los modelos de lenguaje locales:

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

## 🎯 ¿Qué modo de procesamiento debo usar?

El motor geométrico nativo escrito en Rust maneja la gran mayoría del trabajo de forma autónoma. Obtendrás texto limpio, jerarquías de cabeceras, fórmulas en LaTeX y tablas con bordes **por defecto** sin necesidad de flags adicionales. Solo activa el modo IA cuando necesites modelos multimodales locales.

| Documento de Entrada | Modo Recomendado | Requisitos | Comando Recomendado |
|:---|:---|:---|:---|
| **PDF digital estándar** (La gran mayoría) | **Nativo** (Default) | Ninguno (solo `pip install`) | `strata parse --input doc.pdf --output out/` |
| **Tablas complejas/sin bordes** | **Híbrido IA** | Ollama encendido localmente | `strata parse --input doc.pdf --output out/ --ia` |
| **PDF escaneado / basado en imágenes** | **IA + OCR** | Ollama encendido localmente | `strata parse --input doc.pdf --output out/ --ia --force-ocr` |
| **Fórmulas matemáticas complejas** | **Nativo** (Default) | Ninguno (detección automática) | `strata parse --input doc.pdf --output out/` |
| **Imágenes e ilustraciones con descripción** | **Híbrido IA** | Ollama encendido localmente | `strata parse --input doc.pdf --output out/ --ia` |

### 🤖 Modo IA: ¿Qué aporta la bandera `--ia`?

| Característica / Bloque | Modo Nativo (Rust-only) | Modo IA (Rust + Ollama VLM) |
|:---|:---|:---|
| **Párrafos de texto** | Extracción geométrica fluida | Extracción geométrica fluida |
| **Tablas con bordes** | Formateadas en Markdown GFM nativo | Formateadas en Markdown GFM nativo |
| **Tablas sin bordes** | Omitidas / Texto crudo | Extraídas y reconstruidas por Qwen2.5-VL |
| **Fórmulas en LaTeX** | Detección espacial y formateo `$$` | Detección espacial y formateo `$$` |
| **Páginas escaneadas** | Omitidas (detecta mala calidad) | Procesadas vía Surya OCR / Tesseract |
| **Extracción de figuras** | Exportación de imagen nativa a disco | Exportación de imagen nativa a disco |
| **Descripciones de figuras** | Omitidas | Generadas de forma multimodal por Qwen2.5-VL |
| **Metadatos de procedencia** | `source: "rust"`, confianza geométrica | `source: "vlm"`, modelo, latencia en ms |

---

## 📂 Salidas para RAG y Graph-RAG

| Formato de Salida | Archivo Generado | Caso de Uso Principal |
|:---|:---|:---|
| `--format md` | `{output}/{stem}.md` | **Vector RAG** tradicional (Chroma, Pinecone, FAISS) |
| `--format json` | `{output}/{stem}.json` | **Graph-RAG** o bases de conocimiento estructuradas (Neo4j) |
| `--format md+json` | Ambos archivos | Ingesta híbrida y sincronizada para RAG multiruta |

*Nota: El `stem` corresponde al nombre base del archivo PDF (ej. `paper.pdf` generará `paper.md` y `paper.json`).*

---

## 🛠️ Estructura del Repositorio

El monorepo está estructurado de forma modular y altamente desacoplada:

```text
strata-reader/
├── crates/                            # Workspace de Rust Core (Alto Rendimiento)
│   ├── strata-core/                   # AST inmutable, BBoxes y tipos del dominio
│   ├── strata-pdf/                    # Decodificador de PDFium (Glifos y paths nativos)
│   ├── strata-geometry/               # XY-Cut++, R-Tree, detección de tablas, ruido y párrafos
│   ├── strata-quality/                # Detector de calidad de fuentes CID rotas y escaneos
│   ├── strata-triage/                 # Árbol lógico de decisiones y renderizado de crops
│   ├── strata-ia-bridge/              # Cliente de comunicación gRPC (Tonic) hacia Python IA
│   ├── strata-fusion/                 # Re-ensamblaje y jerarquización espacial de contenidos
│   ├── strata-serialize/              # Renderizadores de Markdown y JSON Graph-RAG
│   ├── strata-runtime/                # Planificador Tokio, monitor de GPU y backpressure
│   ├── strata-cli/                    # Binario ejecutable de consola `strata`
│   ├── strata-server/                 # Servidor microservicio HTTP (Axum)
│   └── strata-py/                     # Bindings de Python nativos usando PyO3
├── python/                            # Capa de Inferencia y SDK de Python
│   ├── strata_ia/                     # FastAPI + Servidor gRPC de IA local (Ollama/Surya)
│   └── strata_reader/                 # Interfaz pública del SDK de Python (wheel)
└── tests/                             # Pruebas de integración, E2E y fixtures golden
```

---

## 🔧 Compilación y Configuración desde Código Fuente

> **Nota:** Si instalas mediante `pip install strata-reader`, puedes omitir este apartado. Esto es exclusivamente para desarrolladores que desean compilar el núcleo nativo de Rust directamente.

### Prerrequisitos
- Rust 1.88+
- Python 3.12+ con `uv`
- Ollama (con los modelos correspondientes descargados)

### 1. Configurar libpdfium (Windows)
```powershell
New-Item -ItemType Directory -Path "$env:LOCALAPPDATA\pdfium" -Force
curl.exe -L -o $env:TEMP\pdfium-win-x64.tgz "https://github.com/bblanchon/pdfium-binaries/releases/download/chromium/7843/pdfium-win-x64.tgz"
tar -xzf $env:TEMP\pdfium-win-x64.tgz -C $env:LOCALAPPDATA\pdfium
[Environment]::SetEnvironmentVariable("STRATA_PDFIUM_LIB_PATH", "$env:LOCALAPPDATA\pdfium\bin", "User")
```

### 2. Compilar el Crate CLI
```bash
cargo build -p strata-cli --release
```

### 3. Ejecutar Análisis Local (Sin Dependencia de IA)
```bash
# Procesa un PDF y genera el Markdown y JSON estructurado
./target/release/strata parse --input paper.pdf --output out/ --format md+json --no-ia
```

---

## 📦 Tres Superficies de Distribución

Strata-Reader se adapta a cualquier entorno de despliegue:

1. **Paquete Python Wheel (pip):** Rueda multiplataforma autocontenida con el núcleo compilado de Rust y pdfium.
2. **Consola Nativa (CLI):** Utilidad portable para procesamiento masivo de terminal.
3. **Servidor HTTP REST / gRPC:** Microservicio escalable listo para desplegar en clústeres de Kubernetes o contenedores Docker en la nube (`strata serve --bind 0.0.0.0:8080`).

---

## 📖 Documentación Relacionada

- 📄 **[Descripción del Proyecto](docs/reference/description_proyect.md)** — Análisis de arquitectura, migración y decisiones de diseño.
- 📋 **[CHANGELOG](CHANGELOG.md)** — Historial de versiones y cambios.

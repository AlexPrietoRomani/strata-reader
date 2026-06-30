//! Módulo: options
//!
//! Descripción:
//! Opciones de configuración del pipeline de parse.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Opciones pasadas al orquestador de pipeline.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParsePipelineOptions {
    /// Ruta al PDF de entrada.
    pub input: PathBuf,
    /// Perfil de triage: "fast" | "balanced" | "scientific".
    pub profile: String,
    /// Habilita el bridge IA (OCR/VLM).
    pub use_ia: bool,
    /// Fuerza OCR para todas las páginas aunque tengan texto nativo.
    pub force_ocr: bool,
    /// Endpoint de Ollama (usado por el embedded worker Python).
    pub ollama_endpoint: String,
    /// Endpoint gRPC externo del microservicio IA.
    /// Si es None y use_ia=true, se lanza un EmbeddedWorker automáticamente.
    pub ia_grpc_endpoint: Option<String>,
    /// Máximo de páginas procesadas en paralelo.
    pub max_concurrent_pages: Option<usize>,
    /// Directorio donde guardar imágenes/crops.
    pub media_dir: Option<PathBuf>,
    /// Guardar imágenes en disco.
    pub save_images: bool,
    /// Backend PDF: "pdfium" | "pure" | "auto".
    pub pdf_backend: String,
}

impl Default for ParsePipelineOptions {
    fn default() -> Self {
        Self {
            input: PathBuf::new(),
            profile: "balanced".into(),
            use_ia: false,
            force_ocr: false,
            ollama_endpoint: "http://localhost:11434".into(),
            ia_grpc_endpoint: None,
            max_concurrent_pages: None,
            media_dir: None,
            save_images: false,
            pdf_backend: "auto".into(),
        }
    }
}

//! Módulo: artifacts
//!
//! Descripción:
//! Artefactos de salida del pipeline de parse.

use std::path::PathBuf;

use strata_core::Document;

/// Salidas producidas por el pipeline tras procesar un PDF.
#[derive(Debug)]
pub struct ParseArtifacts {
    /// AST del documento fusionado (nativo + IA).
    pub document: Document,
    /// Markdown semántico listo para Vector-RAG.
    pub markdown: String,
    /// JSON para Graph-RAG.
    pub graph_json: serde_json::Value,
    /// Rutas de imágenes/crops guardados en disco (si save_images=true).
    pub media_paths: Vec<PathBuf>,
}

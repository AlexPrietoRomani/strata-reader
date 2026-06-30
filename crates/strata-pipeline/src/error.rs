//! Módulo: error
//!
//! Descripción:
//! Errores tipados del pipeline.

use thiserror::Error;

/// Errores que puede producir el pipeline de parse.
#[derive(Debug, Error)]
pub enum PipelineError {
    #[error("failed to open PDF: {0}")]
    PdfOpen(String),

    #[error("page {page} extraction failed: {reason}")]
    PageExtraction { page: usize, reason: String },

    #[error("IA bridge unavailable: {0}\nTip: start Ollama with `ollama serve` and ensure the model is pulled.")]
    IaUnavailable(String),

    #[error("IA bridge RPC failed: {0}")]
    IaRpc(String),

    #[error("serialization error: {0}")]
    Serialization(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

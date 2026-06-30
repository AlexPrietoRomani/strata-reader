//! Módulo: strata-pipeline
//!
//! Descripción:
//! Orquestador de pipeline compartido PDF → Markdown/JSON.
//! Reutilizado por strata-cli, strata-py y strata-server.
//!
//! Estructura Interna:
//! - `artifacts`: Tipos de salida (`ParseArtifacts`).
//! - `error`: Errores tipados (`PipelineError`).
//! - `ia_bridge`: Gestión del bridge gRPC de IA.
//! - `mappers`: Conversión de respuestas proto a `IaPayload`.
//! - `options`: Opciones de configuración (`ParsePipelineOptions`).
//! - `pipeline`: Función principal `parse_document`.
//!
//! Ejemplo de Integración:
//! ```no_run
//! use strata_pipeline::{parse_document, ParsePipelineOptions};
//! ```

#![deny(rust_2018_idioms)]

pub mod artifacts;
pub mod error;
pub mod ia_bridge;
pub mod mappers;
pub mod options;
pub mod pipeline;

pub use artifacts::ParseArtifacts;
pub use error::PipelineError;
pub use options::ParsePipelineOptions;
pub use pipeline::parse_document;

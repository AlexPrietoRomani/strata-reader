//! Top-level decoder: open a PDF and obtain handles to its pages.
//!
//! [`Decoder`] is a thin, lifetime-careful wrapper around abstract [`PdfDoc`].
//! The wrapper exposes only the methods the rest of the pipeline needs.
//!
//! See Plan Maestro §7.T2.1.

use crate::backend::{PdfBackend, PdfDoc, PdfPage};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DecoderError {
    #[error("pdfium native library could not be loaded: {0}")]
    PdfiumLoad(String),

    #[error("failed to read PDF at {path:?}: {source}")]
    Io {
        path: std::path::PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("pdfium rejected the document: {0}")]
    Parse(String),

    #[error("page index {requested} is out of range (document has {total} pages)")]
    PageOutOfRange { requested: usize, total: usize },

    #[error("pdfium internal error: {0}")]
    Internal(String),
}

/// Owning handle around a loaded PDF document.
pub struct Decoder {
    doc: Box<dyn PdfDoc>,
}

impl std::fmt::Debug for Decoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Decoder")
            .field("pages_count", &self.page_count())
            .finish()
    }
}

impl Decoder {
    /// Open a PDF from disk. The full file is read into memory once and passed
    /// to the active backend.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, DecoderError> {
        let path = path.as_ref();
        let bytes = std::fs::read(path).map_err(|source| DecoderError::Io {
            path: path.to_path_buf(),
            source,
        })?;
        Self::from_bytes(bytes.into_boxed_slice())
    }

    /// Open a PDF from in-memory bytes (useful for HTTP uploads).
    pub fn from_bytes(bytes: Box<[u8]>) -> Result<Self, DecoderError> {
        let backend = Self::default_backend();
        let doc = backend.open(&bytes)?;
        Ok(Self { doc })
    }

    /// Selects the default backend based on enabled compile features.
    pub fn default_backend() -> Box<dyn PdfBackend> {
        #[cfg(feature = "pdfium-backend")]
        {
            Box::new(crate::pdfium_backend::PdfiumBackend)
        }
        #[cfg(not(feature = "pdfium-backend"))]
        {
            Box::new(crate::pure_backend::PureRustBackend)
        }
    }

    /// Total number of pages in the document.
    pub fn page_count(&self) -> usize {
        self.doc.page_count()
    }

    /// Obtain a page at the given index.
    pub fn page(&self, index: usize) -> Result<Box<dyn PdfPage>, DecoderError> {
        self.doc.page(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_file_yields_io_error() {
        let err = Decoder::open("does-not-exist.pdf").unwrap_err();
        assert!(matches!(err, DecoderError::Io { .. }));
    }
}

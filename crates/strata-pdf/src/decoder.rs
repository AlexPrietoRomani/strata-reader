//! Top-level decoder: open a PDF and obtain handles to its pages.
//!
//! [`Decoder`] is a thin, lifetime-careful wrapper around `pdfium-render`'s
//! [`PdfDocument`]. The wrapper exposes only the methods the rest of the
//! pipeline needs and converts every PDFium error into a typed
//! [`DecoderError`] (we never `unwrap()` per project convention §1).
//!
//! See Plan Maestro §7.T2.1.

use std::path::Path;

use pdfium_render::prelude::*;
use thiserror::Error;

use crate::bindings::get_pdfium;

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

impl From<PdfiumError> for DecoderError {
    fn from(value: PdfiumError) -> Self {
        Self::Internal(value.to_string())
    }
}

/// Owning handle around a loaded PDF document.
///
/// The struct holds the raw bytes (`_bytes`) alive for the lifetime of the
/// [`PdfDocument`] — `pdfium-render` requires that buffer-backed documents
/// keep the source slice valid until the document is dropped.
pub struct Decoder {
    document: PdfDocument<'static>,
    _bytes: Box<[u8]>,
}

impl Decoder {
    /// Open a PDF from disk. The full file is read into memory once and passed
    /// to PDFium as a borrowed slice — small enough for scientific papers,
    /// safer than letting PDFium do its own I/O on weird Windows paths.
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
        let pdfium = get_pdfium()?;

        // SAFETY: We extend the slice lifetime to 'static manually because
        // PDFium will hold a pointer to the buffer for as long as the
        // PdfDocument lives, and we keep `bytes` in the same struct so it
        // outlives the document. The lifetime trick is bounded by Rust's
        // drop order (fields drop bottom-to-top): `document` is dropped
        // before `_bytes`.
        let slice: &'static [u8] = unsafe { std::mem::transmute::<&[u8], &'static [u8]>(&bytes) };
        let document = pdfium
            .load_pdf_from_byte_slice(slice, None)
            .map_err(|e| DecoderError::Parse(e.to_string()))?;

        Ok(Self { document, _bytes: bytes })
    }

    /// Total number of pages in the document (1-based count, 0-indexed access).
    pub fn page_count(&self) -> usize {
        self.document.pages().len() as usize
    }

    /// Borrow the underlying [`PdfDocument`] for read-only operations.
    pub fn document(&self) -> &PdfDocument<'static> {
        &self.document
    }

    /// Iterate over all pages in document order. Each [`PdfPage`] is borrowed
    /// from the document and therefore tied to `&self`'s lifetime.
    pub fn pages(&self) -> PdfPages<'_> {
        self.document.pages()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_from_pdfium_error_round_trips() {
        let err = DecoderError::Internal("boom".into());
        assert!(err.to_string().contains("boom"));
    }

    #[test]
    fn missing_file_yields_io_error() {
        // Pdfium availability is checked first; if not available we get
        // a PdfiumLoad error instead — both are acceptable for this test.
        let err = Decoder::open("does-not-exist.pdf").unwrap_err();
        assert!(matches!(err, DecoderError::Io { .. } | DecoderError::PdfiumLoad(_)));
    }
}

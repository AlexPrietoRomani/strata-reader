//! Abstract PDF decoding traits to support multiple backend engines.
//!
//! Provides the core abstraction layers that allow swapping the PDF decoding
//! engine at compile-time or runtime without touching the downstream geometric
//! and topological pipelines.
//!
//! See Phase 13 Etapa B in `docs/task/tareas.md`.

use strata_core::BBox;
use crate::decoder::DecoderError;
use crate::{Glyph, VectorPath, Image};

/// Core interface for a PDF decoding engine.
pub trait PdfBackend: Send + Sync {
    /// Human-readable identifier of the backend (e.g. "pdfium", "pure-rust").
    fn name(&self) -> &'static str;

    /// Open a PDF document from an in-memory byte buffer.
    fn open(&self, data: &[u8]) -> Result<Box<dyn PdfDoc>, DecoderError>;
}

/// Abstract representation of a loaded PDF document.
pub trait PdfDoc: Send + Sync {
    /// Returns the total number of pages in the document.
    fn page_count(&self) -> usize;

    /// Returns a specific page by its 0-indexed position.
    fn page(&self, index: usize) -> Result<Box<dyn PdfPage>, DecoderError>;
}

/// Abstract representation of a single PDF page.
pub trait PdfPage: Send + Sync {
    /// Returns the width and height of the page box in PDF user points (1/72 inch).
    fn size(&self) -> (f32, f32);

    /// Extracts all characters/glyphs with their respective BBoxes and styles.
    fn glyphs(&self) -> Result<Vec<Glyph>, DecoderError>;

    /// Extracts vector paths and line drawings.
    fn paths(&self) -> Result<Vec<VectorPath>, DecoderError>;

    /// Extracts embedded raster images.
    fn images(&self) -> Result<Vec<Image>, DecoderError>;

    /// Renders a specific rectangular bounding box region to PNG bytes at the given DPI.
    fn render_crop(&self, bbox: BBox, dpi: u16) -> Result<Vec<u8>, DecoderError>;
}

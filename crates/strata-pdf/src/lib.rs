//! Strata-PDF — PDFium-backed and Pure-Rust decoder layer.
//!
//! Public surface:
//!
//! - [`Decoder`] — opens a PDF and exposes pages via abstract [`PdfBackend`].
//! - [`PdfBackend`], [`PdfDoc`], [`PdfPage`] — core abstractions.
//! - [`Glyph`], [`extract_glyphs`] — per-character data with BBox + font info.
//! - [`VectorPath`], [`Segment`], [`extract_paths`] — vector primitives.
//! - [`Image`], [`extract_images`] — embedded raster images, normalized to PNG.
//! - [`is_likely_scan`] — cheap page-level scan detector.
//!
//! See `docs/plan/plan_maestro.md` §7.

#![deny(rust_2018_idioms)]

#[cfg(feature = "pdfium-backend")]
pub mod bindings;
#[cfg(feature = "pdfium-backend")]
pub mod glyph;
#[cfg(feature = "pdfium-backend")]
pub mod image;
#[cfg(feature = "pdfium-backend")]
pub mod vector;

pub mod backend;
pub mod decoder;
pub mod quality;

#[cfg(feature = "pdfium-backend")]
pub mod pdfium_backend;
pub mod pure_backend;

pub use backend::{PdfBackend, PdfDoc, PdfPage};
pub use decoder::{Decoder, DecoderError};
pub use quality::is_likely_scan;

#[cfg(feature = "pdfium-backend")]
pub use bindings::{get_pdfium, pdfium_available};
#[cfg(feature = "pdfium-backend")]
pub use glyph::{extract_glyphs, Glyph};
#[cfg(feature = "pdfium-backend")]
pub use image::{extract_images, Image};
#[cfg(feature = "pdfium-backend")]
pub use vector::{extract_paths, Segment, VectorPath};

// Provide dummy structs if pdfium is compiled out to keep serialization/deserialization code compilable.
#[cfg(not(feature = "pdfium-backend"))]
pub use dummy::*;

#[cfg(not(feature = "pdfium-backend"))]
mod dummy {
    use serde::{Deserialize, Serialize};
    use strata_core::{BBox, Point};

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Glyph {
        pub unicode: char,
        pub bbox: BBox,
        pub font_size: f32,
        pub font_weight: u32,
        pub color_rgba: u32,
        pub rotation: f32,
    }

    #[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "op", rename_all = "kebab-case")]
    pub enum Segment {
        MoveTo(Point),
        LineTo(Point),
        CurveTo { c1: Point, c2: Point, to: Point },
        Close,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct VectorPath {
        pub segments: Vec<Segment>,
        pub stroke: bool,
        pub fill: bool,
        pub bbox: BBox,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Image {
        pub bbox: BBox,
        #[serde(with = "serde_bytes")]
        pub raw_bytes: Vec<u8>,
        pub mime: &'static str,
        pub dpi_estimated: u32,
        pub width_px: u32,
        pub height_px: u32,
    }
}

/// Crate semver.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    #[test]
    fn version_matches_pkg() {
        assert_eq!(super::version(), env!("CARGO_PKG_VERSION"));
    }
}

//! Strata-PDF — PDFium-backed decoder layer.
//!
//! Public surface:
//!
//! - [`Decoder`] — opens a PDF and exposes pages.
//! - [`Glyph`], [`extract_glyphs`] — per-character data with BBox + font info.
//! - [`VectorPath`], [`Segment`], [`extract_paths`] — vector primitives for
//!   the table-border heuristic.
//! - [`Image`], [`extract_images`] — embedded raster images, normalized to PNG.
//! - [`is_likely_scan`] — cheap page-level scan detector.
//!
//! All structs are `Serialize + Deserialize` so they can flow across the
//! gRPC bridge without re-modelling.
//!
//! See `docs/plan/plan_maestro.md` §7.

#![deny(rust_2018_idioms)]

pub mod bindings;
pub mod decoder;
pub mod glyph;
pub mod image;
pub mod quality;
pub mod vector;

pub use bindings::{get_pdfium, pdfium_available};
pub use decoder::{Decoder, DecoderError};
pub use glyph::{extract_glyphs, Glyph};
pub use image::{extract_images, Image};
pub use quality::is_likely_scan;
pub use vector::{extract_paths, Segment, VectorPath};

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

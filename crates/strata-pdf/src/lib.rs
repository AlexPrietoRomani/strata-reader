//! Strata-PDF — PDFium-backed decoder layer.
//!
//! See `docs/plan/plan_maestro.md` §7.

#![deny(rust_2018_idioms)]

pub mod bindings;
pub mod decoder;

pub use bindings::{get_pdfium, pdfium_available};
pub use decoder::{Decoder, DecoderError};

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

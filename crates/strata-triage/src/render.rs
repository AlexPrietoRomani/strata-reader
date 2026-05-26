//! Rasterize an arbitrary [`BBox`] of a [`PdfPage`] to PNG bytes.
//!
//! Used by the Triage Engine to prepare crops for the IA layer. The
//! crop is reproducible byte-for-byte across runs.

use strata_core::BBox;
use strata_pdf::PdfPage;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RenderError {
    #[error("render failed: {0}")]
    Render(String),
    #[error("requested bbox falls entirely outside the page boundary")]
    OutOfPage,
    #[error("png encode failed: {0}")]
    Encode(String),
}

/// Default DPI used when the caller doesn't override it.
pub const DEFAULT_CROP_DPI: u32 = 200;

/// Rasterize the region of `page` enclosed by `bbox` and encode it as PNG.
pub fn render_crop(page: &dyn PdfPage, bbox: BBox, dpi: u32) -> Result<Vec<u8>, RenderError> {
    page.render_crop(bbox, dpi.clamp(72, 6000) as u16)
        .map_err(|e| RenderError::Render(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_dpi_is_200() {
        assert_eq!(DEFAULT_CROP_DPI, 200);
    }
}

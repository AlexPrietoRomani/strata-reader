//! Rasterize an arbitrary [`BBox`] of a [`PdfPage`] to PNG bytes.
//!
//! Used by the Triage Engine to prepare crops for the IA layer. The
//! contract (Plan Maestro §9.T4.4):
//!
//! - The crop is reproducible byte-for-byte across runs (same input ⇒
//!   same SHA-256). This is essential so the SQLite cache hit-rate keeps
//!   the IA bridge cheap.
//! - The crop weighs less than ~500 KB for a typical scientific-paper
//!   table at 200 DPI.
//!
//! Implementation strategy: render the *whole page* at the target DPI
//! once, then crop the resulting bitmap to the requested BBox. This is
//! both simpler and more deterministic than feeding pdfium a custom
//! clip rectangle.

use std::io::Cursor;

use image::{ImageFormat, RgbaImage};
use pdfium_render::prelude::*;
use strata_core::BBox;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RenderError {
    #[error("pdfium render failed: {0}")]
    Pdfium(String),
    #[error("requested bbox falls entirely outside the page boundary")]
    OutOfPage,
    #[error("png encode failed: {0}")]
    Encode(String),
}

impl From<PdfiumError> for RenderError {
    fn from(value: PdfiumError) -> Self {
        Self::Pdfium(value.to_string())
    }
}

/// Default DPI used when the caller doesn't override it. 200 DPI is the
/// usual sweet spot for VLM table extraction — high enough for the model
/// to read small glyphs, low enough to keep payload under 500 KB.
pub const DEFAULT_CROP_DPI: u32 = 200;

/// Hard upper bound on the crop's edge in pixels. Keeps memory bounded
/// when an attacker submits a giant page with a tiny BBox.
const MAX_EDGE_PX: u32 = 6_000;

/// Rasterize the region of `page` enclosed by `bbox` and encode it as PNG.
///
/// `dpi` is the target rendering resolution in dots-per-inch. PDF user
/// space uses 1/72 inch per point, so the page is rendered at
/// `page_size_points × (dpi / 72.0)` pixels before cropping.
pub fn render_crop(page: &PdfPage<'_>, bbox: BBox, dpi: u32) -> Result<Vec<u8>, RenderError> {
    let dpi = dpi.max(72); // guard against ridiculous values
    let scale = dpi as f32 / 72.0;
    let page_w = page.width().value;
    let page_h = page.height().value;

    if page_w <= 0.0 || page_h <= 0.0 {
        return Err(RenderError::OutOfPage);
    }
    if bbox.x1 <= 0.0 || bbox.y1 <= 0.0 || bbox.x0 >= page_w || bbox.y0 >= page_h {
        return Err(RenderError::OutOfPage);
    }

    let target_w = ((page_w * scale).round() as u32).min(MAX_EDGE_PX).max(1);
    let target_h = ((page_h * scale).round() as u32).min(MAX_EDGE_PX).max(1);

    let config = PdfRenderConfig::new()
        .set_target_size(target_w as i32, target_h as i32);
    let bitmap = page.render_with_config(&config)?;
    let img = bitmap.as_image()?;
    let rgba: RgbaImage = img.to_rgba8();

    // PDF coords have origin at the bottom-left; image coords at the
    // top-left. Translate y0 from the bottom to the top before cropping.
    let crop_x = ((bbox.x0.max(0.0)) * scale).round() as u32;
    let crop_y_from_top = ((page_h - bbox.y1.min(page_h)).max(0.0) * scale).round() as u32;
    let crop_w = (bbox.width() * scale).round() as u32;
    let crop_h = (bbox.height() * scale).round() as u32;

    let img_w = rgba.width();
    let img_h = rgba.height();
    let crop_x = crop_x.min(img_w.saturating_sub(1));
    let crop_y = crop_y_from_top.min(img_h.saturating_sub(1));
    let crop_w = crop_w.min(img_w - crop_x).max(1);
    let crop_h = crop_h.min(img_h - crop_y).max(1);

    let cropped = image::DynamicImage::ImageRgba8(rgba)
        .crop_imm(crop_x, crop_y, crop_w, crop_h);

    let mut png = Vec::with_capacity((crop_w * crop_h) as usize);
    cropped
        .write_to(&mut Cursor::new(&mut png), ImageFormat::Png)
        .map_err(|e| RenderError::Encode(e.to_string()))?;
    Ok(png)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_dpi_is_200() {
        assert_eq!(DEFAULT_CROP_DPI, 200);
    }

    #[test]
    fn render_error_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<RenderError>();
    }
}

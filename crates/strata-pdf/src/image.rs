//! Raster-image extraction from PDF page objects.
//!
//! For every [`PdfPageObjectType::Image`] on a page we materialize the
//! decoded pixels as PNG bytes — the format the IA bridge expects when it
//! sends crops to a VLM (Plan Maestro §10.T5.4). PDF's native image stream
//! formats (Flate, DCT, JBIG2, JPX) are normalized to PNG here so the rest
//! of the pipeline only deals with one container.
//!
//! DPI is *estimated* (`ceil(pixels / (points / 72))`) because PDF doesn't
//! carry a DPI field — the same image can be rendered at any size on the
//! page.

use std::io::Cursor;

use image::{ImageFormat, RgbaImage};
use pdfium_render::prelude::*;
use serde::{Deserialize, Serialize};
use strata_core::BBox;

/// One raster image rendered to PNG bytes, ready to send to a VLM.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Image {
    pub bbox: BBox,
    /// PNG-encoded bytes. The MIME is always `"image/png"`.
    #[serde(with = "serde_bytes")]
    pub raw_bytes: Vec<u8>,
    pub mime: &'static str,
    pub dpi_estimated: u32,
    /// Pixel dimensions of the decoded raster (independent of how big the
    /// image is drawn on the page).
    pub width_px: u32,
    pub height_px: u32,
}

/// Extracts every image object from `page`. Errors from individual images
/// are logged via `tracing` and the image is skipped — one corrupt embedded
/// stream must not abort the whole page.
pub fn extract_images(page: &PdfPage<'_>) -> Result<Vec<Image>, PdfiumError> {
    let mut out = Vec::new();
    for obj in page.objects().iter() {
        let PdfPageObject::Image(img_obj) = obj else { continue };
        match image_object_to_png(&img_obj) {
            Ok(Some(img)) => out.push(img),
            Ok(None) => {} // empty / degenerate image
            Err(e) => {
                tracing::warn!(error = %e, "skipping malformed page image");
            }
        }
    }
    Ok(out)
}

fn image_object_to_png(obj: &PdfPageImageObject<'_>) -> Result<Option<Image>, PdfiumError> {
    let raster = obj.get_raw_image()?;
    let rgba: RgbaImage = raster.to_rgba8();
    let (w, h) = rgba.dimensions();
    if w == 0 || h == 0 {
        return Ok(None);
    }

    let bounds = obj.bounds()?;
    let Some(bbox) = bbox_from_quad(&bounds) else { return Ok(None) };

    let mut png = Vec::with_capacity((w * h) as usize);
    if let Err(e) = rgba.write_to(&mut Cursor::new(&mut png), ImageFormat::Png) {
        tracing::warn!(error = %e, "PNG encode failed");
        return Ok(None);
    }

    let dpi = estimate_dpi(bbox, w, h);
    Ok(Some(Image {
        bbox,
        raw_bytes: png,
        mime: "image/png",
        dpi_estimated: dpi,
        width_px: w,
        height_px: h,
    }))
}

fn bbox_from_quad(quad: &PdfQuadPoints) -> Option<BBox> {
    let xs = [quad.x1.value, quad.x2.value, quad.x3.value, quad.x4.value];
    let ys = [quad.y1.value, quad.y2.value, quad.y3.value, quad.y4.value];
    if xs.iter().chain(ys.iter()).any(|v| !v.is_finite()) {
        return None;
    }
    let min_x = xs.iter().cloned().fold(f32::INFINITY, f32::min);
    let min_y = ys.iter().cloned().fold(f32::INFINITY, f32::min);
    let max_x = xs.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let max_y = ys.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    BBox::new(min_x, min_y, max_x, max_y).ok()
}

fn estimate_dpi(bbox: BBox, width_px: u32, height_px: u32) -> u32 {
    // PDF uses 1/72 inch per point. inches = points / 72.
    let inches_w = bbox.width() / 72.0;
    let inches_h = bbox.height() / 72.0;
    let dpi_w = if inches_w > 0.0 { (width_px as f32) / inches_w } else { 0.0 };
    let dpi_h = if inches_h > 0.0 { (height_px as f32) / inches_h } else { 0.0 };
    // Take the smaller axis: a near-square render at 300dpi might say 600x600
    // on one axis if cropped, so the min is the more conservative estimate.
    dpi_w.min(dpi_h).max(0.0).round() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dpi_estimate_basic() {
        // A 300dpi A4 page is 8.27 inches wide × 300 dpi = 2480 px.
        // bbox.width = 8.27 * 72 = 595.44 pt.
        let bbox = BBox::new(0.0, 0.0, 595.44, 841.89).unwrap();
        let dpi = estimate_dpi(bbox, 2480, 3508);
        assert!((dpi as i64 - 300).abs() <= 2, "got {dpi}");
    }

    #[test]
    fn dpi_estimate_zero_when_degenerate() {
        let bbox = BBox::new(10.0, 10.0, 10.0, 50.0).unwrap();
        assert_eq!(estimate_dpi(bbox, 100, 100), 0);
    }
}

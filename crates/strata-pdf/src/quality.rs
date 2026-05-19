//! Cheap per-page heuristics that gate IA usage.
//!
//! Currently exposes [`is_likely_scan`] — a fast predicate that decides
//! whether a page is *probably* a rasterized scan rather than text with real
//! glyphs. The result drives the Triage Engine (Plan Maestro §9.T4.2): scan
//! pages skip native extraction entirely and go directly to OCR.

use pdfium_render::prelude::*;

use crate::glyph::extract_glyphs;
use crate::image::extract_images;

/// Coverage threshold above which a page is considered "image-dominated".
const IMAGE_AREA_RATIO_THRESHOLD: f32 = 0.7;
/// Glyph count below which a page is considered "text-empty".
const MIN_GLYPHS_FOR_TEXT_PAGE: usize = 10;

/// Returns `true` when the page is *probably* a scan rather than native PDF
/// text. The heuristic is intentionally conservative — false positives waste
/// OCR time, false negatives hurt fidelity more.
///
/// Rule (from Plan Maestro §7.T2.5):
///
/// ```text
///   image_area / page_area > 0.7   AND   glyph_count < 10
/// ```
pub fn is_likely_scan(page: &PdfPage<'_>) -> Result<bool, PdfiumError> {
    let page_area = page.width().value * page.height().value;
    if page_area <= 0.0 {
        return Ok(false);
    }

    let glyph_count = extract_glyphs(page)?.len();
    if glyph_count >= MIN_GLYPHS_FOR_TEXT_PAGE {
        return Ok(false);
    }

    let images = extract_images(page)?;
    let image_area: f32 = images.iter().map(|i| i.bbox.area()).sum();
    let ratio = image_area / page_area;
    Ok(ratio > IMAGE_AREA_RATIO_THRESHOLD)
}

#[cfg(test)]
mod tests {
    #[test]
    fn thresholds_are_stable() {
        assert_eq!(super::IMAGE_AREA_RATIO_THRESHOLD, 0.7);
        assert_eq!(super::MIN_GLYPHS_FOR_TEXT_PAGE, 10);
    }
}

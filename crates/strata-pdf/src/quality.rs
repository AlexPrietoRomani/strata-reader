//! Cheap per-page heuristics that gate IA usage.
//!
//! Currently exposes [`is_likely_scan`] — a fast predicate that decides
//! whether a page is *probably* a rasterized scan rather than text with real
//! glyphs. The result drives the Triage Engine (Plan Maestro §9.T4.2): scan
//! pages skip native extraction entirely and go directly to OCR.

use crate::backend::PdfPage;
use crate::decoder::DecoderError;

/// Coverage threshold above which a page is considered "image-dominated".
const IMAGE_AREA_RATIO_THRESHOLD: f32 = 0.7;
/// Glyph count below which a page is considered "text-empty".
const MIN_GLYPHS_FOR_TEXT_PAGE: usize = 10;

/// Returns `true` when the page is *probably* a scan rather than native PDF
/// text. The heuristic is intentionally conservative.
///
/// Rule (from Plan Maestro §7.T2.5):
///
/// ```text
///   image_area / page_area > 0.7   AND   glyph_count < 10
/// ```
pub fn is_likely_scan(page: &dyn PdfPage) -> Result<bool, DecoderError> {
    let (w, h) = page.size();
    let page_area = w * h;
    if page_area <= 0.0 {
        return Ok(false);
    }

    let glyph_count = page.glyphs()?.len();
    if glyph_count >= MIN_GLYPHS_FOR_TEXT_PAGE {
        return Ok(false);
    }

    let images = page.images()?;
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

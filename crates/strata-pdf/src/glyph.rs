//! Glyph extraction from a [`PdfPage`].
//!
//! Iterates the per-page text characters provided by PDFium and converts them
//! to our own [`Glyph`] type — a finite, deterministic record that downstream
//! Rust code can shuffle through R-Trees and clustering without holding on to
//! pdfium-render lifetimes.
//!
//! See Plan Maestro §7.T2.2 and `docs/task/tareas.md` A2.2.1.

use pdfium_render::prelude::*;
use serde::{Deserialize, Serialize};
use strata_core::{BBox, GeometryError};

/// One glyph extracted from a PDF page, with its bounding box already in
/// post-CTM PDF user space. All coordinates are in PDF points (1/72 inch).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Glyph {
    /// Unicode scalar. Glyphs that PDFium cannot map (e.g. CID without
    /// `ToUnicode`) come back as U+FFFD (`'\u{FFFD}'`).
    pub unicode: char,
    pub bbox: BBox,
    pub font_size: f32,
    pub font_weight: u32,
    /// Packed RGBA — useful for distinguishing watermarks / footers.
    pub color_rgba: u32,
    /// Rotation in degrees (0 for upright text).
    pub rotation: f32,
}

/// Extracts every glyph from `page` in stream order.
///
/// Returns an empty `Vec` when the page has no text page (scanned-only
/// pages). Non-finite coordinates returned by PDFium are silently filtered
/// — they cause more downstream pain than they're worth.
pub fn extract_glyphs(page: &PdfPage<'_>) -> Result<Vec<Glyph>, PdfiumError> {
    let text = match page.text() {
        Ok(t) => t,
        Err(_) => return Ok(Vec::new()),
    };
    let chars = text.chars();
    let mut out = Vec::with_capacity(chars.len());
    for ch in chars.iter() {
        let Some(glyph) = char_to_glyph(&ch) else {
            continue;
        };
        out.push(glyph);
    }
    Ok(out)
}

fn char_to_glyph(ch: &PdfPageTextChar<'_>) -> Option<Glyph> {
    let unicode = ch.unicode_char().unwrap_or('\u{FFFD}');
    let bounds = ch.tight_bounds().ok()?;
    let bbox = bbox_from_rect(&bounds)?;
    Some(Glyph {
        unicode,
        bbox,
        font_size: ch.unscaled_font_size().value,
        font_weight: ch
            .font_weight()
            .map(|w| match w {
                PdfFontWeight::Weight100 => 100,
                PdfFontWeight::Weight200 => 200,
                PdfFontWeight::Weight300 => 300,
                PdfFontWeight::Weight400Normal => 400,
                PdfFontWeight::Weight500 => 500,
                PdfFontWeight::Weight600 => 600,
                PdfFontWeight::Weight700Bold => 700,
                PdfFontWeight::Weight800 => 800,
                PdfFontWeight::Weight900 => 900,
                PdfFontWeight::Custom(val) => val,
            })
            .unwrap_or(400),
        color_rgba: pack_rgba(ch.fill_color().ok()),
        rotation: ch.get_rotation_clockwise_degrees(),
    })
}

fn bbox_from_rect(rect: &PdfRect) -> Option<BBox> {
    let x0 = rect.left().value;
    let y0 = rect.bottom().value;
    let x1 = rect.right().value;
    let y1 = rect.top().value;
    if [x0, y0, x1, y1].iter().any(|v| !v.is_finite()) {
        return None;
    }
    BBox::new(x0.min(x1), y0.min(y1), x0.max(x1), y0.max(y1)).ok()
}

fn pack_rgba(color: Option<PdfColor>) -> u32 {
    let Some(c) = color else { return 0xFF_00_00_00 }; // opaque black fallback
    let r = c.red() as u32;
    let g = c.green() as u32;
    let b = c.blue() as u32;
    let a = c.alpha() as u32;
    (r << 24) | (g << 16) | (b << 8) | a
}

// Avoid `GeometryError` warnings while it's not directly used here.
#[allow(dead_code)]
fn _silence(_e: GeometryError) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgba_default_is_opaque_black() {
        assert_eq!(pack_rgba(None), 0xFF_00_00_00);
    }

    #[test]
    fn glyph_round_trips_through_json() {
        let g = Glyph {
            unicode: 'A',
            bbox: BBox::new(0.0, 0.0, 10.0, 10.0).unwrap(),
            font_size: 12.0,
            font_weight: 400,
            color_rgba: 0xFF_00_00_00,
            rotation: 0.0,
        };
        let json = serde_json::to_string(&g).unwrap();
        let back: Glyph = serde_json::from_str(&json).unwrap();
        assert_eq!(g, back);
    }
}

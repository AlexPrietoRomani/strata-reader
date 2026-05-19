//! Vector path extraction.
//!
//! [`extract_paths`] walks the page object list, keeps only PDFium objects
//! whose type is [`PdfPageObjectType::Path`], and converts each to our
//! [`VectorPath`]. Vector paths are the input to the
//! `TableBorderProcessor` heuristic (Plan Maestro §8.T3.4) — we keep
//! enough fidelity to detect axis-aligned line segments and rectangles, no
//! more.

use pdfium_render::prelude::*;
use serde::{Deserialize, Serialize};
use strata_core::{BBox, Point};

/// One drawing command inside a [`VectorPath`].
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "kebab-case")]
pub enum Segment {
    MoveTo(Point),
    LineTo(Point),
    /// Cubic Bézier — control1, control2, end.
    CurveTo([Point; 3]),
    Close,
}

/// One vector path on a page, with stroke / fill flags and post-CTM bounds.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VectorPath {
    pub segments: Vec<Segment>,
    pub stroke: bool,
    pub fill: bool,
    pub bbox: BBox,
}

/// Extracts every vector path object from `page`.
pub fn extract_paths(page: &PdfPage<'_>) -> Result<Vec<VectorPath>, PdfiumError> {
    let mut out = Vec::new();
    for obj in page.objects().iter() {
        let PdfPageObject::Path(path_obj) = obj else { continue };
        let Some(vp) = path_object_to_vector(&path_obj)? else { continue };
        out.push(vp);
    }
    Ok(out)
}

fn path_object_to_vector(path: &PdfPagePathObject<'_>) -> Result<Option<VectorPath>, PdfiumError> {
    let bounds = path.bounds()?;
    let Some(bbox) = bbox_from_quad(&bounds) else { return Ok(None) };

    let mut segments = Vec::new();
    for seg in path.segments().iter() {
        let p = Point { x: seg.x() as f32, y: seg.y() as f32 };
        if !p.x.is_finite() || !p.y.is_finite() {
            continue;
        }
        let s = match seg.segment_type() {
            PdfPathSegmentType::MoveTo => Segment::MoveTo(p),
            PdfPathSegmentType::LineTo => Segment::LineTo(p),
            PdfPathSegmentType::BezierTo => Segment::CurveTo([p, p, p]),
            PdfPathSegmentType::Unknown => continue,
        };
        segments.push(s);
        if seg.is_close().unwrap_or(false) {
            segments.push(Segment::Close);
        }
    }

    if segments.is_empty() {
        return Ok(None);
    }

    Ok(Some(VectorPath {
        segments,
        stroke: path.stroke_color().ok().is_some(),
        fill: path.fill_color().ok().is_some(),
        bbox,
    }))
}

fn bbox_from_quad(quad: &PdfQuadPoints) -> Option<BBox> {
    let xs = [
        quad.x1.value, quad.x2.value, quad.x3.value, quad.x4.value,
    ];
    let ys = [
        quad.y1.value, quad.y2.value, quad.y3.value, quad.y4.value,
    ];
    if xs.iter().chain(ys.iter()).any(|v| !v.is_finite()) {
        return None;
    }
    let min_x = xs.iter().cloned().fold(f32::INFINITY, f32::min);
    let min_y = ys.iter().cloned().fold(f32::INFINITY, f32::min);
    let max_x = xs.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let max_y = ys.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    BBox::new(min_x, min_y, max_x, max_y).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segment_serializes_with_tagged_op() {
        let s = Segment::MoveTo(Point { x: 1.0, y: 2.0 });
        let json = serde_json::to_string(&s).unwrap();
        assert!(json.contains("\"op\":\"move-to\""), "{json}");
    }

    #[test]
    fn segment_round_trip() {
        for s in [
            Segment::MoveTo(Point { x: 0.0, y: 0.0 }),
            Segment::LineTo(Point { x: 5.0, y: 5.0 }),
            Segment::CurveTo([
                Point { x: 1.0, y: 1.0 },
                Point { x: 2.0, y: 2.0 },
                Point { x: 3.0, y: 3.0 },
            ]),
            Segment::Close,
        ] {
            let j = serde_json::to_string(&s).unwrap();
            let back: Segment = serde_json::from_str(&j).unwrap();
            assert_eq!(s, back);
        }
    }
}

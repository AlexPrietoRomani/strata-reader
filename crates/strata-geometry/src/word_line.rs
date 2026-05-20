//! Cluster individual glyphs into [`Line`]s and [`Word`]s.
//!
//! PDFium hands us glyphs in stream order — that is _draw order_, which has
//! no relation to reading order. The two routines in this module rebuild
//! the textual structure:
//!
//! 1. [`cluster_lines`] groups glyphs by similar Y-baseline. Tolerance is
//!    adaptive to the dominant font size (`0.4 × median font size`) so the
//!    same algorithm handles 8 pt footnotes and 24 pt section titles on
//!    the same page.
//! 2. [`words_from_line`] splits a single line at horizontal gaps larger
//!    than `0.3 × average glyph width` — the canonical heuristic used by
//!    PDFium itself and most PDF text extractors.
//!
//! Both functions operate on a generic [`GlyphInput`] PoD so the caller can
//! adapt their representation (e.g. `strata-pdf::Glyph`) without coupling
//! crates.
//!
//! See Plan Maestro §8.T3.2.

use serde::{Deserialize, Serialize};
use strata_core::BBox;

/// Minimal glyph view consumed by the clustering routines. Designed to be
/// trivially built from `strata_pdf::Glyph` without taking that crate as a
/// dependency.
#[derive(Copy, Clone, Debug)]
pub struct GlyphInput {
    pub bbox: BBox,
    /// Unscaled font size in PDF points.
    pub font_size: f32,
    /// Unicode scalar (use `'\u{FFFD}'` if unknown).
    pub unicode: char,
}

impl GlyphInput {
    fn baseline_y(&self) -> f32 {
        // For latin scripts the baseline sits at ~y0 (bottom of the BBox);
        // PDF coordinates are bottom-left so y0 is the lower edge.
        self.bbox.y0
    }
}

/// A horizontal run of glyphs sharing approximately the same baseline.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Line {
    pub bbox: BBox,
    pub baseline_y: f32,
    /// Indices into the original glyph slice, sorted left-to-right.
    pub glyph_indices: Vec<usize>,
}

/// A word — a contiguous run of glyphs inside a [`Line`] not separated by a
/// large horizontal gap.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Word {
    pub bbox: BBox,
    /// Indices into the original glyph slice, sorted left-to-right.
    pub glyph_indices: Vec<usize>,
    /// Concatenated unicode of the contained glyphs.
    pub text: String,
}

const LINE_TOLERANCE_FACTOR: f32 = 0.4;
const WORD_GAP_FACTOR: f32 = 0.3;

/// Group `glyphs` into [`Line`]s by clustering similar Y-baselines.
///
/// Tolerance is `LINE_TOLERANCE_FACTOR (= 0.4) × median(font_size)`. Lines
/// come back sorted from top of page to bottom (PDF coords: high-Y first).
pub fn cluster_lines(glyphs: &[GlyphInput]) -> Vec<Line> {
    if glyphs.is_empty() {
        return Vec::new();
    }
    let tolerance = LINE_TOLERANCE_FACTOR * median_font_size(glyphs);

    // Sort glyph indices by baseline, descending (top of page first).
    let mut order: Vec<usize> = (0..glyphs.len()).collect();
    order.sort_by(|&a, &b| {
        glyphs[b]
            .baseline_y()
            .partial_cmp(&glyphs[a].baseline_y())
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut lines: Vec<Line> = Vec::new();
    for idx in order {
        let g = glyphs[idx];
        let pushed = lines
            .last_mut()
            .filter(|line| (line.baseline_y - g.baseline_y()).abs() <= tolerance)
            .map(|line| {
                line.glyph_indices.push(idx);
                line.bbox = line.bbox.union(g.bbox);
                line.baseline_y =
                    running_mean(line.baseline_y, g.baseline_y(), line.glyph_indices.len());
            });
        if pushed.is_none() {
            lines.push(Line {
                bbox: g.bbox,
                baseline_y: g.baseline_y(),
                glyph_indices: vec![idx],
            });
        }
    }

    // Sort each line's glyph_indices left-to-right.
    for line in &mut lines {
        line.glyph_indices.sort_by(|&a, &b| {
            glyphs[a]
                .bbox
                .x0
                .partial_cmp(&glyphs[b].bbox.x0)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    lines
}

/// Split a `line` (already sorted left-to-right) at horizontal gaps larger
/// than `WORD_GAP_FACTOR (= 0.3) × average glyph width`.
pub fn words_from_line(line: &Line, glyphs: &[GlyphInput]) -> Vec<Word> {
    if line.glyph_indices.is_empty() {
        return Vec::new();
    }
    let avg_w = average_glyph_width(line, glyphs);
    let gap_threshold = WORD_GAP_FACTOR * avg_w;

    let mut words: Vec<Word> = Vec::new();
    let mut current: Vec<usize> = Vec::new();
    let mut current_bbox: Option<BBox> = None;
    let mut prev_right: Option<f32> = None;

    for &idx in &line.glyph_indices {
        let g = glyphs[idx];
        let gap = match prev_right {
            Some(r) => g.bbox.x0 - r,
            None => 0.0,
        };
        if gap > gap_threshold && !current.is_empty() {
            words.push(close_word(
                &current,
                current_bbox.expect("non-empty"),
                glyphs,
            ));
            current.clear();
            current_bbox = None;
        }
        current.push(idx);
        current_bbox = Some(match current_bbox {
            Some(b) => b.union(g.bbox),
            None => g.bbox,
        });
        prev_right = Some(g.bbox.x1);
    }
    if let Some(b) = current_bbox {
        words.push(close_word(&current, b, glyphs));
    }
    words
}

fn close_word(indices: &[usize], bbox: BBox, glyphs: &[GlyphInput]) -> Word {
    let text: String = indices.iter().map(|&i| glyphs[i].unicode).collect();
    Word {
        bbox,
        glyph_indices: indices.to_vec(),
        text,
    }
}

fn median_font_size(glyphs: &[GlyphInput]) -> f32 {
    let mut sizes: Vec<f32> = glyphs
        .iter()
        .map(|g| g.font_size)
        .filter(|s| s.is_finite() && *s > 0.0)
        .collect();
    if sizes.is_empty() {
        return 1.0; // arbitrary non-zero default
    }
    sizes.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    sizes[sizes.len() / 2]
}

fn average_glyph_width(line: &Line, glyphs: &[GlyphInput]) -> f32 {
    if line.glyph_indices.is_empty() {
        return 1.0;
    }
    let total: f32 = line
        .glyph_indices
        .iter()
        .map(|&i| glyphs[i].bbox.width())
        .sum();
    (total / line.glyph_indices.len() as f32).max(0.001)
}

fn running_mean(current: f32, new: f32, n: usize) -> f32 {
    // Iterative arithmetic mean for streaming updates.
    current + (new - current) / (n as f32)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn glyph(unicode: char, x0: f32, y0: f32, w: f32, size: f32) -> GlyphInput {
        GlyphInput {
            bbox: BBox::new(x0, y0, x0 + w, y0 + size).unwrap(),
            font_size: size,
            unicode,
        }
    }

    #[test]
    fn empty_input_yields_no_lines() {
        let lines = cluster_lines(&[]);
        assert!(lines.is_empty());
    }

    #[test]
    fn six_glyphs_on_two_baselines() {
        // Line at y=100 ("abc"), line at y=80 ("def"), font size 10.
        let glyphs = vec![
            glyph('a', 0.0, 100.0, 8.0, 10.0),
            glyph('b', 9.0, 100.0, 8.0, 10.0),
            glyph('c', 18.0, 100.0, 8.0, 10.0),
            glyph('d', 0.0, 80.0, 8.0, 10.0),
            glyph('e', 9.0, 80.0, 8.0, 10.0),
            glyph('f', 18.0, 80.0, 8.0, 10.0),
        ];
        let lines = cluster_lines(&glyphs);
        assert_eq!(lines.len(), 2);
        // Top of page first.
        assert!(lines[0].baseline_y > lines[1].baseline_y);
        // 3 glyphs each.
        assert_eq!(lines[0].glyph_indices.len(), 3);
        assert_eq!(lines[1].glyph_indices.len(), 3);
    }

    #[test]
    fn jitter_under_tolerance_collapses_to_one_line() {
        // Glyphs at y=100, 100.5, 101 with font 10 → tolerance = 4 → all one line.
        let glyphs = vec![
            glyph('a', 0.0, 100.0, 8.0, 10.0),
            glyph('b', 9.0, 100.5, 8.0, 10.0),
            glyph('c', 18.0, 101.0, 8.0, 10.0),
        ];
        let lines = cluster_lines(&glyphs);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].glyph_indices, vec![0, 1, 2]);
    }

    #[test]
    fn words_split_on_large_gap() {
        // "ab  cd" — gap between 'b' and 'c' is huge.
        let glyphs = vec![
            glyph('a', 0.0, 100.0, 8.0, 10.0),
            glyph('b', 9.0, 100.0, 8.0, 10.0),
            glyph('c', 60.0, 100.0, 8.0, 10.0), // big gap
            glyph('d', 69.0, 100.0, 8.0, 10.0),
        ];
        let lines = cluster_lines(&glyphs);
        let words = words_from_line(&lines[0], &glyphs);
        assert_eq!(words.len(), 2);
        assert_eq!(words[0].text, "ab");
        assert_eq!(words[1].text, "cd");
    }

    #[test]
    fn small_gap_keeps_single_word() {
        // "ab" with intra-glyph 1 pt gap (well below threshold).
        let glyphs = vec![
            glyph('a', 0.0, 100.0, 8.0, 10.0),
            glyph('b', 9.0, 100.0, 8.0, 10.0),
        ];
        let lines = cluster_lines(&glyphs);
        let words = words_from_line(&lines[0], &glyphs);
        assert_eq!(words.len(), 1);
        assert_eq!(words[0].text, "ab");
    }
}

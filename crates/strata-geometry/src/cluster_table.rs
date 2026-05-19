//! Heuristic detector for borderless tables (e.g. LaTeX `booktabs`).
//!
//! Borderless tables have **no vector primitives** — only text laid out in
//! aligned columns. The trick is to spot vertical columns by clustering word
//! left-edges (and right-edges, for ragged-right columns) along the X axis,
//! then validate that those columns share enough vertical extent to look like
//! a single table region.
//!
//! The output is a list of *suspicious* regions; the actual table extraction
//! is delegated to the IA layer (VLM) in Fase 5. This module is intentionally
//! permissive — false negatives hurt fidelity more than false positives.
//!
//! See Plan Maestro §8.T3.5.

use serde::{Deserialize, Serialize};
use strata_core::BBox;

use crate::word_line::Word;

/// One bounding region that *might* contain a borderless table. The IA
/// router decides whether to dispatch to the VLM table extractor.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BorderlessCandidate {
    pub bbox: BBox,
    /// Number of column-anchors detected inside the region.
    pub columns: usize,
    /// Number of distinct rows (lines) participating in the region.
    pub rows: usize,
    /// `[0.0, 1.0]` heuristic score — caller can use it for ranking.
    pub score: f32,
}

/// Minimum number of words sharing the same left-edge X (within tolerance)
/// for the X coordinate to be considered a column anchor.
const MIN_WORDS_PER_COLUMN: usize = 3;
/// Tolerance, in PDF points, used when grouping words with similar X.
const X_ALIGN_TOLERANCE_PT: f32 = 2.0;
/// At least this many column-anchors with this many shared rows are needed
/// before we emit a candidate.
const MIN_COLUMNS: usize = 2;
const MIN_ROWS: usize = 2;

/// Detect candidate borderless-table regions among `words`.
pub fn detect_table_candidates(words: &[Word]) -> Vec<BorderlessCandidate> {
    if words.len() < MIN_WORDS_PER_COLUMN * MIN_COLUMNS {
        return Vec::new();
    }
    let anchors = compute_column_anchors(words);
    if anchors.len() < MIN_COLUMNS {
        return Vec::new();
    }
    // Build a per-row signature: which anchors are populated on that line?
    let rows = group_words_by_line(words);
    if rows.len() < MIN_ROWS {
        return Vec::new();
    }
    let participating: Vec<&LineGroup> =
        rows.iter().filter(|r| count_hits(&anchors, r) >= MIN_COLUMNS).collect();
    if participating.len() < MIN_ROWS {
        return Vec::new();
    }
    let region = bounding_region(&participating, &anchors);
    let Some(bbox) = region else { return Vec::new() };

    let score = score_candidate(&anchors, &participating);
    vec![BorderlessCandidate {
        bbox,
        columns: anchors.len(),
        rows: participating.len(),
        score,
    }]
}

#[derive(Debug)]
struct ColumnAnchor {
    x_center: f32,
    members: Vec<usize>, // word indices
}

#[derive(Debug)]
struct LineGroup {
    y: f32,
    bbox: BBox,
    members: Vec<usize>, // word indices
}

fn compute_column_anchors(words: &[Word]) -> Vec<ColumnAnchor> {
    let mut by_x: Vec<(f32, usize)> = words.iter().enumerate().map(|(i, w)| (w.bbox.x0, i)).collect();
    by_x.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    let mut anchors: Vec<ColumnAnchor> = Vec::new();
    for (x, idx) in by_x {
        match anchors.last_mut() {
            Some(a) if (a.x_center - x).abs() <= X_ALIGN_TOLERANCE_PT => {
                let n = a.members.len() as f32;
                a.x_center = a.x_center + (x - a.x_center) / (n + 1.0);
                a.members.push(idx);
            }
            _ => anchors.push(ColumnAnchor { x_center: x, members: vec![idx] }),
        }
    }
    anchors.retain(|a| a.members.len() >= MIN_WORDS_PER_COLUMN);
    anchors
}

fn group_words_by_line(words: &[Word]) -> Vec<LineGroup> {
    if words.is_empty() {
        return Vec::new();
    }
    let mut by_y: Vec<usize> = (0..words.len()).collect();
    by_y.sort_by(|&a, &b| {
        words[b]
            .bbox
            .center()
            .y
            .partial_cmp(&words[a].bbox.center().y)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let tol = median_line_height(words) * 0.4;

    let mut groups: Vec<LineGroup> = Vec::new();
    for idx in by_y {
        let w = &words[idx];
        let y = w.bbox.center().y;
        match groups.last_mut() {
            Some(g) if (g.y - y).abs() <= tol => {
                g.members.push(idx);
                g.bbox = g.bbox.union(w.bbox);
                let n = g.members.len() as f32;
                g.y = g.y + (y - g.y) / n;
            }
            _ => groups.push(LineGroup { y, bbox: w.bbox, members: vec![idx] }),
        }
    }
    groups
}

fn median_line_height(words: &[Word]) -> f32 {
    let mut hs: Vec<f32> = words.iter().map(|w| w.bbox.height()).filter(|h| h.is_finite() && *h > 0.0).collect();
    if hs.is_empty() {
        return 1.0;
    }
    hs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    hs[hs.len() / 2]
}

fn count_hits(anchors: &[ColumnAnchor], row: &LineGroup) -> usize {
    anchors.iter().filter(|a| row.members.iter().any(|_| true) && row_has_anchor(a, row)).count()
}

fn row_has_anchor(anchor: &ColumnAnchor, row: &LineGroup) -> bool {
    row.members.iter().any(|i| anchor.members.contains(i))
}

fn bounding_region(rows: &[&LineGroup], anchors: &[ColumnAnchor]) -> Option<BBox> {
    if rows.is_empty() || anchors.is_empty() {
        return None;
    }
    let min_x = anchors.iter().map(|a| a.x_center).fold(f32::INFINITY, f32::min);
    let max_x = anchors.iter().map(|a| a.x_center).fold(f32::NEG_INFINITY, f32::max);
    let min_y = rows.iter().map(|r| r.bbox.y0).fold(f32::INFINITY, f32::min);
    let max_y = rows.iter().map(|r| r.bbox.y1).fold(f32::NEG_INFINITY, f32::max);
    BBox::new(min_x, min_y, max_x, max_y).ok()
}

fn score_candidate(anchors: &[ColumnAnchor], rows: &[&LineGroup]) -> f32 {
    // Density: fraction of (column × row) cells that are populated.
    let total = (anchors.len() * rows.len()) as f32;
    if total <= 0.0 {
        return 0.0;
    }
    let mut populated = 0u32;
    for a in anchors {
        for r in rows {
            if row_has_anchor(a, r) {
                populated += 1;
            }
        }
    }
    (populated as f32 / total).clamp(0.0, 1.0)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn word(text: &str, x0: f32, y0: f32, w: f32) -> Word {
        Word {
            bbox: BBox::new(x0, y0, x0 + w, y0 + 10.0).unwrap(),
            glyph_indices: vec![],
            text: text.to_string(),
        }
    }

    #[test]
    fn no_words_yields_no_candidate() {
        assert!(detect_table_candidates(&[]).is_empty());
    }

    #[test]
    fn paragraph_text_yields_no_candidate() {
        // Words flowing left-to-right with normal inter-word gaps and no
        // column alignment.
        let mut words = Vec::new();
        for row in 0..3 {
            let y = 100.0 - (row as f32) * 12.0;
            for col in 0..5 {
                let x = 0.0 + (col as f32) * 25.0 + (row as f32) * 7.0; // shifted per row
                words.push(word("w", x, y, 20.0));
            }
        }
        assert!(detect_table_candidates(&words).is_empty());
    }

    #[test]
    fn aligned_columns_yield_candidate() {
        // 4 rows × 3 columns at fixed X positions → should fire.
        let xs = [10.0, 80.0, 150.0];
        let mut words = Vec::new();
        for row in 0..4 {
            let y = 100.0 - (row as f32) * 12.0;
            for &x in &xs {
                words.push(word("v", x, y, 30.0));
            }
        }
        let cands = detect_table_candidates(&words);
        assert_eq!(cands.len(), 1);
        let c = &cands[0];
        assert_eq!(c.columns, 3);
        assert_eq!(c.rows, 4);
        assert!(c.score > 0.9);
    }

    #[test]
    fn single_column_does_not_fire() {
        let mut words = Vec::new();
        for row in 0..4 {
            let y = 100.0 - (row as f32) * 12.0;
            words.push(word("v", 10.0, y, 50.0));
        }
        assert!(detect_table_candidates(&words).is_empty());
    }
}

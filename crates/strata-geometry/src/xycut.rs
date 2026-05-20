//! XY-Cut++ reading-order algorithm.
//!
//! See ADR 0001 (`docs/adr/0001-xycut.md`) for pseudocode and rationale.
//! Plan Maestro reference: §8.T3.3.

use serde::{Deserialize, Serialize};
use strata_core::BBox;

/// Cartesian axis. PDF coordinates have +Y pointing up.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Axis {
    X,
    Y,
}

impl Axis {
    fn flip(self) -> Self {
        match self {
            Self::X => Self::Y,
            Self::Y => Self::X,
        }
    }
}

/// Direction of the secondary axis (typically the writing system).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ScriptDirection {
    /// Left-to-right, top-to-bottom (Latin, Han, Hangul, ...).
    Ltr,
    /// Right-to-left, top-to-bottom (Arabic, Hebrew, ...).
    Rtl,
}

/// Configurable thresholds. Defaults follow ADR 0001.
#[derive(Copy, Clone, Debug)]
pub struct XyCutConfig {
    /// Minimum gap, as a fraction of the median block height, that counts as
    /// a real cut. Below this, the algorithm falls back to single-axis sort.
    pub min_gap_factor: f32,
    /// Script direction used to order siblings on the X axis.
    pub script: ScriptDirection,
}

impl Default for XyCutConfig {
    fn default() -> Self {
        Self { min_gap_factor: 0.3, script: ScriptDirection::Ltr }
    }
}

/// Compute the reading order of `blocks` and return the resulting permutation
/// of indices into `blocks`. The vector length always matches `blocks.len()`.
pub fn xy_cut_plus_plus(blocks: &[BBox], config: XyCutConfig) -> Vec<usize> {
    if blocks.is_empty() {
        return Vec::new();
    }
    if blocks.len() == 1 {
        return vec![0];
    }
    let min_gap = config.min_gap_factor * median_height(blocks);
    let initial: Vec<usize> = (0..blocks.len()).collect();
    let mut out = Vec::with_capacity(blocks.len());
    cut_recursive(&initial, blocks, Axis::Y, min_gap, config.script, &mut out);
    debug_assert_eq!(out.len(), blocks.len(), "every block must be emitted exactly once");
    out
}

fn cut_recursive(
    indices: &[usize],
    blocks: &[BBox],
    axis: Axis,
    min_gap: f32,
    script: ScriptDirection,
    out: &mut Vec<usize>,
) {
    if indices.len() <= 1 {
        out.extend(indices.iter().copied());
        return;
    }

    if let Some(cut) = find_largest_gap(indices, blocks, axis, min_gap, script) {
        let (a, b) = partition(indices, blocks, axis, cut);
        // Emit blocks in reading order: top-of-page-first for Y, script-order for X.
        let (first, second) = order_partitions(a, b, axis, blocks, script);
        cut_recursive(&first, blocks, axis.flip(), min_gap, script, out);
        cut_recursive(&second, blocks, axis.flip(), min_gap, script, out);
        return;
    }

    // No gap on this axis above the threshold — try the other one.
    let other = axis.flip();
    if let Some(cut) = find_largest_gap(indices, blocks, other, min_gap, script) {
        let (a, b) = partition(indices, blocks, other, cut);
        let (first, second) = order_partitions(a, b, other, blocks, script);
        cut_recursive(&first, blocks, other.flip(), min_gap, script, out);
        cut_recursive(&second, blocks, other.flip(), min_gap, script, out);
        return;
    }

    // No further cuts possible: emit in fallback order
    // (Y desc primary, X per script direction secondary).
    let mut sorted = indices.to_vec();
    sorted.sort_by(|&i, &j| {
        let a = blocks[i].center();
        let b = blocks[j].center();
        // Higher Y first
        match b.y.partial_cmp(&a.y).unwrap_or(std::cmp::Ordering::Equal) {
            std::cmp::Ordering::Equal => match script {
                ScriptDirection::Ltr => a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal),
                ScriptDirection::Rtl => b.x.partial_cmp(&a.x).unwrap_or(std::cmp::Ordering::Equal),
            },
            ord => ord,
        }
    });
    out.extend(sorted);
}

#[derive(Copy, Clone, Debug)]
struct Gap {
    /// Coordinate value at which to cut (midpoint of the gap).
    cut: f32,
    width: f32,
}

fn find_largest_gap(
    indices: &[usize],
    blocks: &[BBox],
    axis: Axis,
    min_gap: f32,
    script: ScriptDirection,
) -> Option<f32> {
    if indices.is_empty() {
        return None;
    }
    // Build a sorted list of `(lo, hi)` extents along the axis.
    let mut extents: Vec<(f32, f32)> = indices
        .iter()
        .map(|&i| match axis {
            Axis::X => (blocks[i].x0, blocks[i].x1),
            Axis::Y => (blocks[i].y0, blocks[i].y1),
        })
        .collect();
    extents.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    // Sweep: maintain the running maximum of `hi` and look for the next
    // `lo` that exceeds it by more than `min_gap`.
    let mut max_hi = extents[0].1;
    let mut best: Option<Gap> = None;
    for &(lo, hi) in &extents[1..] {
        if lo > max_hi {
            let w = lo - max_hi;
            if w >= min_gap {
                let cut_val = (lo + max_hi) * 0.5;
                let should_update = match best {
                    None => true,
                    Some(g) => {
                        if (w - g.width).abs() < 1e-4 {
                            match axis {
                                Axis::Y => {
                                    // Y-axis: prefer the higher cut (top of page first)
                                    cut_val > g.cut
                                }
                                Axis::X => match script {
                                    ScriptDirection::Ltr => {
                                        // X-axis Ltr: prefer the smaller cut (leftmost first)
                                        false
                                    }
                                    ScriptDirection::Rtl => {
                                        // X-axis Rtl: prefer the larger cut (rightmost first)
                                        cut_val > g.cut
                                    }
                                },
                            }
                        } else {
                            w > g.width
                        }
                    }
                };
                if should_update {
                    best = Some(Gap { cut: cut_val, width: w });
                }
            }
        }
        if hi > max_hi {
            max_hi = hi;
        }
    }
    best.map(|g| g.cut)
}

fn partition(indices: &[usize], blocks: &[BBox], axis: Axis, cut: f32) -> (Vec<usize>, Vec<usize>) {
    let mut a = Vec::new();
    let mut b = Vec::new();
    for &i in indices {
        let mid = match axis {
            Axis::X => blocks[i].center().x,
            Axis::Y => blocks[i].center().y,
        };
        if mid < cut {
            a.push(i);
        } else {
            b.push(i);
        }
    }
    (a, b)
}

fn order_partitions(
    a: Vec<usize>,
    b: Vec<usize>,
    axis: Axis,
    blocks: &[BBox],
    script: ScriptDirection,
) -> (Vec<usize>, Vec<usize>) {
    match (axis, script) {
        // Y cut: higher-Y first (top of page).
        (Axis::Y, _) => {
            let a_y = a.iter().map(|&i| blocks[i].center().y).fold(f32::NEG_INFINITY, f32::max);
            let b_y = b.iter().map(|&i| blocks[i].center().y).fold(f32::NEG_INFINITY, f32::max);
            if a_y >= b_y { (a, b) } else { (b, a) }
        }
        // X cut: depends on script direction.
        (Axis::X, ScriptDirection::Ltr) => {
            let a_x = a.iter().map(|&i| blocks[i].center().x).fold(f32::INFINITY, f32::min);
            let b_x = b.iter().map(|&i| blocks[i].center().x).fold(f32::INFINITY, f32::min);
            if a_x <= b_x { (a, b) } else { (b, a) }
        }
        (Axis::X, ScriptDirection::Rtl) => {
            let a_x = a.iter().map(|&i| blocks[i].center().x).fold(f32::NEG_INFINITY, f32::max);
            let b_x = b.iter().map(|&i| blocks[i].center().x).fold(f32::NEG_INFINITY, f32::max);
            if a_x >= b_x { (a, b) } else { (b, a) }
        }
    }
}

fn median_height(blocks: &[BBox]) -> f32 {
    let mut heights: Vec<f32> = blocks.iter().map(|b| b.height()).filter(|h| h.is_finite() && *h > 0.0).collect();
    if heights.is_empty() {
        return 1.0;
    }
    heights.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    heights[heights.len() / 2]
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn bb(x0: f32, y0: f32, x1: f32, y1: f32) -> BBox {
        BBox::new(x0, y0, x1, y1).unwrap()
    }

    #[test]
    fn empty_returns_empty() {
        let r = xy_cut_plus_plus(&[], XyCutConfig::default());
        assert!(r.is_empty());
    }

    #[test]
    fn single_block_returns_itself() {
        let r = xy_cut_plus_plus(&[bb(0.0, 0.0, 10.0, 10.0)], XyCutConfig::default());
        assert_eq!(r, vec![0]);
    }

    #[test]
    fn two_blocks_stacked_top_first() {
        // Two blocks at y=100 (top) and y=50 (bottom). Expect top → bottom.
        let blocks = vec![
            bb(0.0, 50.0, 100.0, 60.0), // bottom
            bb(0.0, 100.0, 100.0, 110.0), // top
        ];
        let order = xy_cut_plus_plus(&blocks, XyCutConfig::default());
        assert_eq!(order, vec![1, 0]);
    }

    #[test]
    fn two_column_layout_left_then_right() {
        // Header at top spanning both columns.
        // Two columns below: left at x=0, right at x=150. Each column has
        // two stacked blocks.
        let blocks = vec![
            bb(0.0, 200.0, 300.0, 210.0), // 0: header
            bb(0.0, 150.0, 100.0, 180.0), // 1: left-col top
            bb(0.0, 100.0, 100.0, 130.0), // 2: left-col bottom
            bb(150.0, 150.0, 250.0, 180.0), // 3: right-col top
            bb(150.0, 100.0, 250.0, 130.0), // 4: right-col bottom
        ];
        let order = xy_cut_plus_plus(&blocks, XyCutConfig::default());
        // Expected: header, then left col top→bot, then right col top→bot.
        assert_eq!(order, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn rtl_two_column_right_first() {
        let blocks = vec![
            bb(0.0, 150.0, 100.0, 180.0), // 0: left-col
            bb(150.0, 150.0, 250.0, 180.0), // 1: right-col
        ];
        let order = xy_cut_plus_plus(
            &blocks,
            XyCutConfig { min_gap_factor: 0.3, script: ScriptDirection::Rtl },
        );
        assert_eq!(order, vec![1, 0]);
    }

    #[test]
    fn permutation_is_complete_and_unique() {
        // Random-ish layout: every input block must appear exactly once.
        let blocks = vec![
            bb(10.0, 10.0, 20.0, 20.0),
            bb(30.0, 50.0, 40.0, 60.0),
            bb(70.0, 30.0, 80.0, 40.0),
            bb(15.0, 80.0, 25.0, 90.0),
            bb(60.0, 80.0, 70.0, 90.0),
        ];
        let order = xy_cut_plus_plus(&blocks, XyCutConfig::default());
        assert_eq!(order.len(), blocks.len());
        let mut sorted = order.clone();
        sorted.sort();
        assert_eq!(sorted, vec![0, 1, 2, 3, 4]);
    }
}

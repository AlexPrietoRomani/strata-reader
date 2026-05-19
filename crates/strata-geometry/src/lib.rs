//! Strata-Geometry — pure spatial algorithms (no PDF, no IA).
//!
//! Public surface:
//!
//! - [`SpatialIndex`] — 2-D R-Tree over `(BBox, payload)` pairs.
//! - [`cluster_lines`] + [`words_from_line`] — glyph → Line → Word.
//! - [`xy_cut_plus_plus`] — XY-Cut++ reading-order algorithm (ADR 0001).
//! - [`detect_table_borders`] — bordered tables from path intersections.
//! - [`detect_table_candidates`] — borderless tables by word alignment.
//! - [`classify_headings`] — font-size based heading-level classification.
//!
//! Every module is parametric on simple PoD inputs so any caller — including
//! the strata-pdf glyph layer and the future serializer — can adapt their
//! own types without dragging in cross-crate dependencies.
//!
//! See `docs/plan/plan_maestro.md` §8.

#![deny(rust_2018_idioms)]

pub mod cluster_table;
pub mod headings;
pub mod rtree_index;
pub mod table_border;
pub mod word_line;
pub mod xycut;

pub use cluster_table::{detect_table_candidates, BorderlessCandidate};
pub use headings::{classify_headings, HeadingClass};
pub use rtree_index::{Hit, SpatialIndex};
pub use table_border::{detect_table_borders, LineSegment, TableCandidate};
pub use word_line::{cluster_lines, words_from_line, GlyphInput, Line, Word};
pub use xycut::{xy_cut_plus_plus, Axis, ScriptDirection, XyCutConfig};

/// Crate semver.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    #[test]
    fn version_matches_pkg() {
        assert_eq!(super::version(), env!("CARGO_PKG_VERSION"));
    }
}

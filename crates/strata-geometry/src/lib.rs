//! Strata-Geometry — pure spatial algorithms (no PDF, no IA).
//!
//! See `docs/plan/plan_maestro.md` §8.

#![deny(rust_2018_idioms)]

pub mod rtree_index;
pub mod table_border;
pub mod word_line;
pub mod xycut;

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

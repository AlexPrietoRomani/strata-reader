//! Strata-Geometry — pure spatial algorithms (no PDF, no IA).
//!
//! See `docs/plan/plan_maestro.md` §8.

#![deny(rust_2018_idioms)]

pub mod rtree_index;

pub use rtree_index::{Hit, SpatialIndex};

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

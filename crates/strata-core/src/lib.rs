//! Strata-Reader Core: AST primitives.
//!
//! This crate intentionally exposes only `version()` until Phase 1 is started.
//! See `docs/plan/plan_maestro.md` §6.

#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]

/// Returns the semver of this crate (from `CARGO_PKG_VERSION`).
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_matches_pkg() {
        assert_eq!(version(), env!("CARGO_PKG_VERSION"));
        assert!(!version().is_empty());
    }
}

//! Strata-Quality — per-page diagnostic detectors.
//!
//! Currently exposes:
//!
//! - [`Severity`] / [`CidEvaluation`] / [`evaluate_cid_health`] — detects
//!   pages whose text suffers from broken CID-to-Unicode mappings (Plan
//!   Maestro §9.T4.1).
//!
//! The crate is intentionally pure-Rust — extractors live in `strata-pdf`,
//! we just analyse their output. Future modules (font validation,
//! scan signals) will follow the same convention.

#![deny(rust_2018_idioms)]

pub mod cid_detector;

pub use cid_detector::{evaluate_cid_health, CidEvaluation, Severity};

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

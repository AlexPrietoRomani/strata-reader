//! Strata-Triage — per-block routing decisions and IA crop rendering.
//!
//! See `docs/plan/plan_maestro.md` §9.

#![deny(rust_2018_idioms)]

pub mod decision;
pub mod profiles;

pub use decision::{Reason, TriageDecision, TriageRoute};
pub use profiles::{ProfileName, TriageProfile};

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

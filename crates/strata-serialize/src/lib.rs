//! Strata-Serialize — Markdown (Vector-RAG) and JSON Graph-RAG output.
//!
//! See `docs/plan/plan_maestro.md` §12.

#![deny(rust_2018_idioms)]

pub mod markdown;

pub use markdown::{render as render_markdown, ImageStrategy, MarkdownOptions};

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

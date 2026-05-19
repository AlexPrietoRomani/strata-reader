//! Strata-Fusion — re-assembly of the native AST with IA payloads, plus
//! the section-tree builder and the semantic chunker for RAG.
//!
//! See `docs/plan/plan_maestro.md` §12.

#![deny(rust_2018_idioms)]

pub mod chunker;
pub mod fuser;
pub mod sections;

pub use chunker::{chunk, Chunk, ChunkOptions};
pub use fuser::{merge, validate, FusionError, IaPayload};
pub use sections::{build_tree, Section, SectionChild};

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

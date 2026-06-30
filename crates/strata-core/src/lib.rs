//! Strata-Reader Core — AST primitives (BBox, Block, Page, Document) plus
//! Provenance metadata for PRISMA traceability.
//!
//! See `docs/plan/plan_maestro.md` §6 for the full specification.

#![forbid(unsafe_code)]
#![deny(rust_2018_idioms)]

pub mod bbox;
pub mod block;
pub mod document;
pub mod geometry;
pub mod page;
pub mod provenance;

pub use bbox::{BBox, GeometryError, Point, Size};
pub use block::{Block, BlockId, BlockMetadata, BlockType};
pub use document::{DocMeta, Document};
pub use geometry::Matrix;
pub use page::{Page, PageOrientation};
pub use provenance::{Provenance, ProvenanceError, ProvenanceSource};

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
    }

    #[test]
    fn re_exports_are_reachable() {
        let bbox = BBox::new(0.0, 0.0, 10.0, 10.0).unwrap();
        let _: Point = bbox.center();
        let _: Matrix = Matrix::IDENTITY;
        let _: Provenance = Provenance::rust_native();
        let _: BlockId = BlockId::from_u128(1);
    }
}

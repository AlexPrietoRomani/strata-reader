//! Strata-IA-Bridge — Rust client for the Python IA microservice over gRPC.
//!
//! Public surface:
//!
//! - [`proto`] — generated `strata.ia.v1` protobuf types and the
//!   `IaServiceClient` stub (built at compile time by `build.rs`).
//! - [`BridgeClient`] — high-level wrapper around the gRPC stub with a
//!   re-usable channel, retries, and typed errors.
//! - [`BridgeError`] — closed enum so callers can translate distinct
//!   failure modes into PRISMA-style provenance.
//!
//! See `docs/plan/plan_maestro.md` §11.

#![deny(rust_2018_idioms)]

/// Generated stubs for the `strata.ia.v1` proto contract.
///
/// The build script writes both the Rust modules and a
/// `strata_ia_descriptor.bin` FileDescriptorSet so callers can enable
/// gRPC reflection when needed.
#[allow(clippy::all, missing_docs, unused_qualifications)]
pub mod proto {
    tonic::include_proto!("strata.ia.v1");

    /// Embedded FileDescriptorSet for gRPC reflection.
    pub const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("strata_ia_descriptor");
}

pub mod client;
pub mod error;

pub use client::{BridgeClient, BridgeClientConfig};
pub use error::BridgeError;

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

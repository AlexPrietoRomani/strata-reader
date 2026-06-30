//! Strata-Server — `axum` HTTP/REST microservice. See Plan Maestro §14.

#![deny(rust_2018_idioms)]

pub mod jobs;
pub mod routes;
pub mod state;
pub mod store;
pub mod worker;

pub use jobs::{Job, JobId, JobStatus, JobStore, JobStoreError};
pub use routes::router;
pub use state::AppState;
pub use store::{MemoryJobStore, SqliteJobStore};
pub use worker::BackgroundWorker;

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

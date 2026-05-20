//! `strata-server` binary entry point.
//!
//! `strata-server` reads the bind address from `STRATA_SERVER_BIND`
//! (default `0.0.0.0:8080`) and the store backend from
//! `STRATA_SERVER_STORE` (`memory` or `sqlite:/path/to/file.db`,
//! default `memory`).

#![deny(rust_2018_idioms)]

use std::sync::Arc;

use strata_runtime::Metrics;
use strata_server::{AppState, JobStore, MemoryJobStore, SqliteJobStore};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();
    let bind: String = std::env::var("STRATA_SERVER_BIND").unwrap_or_else(|_| "0.0.0.0:8080".into());
    let store_spec: String = std::env::var("STRATA_SERVER_STORE").unwrap_or_else(|_| "memory".into());

    let store: Arc<dyn JobStore> = match store_spec.as_str() {
        "memory" => Arc::new(MemoryJobStore::new()),
        spec if spec.starts_with("sqlite:") => {
            let path = spec.strip_prefix("sqlite:").unwrap();
            Arc::new(SqliteJobStore::open(path)?)
        }
        other => anyhow::bail!("unknown STRATA_SERVER_STORE: {other}"),
    };

    let metrics = Metrics::new();
    let state = AppState { store, metrics };

    let app = strata_server::router(state);
    let listener = tokio::net::TcpListener::bind(&bind).await?;
    tracing::info!(bind = %bind, store = %store_spec, "strata-server started");
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let _ = tracing_subscriber::fmt().with_env_filter(filter).json().try_init();
}

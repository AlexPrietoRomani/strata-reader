//! Shared application state injected into every handler.

use std::sync::Arc;

use strata_runtime::Metrics;

use crate::jobs::JobStore;

/// State carried by axum's `State<AppState>` extractor.
#[derive(Clone)]
pub struct AppState {
    pub store: Arc<dyn JobStore>,
    pub metrics: Metrics,
}

impl AppState {
    pub fn new<S: JobStore + 'static>(store: S, metrics: Metrics) -> Self {
        Self { store: Arc::new(store), metrics }
    }
}

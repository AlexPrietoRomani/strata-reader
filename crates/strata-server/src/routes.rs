//! HTTP routes — Plan Maestro §14.T9.1.
//!
//! Endpoints:
//!
//! - `POST /v1/parse`           — accept a PDF (multipart) → 202 + `{ jobId }`.
//! - `POST /v1/parse-batch`     — accept several PDFs → 202 + `{ jobIds: [] }`.
//! - `GET  /v1/jobs/{id}`       — Job status + (when Done) artifacts.
//! - `GET  /v1/jobs`            — list all jobs (newest first).
//! - `GET  /healthz`            — liveness probe.
//! - `GET  /readyz`             — readiness probe (touches the job store).
//! - `GET  /metrics`            — Prometheus text exposition.
//! - `GET  /openapi.json`       — minimal hand-written OpenAPI 3.1 doc.

use axum::extract::{Multipart, Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tracing::{info, warn};

use crate::jobs::{Job, JobId, JobStatus};
use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/metrics", get(metrics))
        .route("/openapi.json", get(openapi))
        .route("/v1/parse", post(post_parse))
        .route("/v1/parse-batch", post(post_parse_batch))
        .route("/v1/jobs", get(list_jobs))
        .route("/v1/jobs/:id", get(get_job))
        .with_state(state)
}

// ---------------------------------------------------------------------------
// Health probes
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct HealthBody {
    status: &'static str,
    service: &'static str,
    version: &'static str,
}

async fn healthz() -> impl IntoResponse {
    Json(HealthBody {
        status: "ok",
        service: "strata-server",
        version: env!("CARGO_PKG_VERSION"),
    })
}

#[derive(Serialize)]
struct ReadyBody {
    status: &'static str,
    jobs_in_store: i64,
}

async fn readyz(State(state): State<AppState>) -> impl IntoResponse {
    match state.store.list().await {
        Ok(jobs) => Json(ReadyBody {
            status: "ready",
            jobs_in_store: jobs.len() as i64,
        }),
        Err(_) => Json(ReadyBody {
            status: "starting",
            jobs_in_store: -1,
        }),
    }
}

// ---------------------------------------------------------------------------
// Metrics
// ---------------------------------------------------------------------------

async fn metrics(State(state): State<AppState>) -> Response {
    let body = state.metrics.render();
    Response::builder()
        .header("content-type", "text/plain; version=0.0.4; charset=utf-8")
        .body(body.into())
        .expect("static headers always build")
}

// ---------------------------------------------------------------------------
// Parse + batch
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct EnqueuedBody {
    job_id: JobId,
}

async fn post_parse(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<EnqueuedBody>), ApiError> {
    if let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| ApiError::BadRequest(format!("multipart parse error: {e}")))?
    {
        let filename = field.file_name().unwrap_or("upload.pdf").to_string();
        let bytes = field
            .bytes()
            .await
            .map_err(|e| ApiError::BadRequest(format!("multipart read error: {e}")))?;
        if bytes.is_empty() {
            return Err(ApiError::BadRequest("uploaded file is empty".into()));
        }
        let sha = sha256_hex(&bytes);
        let job = Job::new_queued(filename.clone(), sha);
        let id = state.store.create(job).await.map_err(ApiError::from)?;
        state.metrics.set_queue_depth(
            state
                .store
                .list()
                .await
                .map(|j| j.len() as u64)
                .unwrap_or(0),
        );
        info!(job = %id, filename = %filename, "enqueued parse job");
        return Ok((StatusCode::ACCEPTED, Json(EnqueuedBody { job_id: id })));
    }
    Err(ApiError::BadRequest(
        "multipart did not carry any file field".into(),
    ))
}

#[derive(Serialize)]
struct BatchEnqueuedBody {
    job_ids: Vec<JobId>,
}

async fn post_parse_batch(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<BatchEnqueuedBody>), ApiError> {
    let mut ids = Vec::new();
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| ApiError::BadRequest(format!("multipart parse error: {e}")))?
    {
        let filename = field.file_name().unwrap_or("upload.pdf").to_string();
        let bytes = field
            .bytes()
            .await
            .map_err(|e| ApiError::BadRequest(format!("multipart read error: {e}")))?;
        if bytes.is_empty() {
            warn!(filename = %filename, "skipping empty file in batch");
            continue;
        }
        let sha = sha256_hex(&bytes);
        let job = Job::new_queued(filename, sha);
        let id = state.store.create(job).await.map_err(ApiError::from)?;
        ids.push(id);
    }
    if ids.is_empty() {
        return Err(ApiError::BadRequest("no files in batch".into()));
    }
    state.metrics.set_queue_depth(
        state
            .store
            .list()
            .await
            .map(|j| j.len() as u64)
            .unwrap_or(0),
    );
    Ok((
        StatusCode::ACCEPTED,
        Json(BatchEnqueuedBody { job_ids: ids }),
    ))
}

// ---------------------------------------------------------------------------
// Job retrieval
// ---------------------------------------------------------------------------

async fn list_jobs(State(state): State<AppState>) -> Result<Json<Vec<Job>>, ApiError> {
    let jobs = state.store.list().await.map_err(ApiError::from)?;
    Ok(Json(jobs))
}

async fn get_job(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Job>, ApiError> {
    let id: JobId = id
        .parse()
        .map_err(|_| ApiError::BadRequest("invalid job id".into()))?;
    match state.store.get(id).await.map_err(ApiError::from)? {
        Some(job) => {
            // Touch the queue_depth metric on every poll so dashboards
            // stay fresh even when no new jobs land.
            if matches!(job.status, JobStatus::Done | JobStatus::Failed { .. }) {
                state.metrics.inc_pages_processed();
            }
            Ok(Json(job))
        }
        None => Err(ApiError::NotFound),
    }
}

// ---------------------------------------------------------------------------
// OpenAPI (hand-written, minimal)
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct OpenApiDoc {
    openapi: &'static str,
    info: OpenApiInfo,
    paths: serde_json::Value,
}

#[derive(Serialize)]
struct OpenApiInfo {
    title: &'static str,
    version: &'static str,
    description: &'static str,
}

async fn openapi() -> Json<OpenApiDoc> {
    Json(OpenApiDoc {
        openapi: "3.1.0",
        info: OpenApiInfo {
            title: "strata-server",
            version: env!("CARGO_PKG_VERSION"),
            description:
                "Strata-Reader PDF parsing microservice. See docs/plan/plan_maestro.md §14.",
        },
        paths: serde_json::json!({
            "/healthz": {"get": {"summary": "Liveness probe", "responses": {"200": {"description": "ok"}}}},
            "/readyz":  {"get": {"summary": "Readiness probe", "responses": {"200": {"description": "ready or starting"}}}},
            "/metrics": {"get": {"summary": "Prometheus exposition", "responses": {"200": {"description": "text/plain"}}}},
            "/v1/parse": {"post": {
                "summary": "Enqueue a single PDF for parsing",
                "requestBody": {"required": true, "content": {"multipart/form-data": {}}},
                "responses": {"202": {"description": "accepted; returns {jobId}"}}
            }},
            "/v1/parse-batch": {"post": {
                "summary": "Enqueue several PDFs in one call",
                "responses": {"202": {"description": "accepted; returns {jobIds}"}}
            }},
            "/v1/jobs": {"get": {"summary": "List jobs (newest first)"}},
            "/v1/jobs/{id}": {"get": {
                "summary": "Fetch a single job by ULID",
                "responses": {"200": {"description": "found"}, "404": {"description": "not found"}}
            }},
        }),
    })
}

// ---------------------------------------------------------------------------
// Error type — translates JobStoreError into HTTP status codes.
// ---------------------------------------------------------------------------

#[derive(Debug)]
enum ApiError {
    BadRequest(String),
    NotFound,
    Internal(String),
}

impl From<crate::jobs::JobStoreError> for ApiError {
    fn from(value: crate::jobs::JobStoreError) -> Self {
        match value {
            crate::jobs::JobStoreError::NotFound(_) => Self::NotFound,
            crate::jobs::JobStoreError::Backend(msg) => Self::Internal(msg),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct ErrorBody {
    error: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (code, msg) = match self {
            Self::BadRequest(m) => (StatusCode::BAD_REQUEST, m),
            Self::NotFound => (StatusCode::NOT_FOUND, "not found".into()),
            Self::Internal(m) => (StatusCode::INTERNAL_SERVER_ERROR, m),
        };
        (code, Json(ErrorBody { error: msg })).into_response()
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    let out = h.finalize();
    out.iter().fold(String::with_capacity(64), |mut acc, b| {
        use std::fmt::Write;
        let _ = write!(acc, "{b:02x}");
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::MemoryJobStore;
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt;

    fn app() -> Router {
        let state = AppState::new(MemoryJobStore::new(), strata_runtime::Metrics::new());
        router(state)
    }

    #[tokio::test]
    async fn healthz_returns_ok() {
        let resp = app()
            .oneshot(
                Request::builder()
                    .uri("/healthz")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn readyz_returns_ok_even_with_empty_store() {
        let resp = app()
            .oneshot(
                Request::builder()
                    .uri("/readyz")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn metrics_serves_prometheus_text_format() {
        let resp = app()
            .oneshot(
                Request::builder()
                    .uri("/metrics")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), 1_000_000)
            .await
            .unwrap();
        let text = String::from_utf8(body.to_vec()).unwrap();
        assert!(text.contains("strata_pages_processed_total"));
    }

    #[tokio::test]
    async fn get_unknown_job_returns_404() {
        let resp = app()
            .oneshot(
                Request::builder()
                    .uri(format!("/v1/jobs/{}", JobId::new()))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn get_invalid_job_id_returns_400() {
        let resp = app()
            .oneshot(
                Request::builder()
                    .uri("/v1/jobs/not-an-ulid")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn openapi_lists_every_published_route() {
        let resp = app()
            .oneshot(
                Request::builder()
                    .uri("/openapi.json")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), 1_000_000)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let paths = json["paths"].as_object().unwrap();
        for needed in [
            "/healthz",
            "/readyz",
            "/metrics",
            "/v1/parse",
            "/v1/parse-batch",
            "/v1/jobs",
            "/v1/jobs/{id}",
        ] {
            assert!(
                paths.contains_key(needed),
                "missing {needed} in OpenAPI paths"
            );
        }
    }

    #[tokio::test]
    async fn list_jobs_starts_empty() {
        let resp = app()
            .oneshot(
                Request::builder()
                    .uri("/v1/jobs")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), 1_000_000)
            .await
            .unwrap();
        let jobs: Vec<Job> = serde_json::from_slice(&body).unwrap();
        assert!(jobs.is_empty());
    }
}

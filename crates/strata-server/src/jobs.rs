//! Job model and the [`JobStore`] trait the HTTP endpoints persist to.
//!
//! Plan Maestro §14.T9.2 — the microservice queues parse jobs and lets
//! clients poll `/v1/jobs/{id}` for progress. The store is a trait so
//! the binary can boot with an in-memory queue in dev and swap to
//! [`crate::store::sqlite::SqliteJobStore`] in prod without changing any
//! handler code.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use ulid::Ulid;

/// Stable, lexicographically-sortable job identifier — same shape as
/// `strata_core::BlockId`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct JobId(pub Ulid);

impl JobId {
    pub fn new() -> Self {
        Self(Ulid::new())
    }
}

impl Default for JobId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for JobId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for JobId {
    type Err = ulid::DecodeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ulid::from_string(s).map(JobId)
    }
}

impl Serialize for JobId {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for JobId {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(d)?;
        Ulid::from_string(&raw)
            .map(JobId)
            .map_err(serde::de::Error::custom)
    }
}

/// Lifecycle states a parse job traverses.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "kebab-case")]
pub enum JobStatus {
    /// Accepted, waiting for a worker.
    Queued,
    /// A worker picked it up; progress is the percentage 0..=100.
    Running { progress: u8 },
    /// Finished. Result artifacts live in [`Job::result_md`] /
    /// [`Job::result_json`].
    Done,
    /// Aborted because of a typed error. The message is propagated to the
    /// client verbatim.
    Failed { error: String },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    pub id: JobId,
    pub source_filename: String,
    /// Hex SHA-256 of the source PDF. Doubles as the cache key.
    pub source_sha256: String,
    /// Unix seconds since epoch.
    pub created_at: i64,
    /// Same convention. Updated each time the status changes.
    pub updated_at: i64,
    pub status: JobStatus,
    /// Markdown output (None until status == Done).
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub result_md: Option<String>,
    /// JSON Graph-RAG output (None until status == Done).
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub result_json: Option<String>,
}

impl Job {
    pub fn new_queued(source_filename: String, source_sha256: String) -> Self {
        let now = unix_seconds();
        Self {
            id: JobId::new(),
            source_filename,
            source_sha256,
            created_at: now,
            updated_at: now,
            status: JobStatus::Queued,
            result_md: None,
            result_json: None,
        }
    }
}

fn unix_seconds() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// Errors raised by every [`JobStore`] implementation.
#[derive(Debug, Error)]
pub enum JobStoreError {
    #[error("job {0} not found")]
    NotFound(JobId),
    #[error("storage backend error: {0}")]
    Backend(String),
}

/// The contract every persistence backend honours.
#[async_trait]
pub trait JobStore: Send + Sync {
    /// Insert a freshly built [`Job`]. Returns the assigned id.
    async fn create(&self, job: Job) -> Result<JobId, JobStoreError>;
    /// Fetch a single job by id. Returns `Ok(None)` on not-found instead
    /// of an error so the HTTP layer can decide 404 vs. 500.
    async fn get(&self, id: JobId) -> Result<Option<Job>, JobStoreError>;
    /// List jobs in creation order (newest first). Pagination is the
    /// caller's job (slice the returned vec).
    async fn list(&self) -> Result<Vec<Job>, JobStoreError>;
    /// Replace the whole job record. Used to update status / artifacts.
    async fn put(&self, job: Job) -> Result<(), JobStoreError>;
    /// Delete a job. Idempotent.
    async fn delete(&self, id: JobId) -> Result<(), JobStoreError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn job_id_round_trips_through_json() {
        let id = JobId::new();
        let s = serde_json::to_string(&id).unwrap();
        let back: JobId = serde_json::from_str(&s).unwrap();
        assert_eq!(id, back);
    }

    #[test]
    fn job_id_from_str_parses_ulid() {
        let id = JobId::new();
        let parsed: JobId = id.to_string().parse().unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn job_status_serializes_as_kebab_state_tag() {
        let s = JobStatus::Queued;
        assert_eq!(serde_json::to_string(&s).unwrap(), r#"{"state":"queued"}"#);
        let s = JobStatus::Running { progress: 42 };
        let j = serde_json::to_string(&s).unwrap();
        assert!(j.contains(r#""state":"running""#));
        assert!(j.contains(r#""progress":42"#));
    }

    #[test]
    fn job_round_trip_through_json_preserves_optional_results() {
        let mut job = Job::new_queued("paper.pdf".into(), "0".repeat(64));
        job.status = JobStatus::Done;
        job.result_md = Some("# Title".into());
        job.result_json = Some(r#"{"nodes":[]}"#.into());
        let json = serde_json::to_string(&job).unwrap();
        let back: Job = serde_json::from_str(&json).unwrap();
        assert_eq!(job, back);
    }

    #[test]
    fn job_omits_result_fields_when_none() {
        let job = Job::new_queued("p.pdf".into(), "abc".into());
        let json = serde_json::to_string(&job).unwrap();
        assert!(!json.contains("resultMd"));
        assert!(!json.contains("resultJson"));
    }

    #[test]
    fn job_store_error_has_useful_display() {
        let id = JobId::new();
        let err = JobStoreError::NotFound(id);
        assert!(err.to_string().contains(&id.to_string()));
    }
}

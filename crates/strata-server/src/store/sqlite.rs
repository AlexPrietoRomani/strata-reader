//! SQLite-backed [`JobStore`].
//!
//! Plan Maestro §14.T9.2 — survive restarts. Schema:
//!
//! ```sql
//! CREATE TABLE jobs (
//!     id           TEXT PRIMARY KEY,        -- ULID
//!     filename     TEXT NOT NULL,
//!     sha256       TEXT NOT NULL,
//!     created_at   INTEGER NOT NULL,
//!     updated_at   INTEGER NOT NULL,
//!     status_json  TEXT NOT NULL,           -- serialized JobStatus
//!     result_md    TEXT,
//!     result_json  TEXT
//! );
//! CREATE INDEX idx_jobs_created ON jobs(created_at);
//! ```
//!
//! `rusqlite` is synchronous; we wrap blocking calls in
//! `tokio::task::spawn_blocking`. The connection lives behind a
//! `parking_lot::Mutex` because `rusqlite::Connection` is `Send` but not
//! `Sync`.

use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use parking_lot::Mutex;
use rusqlite::{params, Connection, OptionalExtension};

use crate::jobs::{Job, JobId, JobStatus, JobStore, JobStoreError};

const SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS jobs (
    id          TEXT PRIMARY KEY,
    filename    TEXT NOT NULL,
    sha256      TEXT NOT NULL,
    created_at  INTEGER NOT NULL,
    updated_at  INTEGER NOT NULL,
    status_json TEXT NOT NULL,
    result_md   TEXT,
    result_json TEXT
);
CREATE INDEX IF NOT EXISTS idx_jobs_created ON jobs(created_at);
"#;

pub struct SqliteJobStore {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteJobStore {
    /// Open or create the database at `path`. The schema is created on
    /// first use; subsequent opens are idempotent.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, JobStoreError> {
        let conn = Connection::open(path).map_err(map_err)?;
        conn.execute_batch("PRAGMA journal_mode=WAL;")
            .map_err(map_err)?;
        conn.execute_batch(SCHEMA).map_err(map_err)?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// In-memory store, useful for tests.
    pub fn in_memory() -> Result<Self, JobStoreError> {
        let conn = Connection::open_in_memory().map_err(map_err)?;
        conn.execute_batch(SCHEMA).map_err(map_err)?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// Row count — useful for tests / metrics.
    pub fn count(&self) -> Result<i64, JobStoreError> {
        let conn = self.conn.lock();
        let n: i64 = conn
            .query_row("SELECT COUNT(*) FROM jobs", [], |r| r.get(0))
            .map_err(map_err)?;
        Ok(n)
    }
}

fn map_err(e: rusqlite::Error) -> JobStoreError {
    JobStoreError::Backend(e.to_string())
}

fn row_to_job(row: &rusqlite::Row<'_>) -> rusqlite::Result<Job> {
    let id: String = row.get("id")?;
    let id: JobId = id
        .parse()
        .map_err(|e: ulid::DecodeError| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    let status_json: String = row.get("status_json")?;
    let status: JobStatus = serde_json::from_str(&status_json)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    Ok(Job {
        id,
        source_filename: row.get("filename")?,
        source_sha256: row.get("sha256")?,
        created_at: row.get("created_at")?,
        updated_at: row.get("updated_at")?,
        status,
        result_md: row.get("result_md")?,
        result_json: row.get("result_json")?,
    })
}

fn write_job(conn: &Connection, job: &Job, upsert: bool) -> Result<(), JobStoreError> {
    let status_json =
        serde_json::to_string(&job.status).map_err(|e| JobStoreError::Backend(e.to_string()))?;
    let id = job.id.to_string();
    let sql = if upsert {
        "INSERT INTO jobs (id, filename, sha256, created_at, updated_at, status_json, result_md, result_json)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
         ON CONFLICT(id) DO UPDATE SET
            filename    = excluded.filename,
            sha256      = excluded.sha256,
            updated_at  = excluded.updated_at,
            status_json = excluded.status_json,
            result_md   = excluded.result_md,
            result_json = excluded.result_json"
    } else {
        "INSERT INTO jobs (id, filename, sha256, created_at, updated_at, status_json, result_md, result_json)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)"
    };
    conn.execute(
        sql,
        params![
            id,
            job.source_filename,
            job.source_sha256,
            job.created_at,
            job.updated_at,
            status_json,
            job.result_md,
            job.result_json,
        ],
    )
    .map_err(map_err)?;
    Ok(())
}

#[async_trait]
impl JobStore for SqliteJobStore {
    async fn create(&self, job: Job) -> Result<JobId, JobStoreError> {
        let id = job.id;
        let conn = Arc::clone(&self.conn);
        tokio::task::spawn_blocking(move || {
            let guard = conn.lock();
            write_job(&guard, &job, false)
        })
        .await
        .map_err(|e| JobStoreError::Backend(format!("blocking task: {e}")))??;
        Ok(id)
    }

    async fn get(&self, id: JobId) -> Result<Option<Job>, JobStoreError> {
        let conn = Arc::clone(&self.conn);
        let id_str = id.to_string();
        let res = tokio::task::spawn_blocking(move || -> Result<Option<Job>, JobStoreError> {
            let guard = conn.lock();
            guard
                .query_row(
                    "SELECT id, filename, sha256, created_at, updated_at, status_json, result_md, result_json
                     FROM jobs WHERE id = ?1",
                    [id_str],
                    row_to_job,
                )
                .optional()
                .map_err(map_err)
        })
        .await
        .map_err(|e| JobStoreError::Backend(format!("blocking task: {e}")))??;
        Ok(res)
    }

    async fn list(&self) -> Result<Vec<Job>, JobStoreError> {
        let conn = Arc::clone(&self.conn);
        let res = tokio::task::spawn_blocking(move || -> Result<Vec<Job>, JobStoreError> {
            let guard = conn.lock();
            let mut stmt = guard
                .prepare(
                    "SELECT id, filename, sha256, created_at, updated_at, status_json, result_md, result_json
                     FROM jobs ORDER BY created_at DESC",
                )
                .map_err(map_err)?;
            let rows = stmt
                .query_map([], row_to_job)
                .map_err(map_err)?
                .map(|r| r.map_err(map_err))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(rows)
        })
        .await
        .map_err(|e| JobStoreError::Backend(format!("blocking task: {e}")))??;
        Ok(res)
    }

    async fn put(&self, job: Job) -> Result<(), JobStoreError> {
        let conn = Arc::clone(&self.conn);
        tokio::task::spawn_blocking(move || {
            let guard = conn.lock();
            // Verify existence first to keep parity with MemoryJobStore.
            let id_str = job.id.to_string();
            let count: i64 = guard
                .query_row("SELECT COUNT(*) FROM jobs WHERE id = ?1", [id_str], |r| {
                    r.get(0)
                })
                .map_err(map_err)?;
            if count == 0 {
                return Err(JobStoreError::NotFound(job.id));
            }
            write_job(&guard, &job, true)
        })
        .await
        .map_err(|e| JobStoreError::Backend(format!("blocking task: {e}")))??;
        Ok(())
    }

    async fn delete(&self, id: JobId) -> Result<(), JobStoreError> {
        let conn = Arc::clone(&self.conn);
        let id_str = id.to_string();
        tokio::task::spawn_blocking(move || -> Result<(), JobStoreError> {
            let guard = conn.lock();
            guard
                .execute("DELETE FROM jobs WHERE id = ?1", [id_str])
                .map_err(map_err)?;
            Ok(())
        })
        .await
        .map_err(|e| JobStoreError::Backend(format!("blocking task: {e}")))?
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jobs::JobStatus;

    #[tokio::test]
    async fn in_memory_store_round_trips_a_job() {
        let store = SqliteJobStore::in_memory().unwrap();
        let job = Job::new_queued("p.pdf".into(), "h".into());
        let id = job.id;
        store.create(job.clone()).await.unwrap();
        let back = store.get(id).await.unwrap().expect("found");
        assert_eq!(back, job);
    }

    #[tokio::test]
    async fn persistence_survives_a_new_handle() {
        // Plan AC §14.T9.2 — jobs in flight survive a server restart when
        // backed by SQLite. We model "restart" as opening a fresh handle
        // on the same file.
        let tmp = tempfile_path();
        let job = {
            let store = SqliteJobStore::open(&tmp).unwrap();
            let job = Job::new_queued("paper.pdf".into(), "abc".into());
            store.create(job.clone()).await.unwrap();
            job
        };

        // "Restart" — new handle, same file.
        let reopened = SqliteJobStore::open(&tmp).unwrap();
        let recovered = reopened.get(job.id).await.unwrap().expect("survived");
        assert_eq!(recovered.id, job.id);
        assert_eq!(recovered.source_filename, "paper.pdf");
        assert_eq!(recovered.status, JobStatus::Queued);

        let _ = std::fs::remove_file(&tmp);
    }

    #[tokio::test]
    async fn put_persists_status_changes() {
        let store = SqliteJobStore::in_memory().unwrap();
        let mut job = Job::new_queued("p.pdf".into(), "h".into());
        let id = job.id;
        store.create(job.clone()).await.unwrap();

        job.status = JobStatus::Done;
        job.result_md = Some("# Done".into());
        store.put(job.clone()).await.unwrap();

        let back = store.get(id).await.unwrap().unwrap();
        assert_eq!(back.status, JobStatus::Done);
        assert_eq!(back.result_md.as_deref(), Some("# Done"));
    }

    #[tokio::test]
    async fn put_unknown_id_errors() {
        let store = SqliteJobStore::in_memory().unwrap();
        let j = Job::new_queued("p.pdf".into(), "h".into());
        let err = store.put(j).await.unwrap_err();
        assert!(matches!(err, JobStoreError::NotFound(_)));
    }

    #[tokio::test]
    async fn list_orders_by_created_desc() {
        let store = SqliteJobStore::in_memory().unwrap();
        for name in ["a.pdf", "b.pdf", "c.pdf"] {
            let j = Job::new_queued(name.into(), "h".into());
            store.create(j).await.unwrap();
            tokio::time::sleep(std::time::Duration::from_millis(1100)).await;
        }
        let list = store.list().await.unwrap();
        assert_eq!(list.len(), 3);
        // Newest first.
        assert_eq!(list[0].source_filename, "c.pdf");
    }

    fn tempfile_path() -> std::path::PathBuf {
        let mut p = std::env::temp_dir();
        p.push(format!("strata-jobs-{}.sqlite", ulid::Ulid::new()));
        p
    }
}

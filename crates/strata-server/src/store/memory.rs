//! In-memory [`JobStore`] backend.
//!
//! The default for `strata-server` when no `--store sqlite:...` flag is
//! passed. Loses state on restart — fine for development and short-lived
//! batch workers. Production deployments wire the SQLite backend.

use std::collections::BTreeMap;

use async_trait::async_trait;
use parking_lot::RwLock;

use crate::jobs::{Job, JobId, JobStore, JobStoreError};

#[derive(Default)]
pub struct MemoryJobStore {
    inner: RwLock<BTreeMap<JobId, Job>>,
}

impl MemoryJobStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Snapshot of how many jobs are currently held. Useful for tests
    /// and metrics dashboards.
    pub fn len(&self) -> usize {
        self.inner.read().len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.read().is_empty()
    }
}

#[async_trait]
impl JobStore for MemoryJobStore {
    async fn create(&self, job: Job) -> Result<JobId, JobStoreError> {
        let id = job.id;
        self.inner.write().insert(id, job);
        Ok(id)
    }

    async fn get(&self, id: JobId) -> Result<Option<Job>, JobStoreError> {
        Ok(self.inner.read().get(&id).cloned())
    }

    async fn list(&self) -> Result<Vec<Job>, JobStoreError> {
        // BTreeMap orders by JobId (= ULID = lexicographic timestamp).
        // Reverse so newest comes first.
        let v: Vec<Job> = self.inner.read().values().rev().cloned().collect();
        Ok(v)
    }

    async fn put(&self, job: Job) -> Result<(), JobStoreError> {
        let id = job.id;
        let mut guard = self.inner.write();
        if !guard.contains_key(&id) {
            return Err(JobStoreError::NotFound(id));
        }
        guard.insert(id, job);
        Ok(())
    }

    async fn delete(&self, id: JobId) -> Result<(), JobStoreError> {
        self.inner.write().remove(&id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jobs::{Job, JobStatus};

    fn job(name: &str) -> Job {
        Job::new_queued(name.into(), "abc".into())
    }

    #[tokio::test]
    async fn create_then_get_round_trips() {
        let store = MemoryJobStore::new();
        let j = job("paper.pdf");
        let id = j.id;
        store.create(j.clone()).await.unwrap();
        let back = store.get(id).await.unwrap().expect("found");
        assert_eq!(back, j);
    }

    #[tokio::test]
    async fn get_unknown_returns_none() {
        let store = MemoryJobStore::new();
        assert!(store.get(JobId::new()).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn list_orders_newest_first() {
        let store = MemoryJobStore::new();
        let a = job("a.pdf");
        let b = job("b.pdf");
        // Ensure distinct ULIDs across ms boundaries.
        tokio::time::sleep(std::time::Duration::from_millis(2)).await;
        let c = job("c.pdf");
        store.create(a.clone()).await.unwrap();
        store.create(b.clone()).await.unwrap();
        store.create(c.clone()).await.unwrap();

        let list = store.list().await.unwrap();
        assert_eq!(list.len(), 3);
        // Last inserted ULID is greatest, so .rev() in the impl puts it first.
        assert_eq!(list[0].source_filename, "c.pdf");
    }

    #[tokio::test]
    async fn put_unknown_id_returns_not_found() {
        let store = MemoryJobStore::new();
        let mut j = job("p.pdf");
        // Never created — put should fail.
        j.status = JobStatus::Running { progress: 10 };
        let err = store.put(j).await.unwrap_err();
        assert!(matches!(err, JobStoreError::NotFound(_)));
    }

    #[tokio::test]
    async fn put_updates_existing_job() {
        let store = MemoryJobStore::new();
        let j = job("p.pdf");
        let id = j.id;
        store.create(j.clone()).await.unwrap();

        let mut updated = j;
        updated.status = JobStatus::Done;
        updated.result_md = Some("# done".into());
        store.put(updated.clone()).await.unwrap();

        let got = store.get(id).await.unwrap().unwrap();
        assert_eq!(got.status, JobStatus::Done);
        assert_eq!(got.result_md.as_deref(), Some("# done"));
    }

    #[tokio::test]
    async fn delete_is_idempotent() {
        let store = MemoryJobStore::new();
        let id = JobId::new();
        // Delete a non-existing id — should succeed quietly.
        store.delete(id).await.unwrap();
    }
}

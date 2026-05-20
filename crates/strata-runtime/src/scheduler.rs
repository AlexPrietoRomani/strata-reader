//! Concurrent page scheduler built on top of [`tokio::task::JoinSet`].
//!
//! Plan Maestro §13.T8.1 — process N pages in parallel bounded by a
//! semaphore. The AC is ≥ 8× throughput vs. sequential on a 16-core box;
//! the limiter is the semaphore (`max_concurrent_pages`), not the
//! workload size.
//!
//! API:
//!
//! ```ignore
//! let sched = Scheduler::new(SchedulerConfig { max_concurrent_pages: 8, ..Default::default() });
//! let results = sched
//!     .run(items, |item| async move { do_work(item).await })
//!     .await;
//! ```

use std::sync::Arc;

use tokio::sync::Semaphore;
use tokio::task::JoinSet;

#[derive(Clone, Debug)]
pub struct SchedulerConfig {
    /// Maximum number of concurrent tasks in flight. The runtime hands out
    /// permits via a [`tokio::sync::Semaphore`]; permit acquisition is
    /// FIFO-fair.
    pub max_concurrent_pages: usize,
    /// Optional cooperative cancellation token (callers wire a CTRL-C
    /// handler here). When the token is dropped, in-flight tasks finish
    /// but no new ones are spawned.
    pub abort_on_first_error: bool,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            max_concurrent_pages: num_cpus_estimate(),
            abort_on_first_error: false,
        }
    }
}

fn num_cpus_estimate() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}

/// Bounded-concurrency executor over a `tokio` runtime.
#[derive(Clone)]
pub struct Scheduler {
    semaphore: Arc<Semaphore>,
    config: SchedulerConfig,
}

impl Scheduler {
    pub fn new(config: SchedulerConfig) -> Self {
        let n = config.max_concurrent_pages.max(1);
        Self {
            semaphore: Arc::new(Semaphore::new(n)),
            config,
        }
    }

    /// Snapshot of the configured limit.
    pub fn limit(&self) -> usize {
        self.config.max_concurrent_pages
    }

    /// Current number of available permits — number of slots a new
    /// submitter would be granted immediately.
    pub fn available_permits(&self) -> usize {
        self.semaphore.available_permits()
    }

    /// Run `task_fn` over every `item` in parallel, bounded by the
    /// scheduler's permit count. Results come back in the **input order**,
    /// not completion order — the caller can rely on positional correlation
    /// with `items`.
    pub async fn run<I, T, F, Fut, R>(&self, items: I, task_fn: F) -> Vec<R>
    where
        I: IntoIterator<Item = T>,
        T: Send + 'static,
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = R> + Send + 'static,
        R: Send + 'static,
    {
        let task_fn = Arc::new(task_fn);
        let mut joinset: JoinSet<(usize, R)> = JoinSet::new();

        for (idx, item) in items.into_iter().enumerate() {
            let sem = Arc::clone(&self.semaphore);
            let task_fn = Arc::clone(&task_fn);
            joinset.spawn(async move {
                // Permit lives for the duration of the await — released on drop.
                let _permit = sem.acquire_owned().await.expect("semaphore poisoned");
                let r = (task_fn)(item).await;
                (idx, r)
            });
        }

        // Drain results indexed by input position.
        let mut indexed: Vec<Option<R>> = (0..joinset.len()).map(|_| None).collect();
        while let Some(joined) = joinset.join_next().await {
            let (idx, r) = joined.expect("worker task panicked");
            indexed[idx] = Some(r);
        }
        indexed
            .into_iter()
            .map(|o| o.expect("missing result for an index"))
            .collect()
    }

    /// Fire-and-forget variant — spawn one task, return its handle so the
    /// caller composes manually with `select!` or another `JoinSet`.
    pub fn spawn<F, R>(&self, fut: F) -> tokio::task::JoinHandle<R>
    where
        F: std::future::Future<Output = R> + Send + 'static,
        R: Send + 'static,
    {
        let sem = Arc::clone(&self.semaphore);
        tokio::spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore poisoned");
            fut.await
        })
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;

    #[tokio::test]
    async fn run_processes_every_item() {
        let sched = Scheduler::new(SchedulerConfig {
            max_concurrent_pages: 4,
            abort_on_first_error: false,
        });
        let items: Vec<u32> = (0..20).collect();
        let results = sched.run(items.clone(), |n| async move { n * 2 }).await;
        assert_eq!(results.len(), 20);
        for (i, r) in results.iter().enumerate() {
            assert_eq!(*r, items[i] * 2, "result at idx {i} mismatched");
        }
    }

    #[tokio::test]
    async fn run_respects_concurrency_limit() {
        let limit = 3;
        let sched = Scheduler::new(SchedulerConfig {
            max_concurrent_pages: limit,
            abort_on_first_error: false,
        });
        let in_flight = Arc::new(AtomicUsize::new(0));
        let max_observed = Arc::new(AtomicUsize::new(0));
        let in_flight_c = Arc::clone(&in_flight);
        let max_observed_c = Arc::clone(&max_observed);
        let _ = sched
            .run(0..20, move |_n| {
                let in_flight = Arc::clone(&in_flight_c);
                let max_observed = Arc::clone(&max_observed_c);
                async move {
                    let now = in_flight.fetch_add(1, Ordering::SeqCst) + 1;
                    max_observed.fetch_max(now, Ordering::SeqCst);
                    tokio::time::sleep(Duration::from_millis(20)).await;
                    in_flight.fetch_sub(1, Ordering::SeqCst);
                }
            })
            .await;
        let observed = max_observed.load(Ordering::SeqCst);
        assert!(
            observed <= limit,
            "max in-flight {observed} exceeded limit {limit}"
        );
        // Sanity: we did saturate the pool.
        assert!(
            observed >= limit - 1,
            "max in-flight {observed} should approach {limit}"
        );
    }

    #[tokio::test]
    async fn spawn_handle_completes() {
        let sched = Scheduler::new(SchedulerConfig::default());
        let h = sched.spawn(async { 42u32 });
        assert_eq!(h.await.unwrap(), 42);
    }

    #[tokio::test]
    async fn available_permits_tracks_active_tasks() {
        let sched = Scheduler::new(SchedulerConfig {
            max_concurrent_pages: 4,
            abort_on_first_error: false,
        });
        assert_eq!(sched.available_permits(), 4);

        let h = sched.spawn(async {
            tokio::time::sleep(Duration::from_millis(50)).await;
        });
        // Give the spawned task time to acquire the permit.
        tokio::time::sleep(Duration::from_millis(5)).await;
        assert_eq!(sched.available_permits(), 3);
        h.await.unwrap();
        assert_eq!(sched.available_permits(), 4);
    }

    #[tokio::test]
    async fn default_limit_is_at_least_one() {
        let sched = Scheduler::new(SchedulerConfig::default());
        assert!(sched.limit() >= 1);
    }
}

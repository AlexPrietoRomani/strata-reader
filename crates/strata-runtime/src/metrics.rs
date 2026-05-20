//! Prometheus metrics exporter.
//!
//! Plan Maestro §13.T8.5 lists exactly which series are surfaced:
//!
//! - `strata_pages_processed_total`  (Counter)
//! - `strata_ia_request_duration_seconds`  (Histogram, labels `task`, `model`)
//! - `strata_vram_used_mb`  (Gauge, label `device`)
//! - `strata_queue_depth`  (Gauge)
//! - `strata_cache_hit_total`  (Counter)
//!
//! [`Metrics`] is process-wide (one Registry per process) so the
//! `strata-server` /metrics handler and the scheduler both observe the
//! same series. Cloneable cheaply via `Arc`.

use std::sync::Arc;

use prometheus::{
    register_counter_with_registry, register_gauge_vec_with_registry, register_gauge_with_registry,
    register_histogram_vec_with_registry, Counter, Encoder, Gauge, GaugeVec, HistogramVec,
    Registry, TextEncoder,
};

#[derive(Clone)]
pub struct Metrics(Arc<Inner>);

struct Inner {
    registry: Registry,
    pages_processed: Counter,
    ia_request_duration: HistogramVec,
    vram_used_mb: GaugeVec,
    queue_depth: Gauge,
    cache_hit: Counter,
}

impl Metrics {
    pub fn new() -> Self {
        let registry = Registry::new();

        let pages_processed = register_counter_with_registry!(
            "strata_pages_processed_total",
            "Total number of pages that completed processing (success path).",
            registry
        )
        .expect("failed to register strata_pages_processed_total");

        // Histogram buckets cover 10ms → 60s, doubling each step.
        let buckets = vec![
            0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0, 25.0, 60.0,
        ];
        let ia_request_duration = register_histogram_vec_with_registry!(
            prometheus::HistogramOpts::new(
                "strata_ia_request_duration_seconds",
                "Wall-clock duration of IA bridge requests by task and model."
            )
            .buckets(buckets),
            &["task", "model"],
            registry
        )
        .expect("failed to register strata_ia_request_duration_seconds");

        let vram_used_mb = register_gauge_vec_with_registry!(
            "strata_vram_used_mb",
            "VRAM used per GPU device, in megabytes.",
            &["device"],
            registry
        )
        .expect("failed to register strata_vram_used_mb");

        let queue_depth = register_gauge_with_registry!(
            "strata_queue_depth",
            "Current number of jobs waiting in the scheduler queue.",
            registry
        )
        .expect("failed to register strata_queue_depth");

        let cache_hit = register_counter_with_registry!(
            "strata_cache_hit_total",
            "Number of IA-result cache hits (idempotency).",
            registry
        )
        .expect("failed to register strata_cache_hit_total");

        Self(Arc::new(Inner {
            registry,
            pages_processed,
            ia_request_duration,
            vram_used_mb,
            queue_depth,
            cache_hit,
        }))
    }

    pub fn registry(&self) -> &Registry {
        &self.0.registry
    }

    // ----- mutators called by the runtime / bridge -----------------

    pub fn inc_pages_processed(&self) {
        self.0.pages_processed.inc();
    }

    pub fn observe_ia_request_seconds(&self, task: &str, model: &str, seconds: f64) {
        self.0
            .ia_request_duration
            .with_label_values(&[task, model])
            .observe(seconds);
    }

    pub fn set_vram_used_mb(&self, device: &str, used_mb: u64) {
        self.0
            .vram_used_mb
            .with_label_values(&[device])
            .set(used_mb as f64);
    }

    pub fn set_queue_depth(&self, depth: u64) {
        self.0.queue_depth.set(depth as f64);
    }

    pub fn inc_cache_hit(&self) {
        self.0.cache_hit.inc();
    }

    // ----- exposition -------------------------------------------------

    /// Render the metrics in Prometheus text format. Used by the
    /// `strata-server` /metrics handler.
    pub fn render(&self) -> String {
        let mut buf = Vec::new();
        let encoder = TextEncoder::new();
        let families = self.0.registry.gather();
        encoder
            .encode(&families, &mut buf)
            .expect("Prometheus encode never fails on Vec<u8>");
        String::from_utf8(buf).expect("Prometheus text format is UTF-8")
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Metrics").finish_non_exhaustive()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registers_all_expected_series() {
        let m = Metrics::new();
        m.inc_pages_processed();
        m.observe_ia_request_seconds("extract-table", "qwen2.5vl:7b", 0.42);
        m.set_vram_used_mb("gpu-0", 4096);
        m.set_queue_depth(7);
        m.inc_cache_hit();

        let exposed = m.render();
        assert!(exposed.contains("strata_pages_processed_total"));
        assert!(exposed.contains("strata_ia_request_duration_seconds"));
        assert!(exposed.contains("strata_vram_used_mb"));
        assert!(exposed.contains("strata_queue_depth"));
        assert!(exposed.contains("strata_cache_hit_total"));
    }

    #[test]
    fn counter_increments_visible_in_output() {
        let m = Metrics::new();
        m.inc_pages_processed();
        m.inc_pages_processed();
        m.inc_pages_processed();
        let out = m.render();
        // Prometheus format prints "strata_pages_processed_total 3"
        let line = out
            .lines()
            .find(|l| l.starts_with("strata_pages_processed_total "))
            .expect("counter line");
        assert!(line.ends_with(" 3"), "expected ' 3' suffix, got {line}");
    }

    #[test]
    fn vram_used_label_separates_devices() {
        let m = Metrics::new();
        m.set_vram_used_mb("gpu-0", 4000);
        m.set_vram_used_mb("gpu-1", 8000);
        let out = m.render();
        assert!(out.contains("strata_vram_used_mb{device=\"gpu-0\"} 4000"));
        assert!(out.contains("strata_vram_used_mb{device=\"gpu-1\"} 8000"));
    }

    #[test]
    fn ia_request_duration_uses_task_and_model_labels() {
        let m = Metrics::new();
        m.observe_ia_request_seconds("extract-table", "qwen2.5vl:7b", 0.5);
        let out = m.render();
        assert!(out.contains("task=\"extract-table\""));
        assert!(out.contains("model=\"qwen2.5vl:7b\""));
    }

    #[test]
    fn queue_depth_gauge_can_decrease() {
        let m = Metrics::new();
        m.set_queue_depth(10);
        m.set_queue_depth(3);
        let out = m.render();
        let line = out
            .lines()
            .find(|l| l.starts_with("strata_queue_depth "))
            .unwrap();
        assert!(line.ends_with(" 3"));
    }

    #[test]
    fn metrics_is_cloneable_and_shares_state() {
        let m = Metrics::new();
        let m2 = m.clone();
        m.inc_cache_hit();
        m2.inc_cache_hit();
        let out = m2.render();
        let line = out
            .lines()
            .find(|l| l.starts_with("strata_cache_hit_total "))
            .unwrap();
        assert!(line.ends_with(" 2"));
    }
}

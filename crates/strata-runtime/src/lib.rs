//! Strata-Runtime — concurrent scheduling, GPU monitoring, AIMD back-pressure
//! and observability (Prometheus + tracing OTLP).
//!
//! See `docs/plan/plan_maestro.md` §13.

#![deny(rust_2018_idioms)]

pub mod backpressure;
pub mod capabilities;
pub mod gpu_monitor;
pub mod gpu_pool;
pub mod metrics;
pub mod scheduler;

pub use backpressure::{BackoffReason, BackpressureConfig, BackpressureController};
pub use capabilities::{Capabilities, OcrPreference, SuggestedProfile};
pub use gpu_monitor::{
    detect as detect_gpu, GpuBackend, GpuDeviceSnapshot, GpuMonitor, GpuMonitorError, GpuSnapshot,
    MetalMonitor, NoopMonitor, NvmlMonitor, RocmMonitor,
};
pub use gpu_pool::{describe_pool, plan_from_monitor, plan_workers, PoolConfig, WorkerSpec};
pub use metrics::Metrics;
pub use scheduler::{Scheduler, SchedulerConfig};

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

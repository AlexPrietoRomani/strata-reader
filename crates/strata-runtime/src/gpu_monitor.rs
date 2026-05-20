//! GPU / VRAM monitoring with pluggable backends.
//!
//! Plan Maestro §13.T8.2 — the runtime needs cheap, fresh snapshots of
//! VRAM use to drive the [`crate::backpressure::BackpressureController`]
//! AIMD loop. NVIDIA hosts use NVML through `nvml-wrapper`; AMD ROCm and
//! Apple Metal hosts fall through to a no-op backend until the
//! corresponding crate dependencies land.
//!
//! Snapshot data is intentionally minimal — `total_mb` / `free_mb` /
//! `util_pct`. The Prometheus exporter (T8.5) picks the same view.

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::warn;

#[derive(Debug, Error)]
pub enum GpuMonitorError {
    #[error("NVML init failed: {0}")]
    NvmlInit(String),
    #[error("device {0} not found")]
    DeviceNotFound(u32),
    #[error("backend not implemented for this platform")]
    NotImplemented,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GpuBackend {
    Nvml,
    Rocm,
    Metal,
    /// No GPU detected / monitoring disabled. The runtime degrades to
    /// CPU-only paths (Plan Maestro §13.T8.6).
    Noop,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GpuDeviceSnapshot {
    pub index: u32,
    pub name: String,
    pub vram_total_mb: u64,
    pub vram_used_mb: u64,
    pub util_pct: f32,
}

impl GpuDeviceSnapshot {
    pub fn vram_free_mb(&self) -> u64 {
        self.vram_total_mb.saturating_sub(self.vram_used_mb)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GpuSnapshot {
    pub backend: GpuBackend,
    pub devices: Vec<GpuDeviceSnapshot>,
}

impl GpuSnapshot {
    pub fn empty(backend: GpuBackend) -> Self {
        Self {
            backend,
            devices: Vec::new(),
        }
    }

    pub fn has_gpu(&self) -> bool {
        !self.devices.is_empty()
    }

    pub fn total_vram_free_mb(&self) -> u64 {
        self.devices.iter().map(|d| d.vram_free_mb()).sum()
    }
}

/// Trait every backend implements. Snapshots are *cheap* — the contract
/// is sub-millisecond per call so the back-pressure loop can poll them
/// at 10 Hz without measurable overhead.
pub trait GpuMonitor: Send + Sync {
    fn backend(&self) -> GpuBackend;
    fn snapshot(&self) -> GpuSnapshot;
}

// ---------------------------------------------------------------------------
// Backends
// ---------------------------------------------------------------------------

/// Backend that always reports "no GPU". Used on CPU-only hosts and as
/// the safe fallback when NVML init fails.
#[derive(Clone, Debug, Default)]
pub struct NoopMonitor;

impl GpuMonitor for NoopMonitor {
    fn backend(&self) -> GpuBackend {
        GpuBackend::Noop
    }
    fn snapshot(&self) -> GpuSnapshot {
        GpuSnapshot::empty(GpuBackend::Noop)
    }
}

/// NVML-backed monitor. Holds an NVML handle for the lifetime of the
/// process — initializing NVML is expensive (~10 ms) so we do it once.
#[cfg(feature = "_nvml_disabled")] // Compile-out the implementation: NVML
                                   // libraries aren't always present on
                                   // dev machines. The fallback below
                                   // covers the common case (cfg-stub).
pub struct NvmlMonitor;

#[cfg(not(feature = "_nvml_disabled"))]
pub struct NvmlMonitor {
    handle: parking_lot::Mutex<nvml_wrapper::Nvml>,
}

#[cfg(not(feature = "_nvml_disabled"))]
impl NvmlMonitor {
    /// Initialize NVML. Returns `Err` when the NVIDIA driver isn't
    /// present — callers should fall through to [`NoopMonitor`].
    pub fn try_new() -> Result<Self, GpuMonitorError> {
        let nvml =
            nvml_wrapper::Nvml::init().map_err(|e| GpuMonitorError::NvmlInit(e.to_string()))?;
        Ok(Self {
            handle: parking_lot::Mutex::new(nvml),
        })
    }
}

#[cfg(not(feature = "_nvml_disabled"))]
impl GpuMonitor for NvmlMonitor {
    fn backend(&self) -> GpuBackend {
        GpuBackend::Nvml
    }
    fn snapshot(&self) -> GpuSnapshot {
        let nvml = self.handle.lock();
        let count = match nvml.device_count() {
            Ok(n) => n,
            Err(e) => {
                warn!("nvml_device_count_failed: {e}");
                return GpuSnapshot::empty(GpuBackend::Nvml);
            }
        };
        let mut devices = Vec::with_capacity(count as usize);
        for i in 0..count {
            match nvml.device_by_index(i) {
                Ok(dev) => {
                    let name = dev.name().unwrap_or_else(|_| format!("gpu-{i}"));
                    let mem = match dev.memory_info() {
                        Ok(m) => m,
                        Err(e) => {
                            warn!("nvml_memory_info_failed device={i}: {e}");
                            continue;
                        }
                    };
                    let util = dev.utilization_rates().map(|u| u.gpu as f32).unwrap_or(0.0);
                    devices.push(GpuDeviceSnapshot {
                        index: i,
                        name,
                        vram_total_mb: mem.total / (1024 * 1024),
                        vram_used_mb: mem.used / (1024 * 1024),
                        util_pct: util,
                    });
                }
                Err(e) => warn!("nvml_device_by_index failed i={i}: {e}"),
            }
        }
        GpuSnapshot {
            backend: GpuBackend::Nvml,
            devices,
        }
    }
}

/// ROCm and Metal placeholders. The crate dependencies for those
/// backends aren't pulled in yet (Plan Maestro §13.T8.2 marks them as
/// follow-up work); they currently behave like [`NoopMonitor`] but
/// report their own backend tag so the metrics endpoint shows the
/// intent.
#[derive(Clone, Debug, Default)]
pub struct RocmMonitor;

impl GpuMonitor for RocmMonitor {
    fn backend(&self) -> GpuBackend {
        GpuBackend::Rocm
    }
    fn snapshot(&self) -> GpuSnapshot {
        GpuSnapshot::empty(GpuBackend::Rocm)
    }
}

#[derive(Clone, Debug, Default)]
pub struct MetalMonitor;

impl GpuMonitor for MetalMonitor {
    fn backend(&self) -> GpuBackend {
        GpuBackend::Metal
    }
    fn snapshot(&self) -> GpuSnapshot {
        GpuSnapshot::empty(GpuBackend::Metal)
    }
}

// ---------------------------------------------------------------------------
// Detection
// ---------------------------------------------------------------------------

/// Try every known backend and return the first one that loads. Always
/// succeeds — on hosts without any GPU lib, returns a [`NoopMonitor`].
pub fn detect() -> Arc<dyn GpuMonitor> {
    #[cfg(not(feature = "_nvml_disabled"))]
    {
        if let Ok(nvml) = NvmlMonitor::try_new() {
            return Arc::new(nvml);
        }
    }
    // TODO(rocm,metal): wire ROCm-smi and IOKit when those backends land.
    Arc::new(NoopMonitor)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noop_reports_no_devices() {
        let m = NoopMonitor;
        assert_eq!(m.backend(), GpuBackend::Noop);
        let snap = m.snapshot();
        assert_eq!(snap.backend, GpuBackend::Noop);
        assert!(!snap.has_gpu());
        assert_eq!(snap.total_vram_free_mb(), 0);
    }

    #[test]
    fn rocm_and_metal_are_noop_with_correct_tag() {
        assert_eq!(RocmMonitor.snapshot().backend, GpuBackend::Rocm);
        assert_eq!(MetalMonitor.snapshot().backend, GpuBackend::Metal);
    }

    #[test]
    fn detect_never_panics_on_cpu_only_host() {
        let monitor = detect();
        let _ = monitor.snapshot();
    }

    #[test]
    fn device_free_subtracts_used_from_total() {
        let dev = GpuDeviceSnapshot {
            index: 0,
            name: "Test".into(),
            vram_total_mb: 24_000,
            vram_used_mb: 18_500,
            util_pct: 87.0,
        };
        assert_eq!(dev.vram_free_mb(), 5_500);
    }

    #[test]
    fn snapshot_round_trips_through_json() {
        let snap = GpuSnapshot {
            backend: GpuBackend::Nvml,
            devices: vec![GpuDeviceSnapshot {
                index: 0,
                name: "NVIDIA RTX 4090".into(),
                vram_total_mb: 24564,
                vram_used_mb: 4000,
                util_pct: 12.0,
            }],
        };
        let json = serde_json::to_string(&snap).unwrap();
        let back: GpuSnapshot = serde_json::from_str(&json).unwrap();
        assert_eq!(snap, back);
    }

    #[test]
    fn backend_serializes_as_kebab_case() {
        assert_eq!(
            serde_json::to_string(&GpuBackend::Nvml).unwrap(),
            "\"nvml\""
        );
        assert_eq!(
            serde_json::to_string(&GpuBackend::Noop).unwrap(),
            "\"noop\""
        );
        assert_eq!(
            serde_json::to_string(&GpuBackend::Rocm).unwrap(),
            "\"rocm\""
        );
        assert_eq!(
            serde_json::to_string(&GpuBackend::Metal).unwrap(),
            "\"metal\""
        );
    }
}

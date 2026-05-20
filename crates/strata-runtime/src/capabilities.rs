//! Detect runtime capabilities so the rest of the pipeline can degrade
//! gracefully on CPU-only hosts.
//!
//! Plan Maestro §13.T8.6 — the same binary should boot on a corp laptop
//! without GPU and on a 4-way RTX 4090 rig. Behaviour bifurcates here:
//!
//! - When [`Capabilities::has_gpu`] is `false`, the OCR cascade in the
//!   Python side prefers Tesseract over Surya, the bridge halves
//!   `max_concurrency` (CPU OCR is single-threaded per process), and
//!   the resource guard skips VRAM checks.
//! - The Triage profile defaults change: borderless tables stay native
//!   instead of going to VLM (the latter would queue indefinitely on
//!   the CPU Ollama).
//!
//! This module is **side-effect-free** — it observes the host, returns
//! a snapshot, never mutates global state. Anyone needing to act on the
//! capabilities should compose with [`crate::Metrics`] and the Triage
//! profile selection on top.

use serde::{Deserialize, Serialize};

use crate::gpu_monitor::{detect, GpuBackend, GpuSnapshot};

/// Suggested OCR backend ordering given the current host. The Python
/// service consults this through gRPC reflection so the cascade picks
/// the right primary without the runtime asking.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OcrPreference {
    /// GPU-accelerated Surya, then Tesseract, then Ollama VLM.
    SuryaPrimary,
    /// Tesseract first (CPU-only host), then Ollama VLM if available.
    TesseractPrimary,
}

/// Suggested Triage profile when none is explicitly configured.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SuggestedProfile {
    /// "balanced" — production default on GPU hosts.
    Balanced,
    /// "fast" — no VLM coverage; for CPU-only / cold-cache benchmarks.
    Fast,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Capabilities {
    /// Whether at least one usable GPU is reachable.
    pub has_gpu: bool,
    /// Sum of total VRAM across reachable GPUs (MB). Zero on CPU-only.
    pub total_vram_mb: u64,
    /// Number of logical CPU cores. Drives the default scheduler limit.
    pub cpu_cores: usize,
    pub ocr_preference: OcrPreference,
    pub suggested_profile: SuggestedProfile,
    pub gpu_backend: GpuBackend,
}

impl Capabilities {
    /// Inspect the host. Always succeeds — falls back to "noop GPU,
    /// suggest fast profile" when nothing is detected.
    pub fn detect() -> Self {
        let monitor = detect();
        let snapshot = monitor.snapshot();
        Self::from_snapshot(&snapshot, num_cpus())
    }

    /// Useful for tests — inject a synthesized snapshot.
    pub fn from_snapshot(snapshot: &GpuSnapshot, cpu_cores: usize) -> Self {
        let has_gpu = snapshot.has_gpu();
        let total_vram_mb: u64 = snapshot.devices.iter().map(|d| d.vram_total_mb).sum();
        let ocr_preference = if has_gpu {
            OcrPreference::SuryaPrimary
        } else {
            OcrPreference::TesseractPrimary
        };
        let suggested_profile = if has_gpu && total_vram_mb >= 8_000 {
            SuggestedProfile::Balanced
        } else {
            SuggestedProfile::Fast
        };
        Self {
            has_gpu,
            total_vram_mb,
            cpu_cores: cpu_cores.max(1),
            ocr_preference,
            suggested_profile,
            gpu_backend: snapshot.backend,
        }
    }

    /// Reasonable default for `max_concurrent_pages` given the host.
    /// Trades CPU saturation against memory pressure.
    pub fn suggested_concurrency(&self) -> usize {
        if self.has_gpu {
            self.cpu_cores.max(2)
        } else {
            // CPU-only path is OCR-bound; over-subscribing thrashes.
            (self.cpu_cores / 2).max(1)
        }
    }
}

fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gpu_monitor::{GpuDeviceSnapshot, GpuSnapshot};

    fn gpu_snap_with(total_mb: u64) -> GpuSnapshot {
        GpuSnapshot {
            backend: GpuBackend::Nvml,
            devices: vec![GpuDeviceSnapshot {
                index: 0,
                name: "RTX 4090".into(),
                vram_total_mb: total_mb,
                vram_used_mb: 0,
                util_pct: 0.0,
            }],
        }
    }

    fn cpu_only() -> GpuSnapshot {
        GpuSnapshot::empty(GpuBackend::Noop)
    }

    #[test]
    fn cpu_only_host_prefers_tesseract_and_fast() {
        let caps = Capabilities::from_snapshot(&cpu_only(), 8);
        assert!(!caps.has_gpu);
        assert_eq!(caps.total_vram_mb, 0);
        assert_eq!(caps.ocr_preference, OcrPreference::TesseractPrimary);
        assert_eq!(caps.suggested_profile, SuggestedProfile::Fast);
        assert_eq!(caps.gpu_backend, GpuBackend::Noop);
    }

    #[test]
    fn big_gpu_host_prefers_surya_and_balanced() {
        let caps = Capabilities::from_snapshot(&gpu_snap_with(24_000), 16);
        assert!(caps.has_gpu);
        assert_eq!(caps.total_vram_mb, 24_000);
        assert_eq!(caps.ocr_preference, OcrPreference::SuryaPrimary);
        assert_eq!(caps.suggested_profile, SuggestedProfile::Balanced);
        assert_eq!(caps.gpu_backend, GpuBackend::Nvml);
    }

    #[test]
    fn small_gpu_falls_back_to_fast_profile() {
        // 4 GB integrated GPU — Surya can still load, but VLM-coverage
        // recommends the "fast" profile.
        let caps = Capabilities::from_snapshot(&gpu_snap_with(4_000), 8);
        assert!(caps.has_gpu);
        assert_eq!(caps.ocr_preference, OcrPreference::SuryaPrimary);
        assert_eq!(caps.suggested_profile, SuggestedProfile::Fast);
    }

    #[test]
    fn suggested_concurrency_halves_on_cpu_only() {
        let caps = Capabilities::from_snapshot(&cpu_only(), 8);
        assert_eq!(caps.suggested_concurrency(), 4); // 8 / 2.
    }

    #[test]
    fn suggested_concurrency_at_least_one() {
        let caps = Capabilities::from_snapshot(&cpu_only(), 1);
        assert!(caps.suggested_concurrency() >= 1);
    }

    #[test]
    fn detect_never_panics() {
        let _ = Capabilities::detect();
    }

    #[test]
    fn round_trip_through_json() {
        let caps = Capabilities::from_snapshot(&gpu_snap_with(16_000), 16);
        let json = serde_json::to_string(&caps).unwrap();
        let back: Capabilities = serde_json::from_str(&json).unwrap();
        assert_eq!(caps, back);
    }
}

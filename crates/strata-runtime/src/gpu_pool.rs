//! Multi-GPU worker pool configuration.
//!
//! Plan Maestro §13.T8.4 — when the host has more than one GPU, each
//! Python IA worker should be pinned to one device so the model loads
//! once per GPU instead of once per process. We don't *spawn* the
//! workers from this module (that's `strata-ia-bridge::embedded` for the
//! wheel path, or Kubernetes / systemd in production); we *describe*
//! them with the correct env vars so any spawner can apply them.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::gpu_monitor::{GpuMonitor, GpuSnapshot};

/// One worker's per-process env. Apply via
/// `Command::envs(spec.env.iter())` before spawning Python.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerSpec {
    /// 0-indexed GPU the worker owns.
    pub gpu_index: u32,
    /// Friendly name for log lines: "rtx-4090-0".
    pub label: String,
    /// Environment variables the spawner must set.
    pub env: HashMap<String, String>,
}

#[derive(Clone, Debug)]
pub struct PoolConfig {
    /// Number of Python workers per GPU. ≥ 2 lets you keep VRAM warm
    /// while one worker is busy in the model's forward pass; 1 is the
    /// safe default for big VLMs.
    pub workers_per_gpu: usize,
    /// Restrict the pool to a subset of GPU indices (e.g. `vec![0, 2]`
    /// when ML team owns GPUs 1 and 3). Empty = all visible GPUs.
    pub gpu_allowlist: Vec<u32>,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self { workers_per_gpu: 1, gpu_allowlist: Vec::new() }
    }
}

/// Build the worker specs from a [`GpuSnapshot`]. Returns an empty
/// `Vec` on CPU-only hosts — the caller's spawner should then start a
/// single CPU-bound worker without GPU env vars.
pub fn plan_workers(snap: &GpuSnapshot, config: &PoolConfig) -> Vec<WorkerSpec> {
    let mut specs = Vec::new();
    for device in &snap.devices {
        if !config.gpu_allowlist.is_empty() && !config.gpu_allowlist.contains(&device.index) {
            continue;
        }
        for replica in 0..config.workers_per_gpu.max(1) {
            let mut env = HashMap::new();
            // The canonical pin var for PyTorch / TF / Surya / Ollama.
            env.insert("CUDA_VISIBLE_DEVICES".to_string(), device.index.to_string());
            // ROCm equivalent — harmless on NVIDIA hosts.
            env.insert("HIP_VISIBLE_DEVICES".to_string(), device.index.to_string());
            // Strata-side label used in metrics / logs.
            env.insert("STRATA_WORKER_GPU".to_string(), device.index.to_string());
            env.insert("STRATA_WORKER_REPLICA".to_string(), replica.to_string());

            specs.push(WorkerSpec {
                gpu_index: device.index,
                label: format!("{}-{}-r{}", sanitize(&device.name), device.index, replica),
                env,
            });
        }
    }
    specs
}

/// Snapshot helper for the metrics endpoint. Returns a stable string
/// representation that doesn't change for the same input.
pub fn describe_pool(specs: &[WorkerSpec]) -> String {
    let mut lines: Vec<String> = specs
        .iter()
        .map(|s| format!("{} -> gpu {} ({} env vars)", s.label, s.gpu_index, s.env.len()))
        .collect();
    lines.sort();
    lines.join("\n")
}

fn sanitize(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() { c.to_ascii_lowercase() } else { '-' })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

/// Convenience: build a pool from a [`GpuMonitor`].
pub fn plan_from_monitor(monitor: &dyn GpuMonitor, config: &PoolConfig) -> Vec<WorkerSpec> {
    plan_workers(&monitor.snapshot(), config)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gpu_monitor::{GpuBackend, GpuDeviceSnapshot};

    fn snap(devs: Vec<u32>) -> GpuSnapshot {
        GpuSnapshot {
            backend: GpuBackend::Nvml,
            devices: devs
                .into_iter()
                .map(|i| GpuDeviceSnapshot {
                    index: i,
                    name: format!("NVIDIA RTX 4090 #{i}"),
                    vram_total_mb: 24_000,
                    vram_used_mb: 0,
                    util_pct: 0.0,
                })
                .collect(),
        }
    }

    #[test]
    fn cpu_only_host_yields_empty_pool() {
        let s = GpuSnapshot::empty(GpuBackend::Noop);
        let workers = plan_workers(&s, &PoolConfig::default());
        assert!(workers.is_empty());
    }

    #[test]
    fn one_worker_per_gpu_by_default() {
        let s = snap(vec![0, 1, 2]);
        let workers = plan_workers(&s, &PoolConfig::default());
        assert_eq!(workers.len(), 3);
        for (i, w) in workers.iter().enumerate() {
            assert_eq!(w.gpu_index, i as u32);
            assert_eq!(w.env["CUDA_VISIBLE_DEVICES"], i.to_string());
            assert_eq!(w.env["HIP_VISIBLE_DEVICES"], i.to_string());
        }
    }

    #[test]
    fn multiple_replicas_per_gpu() {
        let s = snap(vec![0]);
        let workers =
            plan_workers(&s, &PoolConfig { workers_per_gpu: 4, gpu_allowlist: Vec::new() });
        assert_eq!(workers.len(), 4);
        for (replica, w) in workers.iter().enumerate() {
            assert_eq!(w.gpu_index, 0);
            assert_eq!(w.env["STRATA_WORKER_REPLICA"], replica.to_string());
            assert!(w.label.ends_with(&format!("r{replica}")));
        }
    }

    #[test]
    fn allowlist_filters_gpus() {
        let s = snap(vec![0, 1, 2, 3]);
        let workers = plan_workers(
            &s,
            &PoolConfig { workers_per_gpu: 1, gpu_allowlist: vec![1, 3] },
        );
        assert_eq!(workers.len(), 2);
        let used_gpus: Vec<u32> = workers.iter().map(|w| w.gpu_index).collect();
        assert_eq!(used_gpus, vec![1, 3]);
    }

    #[test]
    fn labels_are_sanitized() {
        let s = snap(vec![0]);
        let workers = plan_workers(&s, &PoolConfig::default());
        // "NVIDIA RTX 4090 #0" → "nvidia-rtx-4090-0" + "-0-r0".
        assert!(workers[0].label.contains("nvidia-rtx-4090"));
        assert!(!workers[0].label.contains('#'));
        assert!(!workers[0].label.contains(' '));
    }

    #[test]
    fn describe_pool_is_deterministic() {
        let s = snap(vec![0, 1]);
        let workers = plan_workers(&s, &PoolConfig::default());
        let a = describe_pool(&workers);
        let b = describe_pool(&workers);
        assert_eq!(a, b);
        assert!(a.contains("gpu 0"));
        assert!(a.contains("gpu 1"));
    }

    #[test]
    fn worker_spec_round_trip_json() {
        let s = snap(vec![0]);
        let workers = plan_workers(&s, &PoolConfig::default());
        let json = serde_json::to_string(&workers[0]).unwrap();
        let back: WorkerSpec = serde_json::from_str(&json).unwrap();
        assert_eq!(workers[0], back);
    }
}

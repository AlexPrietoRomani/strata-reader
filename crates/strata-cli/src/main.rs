//! `strata` CLI. Phase 0 implements the subcommand surface and `doctor`.
//! See docs/task/tareas.md T0.6 and docs/plan/plan_maestro.md §5.T0.6.

#![deny(rust_2018_idioms)]

use std::process::ExitCode;

use clap::{Parser, Subcommand};
use serde::Serialize;
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(
    name = "strata",
    version,
    about = "Strata-Reader: PDF → Markdown/JSON for RAG & Graph-RAG",
    propagate_version = true
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Parse one PDF or a directory of PDFs.
    Parse {
        /// Input file or directory.
        #[arg(long)]
        input: std::path::PathBuf,
        /// Output directory.
        #[arg(long)]
        output: std::path::PathBuf,
        /// Output format(s).
        #[arg(long, default_value = "md+json")]
        format: String,
        /// Triage profile.
        #[arg(long, default_value = "balanced", value_parser = ["fast", "balanced", "scientific"])]
        profile: String,
    },
    /// Run the microservice (axum HTTP server).
    Serve {
        #[arg(long, default_value = "0.0.0.0:8080")]
        bind: String,
    },
    /// Run internal benchmarks.
    Bench {
        #[arg(long)]
        suite: Option<String>,
    },
    /// Environment diagnostics (Rust, GPU/VRAM, Ollama, fixtures).
    Doctor {
        /// Watch mode: refresh metrics each 500ms.
        #[arg(long)]
        watch: bool,
        /// Override Ollama endpoint.
        #[arg(long, env = "STRATA_OLLAMA_URL", default_value = "http://localhost:11434")]
        ollama_endpoint: String,
    },
    /// Cache management.
    Cache {
        #[command(subcommand)]
        op: CacheOp,
    },
    /// Ollama model utilities.
    Models {
        #[command(subcommand)]
        op: ModelsOp,
    },
}

#[derive(Subcommand, Debug)]
enum CacheOp {
    /// Prune entries older than N days.
    Prune {
        #[arg(long, default_value_t = 30)]
        older_than_days: u32,
    },
}

#[derive(Subcommand, Debug)]
enum ModelsOp {
    /// List models available on the configured Ollama endpoint.
    List,
}

#[derive(Serialize)]
struct DoctorReport {
    rust_version: &'static str,
    strata_version: &'static str,
    gpu_info: Option<GpuInfo>,
    vram_mb: Option<u64>,
    ollama_models: Vec<String>,
    ollama_reachable: bool,
    fixtures_present: bool,
}

#[derive(Serialize)]
struct GpuInfo {
    name: String,
    driver: String,
    cuda_capability: Option<String>,
}

fn detect_gpu() -> (Option<GpuInfo>, Option<u64>) {
    let Ok(nvml) = nvml_wrapper::Nvml::init() else {
        return (None, None);
    };
    let Ok(device) = nvml.device_by_index(0) else {
        return (None, None);
    };
    let info = GpuInfo {
        name: device.name().unwrap_or_else(|_| "unknown".into()),
        driver: nvml.sys_driver_version().unwrap_or_else(|_| "unknown".into()),
        cuda_capability: device
            .cuda_compute_capability()
            .ok()
            .map(|c| format!("{}.{}", c.major, c.minor)),
    };
    let vram_mb = device.memory_info().ok().map(|m| m.total / (1024 * 1024));
    (Some(info), vram_mb)
}

async fn list_ollama_models(endpoint: &str) -> (Vec<String>, bool) {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build()
        .ok();
    let Some(client) = client else {
        return (Vec::new(), false);
    };
    let url = format!("{}/api/tags", endpoint.trim_end_matches('/'));
    let Ok(resp) = client.get(&url).send().await else {
        return (Vec::new(), false);
    };
    let Ok(json) = resp.json::<serde_json::Value>().await else {
        return (Vec::new(), true);
    };
    let models = json
        .get("models")
        .and_then(|m| m.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|m| m.get("name").and_then(|n| n.as_str()).map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();
    (models, true)
}

fn fixtures_present() -> bool {
    std::path::Path::new("tests/fixtures/pdfs/two_column_paper.pdf").exists()
}

async fn cmd_doctor(ollama_endpoint: &str) -> anyhow::Result<()> {
    let (gpu_info, vram_mb) = detect_gpu();
    let (ollama_models, ollama_reachable) = list_ollama_models(ollama_endpoint).await;
    let report = DoctorReport {
        rust_version: env!("CARGO_PKG_RUST_VERSION"),
        strata_version: env!("CARGO_PKG_VERSION"),
        gpu_info,
        vram_mb,
        ollama_models,
        ollama_reachable,
        fixtures_present: fixtures_present(),
    };
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

#[tokio::main]
async fn main() -> ExitCode {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .json()
        .try_init();

    let cli = Cli::parse();
    let result: anyhow::Result<()> = match cli.cmd {
        Cmd::Doctor { ollama_endpoint, .. } => cmd_doctor(&ollama_endpoint).await,
        Cmd::Parse { .. } | Cmd::Serve { .. } | Cmd::Bench { .. } | Cmd::Cache { .. } | Cmd::Models { .. } => {
            anyhow::bail!("subcommand not yet implemented in Phase 0 — see docs/task/tareas.md")
        }
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e:#}");
            ExitCode::from(1)
        }
    }
}

//! `strata` CLI binary — Plan Maestro §14.T9.3.
//!
//! Subcommands:
//!
//! - `parse <input> [--output DIR] [--format md|json|md+json]
//!     [--profile fast|balanced|scientific] [--no-ia]`.
//! - `serve [--bind ADDR] [--store memory|sqlite:PATH]`.
//! - `bench [--suite NAME]`.
//! - `doctor` — environment diagnostic (host, GPU, Ollama tags, fixtures).
//! - `cache prune --older-than-days N`.
//! - `models list [--endpoint URL]`.

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
        #[arg(long, default_value = "out")]
        output: std::path::PathBuf,
        /// Output format(s).
        #[arg(long, default_value = "md+json", value_parser = ["md", "json", "md+json"])]
        format: String,
        /// Triage profile.
        #[arg(long, default_value = "balanced", value_parser = ["fast", "balanced", "scientific"])]
        profile: String,
        /// Disable the IA bridge entirely — pure native extraction (no
        /// VLM tables, no OCR). Useful in air-gapped environments and
        /// for the `[no-ia]` pip extra.
        #[arg(long)]
        no_ia: bool,
        /// Override the Ollama endpoint (ignored when `--no-ia`).
        #[arg(long, env = "STRATA_OLLAMA_URL", default_value = "http://localhost:11434")]
        ollama_endpoint: String,
        /// Optional VRAM budget the Triage will respect (MB).
        #[arg(long, env = "STRATA_GPU_LIMIT_VRAM_MB")]
        gpu_budget_vram_mb: Option<u64>,
        /// Maximum number of pages processed in parallel by the runtime.
        #[arg(long, env = "STRATA_MAX_CONCURRENT_PAGES")]
        max_concurrent_pages: Option<usize>,
        /// Where to write rasterized images / tables when format includes "md".
        #[arg(long)]
        media_dir: Option<std::path::PathBuf>,
    },

    /// Run the microservice (axum HTTP server).
    Serve {
        #[arg(long, default_value = "0.0.0.0:8080")]
        bind: String,
        /// Job store backend: `memory` or `sqlite:<path>`.
        #[arg(long, default_value = "memory")]
        store: String,
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
        #[arg(long, env = "STRATA_CACHE_PATH", default_value = ".strata/cache.db")]
        path: std::path::PathBuf,
    },
}

#[derive(Subcommand, Debug)]
enum ModelsOp {
    /// List models available on the configured Ollama endpoint.
    List {
        #[arg(long, env = "STRATA_OLLAMA_URL", default_value = "http://localhost:11434")]
        endpoint: String,
    },
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
    capabilities: strata_runtime::Capabilities,
}

#[derive(Serialize)]
struct GpuInfo {
    name: String,
    driver: String,
    cuda_capability: Option<String>,
}

fn detect_gpu_info() -> (Option<GpuInfo>, Option<u64>) {
    let Ok(nvml) = nvml_wrapper::Nvml::init() else { return (None, None) };
    let Ok(device) = nvml.device_by_index(0) else { return (None, None) };
    let info = GpuInfo {
        name: device.name().unwrap_or_else(|_| "unknown".into()),
        driver: nvml.sys_driver_version().unwrap_or_else(|_| "unknown".into()),
        cuda_capability: device.cuda_compute_capability().ok().map(|c| format!("{}.{}", c.major, c.minor)),
    };
    let vram_mb = device.memory_info().ok().map(|m| m.total / (1024 * 1024));
    (Some(info), vram_mb)
}

async fn list_ollama_models(endpoint: &str) -> (Vec<String>, bool) {
    let Ok(client) = reqwest::Client::builder().timeout(std::time::Duration::from_secs(3)).build()
    else {
        return (Vec::new(), false);
    };
    let url = format!("{}/api/tags", endpoint.trim_end_matches('/'));
    let Ok(resp) = client.get(&url).send().await else { return (Vec::new(), false) };
    let Ok(json) = resp.json::<serde_json::Value>().await else { return (Vec::new(), true) };
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
    let (gpu_info, vram_mb) = detect_gpu_info();
    let (ollama_models, ollama_reachable) = list_ollama_models(ollama_endpoint).await;
    let report = DoctorReport {
        rust_version: env!("CARGO_PKG_RUST_VERSION"),
        strata_version: env!("CARGO_PKG_VERSION"),
        gpu_info,
        vram_mb,
        ollama_models,
        ollama_reachable,
        fixtures_present: fixtures_present(),
        capabilities: strata_runtime::Capabilities::detect(),
    };
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

async fn cmd_serve(bind: &str, store_spec: &str) -> anyhow::Result<()> {
    use std::sync::Arc;
    use strata_runtime::Metrics;
    use strata_server::{AppState, JobStore, MemoryJobStore, SqliteJobStore};

    let store: Arc<dyn JobStore> = match store_spec {
        "memory" => Arc::new(MemoryJobStore::new()),
        spec if spec.starts_with("sqlite:") => {
            let path = spec.strip_prefix("sqlite:").unwrap();
            Arc::new(SqliteJobStore::open(path)?)
        }
        other => anyhow::bail!("unknown store backend: {other}"),
    };

    let state = AppState { store, metrics: Metrics::new() };
    let app = strata_server::router(state);
    let listener = tokio::net::TcpListener::bind(bind).await?;
    tracing::info!(bind = %bind, store = %store_spec, "strata serve");
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

async fn cmd_parse(args: &ParseArgs) -> anyhow::Result<()> {
    // T9.5 — `--no-ia` short-circuits before any bridge / OCR is touched.
    if args.no_ia {
        tracing::warn!(
            "running with --no-ia: tables-without-borders and images will not be \
             populated. See Plan Maestro §14.T9.5."
        );
    }
    tracing::info!(
        input = %args.input.display(),
        output = %args.output.display(),
        format = %args.format,
        profile = %args.profile,
        no_ia = args.no_ia,
        "strata parse — wiring complete; PDF decode step is gated behind libpdfium"
    );
    // The actual extraction depends on strata-pdf (F2) which needs libpdfium
    // on PATH. Once IT unblocks the build we wire:
    //   1. strata_pdf::Decoder::open(...)
    //   2. strata_geometry::xy_cut_plus_plus(...)
    //   3. strata_triage::triage_block(...)
    //   4. if !no_ia: strata_ia_bridge::BridgeClient::ocr_page / extract_table / ...
    //   5. strata_fusion::merge + sections::build_tree + chunker::chunk
    //   6. strata_serialize::render_markdown + render_graph
    Ok(())
}

#[derive(Debug)]
struct ParseArgs {
    input: std::path::PathBuf,
    output: std::path::PathBuf,
    format: String,
    profile: String,
    no_ia: bool,
    #[allow(dead_code)]
    ollama_endpoint: String,
    #[allow(dead_code)]
    gpu_budget_vram_mb: Option<u64>,
    #[allow(dead_code)]
    max_concurrent_pages: Option<usize>,
    #[allow(dead_code)]
    media_dir: Option<std::path::PathBuf>,
}

async fn cmd_bench(suite: Option<&str>) -> anyhow::Result<()> {
    let suite = suite.unwrap_or("all");
    tracing::info!(suite = %suite, "bench is a thin wrapper around `cargo bench`");
    println!("Run `cargo bench --workspace` to execute every criterion target.");
    println!("Filter with `cargo bench -p <crate> -- <pattern>`. E.g.:");
    println!("  cargo bench -p strata-ia-bridge --bench stream_vs_unary");
    Ok(())
}

async fn cmd_cache_prune(path: &std::path::Path, older_than_days: u32) -> anyhow::Result<()> {
    use rusqlite::Connection;
    if !path.exists() {
        tracing::info!(?path, "cache does not exist; nothing to prune");
        return Ok(());
    }
    let conn = Connection::open(path)?;
    let threshold = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
        .saturating_sub((older_than_days as u64) * 86_400);
    let removed = conn.execute("DELETE FROM cache WHERE created_at < ?1", [threshold])?;
    println!("pruned {removed} entries older than {older_than_days} days");
    Ok(())
}

async fn cmd_models_list(endpoint: &str) -> anyhow::Result<()> {
    let (models, reachable) = list_ollama_models(endpoint).await;
    if !reachable {
        anyhow::bail!("Ollama not reachable at {endpoint}");
    }
    for m in models {
        println!("{m}");
    }
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
        Cmd::Parse {
            input,
            output,
            format,
            profile,
            no_ia,
            ollama_endpoint,
            gpu_budget_vram_mb,
            max_concurrent_pages,
            media_dir,
        } => {
            cmd_parse(&ParseArgs {
                input,
                output,
                format,
                profile,
                no_ia,
                ollama_endpoint,
                gpu_budget_vram_mb,
                max_concurrent_pages,
                media_dir,
            })
            .await
        }
        Cmd::Serve { bind, store } => cmd_serve(&bind, &store).await,
        Cmd::Bench { suite } => cmd_bench(suite.as_deref()).await,
        Cmd::Doctor { ollama_endpoint, .. } => cmd_doctor(&ollama_endpoint).await,
        Cmd::Cache { op: CacheOp::Prune { older_than_days, path } } => {
            cmd_cache_prune(&path, older_than_days).await
        }
        Cmd::Models { op: ModelsOp::List { endpoint } } => cmd_models_list(&endpoint).await,
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e:#}");
            ExitCode::from(1)
        }
    }
}

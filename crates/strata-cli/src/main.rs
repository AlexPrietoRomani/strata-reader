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
        #[arg(
            long,
            env = "STRATA_OLLAMA_URL",
            default_value = "http://localhost:11434"
        )]
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
        #[arg(
            long,
            env = "STRATA_OLLAMA_URL",
            default_value = "http://localhost:11434"
        )]
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
        #[arg(
            long,
            env = "STRATA_OLLAMA_URL",
            default_value = "http://localhost:11434"
        )]
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
    let Ok(nvml) = nvml_wrapper::Nvml::init() else {
        return (None, None);
    };
    let Ok(device) = nvml.device_by_index(0) else {
        return (None, None);
    };
    let info = GpuInfo {
        name: device.name().unwrap_or_else(|_| "unknown".into()),
        driver: nvml
            .sys_driver_version()
            .unwrap_or_else(|_| "unknown".into()),
        cuda_capability: device
            .cuda_compute_capability()
            .ok()
            .map(|c| format!("{}.{}", c.major, c.minor)),
    };
    let vram_mb = device.memory_info().ok().map(|m| m.total / (1024 * 1024));
    (Some(info), vram_mb)
}

async fn list_ollama_models(endpoint: &str) -> (Vec<String>, bool) {
    let Ok(client) = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build()
    else {
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
                .filter_map(|m| {
                    m.get("name")
                        .and_then(|n| n.as_str())
                        .map(|s| s.to_string())
                })
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

    let state = AppState {
        store,
        metrics: Metrics::new(),
    };
    let app = strata_server::router(state);
    let listener = tokio::net::TcpListener::bind(bind).await?;
    tracing::info!(bind = %bind, store = %store_spec, "strata serve");
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

async fn cmd_parse(args: &ParseArgs) -> anyhow::Result<()> {
    if args.no_ia {
        tracing::warn!(
            "running with --no-ia: tables-without-borders and images will not be \
             populated. See Plan Maestro §14.T9.5."
        );
    }

    // ── 1. Open PDF ────────────────────────────────────────────────────
    let decoder = strata_pdf::Decoder::open(&args.input)
        .map_err(|e| anyhow::anyhow!("failed to open PDF: {e}"))?;
    let page_count = decoder.page_count();
    tracing::info!(pages = page_count, "PDF decoded");

    // ── 2. Per-page extraction ─────────────────────────────────────────
    let mut doc_pages: Vec<std::sync::Arc<strata_core::Page>> = Vec::with_capacity(page_count);
    let sha = sha256_file(&args.input)?;

    for page_idx in 0..page_count {
        let page = decoder
            .pages()
            .get(page_idx as i32)
            .map_err(|e| anyhow::anyhow!("page {page_idx}: {e}"))?;
        let page_w = page.width().value;
        let page_h = page.height().value;

        // Raw extraction
        let glyphs = strata_pdf::extract_glyphs(&page).unwrap_or_default();
        let _paths = strata_pdf::extract_paths(&page).unwrap_or_default();
        let _images = strata_pdf::extract_images(&page).unwrap_or_default();
        let is_scan = strata_pdf::is_likely_scan(&page).unwrap_or(false);

        tracing::debug!(
            page = page_idx,
            glyphs = glyphs.len(),
            is_scan,
            "page extracted"
        );

        // ── 3. Geometry: lines → words ─────────────────────────────────
        let glyph_inputs: Vec<strata_geometry::GlyphInput> = glyphs
            .iter()
            .map(|g| strata_geometry::GlyphInput {
                bbox: g.bbox,
                font_size: g.font_size,
                unicode: g.unicode,
            })
            .collect();
        let lines = strata_geometry::cluster_lines(&glyph_inputs);

        // ── 4. Build blocks from lines ─────────────────────────────────
        let mut blocks: Vec<strata_core::Block> = Vec::new();
        let font_sizes: Vec<f32> = lines
            .iter()
            .map(|l| {
                l.glyph_indices
                    .iter()
                    .map(|&i| glyph_inputs[i].font_size)
                    .sum::<f32>()
                    / l.glyph_indices.len().max(1) as f32
            })
            .collect();
        let headings = strata_geometry::classify_headings(&font_sizes);

        for (line_idx, line) in lines.iter().enumerate() {
            let words = strata_geometry::words_from_line(line, &glyph_inputs);
            let content: String = words
                .iter()
                .map(|w| w.text.as_str())
                .collect::<Vec<_>>()
                .join(" ");
            if content.trim().is_empty() {
                continue;
            }
            let kind = match &headings.get(line_idx) {
                Some(strata_geometry::HeadingClass::Heading { level }) => {
                    strata_core::BlockType::Heading { level: *level }
                }
                _ => strata_core::BlockType::Paragraph,
            };
            blocks.push(strata_core::Block {
                id: strata_core::BlockId::new(),
                kind,
                bbox: line.bbox,
                content,
                children: vec![],
                provenance: strata_core::Provenance::try_new(
                    strata_core::ProvenanceSource::Rust,
                    None,
                    1.0,
                    0,
                    0,
                )
                .unwrap(),
            });
        }

        // ── 5. Reading order via XY-Cut++ ──────────────────────────────
        let bboxes: Vec<strata_core::BBox> = blocks.iter().map(|b| b.bbox).collect();
        let order =
            strata_geometry::xy_cut_plus_plus(&bboxes, strata_geometry::XyCutConfig::default());
        let reading_order: Vec<strata_core::BlockId> =
            order.iter().map(|&i| blocks[i].id).collect();

        let block_arcs: Vec<std::sync::Arc<strata_core::Block>> =
            blocks.into_iter().map(std::sync::Arc::new).collect();

        let media_box = strata_core::BBox::new(0.0, 0.0, page_w, page_h)
            .unwrap_or(strata_core::BBox::new(0.0, 0.0, 595.0, 842.0).unwrap());

        doc_pages.push(std::sync::Arc::new(strata_core::Page {
            number: (page_idx + 1) as u32,
            size: strata_core::Size::new(page_w, page_h)
                .unwrap_or(strata_core::Size::new(595.0, 842.0).unwrap()),
            orientation: if page_w > page_h {
                strata_core::PageOrientation::Landscape
            } else {
                strata_core::PageOrientation::Portrait
            },
            blocks: block_arcs,
            reading_order,
            media_box,
        }));
    }

    // ── 6. Build Document ──────────────────────────────────────────────
    let mut doc = strata_core::Document::new(strata_core::DocMeta {
        source_sha256: sha,
        source_filename: args
            .input
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default(),
        schema_version: strata_core::version().to_string(),
        profile: args.profile.clone(),
        extra: std::collections::BTreeMap::new(),
    });
    doc.pages = doc_pages;

    // ── 7. Serialize ───────────────────────────────────────────────────
    std::fs::create_dir_all(&args.output)?;

    let stem = args
        .input
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "output".into());

    if args.format.contains("md") || args.format == "md+json" {
        let md =
            strata_serialize::render_markdown(&doc, &strata_serialize::MarkdownOptions::default());
        let md_path = args.output.join(format!("{stem}.md"));
        std::fs::write(&md_path, &md)?;
        tracing::info!(path = %md_path.display(), "markdown written");
    }

    if args.format.contains("json") || args.format == "md+json" {
        let graph = strata_serialize::render_graph(&doc);
        let json = serde_json::to_string_pretty(&graph)?;
        let json_path = args.output.join(format!("{stem}.json"));
        std::fs::write(&json_path, &json)?;
        tracing::info!(path = %json_path.display(), "json written");
    }

    tracing::info!(pages = page_count, "parse complete");
    Ok(())
}

#[derive(Debug)]
#[allow(dead_code)]
struct ParseArgs {
    input: std::path::PathBuf,
    output: std::path::PathBuf,
    format: String,
    profile: String,
    no_ia: bool,
    ollama_endpoint: String,
    gpu_budget_vram_mb: Option<u64>,
    max_concurrent_pages: Option<usize>,
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
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
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
        Cmd::Doctor {
            ollama_endpoint, ..
        } => cmd_doctor(&ollama_endpoint).await,
        Cmd::Cache {
            op:
                CacheOp::Prune {
                    older_than_days,
                    path,
                },
        } => cmd_cache_prune(&path, older_than_days).await,
        Cmd::Models {
            op: ModelsOp::List { endpoint },
        } => cmd_models_list(&endpoint).await,
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e:#}");
            ExitCode::from(1)
        }
    }
}

fn sha256_file(path: &std::path::Path) -> anyhow::Result<String> {
    use sha2::Digest;
    let bytes = std::fs::read(path)?;
    let hash = sha2::Sha256::digest(&bytes);
    Ok(format!("{hash:x}"))
}

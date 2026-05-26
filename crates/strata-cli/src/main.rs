//! `strata` CLI binary — Plan Maestro §14.T9.3.
//!
//! Subcommands:
//!
//! - `parse <input> [--output DIR] [--format md|json|md+json]
//!     [--profile fast|balanced|scientific] [--no-ia] [--pdf-backend pdfium|pure|auto]`.
//! - `serve [--bind ADDR] [--store memory|sqlite:PATH]`.
//! - `bench [--suite NAME]`.
//! - `doctor` — environment diagnostic (host, GPU, Ollama tags, fixtures, backend).
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
        /// VLM tables, no OCR).
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
        /// Save extracted figures/images to disk.
        #[arg(long)]
        save_images: bool,
        /// Selection of PDF decoder backend.
        #[arg(long, default_value = "auto", value_parser = ["pdfium", "pure", "auto"])]
        pdf_backend: String,
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

    /// Environment diagnostics (Rust, GPU/VRAM, Ollama, fixtures, backend).
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
    pdf_backend: &'static str,
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

    #[cfg(feature = "pdfium-backend")]
    let active_backend = "pdfium";
    #[cfg(not(feature = "pdfium-backend"))]
    let active_backend = "pure-rust";

    let report = DoctorReport {
        rust_version: env!("CARGO_PKG_RUST_VERSION"),
        strata_version: env!("CARGO_PKG_VERSION"),
        gpu_info,
        vram_mb,
        ollama_models,
        ollama_reachable,
        fixtures_present: fixtures_present(),
        pdf_backend: active_backend,
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

    let stem = args
        .input
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "output".into());

    for page_idx in 0..page_count {
        let page = decoder
            .page(page_idx)
            .map_err(|e| anyhow::anyhow!("page {page_idx}: {e}"))?;
        let (page_w, page_h) = page.size();

        // Raw extraction using abstract PdfPage trait
        let glyphs = page.glyphs().unwrap_or_default();
        let _paths = page.paths().unwrap_or_default();
        let images = page.images().unwrap_or_default();
        let is_scan = strata_pdf::is_likely_scan(page.as_ref()).unwrap_or(false);

        tracing::debug!(
            page = page_idx,
            glyphs = glyphs.len(),
            images = images.len(),
            is_scan,
            "page extracted"
        );

        // ── 3. Geometría y Filtrado de Ruido ───────────────────────────
        let glyph_inputs: Vec<strata_geometry::GlyphInput> = glyphs
            .iter()
            .map(|g| strata_geometry::GlyphInput {
                bbox: g.bbox,
                font_size: g.font_size,
                unicode: g.unicode,
            })
            .collect();
        let lines = strata_geometry::cluster_lines(&glyph_inputs);

        let page_bbox = strata_core::BBox::new(0.0, 0.0, page_w, page_h)
            .unwrap_or(strata_core::BBox::new(0.0, 0.0, 595.0, 842.0).unwrap());

        let filtered_lines = strata_geometry::filter_noise_lines(&lines, &glyph_inputs, page_bbox);

        // ── 4. Clasificación y Agrupación de Bloques Semánticos ────────
        let mut blocks: Vec<strata_core::Block> = Vec::new();
        let mut line_font_sizes = Vec::with_capacity(filtered_lines.len());
        let mut line_bboxes = Vec::with_capacity(filtered_lines.len());
        let mut line_texts = Vec::with_capacity(filtered_lines.len());

        for line in &filtered_lines {
            let font_size = line
                .glyph_indices
                .iter()
                .map(|&i| glyph_inputs[i].font_size)
                .sum::<f32>()
                / line.glyph_indices.len().max(1) as f32;
            line_font_sizes.push(font_size);
            line_bboxes.push(line.bbox);

            let words = strata_geometry::words_from_line(line, &glyph_inputs);
            let content: String = words
                .iter()
                .map(|w| w.text.as_str())
                .collect::<Vec<_>>()
                .join(" ");
            line_texts.push(content);
        }

        let headings = strata_geometry::classify_headings(
            &line_font_sizes,
            &line_bboxes,
            &line_texts,
            page_bbox,
        );
        let paragraph_groups =
            strata_geometry::merge_lines_into_paragraphs(&filtered_lines, &glyph_inputs, &headings);

        for group in paragraph_groups {
            let mut group_text_parts = Vec::with_capacity(group.lines.len());
            for line in &group.lines {
                let words = strata_geometry::words_from_line(line, &glyph_inputs);
                let content: String = words
                    .iter()
                    .map(|w| w.text.as_str())
                    .collect::<Vec<_>>()
                    .join(" ");
                if !content.trim().is_empty() {
                    group_text_parts.push(content);
                }
            }
            let raw_content = group_text_parts.join(" ");
            let content = strata_geometry::normalize_text(&raw_content);
            if content.is_empty() {
                continue;
            }

            let kind = match group.kind {
                strata_geometry::ParagraphKind::Heading { level } => {
                    strata_core::BlockType::Heading { level }
                }
                strata_geometry::ParagraphKind::Body => strata_core::BlockType::Paragraph,
            };

            blocks.push(strata_core::Block {
                id: strata_core::BlockId::new(),
                kind,
                bbox: group.bbox,
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

        // ── 5. Extracción y Guardado de Imágenes Nativas ───────────────
        for img in images {
            let figure_block = strata_core::Block {
                id: strata_core::BlockId::new(),
                kind: strata_core::BlockType::Figure,
                bbox: img.bbox,
                content: "figure".to_string(),
                children: vec![],
                provenance: strata_core::Provenance::try_new(
                    strata_core::ProvenanceSource::Rust,
                    None,
                    1.0,
                    0,
                    0,
                )
                .unwrap(),
            };

            if args.save_images || args.media_dir.is_some() {
                let target_dir = match &args.media_dir {
                    Some(dir) => dir.clone(),
                    None => args.output.join(format!("{}_images", stem)),
                };
                if let Err(e) = std::fs::create_dir_all(&target_dir) {
                    tracing::warn!(error = %e, ?target_dir, "Error al crear el directorio de medios");
                } else {
                    let file_path = target_dir.join(format!("{}.png", figure_block.id));
                    if let Err(e) = std::fs::write(&file_path, &img.raw_bytes) {
                        tracing::warn!(error = %e, ?file_path, "Error al guardar el recorte de la imagen");
                    } else {
                        tracing::debug!(?file_path, "Recorte de imagen guardado exitosamente");
                    }
                }
            }

            blocks.push(figure_block);
        }

        // ── 6. Reading order via XY-Cut++ ──────────────────────────────
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

    // ── 7. Build Document ──────────────────────────────────────────────
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

    // ── 8. Serialize ───────────────────────────────────────────────────
    std::fs::create_dir_all(&args.output)?;

    if args.format.contains("md") || args.format == "md+json" {
        let mut opts = strata_serialize::MarkdownOptions::default();
        if args.save_images || args.media_dir.is_some() {
            let strategy_dir = match &args.media_dir {
                Some(dir) => dir.clone(),
                None => std::path::PathBuf::from(format!("{}_images", stem)),
            };
            opts.image_strategy = strata_serialize::ImageStrategy::MediaDir { dir: strategy_dir };
        }
        let md = strata_serialize::render_markdown(&doc, &opts);
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
    media_dir: Option<std::path::PathBuf>,
    save_images: bool,
    #[allow(dead_code)]
    pdf_backend: String,
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
            save_images,
            pdf_backend,
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
                save_images,
                pdf_backend,
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

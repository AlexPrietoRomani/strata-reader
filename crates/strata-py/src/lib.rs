//! PyO3 bindings — the `strata_reader._native` extension module.
//!
//! Public surface (Plan Maestro §14.T9.4):
//!
//! ```python
//! from strata_reader import parse, parse_batch, ParseOptions, version
//! doc = parse("paper.pdf", options=ParseOptions(profile="scientific", use_ia=True))
//! md   = doc.to_markdown()
//! data = doc.to_graph_json()
//! ```
//!
//! Implementation notes (Context7-verified, PyO3 0.28):
//! - `#[pymodule] fn _native(m: &Bound<'_, PyModule>) -> PyResult<()>`.
//! - `Bound<'_, PyAny>` and `wrap_pyfunction!` for free functions.
//! - `PyDocument` wraps a strata-core::Document and exposes the two
//!   serializer paths (Markdown, Graph-RAG JSON).

use pyo3::exceptions::{PyFileNotFoundError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use strata_core::Document;
use strata_serialize as serialize;

/// Caller-tunable knobs for [`parse`] / [`parse_batch`].
///
/// All fields default to the same values as the CLI subcommand (Plan §14.T9.3).
#[pyclass(
    name = "ParseOptions",
    module = "strata_reader._native",
    from_py_object
)]
#[derive(Clone, Debug)]
struct PyParseOptions {
    #[pyo3(get, set)]
    profile: String,
    #[pyo3(get, set)]
    use_ia: bool,
    #[pyo3(get, set)]
    max_concurrent_pages: Option<usize>,
    #[pyo3(get, set)]
    media_dir: Option<String>,
    #[pyo3(get, set)]
    ollama_endpoint: String,
}

#[pymethods]
impl PyParseOptions {
    #[new]
    #[pyo3(signature = (
        profile = "balanced".to_string(),
        use_ia = true,
        max_concurrent_pages = None,
        media_dir = None,
        ollama_endpoint = "http://localhost:11434".to_string(),
    ))]
    fn new(
        profile: String,
        use_ia: bool,
        max_concurrent_pages: Option<usize>,
        media_dir: Option<String>,
        ollama_endpoint: String,
    ) -> PyResult<Self> {
        match profile.as_str() {
            "fast" | "balanced" | "scientific" => {}
            other => {
                return Err(PyValueError::new_err(format!(
                    "profile must be one of fast / balanced / scientific, got {other:?}"
                )))
            }
        }
        Ok(Self {
            profile,
            use_ia,
            max_concurrent_pages,
            media_dir,
            ollama_endpoint,
        })
    }

    fn __repr__(&self) -> String {
        format!(
            "ParseOptions(profile={:?}, use_ia={}, max_concurrent_pages={:?}, ollama_endpoint={:?})",
            self.profile, self.use_ia, self.max_concurrent_pages, self.ollama_endpoint
        )
    }
}

/// Lightweight wrapper around `strata_core::Document` that exposes only
/// the two methods the Python side needs.
#[pyclass(name = "Document", module = "strata_reader._native")]
struct PyDocument {
    inner: Document,
}

#[pymethods]
impl PyDocument {
    /// Render this document to GFM Markdown for Vector-RAG ingestion.
    fn to_markdown(&self) -> PyResult<String> {
        Ok(serialize::render_markdown(
            &self.inner,
            &serialize::MarkdownOptions::default(),
        ))
    }

    /// Render this document to the Graph-RAG JSON shape. Returns a Python
    /// dict (parsed from the canonical JSON) so consumers can poke at
    /// nodes / edges without round-tripping through string parsing.
    fn to_graph_json<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let graph = serialize::render_graph(&self.inner);
        let text = serde_json::to_string(&graph)
            .map_err(|e| PyValueError::new_err(format!("graph serialization failed: {e}")))?;
        let json_module = py.import("json")?;
        json_module.call_method1("loads", (text,))
    }

    /// Count of source pages — handy for sanity checks.
    fn __len__(&self) -> usize {
        self.inner.page_count()
    }

    fn __repr__(&self) -> String {
        format!(
            "Document(pages={}, sha256={})",
            self.inner.page_count(),
            self.inner.meta.source_sha256
        )
    }
}

fn sha256_file(path: &std::path::Path) -> PyResult<String> {
    use sha2::Digest;
    let bytes = std::fs::read(path)
        .map_err(|e| PyValueError::new_err(format!("failed to read PDF: {e}")))?;
    let hash = sha2::Sha256::digest(&bytes);
    Ok(format!("{hash:x}"))
}

/// Parse a single PDF using the native Rust geometry and abstract backend extraction pipeline.
#[pyfunction]
#[pyo3(signature = (path, options = None))]
fn parse(path: String, options: Option<PyParseOptions>) -> PyResult<PyDocument> {
    let options = options.unwrap_or_else(|| PyParseOptions {
        profile: "balanced".into(),
        use_ia: true,
        max_concurrent_pages: None,
        media_dir: None,
        ollama_endpoint: "http://localhost:11434".into(),
    });
    let pdf_path = std::path::Path::new(&path);
    if !pdf_path.exists() {
        return Err(PyFileNotFoundError::new_err(format!(
            "PDF not found: {path}"
        )));
    }

    // 1. Open PDF
    let decoder = strata_pdf::Decoder::open(pdf_path)
        .map_err(|e| PyValueError::new_err(format!("failed to open PDF: {e}")))?;
    let page_count = decoder.page_count();

    // 2. Per-page extraction
    let mut doc_pages: Vec<std::sync::Arc<strata_core::Page>> = Vec::with_capacity(page_count);
    let sha = sha256_file(pdf_path)?;

    for page_idx in 0..page_count {
        let page = decoder
            .page(page_idx)
            .map_err(|e| PyValueError::new_err(format!("page {page_idx}: {e}")))?;
        let (page_w, page_h) = page.size();

        // Raw extraction using the abstract traits
        let glyphs = page.glyphs().unwrap_or_default();
        let images = page.images().unwrap_or_default();

        // 3. Geometry and noise filtering
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

        // 4. Semantics & Headings classification
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

        // 5. Image block integration
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

            if let Some(media_dir_str) = &options.media_dir {
                let target_dir = std::path::Path::new(media_dir_str);
                if let Err(e) = std::fs::create_dir_all(target_dir) {
                    tracing::warn!("Failed to create media dir: {e}");
                } else {
                    let file_path = target_dir.join(format!("{}.png", figure_block.id));
                    if let Err(e) = std::fs::write(&file_path, &img.raw_bytes) {
                        tracing::warn!("Failed to write figure image: {e}");
                    }
                }
            }

            blocks.push(figure_block);
        }

        // 6. XY-Cut++ ordering
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

    // 7. Assemble final Document
    let mut doc = strata_core::Document::new(strata_core::DocMeta {
        source_sha256: sha,
        source_filename: pdf_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default(),
        schema_version: strata_core::version().to_string(),
        profile: options.profile.clone(),
        extra: std::collections::BTreeMap::new(),
    });
    doc.pages = doc_pages;

    Ok(PyDocument { inner: doc })
}

/// Parse several PDFs. Returns a dict `{path: Document}` so callers can
/// quickly distinguish results from failures.
#[pyfunction]
#[pyo3(signature = (paths, options = None))]
fn parse_batch<'py>(
    py: Python<'py>,
    paths: Vec<String>,
    options: Option<PyParseOptions>,
) -> PyResult<Bound<'py, PyDict>> {
    let dict = PyDict::new(py);
    for path in paths {
        match parse(path.clone(), options.clone()) {
            Ok(doc) => dict.set_item(path, Py::new(py, doc)?)?,
            Err(e) => dict.set_item(path, e.to_string())?,
        }
    }
    Ok(dict)
}

/// Returns the semver of the Rust core crate. Phase 0 already exposed
/// this; we keep the symbol so downstream code never has to change its
/// import.
#[pyfunction]
fn version() -> &'static str {
    strata_core::version()
}

/// A Python module implemented in Rust.
#[pymodule]
fn _native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(version, m)?)?;
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    m.add_function(wrap_pyfunction!(parse_batch, m)?)?;
    m.add_class::<PyParseOptions>()?;
    m.add_class::<PyDocument>()?;
    Ok(())
}

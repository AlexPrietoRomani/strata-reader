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

use std::collections::HashMap;
use std::sync::Arc;

use pyo3::exceptions::{PyFileNotFoundError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use strata_core::{BBox, BlockId, BlockType, DocMeta, Document, Page, PageOrientation, Provenance, ProvenanceSource, Size};
use strata_fusion as fusion;
use strata_serialize as serialize;

/// Caller-tunable knobs for [`parse`] / [`parse_batch`].
///
/// All fields default to the same values as the CLI subcommand (Plan §14.T9.3).
#[pyclass(name = "ParseOptions", module = "strata_reader._native")]
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
        Ok(Self { profile, use_ia, max_concurrent_pages, media_dir, ollama_endpoint })
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
        Ok(serialize::render_markdown(&self.inner, &serialize::MarkdownOptions::default()))
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
        format!("Document(pages={}, sha256={})", self.inner.page_count(), self.inner.meta.source_sha256)
    }
}

/// Parse a single PDF (placeholder — see notes below).
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
        return Err(PyFileNotFoundError::new_err(format!("PDF not found: {path}")));
    }

    // Phase 9 ships the API surface. The real wiring waits on strata-pdf
    // (libpdfium linker, blocked by corp EDR — see docs/usage/IT_request.md).
    // The placeholder returns a minimal Document so consumers can write
    // integration tests against the Python surface today.
    let bytes = std::fs::read(pdf_path)
        .map_err(|e| PyValueError::new_err(format!("read failed: {e}")))?;
    let sha = sha256_hex(&bytes);
    let doc = empty_document(pdf_path, &sha, &options.profile);
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

// ---------------------------------------------------------------------------
// Internals — placeholder document construction. To be replaced by the
// strata-pdf → fusion pipeline once the linker is unblocked.
// ---------------------------------------------------------------------------

fn empty_document(pdf_path: &std::path::Path, sha256: &str, profile: &str) -> Document {
    let meta = DocMeta {
        source_sha256: sha256.to_string(),
        source_filename: pdf_path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default(),
        schema_version: strata_core::version().to_string(),
        profile: profile.to_string(),
        extra: HashMap::default().into_iter().collect(),
    };
    let mut doc = Document::new(meta);
    // One empty page so downstream serializers don't crash on zero-page docs.
    let page = Page {
        number: 1,
        size: Size::new(595.0, 842.0).unwrap(),
        orientation: PageOrientation::Portrait,
        blocks: vec![Arc::new(strata_core::Block {
            id: BlockId::new(),
            kind: BlockType::Paragraph,
            bbox: BBox::new(0.0, 0.0, 595.0, 842.0).unwrap(),
            content: "_(strata-pdf decoder blocked by EDR — see docs/usage/IT_request.md)_".into(),
            children: vec![],
            provenance: Provenance::try_new(ProvenanceSource::Rust, None, 0.0, 0, 0).unwrap(),
        })],
        reading_order: Vec::new(),
        media_box: BBox::new(0.0, 0.0, 595.0, 842.0).unwrap(),
    };
    let _ = &fusion::validate; // silence unused-import on placeholder build.
    doc.pages = vec![Arc::new(page)];
    doc
}

fn sha256_hex(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    std::hash::Hasher::write(&mut hasher, bytes);
    // NOTE: the placeholder uses a 64-bit hash so PyDocument has a stable
    // shape during F9. Real SHA-256 lives in strata-server::routes and
    // will be plumbed once strata-pdf compiles end-to-end.
    let mut out = String::with_capacity(64);
    let v = std::hash::Hasher::finish(&hasher);
    for byte in v.to_be_bytes() {
        let _ = write!(out, "{byte:02x}");
    }
    while out.len() < 64 {
        out.push('0');
    }
    out
}

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



/// Parse a single PDF using the native Rust geometry and abstract backend extraction pipeline.
#[pyfunction]
#[pyo3(signature = (path, options = None))]
fn parse(_py: Python<'_>, path: String, options: Option<PyParseOptions>) -> PyResult<PyDocument> {
    use std::path::PathBuf;
    use strata_pipeline::{parse_document, ParsePipelineOptions};

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

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|e| PyValueError::new_err(format!("failed to start tokio runtime: {e}")))?;

    let has_media_dir = options.media_dir.is_some();
    let opts = ParsePipelineOptions {
        input: pdf_path.to_path_buf(),
        profile: options.profile,
        use_ia: options.use_ia,
        force_ocr: false,
        ollama_endpoint: options.ollama_endpoint,
        ia_grpc_endpoint: None,
        max_concurrent_pages: options.max_concurrent_pages,
        media_dir: options.media_dir.map(PathBuf::from),
        save_images: has_media_dir,
        pdf_backend: "auto".into(),
    };

    let artifacts = rt.block_on(async move {
        parse_document(opts).await
    }).map_err(|e| PyValueError::new_err(format!("pipeline failed: {e}")))?;

    Ok(PyDocument { inner: artifacts.document })
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
        match parse(py, path.clone(), options.clone()) {
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

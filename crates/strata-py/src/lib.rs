//! PyO3 bindings for strata-reader. Phase 0 only exposes `version()`.
//!
//! Context7-verified: PyO3 0.28 idiom is
//!   `fn name(m: &Bound<'_, PyModule>) -> PyResult<()>`
//! with `[lib] name = "_native"` in Cargo.toml matching the `#[pymodule]` ident.

use pyo3::prelude::*;

/// Returns the semver of the underlying Rust crate.
#[pyfunction]
fn version() -> &'static str {
    strata_core::version()
}

/// A Python module implemented in Rust.
#[pymodule]
fn _native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(version, m)?)?;
    Ok(())
}

//! Process-wide initialization of the PDFium native library.
//!
//! `pdfium-render` is a binding around the C++ PDFium engine; the library
//! must be loaded exactly once per process. We expose a single
//! [`get_pdfium`] function that returns a lazy singleton.
//!
//! ## Loading strategy
//!
//! Resolution order (first hit wins):
//!
//! 1. Environment variable `STRATA_PDFIUM_LIB_PATH` pointing to the directory
//!    containing `pdfium.dll` / `libpdfium.{so,dylib}`. Useful for wheel
//!    bundling (the Python SDK sets this to its own resource dir).
//! 2. `Pdfium::bind_to_system_library()` — looks in the OS standard search
//!    path. Set up by `vcpkg install pdfium` on Windows, `apt install
//!    libpdfium-dev` on Debian/Ubuntu, or `brew install pdfium-binaries` on
//!    macOS.
//!
//! On Windows without admin rights, the easiest path is to download a
//! pre-built archive from <https://github.com/bblanchon/pdfium-binaries>
//! into `%LOCALAPPDATA%\pdfium\` and point `STRATA_PDFIUM_LIB_PATH` at it.
//!
//! ## Errors
//!
//! [`get_pdfium`] returns [`DecoderError::PdfiumLoad`] when no PDFium binary
//! can be located. The caller should surface this to the user — never panic.
//!
//! See `docs/plan/plan_maestro.md` §7.T2.1 and `docs/task/tareas.md` A2.1.1.

use std::sync::OnceLock;

use pdfium_render::prelude::Pdfium;

use crate::decoder::DecoderError;

/// Singleton PDFium handle. Populated on first call to [`get_pdfium`].
static PDFIUM: OnceLock<Result<Pdfium, String>> = OnceLock::new();

/// Returns the lazy PDFium singleton, loading the native library on first call.
pub fn get_pdfium() -> Result<&'static Pdfium, DecoderError> {
    let result = PDFIUM.get_or_init(load_pdfium);
    match result {
        Ok(p) => Ok(p),
        Err(msg) => Err(DecoderError::PdfiumLoad(msg.clone())),
    }
}

fn load_pdfium() -> Result<Pdfium, String> {
    // 1) Explicit override via env var (preferred for wheel bundling).
    #[allow(clippy::disallowed_methods)]
    if let Ok(dir) = std::env::var("STRATA_PDFIUM_LIB_PATH") {
        let bindings = Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path(&dir))
            .map_err(|e| format!("STRATA_PDFIUM_LIB_PATH={dir}: {e}"))?;
        return Ok(Pdfium::new(bindings));
    }

    // 2) System library (set up by IT / package manager).
    #[allow(clippy::disallowed_methods)]
    let bindings =
        Pdfium::bind_to_system_library().map_err(|e| format!("system pdfium not found: {e}"))?;
    Ok(Pdfium::new(bindings))
}

/// Returns `true` when the PDFium binary is reachable on the current host.
/// Used by integration tests to skip gracefully when the env is not ready.
pub fn pdfium_available() -> bool {
    get_pdfium().is_ok()
}

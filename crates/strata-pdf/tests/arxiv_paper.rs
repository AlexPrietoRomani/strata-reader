//! End-to-end test against the real arXiv 1706.03762 fixture.
//!
//! Skips gracefully when:
//!   - The libpdfium binary is not available on the host (CI without IT
//!     allowlist, dev machine without `STRATA_PDFIUM_LIB_PATH`).
//!   - The fixture file is missing.
//!
//! See `docs/task/tareas.md` T2.1.A2.1.2.AC and T2.2.A2.2.{1,2,3,4}.

use std::path::PathBuf;

use strata_pdf::{
    extract_glyphs, extract_images, extract_paths, is_likely_scan, pdfium_available, Decoder,
};

fn fixture_path() -> Option<PathBuf> {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // crates/strata-pdf/ → repo root → tests/fixtures/pdfs/two_column_paper.pdf
    p.pop();
    p.pop();
    p.push("tests/fixtures/pdfs/two_column_paper.pdf");
    if p.exists() {
        Some(p)
    } else {
        None
    }
}

fn skip_if_unavailable() -> Option<Decoder> {
    if !pdfium_available() {
        eprintln!(
            "SKIP — libpdfium not available (set STRATA_PDFIUM_LIB_PATH or install system-wide)"
        );
        return None;
    }
    let path = fixture_path()?;
    Decoder::open(&path).ok()
}

#[test]
fn opens_arxiv_paper_and_reports_pages() {
    let Some(dec) = skip_if_unavailable() else {
        return;
    };
    let n = dec.page_count();
    assert!(n >= 8, "arxiv 1706.03762 has at least 8 pages, got {n}");
}

#[test]
fn first_page_has_glyphs() {
    let Some(dec) = skip_if_unavailable() else {
        return;
    };
    let page = dec.pages().get(0).expect("page 0 should exist");
    let glyphs = extract_glyphs(&page).expect("glyph extraction must not error");
    assert!(
        glyphs.len() > 100,
        "page 1 of a paper should have hundreds of glyphs, got {}",
        glyphs.len()
    );
    // All glyph BBoxes should fit within the page.
    let media_w = page.width().value;
    let media_h = page.height().value;
    for g in &glyphs {
        assert!(g.bbox.x0 >= 0.0 && g.bbox.x1 <= media_w + 1.0);
        assert!(g.bbox.y0 >= 0.0 && g.bbox.y1 <= media_h + 1.0);
    }
}

#[test]
fn extracts_vector_paths_without_panic() {
    let Some(dec) = skip_if_unavailable() else {
        return;
    };
    let page = dec.pages().get(0).expect("page 0 should exist");
    let paths = extract_paths(&page).expect("path extraction must not error");
    // We don't assert a count — the paper may or may not have vector primitives
    // on page 1. The contract is: no panic, all paths have ≥ 1 segment.
    for p in &paths {
        assert!(!p.segments.is_empty());
    }
}

#[test]
fn extracts_images_without_panic() {
    let Some(dec) = skip_if_unavailable() else {
        return;
    };
    let page = dec.pages().get(0).expect("page 0 should exist");
    let _ = extract_images(&page).expect("image extraction must not error");
}

#[test]
fn arxiv_paper_is_not_a_scan() {
    let Some(dec) = skip_if_unavailable() else {
        return;
    };
    let page = dec.pages().get(0).expect("page 0 should exist");
    let is_scan = is_likely_scan(&page).expect("scan detector must not error");
    assert!(
        !is_scan,
        "the arXiv PDF has native text; is_likely_scan must return false"
    );
}

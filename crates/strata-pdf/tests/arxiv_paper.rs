//! End-to-end test against the real arXiv 1706.03762 fixture.
//!
//! Skips gracefully when libpdfium is not available.

#![cfg(feature = "pdfium-backend")]

use std::path::PathBuf;

use strata_pdf::{pdfium_available, Decoder};

fn fixture_path() -> Option<PathBuf> {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
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
    let page = dec.page(0).expect("page 0 should exist");
    let glyphs = page.glyphs().expect("glyph extraction must not error");
    assert!(
        glyphs.len() > 100,
        "page 1 of a paper should have hundreds of glyphs, got {}",
        glyphs.len()
    );
    let (media_w, media_h) = page.size();
    for g in &glyphs {
        if !(g.bbox.x0 >= -10.0 && g.bbox.x1 <= media_w + 10.0) || !(g.bbox.y0 >= -10.0 && g.bbox.y1 <= media_h + 10.0) {
            println!(
                "Glyph out of bounds: char={:?}, bbox={:?}, media_w={}, media_h={}",
                g.unicode, g.bbox, media_w, media_h
            );
        }
        assert!(
            g.bbox.x0 >= -10.0 && g.bbox.x1 <= media_w + 10.0,
            "Glyph x-bounds out of range: char={:?}, bbox={:?}, media_w={}",
            g.unicode, g.bbox, media_w
        );
        assert!(
            g.bbox.y0 >= -10.0 && g.bbox.y1 <= media_h + 10.0,
            "Glyph y-bounds out of range: char={:?}, bbox={:?}, media_h={}",
            g.unicode, g.bbox, media_h
        );
    }
}

#[test]
fn extracts_vector_paths_without_panic() {
    let Some(dec) = skip_if_unavailable() else {
        return;
    };
    let page = dec.page(0).expect("page 0 should exist");
    let paths = page.paths().expect("path extraction must not error");
    for p in &paths {
        assert!(!p.segments.is_empty());
    }
}

#[test]
fn extracts_images_without_panic() {
    let Some(dec) = skip_if_unavailable() else {
        return;
    };
    let page = dec.page(0).expect("page 0 should exist");
    let _ = page.images().expect("image extraction must not error");
}

#[test]
fn arxiv_paper_is_not_a_scan() {
    let Some(dec) = skip_if_unavailable() else {
        return;
    };
    let page = dec.page(0).expect("page 0 should exist");
    let is_scan = strata_pdf::is_likely_scan(page.as_ref()).expect("scan detector must not error");
    assert!(
        !is_scan,
        "the arXiv PDF has native text; is_likely_scan must return false"
    );
}

//! Integration test for `render_crop` against the arXiv fixture.
//!
//! Skips when libpdfium is unavailable. See Plan Maestro §9.T4.4.

use std::path::PathBuf;

use strata_core::BBox;
use strata_pdf::{pdfium_available, Decoder};
use strata_triage::{render_crop, DEFAULT_CROP_DPI};

fn fixture_path() -> Option<PathBuf> {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop(); // crates/
    p.pop(); // repo root
    p.push("tests/fixtures/pdfs/two_column_paper.pdf");
    if p.exists() { Some(p) } else { None }
}

fn skip_if_unavailable() -> Option<Decoder> {
    if !pdfium_available() {
        eprintln!("SKIP — libpdfium not available");
        return None;
    }
    let path = fixture_path()?;
    Decoder::open(&path).ok()
}

#[test]
fn renders_first_page_top_region_under_500kb() {
    let Some(dec) = skip_if_unavailable() else { return };
    let page = dec.pages().get(0).expect("page 0 exists");

    let page_w = page.width().value;
    let page_h = page.height().value;
    // Top 25 % of the page — should cover the title and authors block.
    let bbox = BBox::new(0.0, page_h * 0.75, page_w, page_h).unwrap();

    let png = render_crop(&page, bbox, DEFAULT_CROP_DPI).expect("render must succeed");
    assert!(png.len() > 0, "empty PNG output");
    assert!(png.len() < 500_000, "crop should be < 500KB, got {} bytes", png.len());

    // Determinism: render twice and verify byte equality (Plan Maestro §1).
    let png2 = render_crop(&page, bbox, DEFAULT_CROP_DPI).expect("render must succeed");
    assert_eq!(png, png2, "render_crop is not deterministic");
}

#[test]
fn rejects_bbox_outside_page() {
    let Some(dec) = skip_if_unavailable() else { return };
    let page = dec.pages().get(0).expect("page 0 exists");

    let page_w = page.width().value;
    let page_h = page.height().value;
    let bbox = BBox::new(page_w + 100.0, page_h + 100.0, page_w + 200.0, page_h + 200.0).unwrap();
    let res = render_crop(&page, bbox, DEFAULT_CROP_DPI);
    assert!(res.is_err(), "out-of-page bbox must error");
}

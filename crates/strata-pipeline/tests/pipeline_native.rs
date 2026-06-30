//! Tests del pipeline en modo nativo (sin IA).

use std::path::PathBuf;

use strata_pipeline::{parse_document, ParsePipelineOptions};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("tests/fixtures/pdfs/crecks")
        .join(name)
}

#[tokio::test]
async fn native_parse_produces_non_empty_document() {
    let path = fixture("native_simple.pdf");
    if !path.exists() {
        // Salta en CI sin fixtures.
        return;
    }
    let opts = ParsePipelineOptions {
        input: path,
        use_ia: false,
        ..Default::default()
    };
    let artifacts = match parse_document(opts).await {
        Ok(art) => art,
        Err(strata_pipeline::PipelineError::PdfOpen(msg))
            if msg.contains("pdfium native library") =>
        {
            // Saltamos el test si la librería pdfium no está instalada en el sistema
            println!("Saltando test: pdfium no disponible");
            return;
        }
        Err(e) => panic!("pipeline falló: {:?}", e),
    };
    assert!(!artifacts.document.pages.is_empty(), "debe tener páginas");

    let total_blocks: usize = artifacts
        .document
        .pages
        .iter()
        .map(|p| p.blocks.len())
        .sum();
    if total_blocks > 0 {
        assert!(
            !artifacts.markdown.is_empty(),
            "markdown no debe estar vacío"
        );
    } else {
        assert!(
            artifacts.markdown.is_empty(),
            "markdown debe estar vacío si no hay bloques"
        );
    }
}

#[tokio::test]
async fn native_parse_with_save_images() {
    let path = fixture("figure_with_caption.pdf");
    if !path.exists() {
        return;
    }
    let tmp = std::env::temp_dir().join("strata_test_pipeline");
    let opts = ParsePipelineOptions {
        input: path,
        use_ia: false,
        save_images: true,
        media_dir: Some(tmp.clone()),
        ..Default::default()
    };
    let artifacts = match parse_document(opts).await {
        Ok(art) => art,
        Err(strata_pipeline::PipelineError::PdfOpen(msg))
            if msg.contains("pdfium native library") =>
        {
            println!("Saltando test: pdfium no disponible");
            let _ = std::fs::remove_dir_all(&tmp);
            return;
        }
        Err(e) => panic!("pipeline falló: {:?}", e),
    };
    assert!(!artifacts.document.pages.is_empty());
    let _ = std::fs::remove_dir_all(&tmp);
}

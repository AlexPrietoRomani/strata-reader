//! Módulo: mappers
//!
//! Descripción:
//! Mappers de respuestas gRPC a IaPayload de fusión.
//! Convierte los tipos proto generados por prost en los tipos de dominio de strata-fusion.

use strata_core::{Provenance, ProvenanceSource};
use strata_fusion::IaPayload;
use strata_ia_bridge::proto::{FormulaResponse, ImageResponse, OcrResponse, TableResponse};

/// Construye `Provenance` desde los campos del proto `Provenance`.
///
/// Elige la fuente según el campo `backend`: "ocr", "surya" y "tesseract" →
/// `ProvenanceSource::Ocr`; cualquier otro → `ProvenanceSource::Vlm`.
fn build_provenance(model_id: &str, backend: &str, latency_ms: u32, retries: u32) -> Provenance {
    let source = if matches!(backend, "ocr" | "surya" | "tesseract") {
        ProvenanceSource::Ocr
    } else {
        ProvenanceSource::Vlm
    };
    // Confianza heurística de 0.9 cuando el proto no la expone a nivel de Provenance.
    // El cast u32 → u8 satura en 255 para evitar panic.
    Provenance::try_new(
        source,
        Some(model_id.to_string()),
        0.9,
        latency_ms,
        retries.min(u8::MAX as u32) as u8,
    )
    .unwrap_or_else(|_| Provenance::rust_native())
}

/// Convierte `OcrResponse` en `IaPayload::Ocr`.
///
/// Retorna `None` si el texto resultante está vacío.
pub fn ocr_to_payload(resp: &OcrResponse) -> Option<IaPayload> {
    let result = resp.result.as_ref()?;
    let prov = resp.provenance.as_ref()?;
    if result.text.trim().is_empty() {
        return None;
    }
    Some(IaPayload::Ocr {
        text: result.text.clone(),
        provenance: build_provenance(&prov.model_id, &prov.backend, prov.latency_ms, prov.retries),
    })
}

/// Convierte `TableResponse` en `IaPayload::Table` con Markdown GFM.
///
/// Retorna `None` si la tabla no tiene filas.
pub fn table_to_payload(resp: &TableResponse) -> Option<IaPayload> {
    let result = resp.result.as_ref()?;
    let prov = resp.provenance.as_ref()?;
    if result.rows.is_empty() {
        return None;
    }
    let gfm = rows_to_gfm(&result.rows);
    Some(IaPayload::Table {
        gfm_markdown: gfm,
        provenance: build_provenance(&prov.model_id, &prov.backend, prov.latency_ms, prov.retries),
    })
}

/// Convierte filas proto en tabla GFM.
fn rows_to_gfm(rows: &[strata_ia_bridge::proto::TableRow]) -> String {
    if rows.is_empty() {
        return String::new();
    }
    let mut lines = Vec::new();
    for (i, row) in rows.iter().enumerate() {
        let cells: Vec<String> = row
            .cells
            .iter()
            .map(|c| c.text.replace('|', "\\|").replace('\n', " "))
            .collect();
        lines.push(format!("| {} |", cells.join(" | ")));
        if i == 0 {
            // Fila separadora después del encabezado.
            let sep: Vec<&str> = row.cells.iter().map(|_| "---").collect();
            lines.push(format!("| {} |", sep.join(" | ")));
        }
    }
    lines.join("\n")
}

/// Convierte `ImageResponse` en `IaPayload::Image`.
pub fn image_to_payload(resp: &ImageResponse) -> Option<IaPayload> {
    let result = resp.result.as_ref()?;
    let prov = resp.provenance.as_ref()?;
    Some(IaPayload::Image {
        caption: result.caption.clone(),
        long_description: result.description.clone(),
        alt_text: if result.alt_text.is_empty() {
            result.caption.clone()
        } else {
            result.alt_text.clone()
        },
        provenance: build_provenance(&prov.model_id, &prov.backend, prov.latency_ms, prov.retries),
    })
}

/// Convierte `FormulaResponse` en `IaPayload::Formula`.
///
/// Retorna `None` si el LaTeX resultante está vacío.
pub fn formula_to_payload(resp: &FormulaResponse) -> Option<IaPayload> {
    let result = resp.result.as_ref()?;
    let prov = resp.provenance.as_ref()?;
    if result.latex.trim().is_empty() {
        return None;
    }
    Some(IaPayload::Formula {
        latex: result.latex.clone(),
        mathml: if result.mathml.is_empty() {
            None
        } else {
            Some(result.mathml.clone())
        },
        provenance: build_provenance(&prov.model_id, &prov.backend, prov.latency_ms, prov.retries),
    })
}

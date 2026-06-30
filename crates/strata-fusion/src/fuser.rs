//! Merge a native (Rust-only) [`Document`] with the IA payloads that came
//! back from the Python bridge.
//!
//! Inputs:
//!
//! - `native`: an `Arc<Document>` whose Blocks were produced by the Rust
//!   pipeline (Fases 1-4). Blocks marked for IA processing have empty
//!   `content` and a `Provenance` of source `Rust` *with* a low
//!   `confidence` flag.
//! - `ia_results`: a map from `BlockId` to the typed payload that the
//!   bridge returned (see [`IaPayload`]).
//!
//! Output:
//!
//! - A *new* `Document`. The original is never mutated — Plan Maestro
//!   §1 mandates functional, immutable transforms across pipeline
//!   stages.
//!
//! Reference: Plan Maestro §12.T7.1.

use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use strata_core::{Block, BlockId, BlockType, Document, Page, Provenance};

/// Tagged union of the payloads the IA side can hand back. Mirrors the
/// 4 RPCs in `strata.ia.v1.IaService` but is purely Rust — keeps this
/// crate independent of `strata-ia-bridge`'s generated stubs.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum IaPayload {
    /// OCR transcription — populates the block's `content` with plain text.
    Ocr {
        text: String,
        provenance: Provenance,
    },
    /// Table extraction — GFM-flavoured Markdown ready to embed.
    Table {
        gfm_markdown: String,
        provenance: Provenance,
    },
    /// Image description — a short caption goes into `content`; the
    /// long description and alt-text land in `extra` so the serializer
    /// can include them as a figure caption + alt attribute.
    Image {
        caption: String,
        long_description: String,
        alt_text: String,
        provenance: Provenance,
    },
    /// Formula → LaTeX. Wrapped in display math (`$$ … $$`) when
    /// rendered.
    Formula {
        latex: String,
        mathml: Option<String>,
        provenance: Provenance,
    },
}

impl IaPayload {
    fn provenance(&self) -> &Provenance {
        match self {
            Self::Ocr { provenance, .. }
            | Self::Table { provenance, .. }
            | Self::Image { provenance, .. }
            | Self::Formula { provenance, .. } => provenance,
        }
    }

    fn content(&self) -> String {
        match self {
            Self::Ocr { text, .. } => text.clone(),
            Self::Table { gfm_markdown, .. } => gfm_markdown.clone(),
            Self::Image { caption, .. } => caption.clone(),
            Self::Formula { latex, .. } => latex.clone(),
        }
    }
}

/// Merge `native` with `ia_results`. Returns a *new* [`Document`]; the
/// input arcs stay untouched.
///
/// Blocks not present in `ia_results` are passed through verbatim. Blocks
/// found in the map get a freshly built copy with the IA-provided content
/// and provenance.
pub fn merge(native: &Document, ia_results: &HashMap<BlockId, IaPayload>) -> Document {
    let new_pages: Vec<Arc<Page>> = native
        .pages
        .iter()
        .map(|page| Arc::new(merge_page(page, ia_results)))
        .collect();

    Document {
        meta: native.meta.clone(),
        pages: new_pages,
    }
}

fn merge_page(page: &Page, ia_results: &HashMap<BlockId, IaPayload>) -> Page {
    let mut blocks: Vec<Arc<Block>> = Vec::with_capacity(page.blocks.len());
    for b in &page.blocks {
        if let Some(payload) = ia_results.get(&b.id) {
            blocks.push(Arc::new(apply_payload(b, payload)));
        } else {
            blocks.push(Arc::clone(b));
        }
    }
    Page {
        number: page.number,
        size: page.size,
        orientation: page.orientation,
        blocks,
        reading_order: page.reading_order.clone(),
        media_box: page.media_box,
    }
}

fn apply_payload(block: &Block, payload: &IaPayload) -> Block {
    let (kind, metadata) = match payload {
        IaPayload::Ocr { .. } => (block.kind.clone(), None),
        IaPayload::Table { .. } => (BlockType::Table, None),
        IaPayload::Image { alt_text, long_description, .. } => {
            let meta = strata_core::BlockMetadata {
                alt_text: Some(alt_text.clone()),
                description: Some(long_description.clone()),
                mathml: None,
                media_path: None,
            };
            (BlockType::Figure, Some(meta))
        }
        IaPayload::Formula { mathml, .. } => {
            let meta = strata_core::BlockMetadata {
                alt_text: None,
                description: None,
                mathml: mathml.clone(),
                media_path: None,
            };
            (BlockType::Equation, Some(meta))
        }
    };

    Block {
        id: block.id,
        kind,
        bbox: block.bbox,
        content: payload.content(),
        children: block.children.clone(),
        metadata,
        provenance: payload.provenance().clone(),
    }
}

/// Errors raised when the post-fusion document fails its invariants.
#[derive(Debug, thiserror::Error)]
pub enum FusionError {
    #[error(
        "fusion contract violated: block {block_id} (kind {kind:?}) still has empty content after merge"
    )]
    EmptyContent { block_id: BlockId, kind: BlockType },
}

/// Validate the AC from Plan Maestro §12.T7.1: every block must have
/// non-empty content after fusion. Returns the first offending block.
///
/// Page numbers / headers / footers are exempt because the Triage may
/// legitimately drop them.
pub fn validate(doc: &Document) -> Result<(), FusionError> {
    for page in &doc.pages {
        for block in &page.blocks {
            if matches!(
                block.kind,
                BlockType::Header | BlockType::Footer | BlockType::PageNumber | BlockType::Figure
            ) {
                continue;
            }
            if block.content.trim().is_empty() {
                return Err(FusionError::EmptyContent {
                    block_id: block.id,
                    kind: block.kind.clone(),
                });
            }
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use strata_core::{BBox, DocMeta, ProvenanceSource, Size};

    fn make_block(content: &str, kind: BlockType) -> Block {
        Block {
            id: BlockId::new(),
            kind,
            bbox: BBox::new(0.0, 0.0, 10.0, 10.0).unwrap(),
            content: content.into(),
            children: vec![],
            metadata: None,
            provenance: Provenance::try_new(ProvenanceSource::Rust, None, 0.4, 0, 0).unwrap(),
        }
    }

    fn make_doc(blocks: Vec<Block>) -> Document {
        let block_arcs: Vec<Arc<Block>> = blocks.into_iter().map(Arc::new).collect();
        let reading_order = block_arcs.iter().map(|b| b.id).collect();
        let page = Page {
            number: 1,
            size: Size::new(595.0, 842.0).unwrap(),
            orientation: strata_core::PageOrientation::Portrait,
            blocks: block_arcs,
            reading_order,
            media_box: BBox::new(0.0, 0.0, 595.0, 842.0).unwrap(),
        };
        Document {
            meta: DocMeta {
                source_sha256: "0".repeat(64),
                source_filename: "test.pdf".into(),
                schema_version: "0.1.0".into(),
                profile: "balanced".into(),
                extra: Default::default(),
            },
            pages: vec![Arc::new(page)],
        }
    }

    #[test]
    fn merge_replaces_content_for_known_block() {
        let native = make_block("", BlockType::Table);
        let block_id = native.id;
        let doc = make_doc(vec![native]);

        let mut results = HashMap::new();
        results.insert(
            block_id,
            IaPayload::Table {
                gfm_markdown: "| a | b |\n|---|---|\n| 1 | 2 |".into(),
                provenance: Provenance::try_new(
                    ProvenanceSource::Vlm,
                    Some("qwen2.5vl:7b".into()),
                    0.92,
                    230,
                    0,
                )
                .unwrap(),
            },
        );

        let fused = merge(&doc, &results);
        let new_block = &fused.pages[0].blocks[0];
        assert!(new_block.content.contains("| a | b |"));
        assert_eq!(new_block.provenance.source, ProvenanceSource::Vlm);
        assert_eq!(new_block.provenance.confidence, 0.92);
    }

    #[test]
    fn merge_preserves_blocks_not_in_results() {
        let native_a = make_block("already-extracted", BlockType::Paragraph);
        let id_a = native_a.id;
        let native_b = make_block("", BlockType::Table);
        let id_b = native_b.id;
        let doc = make_doc(vec![native_a, native_b]);

        let mut results = HashMap::new();
        results.insert(
            id_b,
            IaPayload::Table {
                gfm_markdown: "table".into(),
                provenance: Provenance::try_new(ProvenanceSource::Vlm, None, 0.9, 10, 0).unwrap(),
            },
        );

        let fused = merge(&doc, &results);
        assert_eq!(fused.pages[0].blocks[0].content, "already-extracted");
        assert_eq!(fused.pages[0].blocks[0].id, id_a);
        assert_eq!(fused.pages[0].blocks[1].content, "table");
        assert_eq!(fused.pages[0].blocks[1].id, id_b);
    }

    #[test]
    fn merge_returns_new_arcs() {
        let native = make_block("", BlockType::Table);
        let block_id = native.id;
        let doc = make_doc(vec![native]);

        let mut results = HashMap::new();
        results.insert(
            block_id,
            IaPayload::Ocr {
                text: "hello".into(),
                provenance: Provenance::try_new(ProvenanceSource::Ocr, None, 0.85, 50, 0).unwrap(),
            },
        );

        let fused = merge(&doc, &results);
        // Original block still has empty content — merge did not mutate.
        assert_eq!(doc.pages[0].blocks[0].content, "");
        assert_eq!(fused.pages[0].blocks[0].content, "hello");
    }

    #[test]
    fn validate_fails_on_empty_text_block() {
        let native = make_block("", BlockType::Paragraph);
        let doc = make_doc(vec![native]);
        let err = validate(&doc).unwrap_err();
        assert!(matches!(err, FusionError::EmptyContent { .. }));
    }

    #[test]
    fn validate_allows_empty_figure_block() {
        let native = make_block("", BlockType::Figure);
        let doc = make_doc(vec![native]);
        assert!(validate(&doc).is_ok());
    }

    #[test]
    fn ia_payload_round_trip_json() {
        let p = IaPayload::Image {
            caption: "cat".into(),
            long_description: "A cute cat on a chair.".into(),
            alt_text: "cat photo".into(),
            provenance: Provenance::try_new(ProvenanceSource::Vlm, None, 0.9, 100, 0).unwrap(),
        };
        let json = serde_json::to_string(&p).unwrap();
        let back: IaPayload = serde_json::from_str(&json).unwrap();
        assert_eq!(p, back);
    }
}

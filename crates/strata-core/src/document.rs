//! [`Document`] root of the AST and its [`DocMeta`] envelope.

use std::sync::Arc;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::page::Page;

/// Top-level metadata that travels alongside every parsed document. Fields are
/// intentionally minimal — the IA bridge writes `model`, `models`, `pipeline`,
/// etc. into [`DocMeta::extra`] when richer context is needed.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DocMeta {
    /// SHA-256 hex digest of the source PDF — used for idempotency / caching.
    pub source_sha256: String,
    /// Original file name (basename) of the source PDF.
    pub source_filename: String,
    /// Schema version producing this document (semver of `strata-core`).
    pub schema_version: String,
    /// Pipeline / profile that produced this run ("fast" / "balanced" / "scientific").
    pub profile: String,
    /// Free-form extension map for non-canonical metadata.
    #[serde(
        default,
        skip_serializing_if = "::std::collections::BTreeMap::is_empty"
    )]
    pub extra: std::collections::BTreeMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub meta: DocMeta,
    pub pages: Vec<Arc<Page>>,
}

impl Document {
    pub fn new(meta: DocMeta) -> Self {
        Self {
            meta,
            pages: Vec::new(),
        }
    }

    /// Convenience: total number of pages in the document.
    pub fn page_count(&self) -> usize {
        self.pages.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_meta() -> DocMeta {
        DocMeta {
            source_sha256: "0".repeat(64),
            source_filename: "test.pdf".into(),
            schema_version: env!("CARGO_PKG_VERSION").into(),
            profile: "balanced".into(),
            extra: Default::default(),
        }
    }

    #[test]
    fn document_round_trips_through_json() {
        let doc = Document::new(sample_meta());
        let json = serde_json::to_string(&doc).unwrap();
        let parsed: Document = serde_json::from_str(&json).unwrap();
        assert_eq!(doc, parsed);
    }

    #[test]
    fn round_trip_is_stable_byte_for_byte() {
        // Verifies determinism contract from §1: re-serializing a deserialized
        // document must produce byte-identical output.
        let doc = Document::new(sample_meta());
        let s1 = serde_json::to_string(&doc).unwrap();
        let parsed: Document = serde_json::from_str(&s1).unwrap();
        let s2 = serde_json::to_string(&parsed).unwrap();
        assert_eq!(s1, s2);
    }
}

//! Provenance metadata — PRISMA-style traceability for every [`Block`].
//!
//! Every node produced anywhere in the pipeline must declare *who* produced it
//! (`Rust` native parser, OCR engine, VLM), with what *model* (model id or
//! `None` for native code), with what *confidence*, and how long it took.
//! See Plan Maestro §1 (rule "Cero unwraps") and §6.T1.4.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ProvenanceError {
    #[error("confidence {0} is outside [0.0, 1.0]")]
    ConfidenceOutOfRange(String),
}

/// Origin of a [`Block`]'s payload.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum ProvenanceSource {
    /// Produced by deterministic Rust code (no IA).
    Rust,
    /// Produced by an OCR pipeline (Surya, Tesseract, …).
    Ocr,
    /// Produced by a Vision-Language Model via the IA bridge.
    Vlm,
}

/// Trace metadata attached to every [`crate::Block`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Provenance {
    pub source: ProvenanceSource,
    /// Model identifier (e.g. `"qwen2.5vl:7b"`); `None` when `source == Rust`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub model: Option<String>,
    /// Confidence in `[0.0, 1.0]`. Native Rust paths use `1.0`.
    pub confidence: f32,
    /// Wall-clock latency of the producing stage, in milliseconds.
    pub latency_ms: u32,
    /// How many retries were needed (0 if first attempt succeeded).
    pub retries: u8,
}

impl Provenance {
    /// Construct a [`Provenance`] enforcing the `confidence ∈ [0, 1]` invariant.
    pub fn try_new(
        source: ProvenanceSource,
        model: Option<String>,
        confidence: f32,
        latency_ms: u32,
        retries: u8,
    ) -> Result<Self, ProvenanceError> {
        if !confidence.is_finite() || !(0.0..=1.0).contains(&confidence) {
            return Err(ProvenanceError::ConfidenceOutOfRange(
                confidence.to_string(),
            ));
        }
        Ok(Self {
            source,
            model,
            confidence,
            latency_ms,
            retries,
        })
    }

    /// Shortcut for "produced by deterministic Rust code with full confidence".
    pub fn rust_native() -> Self {
        Self {
            source: ProvenanceSource::Rust,
            model: None,
            confidence: 1.0,
            latency_ms: 0,
            retries: 0,
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_negative_confidence() {
        assert!(Provenance::try_new(ProvenanceSource::Rust, None, -0.1, 0, 0).is_err());
    }

    #[test]
    fn rejects_above_one_confidence() {
        assert!(Provenance::try_new(ProvenanceSource::Vlm, None, 1.01, 0, 0).is_err());
    }

    #[test]
    fn rejects_nan_confidence() {
        assert!(Provenance::try_new(ProvenanceSource::Ocr, None, f32::NAN, 0, 0).is_err());
    }

    #[test]
    fn accepts_boundary_values() {
        assert!(Provenance::try_new(ProvenanceSource::Rust, None, 0.0, 0, 0).is_ok());
        assert!(Provenance::try_new(ProvenanceSource::Rust, None, 1.0, 0, 0).is_ok());
    }

    #[test]
    fn source_serializes_as_kebab_case() {
        assert_eq!(
            serde_json::to_string(&ProvenanceSource::Vlm).unwrap(),
            r#""vlm""#
        );
        assert_eq!(
            serde_json::to_string(&ProvenanceSource::Ocr).unwrap(),
            r#""ocr""#
        );
        assert_eq!(
            serde_json::to_string(&ProvenanceSource::Rust).unwrap(),
            r#""rust""#
        );
    }

    #[test]
    fn round_trip_through_json() {
        let p = Provenance::try_new(
            ProvenanceSource::Vlm,
            Some("qwen2.5vl:7b".into()),
            0.91,
            230,
            1,
        )
        .unwrap();
        let s = serde_json::to_string(&p).unwrap();
        let back: Provenance = serde_json::from_str(&s).unwrap();
        assert_eq!(p, back);
    }
}

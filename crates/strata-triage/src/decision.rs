//! Triage decision types — the small, closed vocabulary returned by the
//! decision tree in [`crate::triage`].
//!
//! Every block in the pipeline ends up tagged with one of these variants
//! and the [`Reason`] that explains *why*; the reason flows into the
//! `Provenance` of the resulting AST node so PRISMA traceability is end
//! to end (Plan Maestro §1).

use serde::{Deserialize, Serialize};

/// Where to send a block for content extraction.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TriageRoute {
    /// Resolve with the Rust native pipeline (text + simple tables).
    Native,
    /// Send the entire page to OCR (scanned page or broken CID).
    OcrFullPage,
    /// Crop the table region and send it to the VLM table extractor.
    VlmTable,
    /// Crop the image region and send it to the VLM image describer.
    VlmImage,
    /// Crop the formula region and send it to the VLM/pix2tex converter.
    VlmFormula,
}

/// Human-readable rationale for a [`TriageRoute`]. Kept as a closed
/// enum so the Triage decisions are auditable in snapshot tests.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Reason {
    /// Native: nothing on the block triggered any IA path.
    DefaultNative,
    /// OCR: page-level scan signal fired.
    PageIsScanned,
    /// OCR: CID-encoded text failed the health check.
    CidUnmappedOrLowEntropy,
    /// VLM table: a borderless / complex table needs ML extraction.
    BorderlessTableCandidate,
    /// VLM image: a figure-area block exceeds the size threshold.
    ImageAreaOverThreshold,
    /// VLM formula: the block contains math symbols with low confidence.
    MathSymbolsLowConfidence,
    /// The active profile demands aggressive VLM coverage.
    ProfilePrefersVlm,
}

/// Final per-block decision returned by [`crate::triage_block`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TriageDecision {
    pub route: TriageRoute,
    pub reason: Reason,
}

impl TriageDecision {
    pub const fn new(route: TriageRoute, reason: Reason) -> Self {
        Self { route, reason }
    }

    pub fn is_native(self) -> bool {
        matches!(self.route, TriageRoute::Native)
    }

    pub fn requires_ia(self) -> bool {
        !self.is_native()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decision_round_trips_through_json() {
        let d = TriageDecision::new(TriageRoute::VlmTable, Reason::BorderlessTableCandidate);
        let s = serde_json::to_string(&d).unwrap();
        let back: TriageDecision = serde_json::from_str(&s).unwrap();
        assert_eq!(d, back);
    }

    #[test]
    fn route_strings_are_kebab_case() {
        assert_eq!(
            serde_json::to_string(&TriageRoute::OcrFullPage).unwrap(),
            "\"ocr-full-page\""
        );
        assert_eq!(
            serde_json::to_string(&TriageRoute::VlmTable).unwrap(),
            "\"vlm-table\""
        );
    }

    #[test]
    fn classification_helpers_match() {
        assert!(TriageDecision::new(TriageRoute::Native, Reason::DefaultNative).is_native());
        assert!(!TriageDecision::new(TriageRoute::Native, Reason::DefaultNative).requires_ia());
        assert!(
            TriageDecision::new(TriageRoute::VlmImage, Reason::ImageAreaOverThreshold)
                .requires_ia()
        );
    }
}

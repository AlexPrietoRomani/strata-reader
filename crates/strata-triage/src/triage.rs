//! The triage decision tree per Plan Maestro §9.T4.2.
//!
//! ```text
//!   if page.is_scanned                          → OcrFullPage
//!   elif page.cid_severity == Critical          → OcrFullPage
//!   elif block.is_table_candidate
//!        && !block.has_borders                  → VlmTable
//!   elif block.is_image
//!        && area_pct >= profile.image_min_pct   → VlmImage
//!   elif block.contains_math_symbols
//!        && block.confidence < profile.math_th  → VlmFormula
//!   else                                        → Native
//! ```
//!
//! Inputs are simple PoDs (`PageContext`, `BlockContext`) so the rest of the
//! pipeline can build them from any glyph / block representation without
//! coupling crates.

use serde::{Deserialize, Serialize};
use strata_core::BBox;
use strata_quality::Severity;

use crate::decision::{Reason, TriageDecision, TriageRoute};
use crate::profiles::TriageProfile;

/// Aggregate signals at the *page* level — produced once per page and
/// reused for every block on it.
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageContext {
    pub is_scanned: bool,
    pub cid_severity: Severity,
    /// Media-box area in PDF points² — denominator for the image area ratio.
    pub page_area: f32,
}

/// Per-block hint as understood by the Triage. Booleans are explicit so
/// the decision tree is auditable.
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockContext {
    pub bbox: BBox,
    /// `true` when the block was tagged as a table candidate by the
    /// geometry layer (vector borders or word-alignment heuristic).
    pub is_table_candidate: bool,
    /// `true` only for table candidates whose borders were detected
    /// via vector paths (TableBorderProcessor); borderless tables set
    /// this to `false`.
    pub has_borders: bool,
    /// `true` when the page object is a raster image embedded in the PDF.
    pub is_image: bool,
    /// `true` when the block's text contains > 1 math-symbol codepoint
    /// (per the caller's classification).
    pub contains_math_symbols: bool,
    /// Caller's confidence in the native extraction of this block — 1.0
    /// means "perfectly recognised", 0.0 means "no idea". The Triage uses
    /// this together with [`TriageProfile::math_confidence_threshold`] to
    /// decide when to escalate to a VLM.
    pub confidence: f32,
}

impl BlockContext {
    /// Image coverage of the page as a fraction `[0.0, 1.0]`.
    pub fn area_ratio(&self, page_area: f32) -> f32 {
        if page_area <= 0.0 {
            0.0
        } else {
            (self.bbox.area() / page_area).clamp(0.0, 1.0)
        }
    }
}

/// Apply the decision tree to one block.
///
/// Implementation note: the order of the `if` chain is fixed and matches
/// Plan Maestro §9.T4.2 exactly — do **not** reorder without updating the
/// snapshot tests across the 8 golden PDFs.
pub fn triage_block(block: &BlockContext, page: &PageContext, profile: &TriageProfile) -> TriageDecision {
    // Page-level signals dominate per-block ones.
    if page.is_scanned {
        return TriageDecision::new(TriageRoute::OcrFullPage, Reason::PageIsScanned);
    }
    if page.cid_severity == Severity::Critical {
        return TriageDecision::new(TriageRoute::OcrFullPage, Reason::CidUnmappedOrLowEntropy);
    }

    // Block-level signals.
    if block.is_table_candidate && !block.has_borders {
        return TriageDecision::new(TriageRoute::VlmTable, Reason::BorderlessTableCandidate);
    }
    if block.is_image && block.area_ratio(page.page_area) >= profile.image_min_area_ratio {
        return TriageDecision::new(TriageRoute::VlmImage, Reason::ImageAreaOverThreshold);
    }
    if block.contains_math_symbols && block.confidence < profile.math_confidence_threshold {
        return TriageDecision::new(TriageRoute::VlmFormula, Reason::MathSymbolsLowConfidence);
    }

    // Profile-driven aggressive VLM coverage (the `scientific` profile
    // dispatches *all* tables to VLM even if borders exist).
    if profile.always_vlm_tables && block.is_table_candidate {
        return TriageDecision::new(TriageRoute::VlmTable, Reason::ProfilePrefersVlm);
    }

    TriageDecision::new(TriageRoute::Native, Reason::DefaultNative)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn page() -> PageContext {
        PageContext { is_scanned: false, cid_severity: Severity::None, page_area: 1000.0 * 1000.0 }
    }

    fn block() -> BlockContext {
        BlockContext {
            bbox: BBox::new(0.0, 0.0, 100.0, 100.0).unwrap(),
            is_table_candidate: false,
            has_borders: false,
            is_image: false,
            contains_math_symbols: false,
            confidence: 1.0,
        }
    }

    #[test]
    fn scanned_page_always_goes_to_ocr() {
        let mut p = page();
        p.is_scanned = true;
        let d = triage_block(&block(), &p, &TriageProfile::balanced());
        assert_eq!(d.route, TriageRoute::OcrFullPage);
        assert_eq!(d.reason, Reason::PageIsScanned);
    }

    #[test]
    fn cid_critical_overrides_block_signals() {
        let mut p = page();
        p.cid_severity = Severity::Critical;
        let mut b = block();
        b.is_table_candidate = true;
        let d = triage_block(&b, &p, &TriageProfile::balanced());
        assert_eq!(d.route, TriageRoute::OcrFullPage);
        assert_eq!(d.reason, Reason::CidUnmappedOrLowEntropy);
    }

    #[test]
    fn borderless_table_routes_to_vlm() {
        let mut b = block();
        b.is_table_candidate = true;
        b.has_borders = false;
        let d = triage_block(&b, &page(), &TriageProfile::balanced());
        assert_eq!(d.route, TriageRoute::VlmTable);
        assert_eq!(d.reason, Reason::BorderlessTableCandidate);
    }

    #[test]
    fn bordered_table_stays_native_in_balanced() {
        let mut b = block();
        b.is_table_candidate = true;
        b.has_borders = true;
        let d = triage_block(&b, &page(), &TriageProfile::balanced());
        assert_eq!(d.route, TriageRoute::Native);
    }

    #[test]
    fn bordered_table_escalates_in_scientific() {
        let mut b = block();
        b.is_table_candidate = true;
        b.has_borders = true;
        let d = triage_block(&b, &page(), &TriageProfile::scientific());
        assert_eq!(d.route, TriageRoute::VlmTable);
        assert_eq!(d.reason, Reason::ProfilePrefersVlm);
    }

    #[test]
    fn small_image_stays_native() {
        let mut b = block();
        b.is_image = true;
        b.bbox = BBox::new(0.0, 0.0, 50.0, 50.0).unwrap(); // 2500 px² / 1M = 0.25%
        let d = triage_block(&b, &page(), &TriageProfile::balanced());
        assert_eq!(d.route, TriageRoute::Native);
    }

    #[test]
    fn big_image_goes_to_vlm() {
        let mut b = block();
        b.is_image = true;
        // 500x500 = 250000 / 1M = 25% — above the balanced threshold (5%).
        b.bbox = BBox::new(0.0, 0.0, 500.0, 500.0).unwrap();
        let d = triage_block(&b, &page(), &TriageProfile::balanced());
        assert_eq!(d.route, TriageRoute::VlmImage);
        assert_eq!(d.reason, Reason::ImageAreaOverThreshold);
    }

    #[test]
    fn math_with_high_confidence_stays_native() {
        let mut b = block();
        b.contains_math_symbols = true;
        b.confidence = 0.95;
        let d = triage_block(&b, &page(), &TriageProfile::balanced());
        assert_eq!(d.route, TriageRoute::Native);
    }

    #[test]
    fn math_with_low_confidence_goes_to_vlm_formula() {
        let mut b = block();
        b.contains_math_symbols = true;
        b.confidence = 0.5;
        let d = triage_block(&b, &page(), &TriageProfile::balanced());
        assert_eq!(d.route, TriageRoute::VlmFormula);
        assert_eq!(d.reason, Reason::MathSymbolsLowConfidence);
    }

    #[test]
    fn default_route_is_native() {
        let d = triage_block(&block(), &page(), &TriageProfile::balanced());
        assert_eq!(d.route, TriageRoute::Native);
        assert_eq!(d.reason, Reason::DefaultNative);
    }
}

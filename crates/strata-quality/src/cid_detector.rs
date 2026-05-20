//! Detect pages with corrupt CID-encoded text (missing `ToUnicode` CMap, …).
//!
//! PDF fonts that lack a `ToUnicode` CMap produce glyphs whose unicode
//! mapping is undefined — `pdfium-render` then emits the Unicode
//! replacement character `U+FFFD` (or `U+0000` for some CID fonts).
//! Pages containing more than a small fraction of such glyphs cannot be
//! processed by native text extraction; they have to go through full-page
//! OCR.
//!
//! This module is **pure Rust** — it takes a slice of `char`s and a few
//! optional hints, returns a [`Severity`]. No PDFium dependency. The
//! actual extraction of the chars from the page lives in `strata-pdf`.
//!
//! See Plan Maestro §9.T4.1 and §1 (PRISMA convention).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Quality severity reported by every detector in this crate.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Severity {
    /// All checks passed; native extraction is safe.
    None,
    /// Some signal is suspicious but not blocking — the Triage may still
    /// keep the page on the native path with extra validation downstream.
    Warning,
    /// The page must skip native extraction and go straight to OCR.
    Critical,
}

impl Severity {
    pub fn is_critical(self) -> bool {
        matches!(self, Self::Critical)
    }
}

/// Full report from [`evaluate_cid_health`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CidEvaluation {
    pub severity: Severity,
    /// Total number of glyphs evaluated.
    pub glyph_count: usize,
    /// Number of glyphs that came back as `U+FFFD` / `U+0000` / control chars.
    pub unmapped_count: usize,
    /// Fraction `[0.0, 1.0]` of unmapped glyphs over total.
    pub unmapped_ratio: f32,
    /// Shannon entropy (bits) of the codepoint distribution. Real text in
    /// any language has entropy ≥ ~3 bits; pages where every glyph maps to
    /// `U+FFFD` have entropy 0 because the distribution is degenerate.
    pub shannon_entropy: f32,
    /// Convenience: human-readable summary string.
    pub reason: String,
}

/// Critical threshold on the unmapped-glyph ratio. Above this the page
/// cannot be salvaged by native extraction.
const CRITICAL_UNMAPPED_RATIO: f32 = 0.30;
/// Warning threshold on the unmapped-glyph ratio.
const WARNING_UNMAPPED_RATIO: f32 = 0.05;
/// Pages with entropy below this are considered "too repetitive" (likely
/// CID corruption masquerading as a single repeated codepoint).
const MIN_ACCEPTABLE_ENTROPY: f32 = 2.0;
/// Below this minimum glyph count we don't have enough signal to judge.
const MIN_GLYPHS_FOR_SIGNAL: usize = 20;

/// Decide whether the page's text needs OCR based on the unicode quality
/// of its glyphs.
pub fn evaluate_cid_health(unicodes: &[char]) -> CidEvaluation {
    let glyph_count = unicodes.len();
    if glyph_count == 0 {
        return CidEvaluation {
            severity: Severity::None,
            glyph_count: 0,
            unmapped_count: 0,
            unmapped_ratio: 0.0,
            shannon_entropy: 0.0,
            reason: "empty page".into(),
        };
    }

    let unmapped_count = unicodes.iter().filter(|c| is_unmapped(**c)).count();
    let unmapped_ratio = unmapped_count as f32 / glyph_count as f32;
    let entropy = shannon_entropy(unicodes);

    let severity = severity_from_signals(glyph_count, unmapped_ratio, entropy);
    let reason = describe(severity, unmapped_ratio, entropy, glyph_count);

    CidEvaluation { severity, glyph_count, unmapped_count, unmapped_ratio, shannon_entropy: entropy, reason }
}

fn is_unmapped(c: char) -> bool {
    // U+FFFD = Unicode replacement; U+0000 = null. Control chars below
    // 0x20 (except \t \n \r) are normally not present in extracted PDF
    // text — they indicate a broken mapping.
    let code = c as u32;
    c == '\u{FFFD}'
        || c == '\u{0000}'
        || (code < 0x20 && code != b'\t' as u32 && code != b'\n' as u32 && code != b'\r' as u32)
}

fn shannon_entropy(unicodes: &[char]) -> f32 {
    if unicodes.is_empty() {
        return 0.0;
    }
    let mut counts: BTreeMap<char, u32> = BTreeMap::new();
    for &c in unicodes {
        *counts.entry(c).or_insert(0) += 1;
    }
    let total = unicodes.len() as f32;
    counts
        .values()
        .map(|&n| {
            let p = n as f32 / total;
            -p * p.log2()
        })
        .sum()
}

fn severity_from_signals(glyphs: usize, unmapped_ratio: f32, entropy: f32) -> Severity {
    if unmapped_ratio >= CRITICAL_UNMAPPED_RATIO {
        return Severity::Critical;
    }
    if glyphs >= MIN_GLYPHS_FOR_SIGNAL && entropy < MIN_ACCEPTABLE_ENTROPY {
        return Severity::Critical;
    }
    if unmapped_ratio >= WARNING_UNMAPPED_RATIO {
        return Severity::Warning;
    }
    Severity::None
}

fn describe(sev: Severity, ratio: f32, entropy: f32, glyphs: usize) -> String {
    match sev {
        Severity::None => format!(
            "ok: {} glyphs, {:.1}% unmapped, entropy {:.2} bits",
            glyphs,
            ratio * 100.0,
            entropy
        ),
        Severity::Warning => format!(
            "warning: {:.1}% glyphs unmapped (threshold {:.1}%)",
            ratio * 100.0,
            WARNING_UNMAPPED_RATIO * 100.0
        ),
        Severity::Critical => {
            if ratio >= CRITICAL_UNMAPPED_RATIO {
                format!(
                    "critical: {:.1}% glyphs unmapped (threshold {:.1}%)",
                    ratio * 100.0,
                    CRITICAL_UNMAPPED_RATIO * 100.0
                )
            } else {
                format!("critical: entropy {:.2} bits < {:.2}", entropy, MIN_ACCEPTABLE_ENTROPY)
            }
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
    fn empty_page_is_severity_none() {
        let r = evaluate_cid_health(&[]);
        assert_eq!(r.severity, Severity::None);
    }

    #[test]
    fn all_unmapped_is_critical() {
        let glyphs: Vec<char> = vec!['\u{FFFD}'; 200];
        let r = evaluate_cid_health(&glyphs);
        assert_eq!(r.severity, Severity::Critical);
        assert!(r.unmapped_ratio > 0.99);
    }

    #[test]
    fn small_amount_of_unmapped_is_warning() {
        // 10% unmapped (above WARNING_UNMAPPED_RATIO = 5%, below CRITICAL = 30%).
        // We use a diverse alphabet repeated to ensure high entropy.
        let mut glyphs = Vec::new();
        for i in 0..90 {
            glyphs.push(((i % 26) as u8 + b'a') as char);
        }
        glyphs.extend(vec!['\u{FFFD}'; 10]);
        let r = evaluate_cid_health(&glyphs);
        assert_eq!(r.severity, Severity::Warning);
    }

    #[test]
    fn clean_english_text_is_severity_none() {
        let glyphs: Vec<char> = "The quick brown fox jumps over the lazy dog. \
                                  Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
                                  Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
            .chars()
            .collect();
        let r = evaluate_cid_health(&glyphs);
        assert_eq!(r.severity, Severity::None, "{}", r.reason);
        assert!(r.shannon_entropy > 3.0);
    }

    #[test]
    fn degenerate_low_entropy_is_critical() {
        // 50 of 'A' (entropy = 0). Above MIN_GLYPHS threshold.
        let glyphs: Vec<char> = vec!['A'; 50];
        let r = evaluate_cid_health(&glyphs);
        assert_eq!(r.severity, Severity::Critical);
        assert!(r.shannon_entropy < 0.01);
    }

    #[test]
    fn null_chars_count_as_unmapped() {
        let mut glyphs = vec!['x'; 70];
        glyphs.extend(vec!['\u{0000}'; 30]);
        let r = evaluate_cid_health(&glyphs);
        assert_eq!(r.severity, Severity::Critical);
    }

    #[test]
    fn shannon_entropy_zero_for_constant() {
        assert!((shannon_entropy(&['a'; 10]) - 0.0).abs() < 1e-6);
    }

    #[test]
    fn evaluation_round_trips_through_json() {
        let r = evaluate_cid_health(&['a', 'b', 'c', 'd']);
        let json = serde_json::to_string(&r).unwrap();
        let back: CidEvaluation = serde_json::from_str(&json).unwrap();
        assert_eq!(r, back);
    }
}

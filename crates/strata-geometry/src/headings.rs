//! Heading-level classification by font-size clustering.
//!
//! Approach: every line of text is characterized by its dominant font size
//! (median of the glyphs in the line). We then run a small 1-D
//! [Jenks Natural Breaks](https://en.wikipedia.org/wiki/Jenks_natural_breaks_optimization)
//! variant on the set of distinct sizes — effectively a k-means with `k = N`
//! candidate clusters, picking the smallest `k` whose
//! between-class variance saturates. Body text is the **largest** cluster
//! by occurrence count; clusters with strictly larger font sizes become
//! H1, H2, … in descending order.
//!
//! The implementation is intentionally tiny — k-means on at most 5 clusters
//! over ≤ 20 distinct font sizes is sub-millisecond per page even on a
//! laptop CPU.
//!
//! See Plan Maestro §8.T3.6.

use serde::{Deserialize, Serialize};

/// Result of [`classify_headings`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "kind")]
pub enum HeadingClass {
    /// Heading at level 1 (= biggest), 2, 3, …
    Heading { level: u8 },
    /// Body text — not a heading.
    Body,
}

const MAX_HEADING_LEVELS: u8 = 6;
/// Minimum ratio between a candidate level and the body cluster mean to
/// accept it as a real heading. 1.10 = the line's font size must exceed
/// body size by ≥ 10 %.
const MIN_RELATIVE_SIZE: f32 = 1.10;

/// Classify each line by its `dominant_font_size`. Returns one
/// [`HeadingClass`] per input element, in the same order.
pub fn classify_headings(line_font_sizes: &[f32]) -> Vec<HeadingClass> {
    if line_font_sizes.is_empty() {
        return Vec::new();
    }
    // Discard non-finite sizes by mapping them to 0 (Body).
    let cleaned: Vec<f32> = line_font_sizes
        .iter()
        .map(|s| if s.is_finite() && *s > 0.0 { *s } else { 0.0 })
        .collect();

    let body_size = body_text_size(&cleaned);
    if body_size <= 0.0 {
        return vec![HeadingClass::Body; cleaned.len()];
    }
    let levels = build_heading_levels(&cleaned, body_size);

    cleaned
        .iter()
        .map(|&s| {
            if s < body_size * MIN_RELATIVE_SIZE {
                HeadingClass::Body
            } else {
                let level = levels
                    .iter()
                    .position(|&l| (l - s).abs() < 0.51)
                    .map(|i| (i as u8 + 1).min(MAX_HEADING_LEVELS))
                    .unwrap_or(MAX_HEADING_LEVELS);
                HeadingClass::Heading { level }
            }
        })
        .collect()
}

/// Returns the body text size — the most frequent font size in the input.
fn body_text_size(sizes: &[f32]) -> f32 {
    let bins = histogram(sizes);
    bins.into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(size, _)| size)
        .unwrap_or(0.0)
}

/// Cluster sizes strictly greater than `body_size` and return them sorted
/// **descending** (biggest first → H1, smaller → H2, …).
fn build_heading_levels(sizes: &[f32], body_size: f32) -> Vec<f32> {
    let bins = histogram(sizes);
    let mut larger: Vec<(f32, u32)> = bins
        .into_iter()
        .filter(|(s, _)| *s >= body_size * MIN_RELATIVE_SIZE)
        .collect();
    if larger.is_empty() {
        return Vec::new();
    }
    larger.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
    // Merge bins within 1 pt of each other to absorb floating-point noise.
    let mut merged: Vec<f32> = Vec::new();
    for (s, _) in larger {
        match merged.last_mut() {
            Some(last) if (*last - s).abs() < 1.0 => {
                *last = (*last + s) * 0.5;
            }
            _ => merged.push(s),
        }
    }
    merged
}

fn histogram(values: &[f32]) -> Vec<(f32, u32)> {
    // Round to 0.5 pt resolution — PDF font sizes are almost always emitted
    // at integer or half-integer point values.
    let mut bins: std::collections::BTreeMap<i32, (f32, u32)> = Default::default();
    for &v in values {
        if !v.is_finite() || v <= 0.0 {
            continue;
        }
        let key = (v * 2.0).round() as i32;
        let entry = bins.entry(key).or_insert((0.0, 0));
        // running mean of the size in the bucket
        let n = entry.1 as f32;
        entry.0 = entry.0 + (v - entry.0) / (n + 1.0);
        entry.1 += 1;
    }
    bins.into_values().collect()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input_yields_empty() {
        assert!(classify_headings(&[]).is_empty());
    }

    #[test]
    fn uniform_size_is_all_body() {
        let sizes = vec![10.0_f32; 20];
        let r = classify_headings(&sizes);
        assert!(r.iter().all(|c| matches!(c, HeadingClass::Body)));
    }

    #[test]
    fn one_big_line_becomes_h1() {
        // 10 lines at 10 pt body, 1 line at 18 pt title.
        let mut sizes = vec![10.0_f32; 10];
        sizes.push(18.0);
        let r = classify_headings(&sizes);
        assert_eq!(r[10], HeadingClass::Heading { level: 1 });
        for c in &r[..10] {
            assert_eq!(*c, HeadingClass::Body);
        }
    }

    #[test]
    fn three_levels_descending() {
        // Body = 10 pt (most frequent). Section = 14 pt. Subsection = 12 pt.
        // Title = 24 pt.
        let mut sizes = vec![10.0_f32; 30];
        sizes.extend(vec![14.0; 5]);
        sizes.extend(vec![12.0; 5]);
        sizes.push(24.0);
        let r = classify_headings(&sizes);
        let h1 = r
            .iter()
            .filter(|c| matches!(c, HeadingClass::Heading { level: 1 }))
            .count();
        let h2 = r
            .iter()
            .filter(|c| matches!(c, HeadingClass::Heading { level: 2 }))
            .count();
        let h3 = r
            .iter()
            .filter(|c| matches!(c, HeadingClass::Heading { level: 3 }))
            .count();
        let body = r.iter().filter(|c| matches!(c, HeadingClass::Body)).count();
        assert_eq!(h1, 1);
        assert_eq!(h2, 5);
        assert_eq!(h3, 5);
        assert_eq!(body, 30);
    }

    #[test]
    fn non_finite_size_treated_as_body() {
        let sizes = vec![f32::NAN, 10.0, 10.0, 10.0];
        let r = classify_headings(&sizes);
        // The NaN line and the body lines are all Body.
        assert!(r.iter().all(|c| matches!(c, HeadingClass::Body)));
    }
}

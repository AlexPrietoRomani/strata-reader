//! `TriageProfile` — knobs that bias the decision tree (Plan Maestro §9.T4.3).
//!
//! Three pre-defined instances are shipped:
//!
//! - [`TriageProfile::fast`]: minimum VLM coverage. Routes only the most
//!   problematic cases (scanned pages, borderless tables, broken CID).
//! - [`TriageProfile::balanced`]: production defaults. The Plan Maestro
//!   reference behaviour.
//! - [`TriageProfile::scientific`]: maximum fidelity for research papers
//!   — dispatches all tables, all sizeable images and all math blocks
//!   to the VLM, accepting slower wall-clock for the gain in fidelity.
//!
//! `proptest`-friendly: every field is a finite `f32` in a tame range.

use serde::{Deserialize, Serialize};

/// Configurable thresholds the Triage Engine consults.
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TriageProfile {
    /// Block must cover at least this fraction of the page area to be
    /// considered a "sizeable image" and sent to the VLM.
    pub image_min_area_ratio: f32,
    /// Math blocks below this confidence are sent to the VLM formula path.
    pub math_confidence_threshold: f32,
    /// When `true`, every table candidate (bordered or borderless) goes
    /// to the VLM — i.e. the native table reconstruction is bypassed.
    pub always_vlm_tables: bool,
    /// Symbolic name of the profile, propagated to `Provenance` and logs.
    pub name: ProfileName,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProfileName {
    Fast,
    Balanced,
    Scientific,
}

impl TriageProfile {
    /// Profile tuned for throughput — sends fewer blocks to the VLM.
    pub const fn fast() -> Self {
        Self {
            image_min_area_ratio: 0.15,
            math_confidence_threshold: 0.50,
            always_vlm_tables: false,
            name: ProfileName::Fast,
        }
    }

    /// Production default. Matches the Plan Maestro §9.T4.2 specification.
    pub const fn balanced() -> Self {
        Self {
            image_min_area_ratio: 0.05,
            math_confidence_threshold: 0.80,
            always_vlm_tables: false,
            name: ProfileName::Balanced,
        }
    }

    /// Scientific corpus profile — maximum fidelity, longer wall-clock.
    pub const fn scientific() -> Self {
        Self {
            image_min_area_ratio: 0.02,
            math_confidence_threshold: 0.95,
            always_vlm_tables: true,
            name: ProfileName::Scientific,
        }
    }

    pub fn by_name(name: ProfileName) -> Self {
        match name {
            ProfileName::Fast => Self::fast(),
            ProfileName::Balanced => Self::balanced(),
            ProfileName::Scientific => Self::scientific(),
        }
    }
}

impl Default for TriageProfile {
    fn default() -> Self {
        Self::balanced()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profiles_are_strictly_ordered_by_aggressiveness() {
        // scientific should dispatch the most to VLM, fast the least.
        let f = TriageProfile::fast();
        let b = TriageProfile::balanced();
        let s = TriageProfile::scientific();
        assert!(f.image_min_area_ratio > b.image_min_area_ratio);
        assert!(b.image_min_area_ratio > s.image_min_area_ratio);
        assert!(f.math_confidence_threshold < b.math_confidence_threshold);
        assert!(b.math_confidence_threshold < s.math_confidence_threshold);
        assert!(!f.always_vlm_tables);
        assert!(!b.always_vlm_tables);
        assert!(s.always_vlm_tables);
    }

    #[test]
    fn by_name_returns_the_named_profile() {
        assert_eq!(
            TriageProfile::by_name(ProfileName::Fast),
            TriageProfile::fast()
        );
        assert_eq!(
            TriageProfile::by_name(ProfileName::Balanced),
            TriageProfile::balanced()
        );
        assert_eq!(
            TriageProfile::by_name(ProfileName::Scientific),
            TriageProfile::scientific()
        );
    }

    #[test]
    fn default_is_balanced() {
        assert_eq!(TriageProfile::default(), TriageProfile::balanced());
    }

    #[test]
    fn round_trips_through_json() {
        let p = TriageProfile::scientific();
        let s = serde_json::to_string(&p).unwrap();
        let back: TriageProfile = serde_json::from_str(&s).unwrap();
        assert_eq!(p, back);
    }

    #[test]
    fn profile_name_kebab_case() {
        assert_eq!(
            serde_json::to_string(&ProfileName::Fast).unwrap(),
            "\"fast\""
        );
        assert_eq!(
            serde_json::to_string(&ProfileName::Scientific).unwrap(),
            "\"scientific\""
        );
    }
}

//! Geometric primitives: [`Point`], [`Size`], [`BBox`].
//!
//! Coordinates follow the PDF convention: origin at the bottom-left of the
//! media box, units in PDF points (1/72 inch). All values are `f32` finite —
//! constructors enforce non-NaN/non-infinite invariants. See Plan Maestro §6.

use std::fmt;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors raised when constructing geometric primitives from raw values.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum GeometryError {
    #[error("coordinate is not finite (NaN or infinite)")]
    NonFinite,
    #[error("bbox has inverted axis: x0 ({x0}) > x1 ({x1}) or y0 ({y0}) > y1 ({y1})")]
    Inverted { x0: String, x1: String, y0: String, y1: String },
}

/// A 2D point in PDF user space.
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    /// Construct a [`Point`]. Returns `Err` if any coordinate is non-finite.
    pub fn new(x: f32, y: f32) -> Result<Self, GeometryError> {
        if !x.is_finite() || !y.is_finite() {
            return Err(GeometryError::NonFinite);
        }
        Ok(Self { x, y })
    }

    /// Origin `(0, 0)`.
    pub const ORIGIN: Self = Self { x: 0.0, y: 0.0 };
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.4}, {:.4})", self.x, self.y)
    }
}

/// A 2D size (width × height). Both dimensions are non-negative and finite.
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Result<Self, GeometryError> {
        if !width.is_finite() || !height.is_finite() || width < 0.0 || height < 0.0 {
            return Err(GeometryError::NonFinite);
        }
        Ok(Self { width, height })
    }
}

/// Axis-aligned bounding box in PDF user space.
///
/// Invariants enforced by [`BBox::new`]: all four coordinates are finite and
/// `x0 ≤ x1`, `y0 ≤ y1`. The struct is immutable — geometric operations
/// (`intersect`, `union`, `expand`, …) return a *new* [`BBox`].
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BBox {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
}

impl BBox {
    /// Construct a [`BBox`]. Coordinates must be finite and ordered
    /// (`x0 ≤ x1`, `y0 ≤ y1`); otherwise returns [`GeometryError::Inverted`]
    /// or [`GeometryError::NonFinite`].
    pub fn new(x0: f32, y0: f32, x1: f32, y1: f32) -> Result<Self, GeometryError> {
        if !x0.is_finite() || !y0.is_finite() || !x1.is_finite() || !y1.is_finite() {
            return Err(GeometryError::NonFinite);
        }
        if x0 > x1 || y0 > y1 {
            return Err(GeometryError::Inverted {
                x0: format!("{x0}"),
                x1: format!("{x1}"),
                y0: format!("{y0}"),
                y1: format!("{y1}"),
            });
        }
        Ok(Self { x0, y0, x1, y1 })
    }

    /// Construct a [`BBox`] from two arbitrary [`Point`]s (auto min/max).
    pub fn from_corners(a: Point, b: Point) -> Self {
        Self {
            x0: a.x.min(b.x),
            y0: a.y.min(b.y),
            x1: a.x.max(b.x),
            y1: a.y.max(b.y),
        }
    }

    pub fn width(self) -> f32 {
        self.x1 - self.x0
    }

    pub fn height(self) -> f32 {
        self.y1 - self.y0
    }

    pub fn area(self) -> f32 {
        self.width() * self.height()
    }

    pub fn center(self) -> Point {
        Point {
            x: (self.x0 + self.x1) * 0.5,
            y: (self.y0 + self.y1) * 0.5,
        }
    }

    /// `true` iff `point` is inside the closed BBox (boundary inclusive).
    pub fn contains_point(self, point: Point) -> bool {
        point.x >= self.x0 && point.x <= self.x1 && point.y >= self.y0 && point.y <= self.y1
    }

    /// `true` iff `other` is fully inside `self` (boundary inclusive).
    pub fn contains_bbox(self, other: BBox) -> bool {
        other.x0 >= self.x0 && other.x1 <= self.x1 && other.y0 >= self.y0 && other.y1 <= self.y1
    }

    /// `true` iff the two boxes overlap (boundary touch counts as overlap).
    pub fn intersects(self, other: BBox) -> bool {
        self.x0 <= other.x1 && self.x1 >= other.x0 && self.y0 <= other.y1 && self.y1 >= other.y0
    }

    /// Intersection box, or `None` if the boxes are disjoint.
    pub fn intersect(self, other: BBox) -> Option<BBox> {
        if !self.intersects(other) {
            return None;
        }
        Some(BBox {
            x0: self.x0.max(other.x0),
            y0: self.y0.max(other.y0),
            x1: self.x1.min(other.x1),
            y1: self.y1.min(other.y1),
        })
    }

    /// Smallest BBox enclosing both `self` and `other`.
    pub fn union(self, other: BBox) -> BBox {
        BBox {
            x0: self.x0.min(other.x0),
            y0: self.y0.min(other.y0),
            x1: self.x1.max(other.x1),
            y1: self.y1.max(other.y1),
        }
    }

    /// Intersection-over-Union ∈ [0.0, 1.0]. Returns 0.0 when both boxes are
    /// disjoint or both are degenerate (area = 0).
    pub fn iou(self, other: BBox) -> f32 {
        let Some(inter) = self.intersect(other) else {
            return 0.0;
        };
        let inter_area = inter.area();
        let union_area = self.area() + other.area() - inter_area;
        if union_area <= 0.0 {
            return 0.0;
        }
        (inter_area / union_area).clamp(0.0, 1.0)
    }

    /// Inflate the BBox by `margin` on every side. Negative margins shrink it;
    /// if the result would be inverted, both axes collapse to the midpoint.
    pub fn expand(self, margin: f32) -> BBox {
        let nx0 = self.x0 - margin;
        let ny0 = self.y0 - margin;
        let nx1 = self.x1 + margin;
        let ny1 = self.y1 + margin;
        BBox {
            x0: nx0.min(nx1),
            y0: ny0.min(ny1),
            x1: nx0.max(nx1),
            y1: ny0.max(ny1),
        }
    }

    /// `true` iff `area() == 0` (zero-width or zero-height).
    pub fn is_degenerate(self) -> bool {
        self.width() == 0.0 || self.height() == 0.0
    }
}

impl fmt::Display for BBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:.4}, {:.4}, {:.4}, {:.4}]", self.x0, self.y0, self.x1, self.y1)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    /// A proptest strategy that yields a finite, non-NaN f32 in a tame range
    /// (PDF pages are bounded — a few thousand points is plenty).
    fn finite_f32() -> impl Strategy<Value = f32> {
        prop::num::f32::POSITIVE
            .prop_filter("must be finite", |v| v.is_finite())
            .prop_map(|v| v.clamp(-10_000.0, 10_000.0))
    }

    fn bbox_strategy() -> impl Strategy<Value = BBox> {
        (finite_f32(), finite_f32(), finite_f32(), finite_f32())
            .prop_map(|(a, b, c, d)| BBox::from_corners(Point { x: a, y: b }, Point { x: c, y: d }))
    }

    #[test]
    fn new_rejects_nan() {
        assert!(BBox::new(f32::NAN, 0.0, 1.0, 1.0).is_err());
        assert!(BBox::new(0.0, 0.0, f32::INFINITY, 1.0).is_err());
    }

    #[test]
    fn new_rejects_inverted_axis() {
        assert!(matches!(BBox::new(10.0, 0.0, 0.0, 5.0), Err(GeometryError::Inverted { .. })));
    }

    #[test]
    fn area_zero_when_degenerate() {
        let b = BBox::new(5.0, 5.0, 5.0, 10.0).unwrap();
        assert_eq!(b.area(), 0.0);
        assert!(b.is_degenerate());
    }

    #[test]
    fn intersect_disjoint_returns_none() {
        let a = BBox::new(0.0, 0.0, 10.0, 10.0).unwrap();
        let b = BBox::new(20.0, 20.0, 30.0, 30.0).unwrap();
        assert!(a.intersect(b).is_none());
        assert_eq!(a.iou(b), 0.0);
    }

    #[test]
    fn intersect_identical_iou_is_one() {
        let a = BBox::new(0.0, 0.0, 10.0, 10.0).unwrap();
        assert!((a.iou(a) - 1.0).abs() < f32::EPSILON);
    }

    proptest! {
        // ⇒ Idempotencia: IoU de un BBox con sí mismo == 1.0 (salvo área = 0).
        #[test]
        fn iou_self_is_one(b in bbox_strategy()) {
            if !b.is_degenerate() {
                prop_assert!((b.iou(b) - 1.0).abs() < 1e-4);
            } else {
                prop_assert_eq!(b.iou(b), 0.0);
            }
        }

        // ⇒ Conmutatividad.
        #[test]
        fn iou_is_symmetric(a in bbox_strategy(), b in bbox_strategy()) {
            let lhs = a.iou(b);
            let rhs = b.iou(a);
            prop_assert!((lhs - rhs).abs() < 1e-4, "lhs={lhs}, rhs={rhs}");
        }

        // ⇒ intersects ⇔ intersect.is_some()
        #[test]
        fn intersects_iff_intersect_exists(a in bbox_strategy(), b in bbox_strategy()) {
            prop_assert_eq!(a.intersects(b), a.intersect(b).is_some());
        }

        // ⇒ La unión contiene ambos operandos.
        #[test]
        fn union_contains_both(a in bbox_strategy(), b in bbox_strategy()) {
            let u = a.union(b);
            prop_assert!(u.contains_bbox(a), "union {} should contain {}", u, a);
            prop_assert!(u.contains_bbox(b), "union {} should contain {}", u, b);
        }

        // ⇒ Conmutatividad de union.
        #[test]
        fn union_is_commutative(a in bbox_strategy(), b in bbox_strategy()) {
            prop_assert_eq!(a.union(b), b.union(a));
        }

        // ⇒ Monotonicidad: si a ⊆ b, entonces area(a) ≤ area(b).
        #[test]
        fn containment_implies_area_order(a in bbox_strategy(), b in bbox_strategy()) {
            if b.contains_bbox(a) {
                prop_assert!(a.area() <= b.area() + 1e-3);
            }
        }
    }
}

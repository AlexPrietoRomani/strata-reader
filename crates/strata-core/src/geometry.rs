//! Affine 2D transformation matrix in the PDF convention (6-element CTM).
//!
//! A PDF page object carries a *Current Transformation Matrix* expressed as
//! six floats `[a b c d e f]` representing:
//!
//! ```text
//!   | a  b  0 |
//!   | c  d  0 |
//!   | e  f  1 |
//! ```
//!
//! Applied to a point `(x, y)` it produces `(a*x + c*y + e,  b*x + d*y + f)`.
//! See ISO 32000-1 §8.3.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::bbox::{BBox, Point};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Matrix {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
}

impl Matrix {
    /// Identity matrix.
    pub const IDENTITY: Self = Self { a: 1.0, b: 0.0, c: 0.0, d: 1.0, e: 0.0, f: 0.0 };

    pub const fn new(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) -> Self {
        Self { a, b, c, d, e, f }
    }

    pub const fn translation(tx: f32, ty: f32) -> Self {
        Self { a: 1.0, b: 0.0, c: 0.0, d: 1.0, e: tx, f: ty }
    }

    pub const fn scale(sx: f32, sy: f32) -> Self {
        Self { a: sx, b: 0.0, c: 0.0, d: sy, e: 0.0, f: 0.0 }
    }

    /// Apply this matrix to a [`Point`].
    pub fn transform_point(self, p: Point) -> Point {
        Point {
            x: self.a * p.x + self.c * p.y + self.e,
            y: self.b * p.x + self.d * p.y + self.f,
        }
    }

    /// Apply this matrix to a [`BBox`]. The axis-aligned bbox of the
    /// transformed quad is returned (may grow under rotation).
    pub fn transform_bbox(self, bb: BBox) -> BBox {
        let corners = [
            Point { x: bb.x0, y: bb.y0 },
            Point { x: bb.x1, y: bb.y0 },
            Point { x: bb.x0, y: bb.y1 },
            Point { x: bb.x1, y: bb.y1 },
        ];
        let transformed: [Point; 4] = corners.map(|p| self.transform_point(p));
        let (mut min_x, mut min_y) = (transformed[0].x, transformed[0].y);
        let (mut max_x, mut max_y) = (min_x, min_y);
        for p in &transformed[1..] {
            min_x = min_x.min(p.x);
            min_y = min_y.min(p.y);
            max_x = max_x.max(p.x);
            max_y = max_y.max(p.y);
        }
        BBox::from_corners(Point { x: min_x, y: min_y }, Point { x: max_x, y: max_y })
    }

    /// Matrix composition `self * rhs`. Applying the result to a point is
    /// equivalent to first applying `rhs`, then `self`.
    pub fn compose(self, rhs: Matrix) -> Matrix {
        Matrix {
            a: self.a * rhs.a + self.c * rhs.b,
            b: self.b * rhs.a + self.d * rhs.b,
            c: self.a * rhs.c + self.c * rhs.d,
            d: self.b * rhs.c + self.d * rhs.d,
            e: self.a * rhs.e + self.c * rhs.f + self.e,
            f: self.b * rhs.e + self.d * rhs.f + self.f,
        }
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Self::IDENTITY
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn finite_f32() -> impl Strategy<Value = f32> {
        prop::num::f32::POSITIVE
            .prop_filter("finite", |v| v.is_finite())
            .prop_map(|v| v.clamp(-1_000.0, 1_000.0))
    }

    fn point_strategy() -> impl Strategy<Value = Point> {
        (finite_f32(), finite_f32()).prop_map(|(x, y)| Point { x, y })
    }

    fn matrix_strategy() -> impl Strategy<Value = Matrix> {
        (finite_f32(), finite_f32(), finite_f32(), finite_f32(), finite_f32(), finite_f32())
            .prop_map(|(a, b, c, d, e, f)| Matrix { a, b, c, d, e, f })
    }

    #[test]
    fn identity_is_neutral() {
        let p = Point { x: 3.0, y: -7.5 };
        assert_eq!(Matrix::IDENTITY.transform_point(p), p);
    }

    #[test]
    fn translation_shifts_point() {
        let m = Matrix::translation(10.0, -5.0);
        let p = m.transform_point(Point { x: 2.0, y: 3.0 });
        assert!((p.x - 12.0).abs() < 1e-5);
        assert!((p.y - -2.0).abs() < 1e-5);
    }

    #[test]
    fn scale_doubles_distance_from_origin() {
        let m = Matrix::scale(2.0, 3.0);
        let p = m.transform_point(Point { x: 4.0, y: 5.0 });
        assert!((p.x - 8.0).abs() < 1e-5);
        assert!((p.y - 15.0).abs() < 1e-5);
    }

    proptest! {
        // ⇒ Identity ∘ P == P
        #[test]
        fn identity_transform_is_identity(p in point_strategy()) {
            let q = Matrix::IDENTITY.transform_point(p);
            prop_assert!((q.x - p.x).abs() < 1e-4);
            prop_assert!((q.y - p.y).abs() < 1e-4);
        }

        // ⇒ Identity ∘ M == M  (left)
        #[test]
        fn identity_left_compose_is_neutral(m in matrix_strategy()) {
            let r = Matrix::IDENTITY.compose(m);
            prop_assert!((r.a - m.a).abs() < 1e-3);
            prop_assert!((r.b - m.b).abs() < 1e-3);
            prop_assert!((r.c - m.c).abs() < 1e-3);
            prop_assert!((r.d - m.d).abs() < 1e-3);
            prop_assert!((r.e - m.e).abs() < 1e-3);
            prop_assert!((r.f - m.f).abs() < 1e-3);
        }

        // ⇒ Compose-then-apply equals apply-then-apply.
        #[test]
        fn compose_matches_sequential_apply(
            m1 in matrix_strategy(),
            m2 in matrix_strategy(),
            p  in point_strategy(),
        ) {
            let composed = m1.compose(m2).transform_point(p);
            let sequential = m1.transform_point(m2.transform_point(p));
            // Larger tolerance: floating-point error scales with matrix entries.
            let tol = 1e-2 * (composed.x.abs() + composed.y.abs() + 1.0);
            prop_assert!(
                (composed.x - sequential.x).abs() < tol,
                "x: composed={}, sequential={}, tol={}", composed.x, sequential.x, tol
            );
            prop_assert!(
                (composed.y - sequential.y).abs() < tol,
                "y: composed={}, sequential={}, tol={}", composed.y, sequential.y, tol
            );
        }
    }
}

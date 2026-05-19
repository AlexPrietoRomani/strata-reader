//! A thin, ergonomic wrapper around [`rstar::RTree`] for axis-aligned bounding
//! boxes in PDF user space.
//!
//! The wrapper exists for three reasons:
//!
//! 1. The default [`rstar::RTree`] is parametric on a custom geometry trait —
//!    every callsite would have to define `RTreeObject` + `PointDistance` by
//!    hand. We do it once here.
//! 2. We always store both a [`BBox`] envelope and a generic payload `T`
//!    (typically an index into a glyph or block vector). The wrapper hides
//!    that twofold storage.
//! 3. Bench target from Plan Maestro §8.T3.1: `query_range < 50 µs` for
//!    pages with ≤ 5 000 glyphs. Using `RTree::bulk_load` (Hilbert R*-tree)
//!    rather than incremental inserts is the recommended pattern here.

use rstar::{primitives::GeomWithData, PointDistance, RTree, RTreeObject, AABB};
use strata_core::{BBox, Point};

/// Internal wrapper that pairs an arbitrary payload with its R-Tree envelope.
/// We pre-compute `[x_center, y_center]` so `PointDistance::distance_2` runs
/// without re-touching the BBox.
#[derive(Clone, Debug)]
struct Envelope<T> {
    bbox: BBox,
    payload: T,
    center: [f32; 2],
}

impl<T> RTreeObject for Envelope<T> {
    type Envelope = AABB<[f32; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_corners([self.bbox.x0, self.bbox.y0], [self.bbox.x1, self.bbox.y1])
    }
}

impl<T> PointDistance for Envelope<T> {
    fn distance_2(&self, point: &[f32; 2]) -> f32 {
        let dx = self.center[0] - point[0];
        let dy = self.center[1] - point[1];
        dx * dx + dy * dy
    }
}

/// One entry returned by [`SpatialIndex::query_range`] / [`SpatialIndex::nearest_k`].
#[derive(Clone, Debug)]
pub struct Hit<'a, T> {
    pub bbox: BBox,
    pub payload: &'a T,
}

/// 2-D spatial index over items with an axis-aligned [`BBox`] envelope.
pub struct SpatialIndex<T> {
    tree: RTree<Envelope<T>>,
}

impl<T: Clone> Default for SpatialIndex<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> SpatialIndex<T> {
    /// Empty index. Prefer [`SpatialIndex::bulk_load`] when all items are
    /// known up front — bulk loading produces a noticeably better tree.
    pub fn new() -> Self {
        Self { tree: RTree::new() }
    }

    /// Build the index from an iterator of `(bbox, payload)` pairs in O(n log n).
    /// Uses `rstar`'s default packed Hilbert R-Tree algorithm.
    pub fn bulk_load(items: impl IntoIterator<Item = (BBox, T)>) -> Self {
        let envs: Vec<Envelope<T>> = items
            .into_iter()
            .map(|(bbox, payload)| {
                let c = bbox.center();
                Envelope { bbox, payload, center: [c.x, c.y] }
            })
            .collect();
        Self { tree: RTree::bulk_load(envs) }
    }

    /// Insert one item. O(log n) amortized.
    pub fn insert(&mut self, bbox: BBox, payload: T) {
        let c = bbox.center();
        self.tree.insert(Envelope { bbox, payload, center: [c.x, c.y] });
    }

    /// Number of items in the index.
    pub fn len(&self) -> usize {
        self.tree.size()
    }

    pub fn is_empty(&self) -> bool {
        self.tree.size() == 0
    }

    /// Return every item whose envelope intersects `query`. Touching edges
    /// count as an intersection (matches [`BBox::intersects`]).
    pub fn query_range(&self, query: BBox) -> Vec<Hit<'_, T>> {
        let env = AABB::from_corners([query.x0, query.y0], [query.x1, query.y1]);
        self.tree
            .locate_in_envelope_intersecting(&env)
            .map(|e| Hit { bbox: e.bbox, payload: &e.payload })
            .collect()
    }

    /// The `k` items closest (by center-to-point Euclidean distance) to
    /// `point`. Returns fewer than `k` items if the index has fewer.
    pub fn nearest_k(&self, point: Point, k: usize) -> Vec<Hit<'_, T>> {
        self.tree
            .nearest_neighbor_iter(&[point.x, point.y])
            .take(k)
            .map(|e| Hit { bbox: e.bbox, payload: &e.payload })
            .collect()
    }

    /// Iterate every (bbox, payload) currently in the index — order is
    /// implementation-defined (depends on R-Tree internals).
    pub fn iter(&self) -> impl Iterator<Item = (BBox, &T)> + '_ {
        self.tree.iter().map(|e| (e.bbox, &e.payload))
    }
}

// Note: `GeomWithData` is re-exported in case downstream code wants to
// build its own RTreeObject; the wrapper above does not need it.
#[allow(dead_code)]
type _GeomWithDataAlias<G, T> = GeomWithData<G, T>;

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn bb(x0: f32, y0: f32, x1: f32, y1: f32) -> BBox {
        BBox::new(x0, y0, x1, y1).unwrap()
    }

    #[test]
    fn empty_index_has_zero_size() {
        let idx: SpatialIndex<u32> = SpatialIndex::new();
        assert!(idx.is_empty());
        assert_eq!(idx.len(), 0);
    }

    #[test]
    fn bulk_load_preserves_count() {
        let items = vec![
            (bb(0.0, 0.0, 10.0, 10.0), 0u32),
            (bb(20.0, 20.0, 30.0, 30.0), 1u32),
            (bb(15.0, 15.0, 25.0, 25.0), 2u32),
        ];
        let idx = SpatialIndex::bulk_load(items);
        assert_eq!(idx.len(), 3);
    }

    #[test]
    fn query_range_returns_only_intersecting() {
        let items = vec![
            (bb(0.0, 0.0, 10.0, 10.0), "a"),
            (bb(20.0, 20.0, 30.0, 30.0), "b"),
            (bb(50.0, 50.0, 60.0, 60.0), "c"),
        ];
        let idx = SpatialIndex::bulk_load(items);
        let hits = idx.query_range(bb(5.0, 5.0, 25.0, 25.0));
        let mut payloads: Vec<&str> = hits.iter().map(|h| *h.payload).collect();
        payloads.sort();
        assert_eq!(payloads, vec!["a", "b"]);
    }

    #[test]
    fn nearest_k_returns_closest_first() {
        let items = vec![
            (bb(0.0, 0.0, 1.0, 1.0), 0u32),    // center 0.5, 0.5
            (bb(100.0, 100.0, 101.0, 101.0), 1u32),
            (bb(10.0, 10.0, 11.0, 11.0), 2u32), // center 10.5, 10.5
        ];
        let idx = SpatialIndex::bulk_load(items);
        let near = idx.nearest_k(Point { x: 0.0, y: 0.0 }, 2);
        assert_eq!(near.len(), 2);
        assert_eq!(*near[0].payload, 0u32);
        assert_eq!(*near[1].payload, 2u32);
    }

    #[test]
    fn insert_then_query() {
        let mut idx = SpatialIndex::<&'static str>::new();
        idx.insert(bb(0.0, 0.0, 10.0, 10.0), "hello");
        idx.insert(bb(100.0, 100.0, 110.0, 110.0), "world");
        let hits = idx.query_range(bb(5.0, 5.0, 6.0, 6.0));
        assert_eq!(hits.len(), 1);
        assert_eq!(*hits[0].payload, "hello");
    }
}

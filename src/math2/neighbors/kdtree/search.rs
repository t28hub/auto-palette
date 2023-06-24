use crate::math2::distance::DistanceMetric;
use crate::math2::neighbors::kdtree::node::KDNode;
use crate::math2::neighbors::neighbor::Neighbor;
use crate::math2::neighbors::search::NeighborSearch;
use crate::number::Float;
use ndarray::{ArrayView1, CowArray, Ix2};
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

/// Struct representing k-d tree search algorithm for neighbor search.
///
/// # Type Parameters
/// * `F` - The float type used for calculations.
pub struct KDTreeSearch<'a, F>
where
    F: Float,
{
    root: Option<Box<KDNode>>,
    points: CowArray<'a, F, Ix2>,
    metric: &'a DistanceMetric,
}

impl<'a, F> KDTreeSearch<'a, F>
where
    F: Float,
{
    /// Creates a new `KDTreeSearch` instance.
    ///
    /// # Arguments
    /// * `points` - The reference of a dataset of points.
    /// * `metric` - The distance metric to use.
    ///
    /// # Returns
    /// A new `KDTreeSearch` instance.
    #[allow(unused)]
    #[inline]
    #[must_use]
    pub fn new(points: CowArray<'a, F, Ix2>, metric: &'a DistanceMetric) -> Self {
        let mut indices: Vec<_> = (0..points.nrows()).collect();
        let root = Self::build_node(&points, &mut indices, 0);
        Self {
            root: root.map(Box::new),
            points,
            metric,
        }
    }

    #[inline]
    #[must_use]
    fn build_node(
        points: &CowArray<'a, F, Ix2>,
        indices: &mut [usize],
        depth: usize,
    ) -> Option<KDNode> {
        if indices.is_empty() {
            return None;
        }

        let axis = depth % points.ncols();
        indices.sort_unstable_by(|&index1, &index2| {
            let value1 = points.row(index1)[axis];
            let value2 = points.row(index2)[axis];
            value1.partial_cmp(&value2).unwrap_or(Ordering::Equal)
        });

        let median = indices.len() / 2;
        let node = KDNode::new(
            indices[median],
            axis,
            Self::build_node(points, &mut indices[..median], depth + 1),
            Self::build_node(points, &mut indices[median + 1..], depth + 1),
        );
        Some(node)
    }

    #[inline]
    fn search_recursively(
        &self,
        root: &Option<Box<KDNode>>,
        query: &ArrayView1<F>,
        k: usize,
        neighbors: &mut BinaryHeap<Reverse<Neighbor<F>>>,
    ) {
        let Some(ref node) = root else {
            return;
        };

        let point = self.points.row(node.index);
        let neighbor = Neighbor::new(node.index, self.metric.measure(&point, query));
        neighbors.push(Reverse(neighbor));

        if node.is_leaf() {
            return;
        }

        let delta = query[node.axis] - point[node.axis];
        let min_distance = neighbors
            .peek()
            .map(|Reverse(neighbor)| neighbor.distance)
            .unwrap_or(F::max_value());
        if neighbors.len() < k || delta.abs() <= min_distance {
            self.search_recursively(node.left(), query, k, neighbors);
            self.search_recursively(node.right(), query, k, neighbors);
        } else if delta < F::zero() {
            self.search_recursively(node.left(), query, k, neighbors);
        } else {
            self.search_recursively(node.right(), query, k, neighbors);
        }
    }

    #[inline]
    fn search_radius_recursively(
        &self,
        root: &Option<Box<KDNode>>,
        query: &ArrayView1<F>,
        radius: F,
        neighbors: &mut BinaryHeap<Reverse<Neighbor<F>>>,
    ) {
        let Some(ref node) = root else {
            return;
        };

        let point = self.points.row(node.index);
        let distance = self.metric.measure(&point, query);
        if distance <= radius {
            let neighbor = Neighbor::new(node.index, distance);
            neighbors.push(Reverse(neighbor));
        }

        if node.is_leaf() {
            return;
        }

        let delta = query[node.axis] - point[node.axis];
        if delta.abs() <= radius {
            self.search_radius_recursively(node.left(), query, radius, neighbors);
            self.search_radius_recursively(node.right(), query, radius, neighbors);
        } else if delta < F::zero() {
            self.search_radius_recursively(node.left(), query, radius, neighbors);
        } else {
            self.search_radius_recursively(node.right(), query, radius, neighbors);
        }
    }
}

impl<'a, F> NeighborSearch<F> for KDTreeSearch<'a, F>
where
    F: Float,
{
    #[inline]
    #[must_use]
    fn search(&self, query: &ArrayView1<F>, k: usize) -> Vec<Neighbor<F>> {
        if k == 0 {
            return Vec::new();
        }

        let mut heap = BinaryHeap::new();
        self.search_recursively(&self.root, query, k, &mut heap);

        let mut neighbors = Vec::with_capacity(k);
        while let Some(Reverse(neighbor)) = heap.pop() {
            neighbors.push(neighbor);
            if neighbors.len() == k {
                break;
            }
        }
        neighbors
    }

    #[inline]
    #[must_use]
    fn search_nearest(&self, query: &ArrayView1<F>) -> Option<Neighbor<F>> {
        let mut heap = BinaryHeap::new();
        self.search_recursively(&self.root, query, 1, &mut heap);
        if let Some(Reverse(neighbor)) = heap.pop() {
            Some(neighbor)
        } else {
            None
        }
    }

    #[inline]
    #[must_use]
    fn search_radius(&self, query: &ArrayView1<F>, radius: F) -> Vec<Neighbor<F>> {
        if radius < F::zero() {
            return Vec::new();
        }

        let mut heap = BinaryHeap::new();
        self.search_radius_recursively(&self.root, query, radius, &mut heap);

        let mut neighbors = Vec::with_capacity(heap.len());
        while let Some(Reverse(neighbor)) = heap.pop() {
            neighbors.push(neighbor);
        }
        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::{array, aview1, Array2};

    #[must_use]
    fn sample_points() -> Array2<f64> {
        array![
            [1.0, 2.0],
            [3.0, 1.0],
            [4.0, 5.0],
            [5.0, 5.0],
            [2.0, 4.0],
            [0.0, 5.0],
            [2.0, 1.0],
            [5.0, 2.0]
        ]
    }

    #[test]
    fn test_search() {
        let points = sample_points();
        let search = KDTreeSearch::new(points.into(), &DistanceMetric::SquaredEuclidean);

        let actual = search.search(&aview1(&[3.0, 2.0]), 0);
        assert_eq!(actual.len(), 0);

        let actual = search.search(&aview1(&[3.0, 2.0]), 1);
        assert_eq!(actual.len(), 1);
        assert_eq!(actual[0], Neighbor::new(1, 1.0));

        let actual = search.search(&aview1(&[3.0, 2.0]), 2);
        assert_eq!(actual.len(), 2);
        assert_eq!(actual[0], Neighbor::new(1, 1.0));
        assert_eq!(actual[1], Neighbor::new(6, 2.0));

        let actual = search.search(&aview1(&[3.0, 2.0]), 16);
        assert_eq!(actual.len(), 8);
    }

    #[test]
    fn test_search_nearest() {
        let points = sample_points();
        let search = KDTreeSearch::new(points.into(), &DistanceMetric::SquaredEuclidean);

        let actual = search.search_nearest(&aview1(&[3.0, 2.0]));
        assert_eq!(actual, Some(Neighbor::new(1, 1.0)));

        let actual = search.search_nearest(&aview1(&[0.0, 0.0]));
        assert_eq!(actual, Some(Neighbor::new(6, 5.0)));

        let actual = search.search_nearest(&aview1(&[6.0, 5.0]));
        assert_eq!(actual, Some(Neighbor::new(3, 1.0)));
    }

    #[test]
    fn test_search_radius() {
        let points = sample_points();
        let search = KDTreeSearch::new(points.into(), &DistanceMetric::SquaredEuclidean);

        let actual = search.search_radius(&aview1(&[3.0, 2.0]), -1.0);
        assert_eq!(actual.len(), 0);

        let actual = search.search_radius(&aview1(&[3.0, 2.0]), 0.0);
        assert_eq!(actual.len(), 0);

        let actual = search.search_radius(&aview1(&[3.0, 3.0]), 2.0);
        assert_eq!(actual.len(), 1);
        assert_eq!(actual[0], Neighbor::new(4, 2.0));

        let actual = search.search_radius(&aview1(&[2.0, 2.5]), 2.5);
        assert_eq!(actual.len(), 3);
        assert_eq!(actual[0], Neighbor::new(0, 1.25));
        assert_eq!(actual[1], Neighbor::new(6, 2.25));
        assert_eq!(actual[2], Neighbor::new(4, 2.25));

        let actual = search.search_radius(&aview1(&[3.0, 2.0]), 18.0);
        assert_eq!(actual.len(), 8);
    }
}

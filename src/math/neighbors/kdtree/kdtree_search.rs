use crate::math::distance::Distance;
use crate::math::neighbors::kdtree::kdnode::KDNode;
use crate::math::neighbors::neighbor::Neighbor;
use crate::math::neighbors::neighbor_search::NeighborSearch;
use crate::math::number::Float;
use crate::math::point::Point;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::marker::PhantomData;
use std::ops::Div;

/// Struct representing kd-tree search algorithm for neighbor search.
///
/// # Type Parameters
/// * `F` - The float type used for calculations.
/// * `P` - The type of points used in the neighbor search algorithm.
#[derive(Debug)]
pub struct KDTreeSearch<'a, F, P>
where
    F: Float,
    P: Point<F>,
{
    root: Option<Box<KDNode>>,
    dataset: &'a [P],
    distance: Distance,
    _marker: PhantomData<F>,
}

impl<'a, F, P> KDTreeSearch<'a, F, P>
where
    F: Float,
    P: Point<F>,
{
    /// Creates a new `KDTreeSearch` instance.
    ///
    /// # Arguments
    /// * `dataset` - The reference of a dataset of points.
    /// * `distance` - The distance metric to use.
    ///
    /// # Returns
    /// A new `KDTreeSearch` instance.
    #[must_use]
    pub fn new(dataset: &'a [P], distance: Distance) -> Self {
        let root = if dataset.is_empty() {
            None
        } else {
            let mut indices: Vec<usize> = (0..dataset.len()).collect();
            Self::build_node(dataset, &mut indices, 0)
        };

        Self {
            root: root.map(Box::new),
            dataset,
            distance,
            _marker: PhantomData::default(),
        }
    }

    fn build_node(dataset: &'a [P], indices: &mut [usize], depth: usize) -> Option<KDNode> {
        if indices.is_empty() {
            return None;
        }

        let axis = depth % dataset[0].dim();
        indices.sort_unstable_by(|index1, index2| {
            let value1 = dataset[*index1].index(axis);
            let value2 = dataset[*index2].index(axis);
            value1.partial_cmp(value2).unwrap_or(Ordering::Equal)
        });

        let node = {
            let median = indices.len().div(2);
            KDNode::new(
                indices[median],
                axis,
                Self::build_node(dataset, &mut indices[..median], depth + 1),
                Self::build_node(dataset, &mut indices[median + 1..], depth + 1),
            )
        };
        Some(node)
    }

    fn search_recursively(
        &self,
        root: &Option<Box<KDNode>>,
        query: &P,
        k: usize,
        heap: &mut BinaryHeap<Reverse<Neighbor<F>>>,
    ) {
        let Some(ref node) = root else {
            return;
        };

        let point = self.dataset[node.index];
        let neighbor = {
            let distance = self.distance.measure(&point, query);
            Neighbor::new(node.index, distance)
        };
        heap.push(Reverse(neighbor));

        if node.is_leaf() {
            return;
        }

        let delta = query[node.axis] - point[node.axis];
        let distance = heap
            .peek()
            .map(|neighbor| neighbor.0.distance)
            .unwrap_or(F::min_value());
        if heap.len() < k || delta.abs() <= distance {
            self.search_recursively(node.left(), query, k, heap);
            self.search_recursively(node.right(), query, k, heap);
        } else if delta < F::zero() {
            self.search_recursively(node.left(), query, k, heap);
        } else {
            self.search_recursively(node.right(), query, k, heap);
        }
    }

    fn search_radius_recursively(
        &self,
        root: &Option<Box<KDNode>>,
        query: &P,
        radius: F,
        heap: &mut BinaryHeap<Reverse<Neighbor<F>>>,
    ) {
        let Some(ref node) = root else {
            return;
        };

        let point = self.dataset[node.index];
        let distance = self.distance.measure(&point, query);
        if distance <= radius {
            heap.push(Reverse(Neighbor::new(node.index, distance)));
        }

        let delta = query[node.axis] - point[node.axis];
        if delta.abs() <= radius {
            self.search_radius_recursively(node.left(), query, radius, heap);
            self.search_radius_recursively(node.right(), query, radius, heap);
        } else if delta < F::zero() {
            self.search_radius_recursively(node.left(), query, radius, heap);
        } else {
            self.search_radius_recursively(node.right(), query, radius, heap);
        }
    }
}

impl<'a, F, P> NeighborSearch<F, P> for KDTreeSearch<'a, F, P>
where
    F: Float,
    P: Point<F>,
{
    #[must_use]
    fn search(&self, query: &P, k: usize) -> Vec<Neighbor<F>> {
        if k == 0 {
            return Vec::new();
        }

        let mut heap = BinaryHeap::new();
        self.search_recursively(&self.root, query, k, &mut heap);

        let mut neighbors = Vec::with_capacity(k);
        while let Some(neighbor) = heap.pop() {
            neighbors.push(neighbor.0);
            if neighbors.len() == k {
                break;
            }
        }
        neighbors
    }

    #[must_use]
    fn search_nearest(&self, query: &P) -> Option<Neighbor<F>> {
        self.search(query, 1).pop()
    }

    #[must_use]
    fn search_radius(&self, query: &P, radius: F) -> Vec<Neighbor<F>> {
        if radius < F::zero() {
            return Vec::new();
        }

        let mut heap = BinaryHeap::new();
        self.search_radius_recursively(&self.root, query, radius, &mut heap);

        let mut neighbors = Vec::with_capacity(heap.len());
        while let Some(neighbor) = heap.pop() {
            neighbors.push(neighbor.0);
        }
        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::point::Point2;

    fn empty_dataset() -> Vec<Point2<f64>> {
        vec![]
    }

    fn sample_dataset() -> Vec<Point2<f64>> {
        vec![
            Point2(1.0, 2.0),
            Point2(3.0, 1.0),
            Point2(4.0, 5.0),
            Point2(5.0, 5.0),
            Point2(2.0, 4.0),
            Point2(0.0, 5.0),
            Point2(2.0, 1.0),
            Point2(5.0, 2.0),
        ]
    }

    #[test]
    fn search_should_return_knearest_neighbors() {
        let dataset = empty_dataset();
        let kdtree_search = KDTreeSearch::new(&dataset, Distance::SquaredEuclidean);
        assert_eq!(kdtree_search.search(&Point2(3.0, 3.0), 4), vec![]);

        let dataset = sample_dataset();
        let kdtree_search = KDTreeSearch::new(&dataset, Distance::SquaredEuclidean);
        assert_eq!(kdtree_search.search(&Point2(3.0, 3.0), 0), vec![]);
        assert_eq!(
            kdtree_search.search(&Point2(3.0, 3.0), 1),
            vec![Neighbor::new(4, 2.0),]
        );
        assert_eq!(
            kdtree_search.search(&Point2(3.0, 3.0), 2),
            vec![Neighbor::new(4, 2.0), Neighbor::new(1, 4.0),]
        );
        assert_eq!(
            kdtree_search.search(&Point2(3.0, 3.0), 10),
            vec![
                Neighbor::new(4, 2.0),
                Neighbor::new(1, 4.0),
                Neighbor::new(6, 5.0),
                Neighbor::new(2, 5.0),
                Neighbor::new(7, 5.0),
                Neighbor::new(0, 5.0),
                Neighbor::new(3, 8.0),
                Neighbor::new(5, 13.0),
            ]
        );
    }

    #[test]
    fn search_nearest_should_return_nearest_neighbor() {
        let dataset = empty_dataset();
        let kdtree_search = KDTreeSearch::new(&dataset, Distance::SquaredEuclidean);
        assert_eq!(kdtree_search.search_nearest(&Point2(2.5, 3.0)), None);

        let dataset = sample_dataset();
        let kdtree_search = KDTreeSearch::new(&dataset, Distance::SquaredEuclidean);
        assert_eq!(
            kdtree_search.search_nearest(&Point2(2.5, 3.0)),
            Some(Neighbor::new(4, 1.25))
        );
    }

    #[test]
    fn search_radius_should_return_neighbors_within_radius() {
        let dataset = empty_dataset();
        let kdtree_search = KDTreeSearch::new(&dataset, Distance::SquaredEuclidean);
        assert_eq!(kdtree_search.search_radius(&Point2(3.0, 3.0), 5.0), vec![]);

        let dataset = sample_dataset();
        let kdtree_search = KDTreeSearch::new(&dataset, Distance::SquaredEuclidean);
        assert_eq!(kdtree_search.search_radius(&Point2(3.0, 3.0), -1.0), vec![]);
        assert_eq!(kdtree_search.search_radius(&Point2(3.0, 3.0), 1.0), vec![]);
        assert_eq!(
            kdtree_search.search_radius(&Point2(3.0, 3.0), 2.0),
            vec![Neighbor::new(4, 2.0),]
        );
        assert_eq!(
            kdtree_search.search_radius(&Point2(2.0, 2.5), 2.5),
            vec![
                Neighbor::new(0, 1.25),
                Neighbor::new(6, 2.25),
                Neighbor::new(4, 2.25),
            ]
        );
        assert_eq!(
            kdtree_search.search_radius(&Point2(3.0, 3.0), 5.0),
            vec![
                Neighbor::new(4, 2.0),
                Neighbor::new(1, 4.0),
                Neighbor::new(6, 5.0),
                Neighbor::new(7, 5.0),
                Neighbor::new(2, 5.0),
                Neighbor::new(0, 5.0),
            ]
        );
        assert_eq!(
            kdtree_search.search_radius(&Point2(1.0, 3.0), 23.0),
            vec![
                Neighbor::new(0, 1.0),
                Neighbor::new(4, 2.0),
                Neighbor::new(6, 5.0),
                Neighbor::new(5, 5.0),
                Neighbor::new(1, 8.0),
                Neighbor::new(2, 13.0),
                Neighbor::new(7, 17.0),
                Neighbor::new(3, 20.0),
            ]
        );
    }
}

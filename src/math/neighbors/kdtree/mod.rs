use crate::math::distance::Distance;
use crate::math::neighbors::nns::{Neighbor, NeighborSearch};
use crate::math::number::Float;
use crate::math::point::Point;
use element::Element;
use node::Node;
use std::cmp::Ordering::Greater;
use std::collections::BinaryHeap;
use std::marker::PhantomData;
use std::ops::Div;

mod element;
mod node;

/// A nearest neighbor search using KDTree.
#[derive(Debug)]
pub(crate) struct KDTree<'a, F, P>
where
    F: Float,
    P: Point<F>,
{
    _t: PhantomData<F>,
    root: Option<Box<Node>>,
    dataset: &'a Vec<P>,
    distance: Distance,
}

impl<'a, F, P> KDTree<'a, F, P>
where
    F: Float,
    P: Point<F>,
{
    /// Create a new KDTree.
    pub fn new(dataset: &'a Vec<P>, distance: Distance) -> Self {
        let mut indices: Vec<usize> = (0..dataset.len()).collect();
        let root = Self::build_node(dataset, &mut indices, 0);
        KDTree {
            _t: PhantomData::default(),
            root: root.map(Box::new),
            dataset,
            distance,
        }
    }

    fn search_recursively(
        &self,
        root: Option<&Box<Node>>,
        query: &P,
        k: usize,
        heap: &mut BinaryHeap<Element<F>>,
    ) {
        let Some(node) = root else {
            return;
        };

        let index = node.index();
        let point = self.dataset[index];
        let element = {
            let distance = self.distance.measure(&point, query);
            Element::new(index, distance)
        };
        heap.push(element);
        if node.is_leaf() {
            return;
        }

        let delta = {
            let axis = node.axis();
            query[axis] - point[axis]
        };
        let distance = heap.peek().map(|e| e.distance()).unwrap_or(F::min_value());
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
        root: Option<&Box<Node>>,
        query: &P,
        radius: F,
        results: &mut BinaryHeap<Element<F>>,
    ) {
        let Some(node) = root else {
            return;
        };

        let index = node.index();
        let point = self.dataset[index];
        let distance = self.distance.measure(&point, query);
        if distance <= radius {
            results.push(Element::new(index, distance));
        }

        let delta = {
            let axis = node.axis();
            query[axis] - point[axis]
        };
        if delta.abs() <= radius {
            self.search_radius_recursively(node.left(), query, radius, results);
            self.search_radius_recursively(node.right(), query, radius, results);
        } else if delta < F::zero() {
            self.search_radius_recursively(node.left(), query, radius, results);
        } else {
            self.search_radius_recursively(node.right(), query, radius, results);
        }
    }

    fn build_node(dataset: &'a [P], indices: &mut [usize], depth: usize) -> Option<Node> {
        if dataset.is_empty() || indices.is_empty() {
            return None;
        }

        let axis = depth % dataset[0].dim();
        indices.sort_unstable_by(|index1, index2| {
            let lhs = dataset[*index1].index(axis);
            let rhs = dataset[*index2].index(axis);
            lhs.partial_cmp(rhs).unwrap_or(Greater)
        });

        let node = {
            let median = indices.len().div(2);
            Node::new(
                indices[median],
                axis,
                Self::build_node(dataset, &mut indices[..median], depth + 1),
                Self::build_node(dataset, &mut indices[median + 1..], depth + 1),
            )
        };
        Some(node)
    }
}

impl<F, P> NeighborSearch<F, P> for KDTree<'_, F, P>
where
    F: Float,
    P: Point<F>,
{
    fn search(&self, query: &P, k: usize) -> Vec<Neighbor<F>> {
        if k < 1 {
            return Vec::new();
        }

        let mut heap: BinaryHeap<Element<F>> = BinaryHeap::new();
        self.search_recursively(self.root.as_ref(), query, k, &mut heap);

        let mut neighbors = Vec::with_capacity(k);
        while let Some(element) = heap.pop() {
            neighbors.push(Neighbor::new(element.index(), element.distance()));
            if neighbors.len() == k {
                break;
            }
        }
        neighbors
    }

    fn search_nearest(&self, query: &P) -> Option<Neighbor<F>> {
        self.search(query, 1).pop()
    }

    fn search_radius(&self, query: &P, radius: F) -> Vec<Neighbor<F>> {
        if radius < F::zero() {
            return Vec::new();
        }

        let mut results: BinaryHeap<Element<F>> = BinaryHeap::new();
        self.search_radius_recursively(self.root.as_ref(), query, radius, &mut results);

        let mut neighbors = Vec::with_capacity(results.len());
        while let Some(element) = results.pop() {
            neighbors.push(Neighbor::new(element.index(), element.distance()));
        }
        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::point::Point2;

    const DATASET: [Point2<f32>; 8] = [
        Point2(1.0, 2.0),
        Point2(3.0, 1.0),
        Point2(4.0, 5.0),
        Point2(5.0, 5.0),
        Point2(2.0, 4.0),
        Point2(0.0, 5.0),
        Point2(2.0, 1.0),
        Point2(5.0, 2.0),
    ];

    #[test]
    fn search_should_return_knearest_neighbors() {
        let dataset = Vec::from(DATASET);
        let kdtree = KDTree::new(&dataset, Distance::SquaredEuclidean);
        assert_eq!(kdtree.search(&Point2(3.0, 3.0), 0), vec![]);
        assert_eq!(
            kdtree.search(&Point2(3.0, 3.0), 1),
            vec![Neighbor::new(4, 2.0),]
        );
        assert_eq!(
            kdtree.search(&Point2(3.0, 3.0), 2),
            vec![Neighbor::new(4, 2.0), Neighbor::new(1, 4.0),]
        );
        assert_eq!(
            kdtree.search(&Point2(3.0, 3.0), 10),
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
    fn search_should_return_neighbors_within_radius() {
        let dataset = Vec::from(DATASET);
        let kdtree = KDTree::new(&dataset, Distance::SquaredEuclidean);
        assert_eq!(kdtree.search_radius(&Point2(3.0, 3.0), -1.0), vec![]);
        assert_eq!(kdtree.search_radius(&Point2(3.0, 3.0), 1.0), vec![]);
        assert_eq!(
            kdtree.search_radius(&Point2(3.0, 3.0), 2.0),
            vec![Neighbor::new(4, 2.0),]
        );
        assert_eq!(
            kdtree.search_radius(&Point2(3.0, 3.0), 2.5),
            vec![Neighbor::new(4, 2.0),]
        );
        assert_eq!(
            kdtree.search_radius(&Point2(3.0, 3.0), 5.0),
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
            kdtree.search_radius(&Point2(3.0, 3.0), 15.0),
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
}

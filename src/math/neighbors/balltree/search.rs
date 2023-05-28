use crate::math::distance::Distance;
use crate::math::neighbors::balltree::node::Node;
use crate::math::neighbors::neighbor::Neighbor;
use crate::math::point::Point;
use crate::number::Float;
use std::borrow::Cow;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

#[derive(Debug)]
pub struct BallTreeSearch<'a, F, P>
where
    F: Float,
    P: Point<F>,
{
    root: Option<Box<Node<F, P>>>,
    dataset: Cow<'a, [P]>,
    distance: &'a Distance,
}

impl<'a, F, P> BallTreeSearch<'a, F, P>
where
    F: Float,
    P: Point<F>,
{
    #[must_use]
    pub fn new(dataset: &'a [P], distance: &'a Distance) -> Self {
        let mut indices: Vec<usize> = (0..dataset.len()).collect();
        let root = Self::build_node(dataset, &mut indices, distance);
        Self {
            root: root.map(Box::new),
            dataset: Cow::Borrowed(dataset),
            distance,
        }
    }

    #[must_use]
    pub fn search(&self, query: &P, k: usize) -> Vec<Neighbor<F>> {
        if k == 0 {
            return Vec::new();
        }

        let mut heap = BinaryHeap::with_capacity(k);
        self.search_recursive(&self.root, query, k, &mut heap);

        let mut neighbors = Vec::with_capacity(k);
        while let Some(Reverse(neighbor)) = heap.pop() {
            neighbors.push(neighbor);
        }
        neighbors
    }

    fn search_recursive(
        &self,
        root: &Option<Box<Node<F, P>>>,
        query: &P,
        k: usize,
        neighbors: &mut BinaryHeap<Reverse<Neighbor<F>>>,
    ) {
        let Some(ref node) = root else {
            return;
        };

        if node.is_leaf() {
            let point_index = node.index_at(0);
            let distance = self.distance.measure(query, node.center());
            let neighbor = Neighbor::new(point_index, distance);
            neighbors.push(Reverse(neighbor));
            return;
        }

        let left = node.left();
        let right = node.right();
        let distance_left = left.as_ref().map_or(F::max_value(), |node| {
            self.distance.measure(query, node.center())
        });
        let distance_right = right.as_ref().map_or(F::max_value(), |node| {
            self.distance.measure(query, node.center())
        });

        let (first, second) = if distance_left < distance_right {
            (left, right)
        } else {
            (right, left)
        };

        self.search_recursive(first, query, k, neighbors);

        if let Some(Reverse(neighbor)) = neighbors.peek() {
            let distance = second.as_ref().map_or(F::max_value(), |node| {
                self.distance.measure(query, node.center())
            });
            if distance < neighbor.distance {
                self.search_recursive(second, query, k, neighbors);
            }
        }
    }

    #[must_use]
    fn build_node(
        dataset: &'a [P],
        indices: &mut [usize],
        distance: &'a Distance,
    ) -> Option<Node<F, P>> {
        if indices.is_empty() {
            return None;
        }

        if indices.len() == 1 {
            let index = indices[0];
            let point = dataset[index];
            return Some(Node::new(point, F::zero(), indices.to_vec(), None, None));
        }

        let mut center = indices.iter().fold(P::zero(), |mut centroid, index| {
            centroid += dataset[*index];
            centroid
        });
        center /= F::from_usize(indices.len());
        let radius = indices
            .iter()
            .map(|index| {
                let point = dataset[*index];
                distance.measure(&point, &center)
            })
            .max_by(|point1, point2| point1.partial_cmp(point2).unwrap_or(Ordering::Equal))
            .unwrap_or(F::zero());

        let max_dimension = Self::find_split_dimension(dataset, indices);
        // Sort the indices by the dimension with the maximum variance
        indices.sort_by(|index1, index2| {
            let point1 = dataset[*index1];
            let point2 = dataset[*index2];
            point1[max_dimension]
                .partial_cmp(&point2[max_dimension])
                .unwrap_or(Ordering::Equal)
        });

        // Split the sorted dataset in the middle
        let median = indices.len() / 2;
        let (left_indices, right_indices) = indices.split_at_mut(median);
        let left = Self::build_node(dataset, left_indices, distance);
        let right = Self::build_node(dataset, right_indices, distance);
        let node = Node::new(center, radius, indices.to_vec(), left, right);
        Some(node)
    }

    #[must_use]
    fn find_split_dimension(dataset: &[P], indices: &mut [usize]) -> usize {
        let dimension = dataset[0].dimension();
        let mut dimension_iter = (0..dimension)
            .into_iter()
            .map(|dimension| {
                let (min, max) =
                    indices
                        .iter()
                        .fold((F::max_value(), F::min_value()), |(min, max), index| {
                            let point = &dataset[*index];
                            (min.min(point[dimension]), max.max(point[dimension]))
                        });
                max - min
            })
            .enumerate();

        let (initial_dimension, initial_delta) =
            dimension_iter.next().expect("No dimensions found");
        let (max_dimension, ..) = dimension_iter.into_iter().skip(1).fold(
            (initial_dimension, initial_delta),
            |(max_dimension, max_delta), (dimension, delta)| {
                if delta.partial_cmp(&max_delta).unwrap_or(Ordering::Equal) == Ordering::Greater {
                    (dimension, delta)
                } else {
                    (max_dimension, max_delta)
                }
            },
        );
        max_dimension
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
            Point2(1.0, 2.0), // 0
            Point2(8.0, 3.0), // 1
            Point2(3.0, 4.0), // 2
            Point2(5.0, 4.0), // 3
            Point2(3.0, 5.0), // 4
            Point2(2.0, 6.0), // 5
            Point2(5.0, 6.0), // 6
            Point2(7.0, 7.0), // 7
            Point2(7.0, 8.0), // 8
        ]
    }

    #[test]
    fn test_ball_tree_search() {
        let dataset = empty_dataset();
        let balltree_search = BallTreeSearch::new(&dataset, &Distance::Euclidean);
        assert!(balltree_search.root.is_none());
        assert_eq!(balltree_search.root, None);

        let dataset = sample_dataset();
        let balltree_search = BallTreeSearch::new(&dataset, &Distance::Euclidean);
        assert!(balltree_search.root.is_some());
    }

    #[test]
    fn test_search() {
        let dataset = empty_dataset();
        let balltree_search = BallTreeSearch::new(&dataset, &Distance::Euclidean);
        assert_eq!(balltree_search.search(&Point2(3.0, 3.0), 2), vec![]);

        let dataset = sample_dataset();
        let balltree_search = BallTreeSearch::new(&dataset, &Distance::Euclidean);
        println!("{:?}", balltree_search);
        assert_eq!(balltree_search.search(&Point2(3.0, 3.0), 0), vec![]);

        assert_eq!(balltree_search.search(&Point2(3.0, 3.0), 10), vec![]);
    }
}

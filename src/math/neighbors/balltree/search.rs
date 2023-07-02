use crate::math::distance::Distance;
use crate::math::neighbors::balltree::node::Node;
use crate::math::neighbors::neighbor::Neighbor;
use crate::math::neighbors::search::NeighborSearch;
use crate::math::point::Point;
use crate::number::Float;
use std::borrow::Cow;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

/// Struct representing a ball tree search algorithm.
///
/// # Type Parameters
/// * `F` - The float type used for calculations.
/// * `P` - The type of points used in the neighbor search algorithm.
#[derive(Debug)]
pub struct BallTreeSearch<'a, F, P>
where
    F: Float,
    P: Point<F>,
{
    root: Option<Box<Node<F, P>>>,
    points: Cow<'a, [P]>,
    metric: &'a Distance,
}

impl<'a, F, P> BallTreeSearch<'a, F, P>
where
    F: Float,
    P: Point<F>,
{
    /// Creates a new `BallTreeSearch` instance.
    ///
    /// # Arguments
    /// * `points` - The reference of a dataset of points.
    /// * `metric` - The distance metric to use.
    ///
    /// # Returns
    /// A new `BallTreeSearch` instance.
    #[allow(unused)]
    #[must_use]
    pub fn new(points: &'a [P], metric: &'a Distance) -> Self {
        let mut indices: Vec<usize> = (0..points.len()).collect();
        let root = Self::build_node(points, &mut indices, metric);
        Self {
            root: root.map(Box::new),
            points: Cow::Borrowed(points),
            metric,
        }
    }

    #[inline]
    #[must_use]
    fn build_node(
        points: &'a [P],
        indices: &mut [usize],
        metric: &'a Distance,
    ) -> Option<Node<F, P>> {
        if indices.is_empty() {
            return None;
        }

        let mut center = indices.iter().fold(P::zero(), |mut centroid, index| {
            centroid += &points[*index];
            centroid
        });
        center /= F::from_usize(indices.len());
        let radius = indices
            .iter()
            .map(|index| {
                let point = points[*index];
                metric.measure(&point, &center)
            })
            .max_by(|point1, point2| point1.partial_cmp(point2).unwrap_or(Ordering::Equal))
            .unwrap_or(F::zero());

        if indices.len() <= 8 {
            return Some(Node::new(center, radius, indices.to_vec(), None, None));
        }

        let max_dimension = Self::find_split_dimension(points, indices);
        // Sort the indices by the dimension with the maximum variance
        indices.sort_by(|index1, index2| {
            let point1 = points[*index1];
            let point2 = points[*index2];
            point1[max_dimension]
                .partial_cmp(&point2[max_dimension])
                .unwrap_or(Ordering::Equal)
        });

        // Split the sorted points in the middle
        let median = indices.len() / 2;
        let left = Self::build_node(points, &mut indices[..median], metric);
        let right = Self::build_node(points, &mut indices[median..], metric);
        let node = Node::new(center, radius, indices.to_vec(), left, right);
        Some(node)
    }

    #[must_use]
    fn find_split_dimension(points: &[P], indices: &mut [usize]) -> usize {
        let dimension = points[0].dimension();
        let mut dimension_iter = (0..dimension)
            .map(|dimension| {
                let (min, max) =
                    indices
                        .iter()
                        .fold((F::max_value(), F::min_value()), |(min, max), index| {
                            let point = &points[*index];
                            (min.min(point[dimension]), max.max(point[dimension]))
                        });
                max - min
            })
            .enumerate();

        let (initial_dimension, initial_delta) =
            dimension_iter.next().expect("No dimensions found");
        let (max_dimension, ..) = dimension_iter.skip(1).fold(
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

    #[inline]
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
            let neighbor_iter = node.indices().iter().map(|index| {
                let point = &self.points[*index];
                let distance = self.metric.measure(query, point);
                let neighbor = Neighbor::new(*index, distance);
                Reverse(neighbor)
            });
            neighbors.extend(neighbor_iter);
            return;
        }

        let left = node.left();
        let left_distance = left.as_ref().map_or(F::max_value(), |node| {
            self.metric.measure(query, node.center())
        });

        let right = node.right();
        let right_distance = right.as_ref().map_or(F::max_value(), |node| {
            self.metric.measure(query, node.center())
        });

        let (first, second, second_distance) = if left_distance < right_distance {
            (left, right, right_distance)
        } else {
            (right, left, left_distance)
        };

        self.search_recursive(first, query, k, neighbors);

        if neighbors.len() < k {
            self.search_recursive(second, query, k, neighbors);
        } else if let Some(Reverse(neighbor)) = neighbors.peek() {
            if second_distance < neighbor.distance {
                self.search_recursive(second, query, k, neighbors);
            }
        }
    }

    #[inline]
    fn search_radius_recursive(
        &self,
        root: &Option<Box<Node<F, P>>>,
        query: &P,
        radius: F,
        neighbors: &mut BinaryHeap<Reverse<Neighbor<F>>>,
    ) {
        let Some(ref node) = root else {
            return;
        };

        if node.is_leaf() {
            let neighbor_iter = node.indices().iter().filter_map(|index| {
                let point = &self.points[*index];
                let distance = self.metric.measure(query, point);
                if distance <= radius {
                    let neighbor = Neighbor::new(*index, distance);
                    Some(Reverse(neighbor))
                } else {
                    None
                }
            });
            neighbors.extend(neighbor_iter);
            return;
        }

        let left = node.left();
        let left_distance = left.as_ref().map_or(F::max_value(), |node| {
            self.metric.measure(query, node.center())
        });

        let right = node.right();
        let right_distance = right.as_ref().map_or(F::max_value(), |node| {
            self.metric.measure(query, node.center())
        });

        if left_distance - node.radius() <= radius {
            self.search_radius_recursive(left, query, radius, neighbors);
        }
        if right_distance - node.radius() <= radius {
            self.search_radius_recursive(right, query, radius, neighbors);
        }
    }
}

impl<'a, F, P> NeighborSearch<F, P> for BallTreeSearch<'a, F, P>
where
    F: Float,
    P: Point<F>,
{
    #[must_use]
    fn search(&self, query: &P, k: usize) -> Vec<Neighbor<F>> {
        if k == 0 {
            return Vec::new();
        }

        let mut heap = BinaryHeap::with_capacity(k);
        self.search_recursive(&self.root, query, k, &mut heap);

        let mut neighbors = Vec::with_capacity(k);
        while let Some(Reverse(neighbor)) = heap.pop() {
            neighbors.push(neighbor);
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
        self.search_radius_recursive(&self.root, query, radius, &mut heap);

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
    use crate::math::point::Point2;
    use statrs::assert_almost_eq;

    #[must_use]
    fn sample_points() -> Vec<Point2<f64>> {
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
        let points: Vec<Point2<f64>> = vec![];
        let balltree = BallTreeSearch::new(&points, &Distance::Euclidean);
        assert!(balltree.root.is_none());
        assert_eq!(balltree.root, None);

        let points = sample_points();
        let balltree_search = BallTreeSearch::new(&points, &Distance::Euclidean);
        assert!(balltree_search.root.is_some());
    }

    #[test]
    fn test_search() {
        let points = sample_points();
        let balltree = BallTreeSearch::new(&points, &Distance::Euclidean);
        let actual = balltree.search(&Point2(3.0, 3.0), 0);
        assert_eq!(actual.len(), 0);

        let actual = balltree.search(&Point2(3.0, 3.0), 1);
        assert_eq!(actual.len(), 1);
        assert_eq!(actual[0].index, 2);
        assert_almost_eq!(actual[0].distance, 1.0, 1e-6);

        let actual = balltree.search(&Point2(3.0, 3.0), 2);
        assert_eq!(actual.len(), 2);
        assert_eq!(actual[1].index, 4);
        assert_almost_eq!(actual[1].distance, 2.0, 1e-6);

        let actual = balltree.search(&Point2(3.0, 3.0), 5);
        assert_eq!(actual.len(), 5);
        assert_eq!(actual[2].index, 0);
        assert_almost_eq!(actual[2].distance, 2.236067, 1e-6);
        assert_eq!(actual[3].index, 3);
        assert_almost_eq!(actual[3].distance, 2.236067, 1e-6);
        assert_eq!(actual[4].index, 5);
        assert_almost_eq!(actual[4].distance, 3.162277, 1e-6);
    }

    #[test]
    fn test_search_empty() {
        let points: Vec<Point2<f64>> = vec![];
        let balltree = BallTreeSearch::new(&points, &Distance::Euclidean);
        assert_eq!(balltree.search(&Point2(3.0, 3.0), 0), vec![]);
        assert_eq!(balltree.search(&Point2(3.0, 3.0), 1), vec![]);
        assert_eq!(balltree.search(&Point2(3.0, 3.0), 2), vec![]);
    }

    #[test]
    fn test_search_nearest() {
        let points = sample_points();
        let balltree = BallTreeSearch::new(&points, &Distance::Euclidean);

        let actual = balltree.search_nearest(&Point2(3.0, 3.0));
        assert!(actual.is_some());
        let neighbor = actual.unwrap();
        assert_eq!(neighbor.index, 2);
        assert_almost_eq!(neighbor.distance, 1.0, 1e-6);

        let actual = balltree.search_nearest(&Point2(2.0, 6.0));
        assert!(actual.is_some());
        let neighbor = actual.unwrap();
        assert_eq!(neighbor.index, 5);
        assert_almost_eq!(neighbor.distance, 0.0, 1e-6);
    }

    #[test]
    fn test_search_nearest_empty() {
        let points: Vec<Point2<f64>> = vec![];
        let balltree = BallTreeSearch::new(&points, &Distance::Euclidean);
        assert!(balltree.search_nearest(&Point2(3.0, 3.0)).is_none());
    }

    #[test]
    fn test_search_radius() {
        let points = sample_points();
        let balltree = BallTreeSearch::new(&points, &Distance::Euclidean);
        let actual = balltree.search_radius(&Point2(3.0, 3.0), 0.0);
        assert_eq!(actual.len(), 0);

        let actual = balltree.search_radius(&Point2(3.0, 3.0), 1.0);
        assert_eq!(actual.len(), 1);
        assert_eq!(actual[0].index, 2);
        assert_almost_eq!(actual[0].distance, 1.0, 1e-6);

        let actual = balltree.search_radius(&Point2(3.0, 3.0), 2.0);
        assert_eq!(actual.len(), 2);
        assert_eq!(actual[1].index, 4);
        assert_almost_eq!(actual[1].distance, 2.0, 1e-6);

        let actual = balltree.search_radius(&Point2(3.0, 3.0), 3.0);
        assert_eq!(actual.len(), 4);
        assert_eq!(actual[2].index, 0);
        assert_almost_eq!(actual[2].distance, 2.236067, 1e-6);
        assert_eq!(actual[3].index, 3);
        assert_almost_eq!(actual[3].distance, 2.236067, 1e-6);

        let actual = balltree.search_radius(&Point2(3.0, 3.0), 8.0);
        assert_eq!(actual.len(), 9);
    }

    #[test]
    fn test_search_radius_empty() {
        let points: Vec<Point2<f64>> = vec![];
        let balltree = BallTreeSearch::new(&points, &Distance::Euclidean);
        assert_eq!(balltree.search_radius(&Point2(3.0, 3.0), -1.0), vec![]);
        assert_eq!(balltree.search_radius(&Point2(3.0, 3.0), 0.0), vec![]);
        assert_eq!(balltree.search_radius(&Point2(3.0, 3.0), 1.0), vec![]);
    }
}

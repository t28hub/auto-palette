use std::{cmp::Ordering, collections::BinaryHeap};

use crate::math::{
    neighbors::{neighbor::Neighbor, search::NeighborSearch},
    DistanceMetric,
    FloatNumber,
    Point,
};

/// k-d tree node type.
#[derive(Debug)]
enum Node {
    /// An internal node of the k-d tree.
    Internal {
        /// Axis used for splitting the points.
        axis: usize,

        /// Index of the pivot point in `points`.
        index: usize,

        /// Left child node.
        left: Option<Box<Node>>,

        /// Right child node.
        right: Option<Box<Node>>,
    },
    /// A leaf node containing point indices.
    Leaf {
        /// Indices of the points in this leaf node.
        indices: Vec<usize>,
    },
}

/// k-d tree structure enabling efficient neighbor searches.
///
/// # Type Parameters
/// * `T` - The floating point type.
/// * `N` - The dimension of the points.
#[derive(Debug)]
pub struct KdTreeSearch<'a, T, const N: usize>
where
    T: FloatNumber,
{
    /// Points stored in the k-d tree.
    points: &'a [Point<T, N>],

    /// Distance metric used for measuring distances between points.
    metric: DistanceMetric,

    /// Root node of the k-d tree structure.
    root: Option<Box<Node>>,
}

impl<'a, T, const N: usize> KdTreeSearch<'a, T, N>
where
    T: 'a + FloatNumber,
{
    /// Builds a new `KDTreeSearch` instance.
    ///
    /// # Arguments
    /// * `points` - The points to search.
    /// * `metric` - The distance metric to use.
    /// * `max_leaf_size` - The maximum number of points in a leaf node.
    ///
    /// # Returns
    /// A new `KDTreeSearch` instance.
    pub fn build(points: &'a [Point<T, N>], metric: DistanceMetric, max_leaf_size: usize) -> Self {
        let mut indices: Vec<_> = (0..points.len()).collect();
        let root = Self::split_node(points, max_leaf_size, &mut indices, 0).map(Box::new);
        Self {
            points,
            metric,
            root,
        }
    }

    #[inline]
    #[must_use]
    fn split_node(
        points: &[Point<T, N>],
        max_leaf_size: usize,
        indices: &mut [usize],
        depth: usize,
    ) -> Option<Node> {
        if indices.is_empty() {
            return None;
        }

        // Return a leaf node when the number of indices <= `max_leaf_size`.
        if indices.len() <= max_leaf_size {
            return Some(Node::Leaf {
                indices: indices.to_vec(),
            });
        }

        // Partition the indices by the median value along the splitting axis.
        let axis = depth % N;
        let median_index = indices.len() / 2;
        let (left_indices, median, right_indices) =
            indices.select_nth_unstable_by(median_index, |&index1, &index2| {
                // Compare the points by the splitting axis.
                points[index1][axis]
                    .partial_cmp(&points[index2][axis])
                    .unwrap_or(Ordering::Less)
            });

        let left = Self::split_node(points, max_leaf_size, left_indices, depth + 1);
        let right = Self::split_node(points, max_leaf_size, right_indices, depth + 1);
        Some(Node::Internal {
            axis,
            index: *median,
            left: left.map(Box::new),
            right: right.map(Box::new),
        })
    }

    #[inline]
    fn visit_indices_with_distance<F>(
        &self,
        indices: &[usize],
        query: &Point<T, N>,
        mut visit_fn: F,
    ) where
        F: FnMut(usize, T),
    {
        for &index in indices {
            let point = &self.points[index];
            let distance = self.metric.measure(point, query);
            visit_fn(index, distance);
        }
    }

    #[allow(dead_code)]
    #[inline]
    fn search_recursive(
        &self,
        root: &Option<Box<Node>>,
        query: &Point<T, N>,
        k: usize,
        neighbors: &mut BinaryHeap<Neighbor<T>>,
    ) {
        let Some(node) = root.as_ref() else {
            return;
        };

        let mut best_distance = neighbors
            .peek()
            .map(|neighbor| neighbor.distance)
            .unwrap_or(T::infinity());
        let mut update_neighbors = |index, distance| {
            if distance >= best_distance {
                return;
            }

            if neighbors.len() == k {
                neighbors.pop();
            }
            neighbors.push(Neighbor::new(index, distance));
            best_distance = neighbors
                .peek()
                .map(|neighbor| neighbor.distance)
                .unwrap_or_else(T::infinity);
        };

        match node.as_ref() {
            Node::Internal {
                axis,
                index,
                left,
                right,
            } => {
                let point = &self.points[*index];
                let distance = self.metric.measure(point, query);
                update_neighbors(*index, distance);

                let delta = (query[*axis] - point[*axis]).abs();
                let (near, far) = if query[*axis] < point[*axis] {
                    (left, right)
                } else {
                    (right, left)
                };

                self.search_recursive(near, query, k, neighbors);
                if delta < best_distance {
                    self.search_recursive(far, query, k, neighbors);
                }
            }
            Node::Leaf { indices } => {
                self.visit_indices_with_distance(indices, query, &mut update_neighbors);
            }
        }
    }

    #[inline]
    fn search_nearest_recursive(
        &self,
        root: &Option<Box<Node>>,
        query: &Point<T, N>,
        nearest: &mut Neighbor<T>,
    ) {
        let Some(node) = root.as_ref() else {
            return;
        };

        let mut update_nearest = |index, distance| {
            if distance < nearest.distance {
                *nearest = Neighbor::new(index, distance);
            }
        };

        match node.as_ref() {
            Node::Internal {
                axis,
                index,
                left,
                right,
            } => {
                let point = &self.points[*index];
                let distance = self.metric.measure(point, query);
                update_nearest(*index, distance);

                let delta = (query[*axis] - point[*axis]).abs();
                let (near, far) = if query[*axis] < point[*axis] {
                    (left, right)
                } else {
                    (right, left)
                };

                self.search_nearest_recursive(near, query, nearest);
                if delta < nearest.distance {
                    self.search_nearest_recursive(far, query, nearest);
                }
            }
            Node::Leaf { indices } => {
                self.visit_indices_with_distance(indices, query, &mut update_nearest);
            }
        }
    }

    #[inline]
    fn search_radius_recursive(
        &self,
        root: &Option<Box<Node>>,
        query: &Point<T, N>,
        radius: T,
        neighbors: &mut Vec<Neighbor<T>>,
    ) {
        let Some(node) = root.as_ref() else {
            return;
        };

        match node.as_ref() {
            Node::Internal {
                axis,
                index,
                left,
                right,
            } => {
                let point = &self.points[*index];
                let distance = self.metric.measure(point, query);
                if distance <= radius {
                    neighbors.push(Neighbor::new(*index, distance));
                }

                let (near, far) = if query[*axis] < point[*axis] {
                    (left, right)
                } else {
                    (right, left)
                };

                self.search_radius_recursive(near, query, radius, neighbors);
                if (query[*axis] - point[*axis]).abs() <= radius {
                    self.search_radius_recursive(far, query, radius, neighbors);
                }
            }
            Node::Leaf { indices } => {
                self.visit_indices_with_distance(indices, query, &mut |index, distance| {
                    if distance <= radius {
                        neighbors.push(Neighbor::new(index, distance));
                    }
                });
            }
        }
    }
}

impl<T, const N: usize> NeighborSearch<T, N> for KdTreeSearch<'_, T, N>
where
    T: FloatNumber,
{
    fn search(&self, query: &Point<T, N>, k: usize) -> Vec<Neighbor<T>> {
        if k == 0 {
            return Vec::new();
        }

        let mut neighbors = BinaryHeap::with_capacity(k);
        self.search_recursive(&self.root, query, k, &mut neighbors);
        neighbors.into_sorted_vec()
    }

    fn search_nearest(&self, query: &Point<T, N>) -> Option<Neighbor<T>> {
        let mut nearest = Neighbor::new(0, T::infinity());
        self.search_nearest_recursive(&self.root, query, &mut nearest);
        if nearest.distance.is_infinite() {
            None
        } else {
            Some(nearest)
        }
    }

    fn search_radius(&self, query: &Point<T, N>, radius: T) -> Vec<Neighbor<T>> {
        if radius < T::zero() {
            return Vec::new();
        }

        let mut neighbors = Vec::new();
        self.search_radius_recursive(&self.root, query, radius, &mut neighbors);
        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[must_use]
    fn sample_points() -> Vec<Point<f32, 3>> {
        vec![
            [1.0, 2.0, 3.0], // 0
            [5.0, 1.0, 2.0], // 1
            [9.0, 3.0, 4.0], // 2
            [3.0, 9.0, 1.0], // 3
            [4.0, 8.0, 3.0], // 4
            [9.0, 1.0, 1.0], // 5
            [5.0, 0.0, 0.0], // 6
            [3.0, 2.0, 1.0], // 7
            [2.0, 5.0, 6.0], // 8
            [1.0, 3.0, 2.0], // 9
            [4.0, 2.0, 1.0], // 10
            [5.0, 3.0, 2.0], // 11
            [6.0, 2.0, 1.0], // 12
            [7.0, 3.0, 2.0], // 13
            [8.0, 2.0, 1.0], // 14
        ]
    }

    #[must_use]
    fn empty_points() -> Vec<Point<f32, 3>> {
        Vec::new()
    }

    #[test]
    fn test_build() {
        // Act
        let points = sample_points();
        let search = KdTreeSearch::build(&points, DistanceMetric::Euclidean, 2);

        // Assert
        assert_eq!(search.points, &points);
        assert_eq!(search.metric, DistanceMetric::Euclidean);
        assert!(search.root.is_some());
    }

    #[test]
    fn test_build_empty() {
        // Act
        let points = empty_points();
        let search: KdTreeSearch<f32, 3> =
            KdTreeSearch::build(&points, DistanceMetric::Euclidean, 2);

        // Assert
        assert_eq!(search.points, &points);
        assert_eq!(search.metric, DistanceMetric::Euclidean);
        assert!(search.root.is_none());
    }

    #[test]
    fn test_search() {
        // Arrange
        let points = sample_points();
        let search = KdTreeSearch::build(&points, DistanceMetric::Euclidean, 2);

        // Act
        let query = [3.0, 5.0, 6.0];
        let neighbors = search.search(&query, 3);

        // Assert
        assert_eq!(neighbors.len(), 3);
        assert_eq!(neighbors[0].index, 8);
        assert_eq!(neighbors[0].distance, 1.0_f32.sqrt());
        assert_eq!(neighbors[1].index, 4);
        assert_eq!(neighbors[1].distance, 19.0_f32.sqrt());
        assert_eq!(neighbors[2].index, 0);
        assert_eq!(neighbors[2].distance, 22.0_f32.sqrt());
    }

    #[test]
    fn test_search_empty() {
        // Arrange
        let points = sample_points();
        let search = KdTreeSearch::build(&points, DistanceMetric::Euclidean, 2);

        // Act
        let query = [3.0, 5.0, 6.0];
        let neighbors = search.search(&query, 0);

        // Assert
        assert!(neighbors.is_empty());
    }

    #[test]
    fn test_search_nearest() {
        // Arrange
        let points = sample_points();
        let search = KdTreeSearch::build(&points, DistanceMetric::Euclidean, 2);

        // Act
        let query = [2.0, 2.0, 1.0];
        let nearest = search.search_nearest(&query).unwrap();

        // Assert
        assert_eq!(nearest.index, 7);
        assert_eq!(nearest.distance, 1.0_f32.sqrt());
    }

    #[test]
    fn test_search_nearest_empty() {
        // Arrange
        let points = empty_points();
        let search = KdTreeSearch::build(&points, DistanceMetric::Euclidean, 2);

        // Act
        let query = [3.0, 2.0, 1.0];
        let nearest = search.search_nearest(&query);

        // Assert
        assert!(nearest.is_none());
    }

    #[test]
    fn test_search_radius() {
        // Arrange
        let points = sample_points();
        let search = KdTreeSearch::build(&points, DistanceMetric::Euclidean, 2);

        // Act
        let query = [3.0, 5.0, 6.0];
        let neighbors = search.search_radius(&query, 4.5);

        // Assert
        assert_eq!(neighbors.len(), 2);
        assert_eq!(neighbors[0].index, 4);
        assert_eq!(neighbors[0].distance, 19.0_f32.sqrt());
        assert_eq!(neighbors[1].index, 8);
        assert_eq!(neighbors[1].distance, 1.0_f32.sqrt());
    }

    #[test]
    fn test_search_radius_empty() {
        // Arrange
        let points = sample_points();
        let search = KdTreeSearch::build(&points, DistanceMetric::Euclidean, 2);

        // Act
        let query = [3.0, 5.0, 6.0];
        let neighbors = search.search_radius(&query, -1.0);

        // Assert
        assert_eq!(neighbors.len(), 0);
    }
}

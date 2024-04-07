use crate::math::neighbors::neighbor::Neighbor;
use crate::math::neighbors::search::NeighborSearch;
use crate::math::{DistanceMetric, Point};
use std::collections::BinaryHeap;

/// Node of a k-d tree.
#[derive(Debug)]
struct Node {
    /// The splitting axis.
    axis: usize,
    /// The indices of the points.
    indices: Vec<usize>,
    /// The left child node.
    left: Option<Box<Node>>,
    /// The right child node.
    right: Option<Box<Node>>,
}

impl Node {
    /// Creates a new internal node.
    ///
    /// # Arguments
    /// * `axis` - The splitting axis.
    /// * `index` - The index of the point.
    /// * `left` - The left child node.
    /// * `right` - The right child node.
    ///
    /// # Returns
    /// A new internal node.
    #[must_use]
    fn new_node(axis: usize, index: usize, left: Option<Node>, right: Option<Node>) -> Self {
        Self {
            axis,
            indices: vec![index],
            left: left.map(Box::new),
            right: right.map(Box::new),
        }
    }

    /// Creates a new leaf node.
    ///
    /// # Arguments
    /// * `axis` - The splitting axis.
    /// * `indices` - The indices of the points.
    ///
    /// # Returns
    /// A new leaf node.
    #[must_use]
    fn new_leaf(axis: usize, indices: &[usize]) -> Self {
        Self {
            axis,
            indices: indices.to_vec(),
            left: None,
            right: None,
        }
    }

    /// Returns the index of the point.
    ///
    /// # Returns
    /// The index of the point.
    #[inline]
    #[must_use]
    fn index(&self) -> usize {
        self.indices[0]
    }

    /// Checks if the node is a leaf.
    ///
    /// # Returns
    /// `true` if the node is a leaf, otherwise `false`.
    #[inline]
    #[must_use]
    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

/// k-d tree search algorithm.
///
/// # Type Parameters
/// * `N` - The dimension of the points.
#[derive(Debug)]
pub struct KDTreeSearch<'a, const N: usize> {
    /// The root node of the tree.
    root: Option<Box<Node>>,
    /// The points in the tree.
    points: &'a [Point<N>],
    /// The distance metric.
    metric: DistanceMetric,
}

impl<'a, const N: usize> KDTreeSearch<'a, N> {
    /// Builds a new `KDTreeSearch` instance.
    ///
    /// # Arguments
    /// * `points` - The points to search.
    /// * `metric` - The distance metric to use.
    /// * `leaf_size` - The maximum number of points in a leaf node.
    ///
    /// # Returns
    /// A new `KDTreeSearch` instance.
    pub fn build(points: &'a [Point<N>], metric: DistanceMetric, leaf_size: usize) -> Self {
        let mut indices: Vec<usize> = (0..points.len()).collect();
        let root = Self::build_node(points, leaf_size, &mut indices, 0);
        Self {
            root: root.map(Box::new),
            points,
            metric,
        }
    }

    #[inline]
    #[must_use]
    fn build_node(
        points: &[Point<N>],
        leaf_size: usize,
        indices: &mut [usize],
        depth: usize,
    ) -> Option<Node> {
        if indices.is_empty() {
            return None;
        }

        let axis = depth % N;
        if indices.len() <= leaf_size {
            return Some(Node::new_leaf(axis, indices));
        }

        indices.sort_by(|&index1, &index2| {
            // Compare the points by the splitting axis.
            points[index1][axis]
                .partial_cmp(&points[index2][axis])
                .unwrap()
        });

        let median = indices.len() / 2;
        let left = Self::build_node(
            points,
            leaf_size,
            &mut indices[..median].to_vec(),
            depth + 1,
        );
        let right = Self::build_node(
            points,
            leaf_size,
            &mut indices[median + 1..].to_vec(),
            depth + 1,
        );
        Some(Node::new_node(axis, indices[median], left, right))
    }

    #[inline]
    fn search_recursive(
        &self,
        root: &Option<Box<Node>>,
        query: &Point<N>,
        k: usize,
        neighbors: &mut BinaryHeap<Neighbor>,
    ) {
        let Some(ref node) = root else {
            return;
        };

        if node.is_leaf() {
            for &index in &node.indices {
                let point = &self.points[index];
                let distance = self.metric.measure(point, query);
                if neighbors.len() < k {
                    neighbors.push(Neighbor::new(index, distance));
                } else if distance
                    < neighbors
                        .peek()
                        .map(|neighbor| neighbor.distance)
                        .unwrap_or(f32::INFINITY)
                {
                    neighbors.pop();
                    neighbors.push(Neighbor::new(index, distance));
                }
            }
            return;
        }

        let index = node.index();
        let point = &self.points[index];
        let distance = self.metric.measure(point, query);
        if neighbors.len() < k {
            neighbors.push(Neighbor::new(index, distance));
        } else if distance
            < neighbors
                .peek()
                .map(|neighbor| neighbor.distance)
                .unwrap_or(f32::INFINITY)
        {
            neighbors.pop();
            neighbors.push(Neighbor::new(index, distance));
        }

        let axis = node.axis;
        let (near, far) = if query[axis] < point[axis] {
            (&node.left, &node.right)
        } else {
            (&node.right, &node.left)
        };
        self.search_recursive(near, query, k, neighbors);
        if let Some(neighbor) = neighbors.peek() {
            if (query[axis] - point[axis]).abs() < neighbor.distance {
                self.search_recursive(far, query, k, neighbors);
            }
        }
    }

    #[inline]
    fn search_nearest_recursive(
        &self,
        root: &Option<Box<Node>>,
        query: &Point<N>,
        nearest: &mut Neighbor,
    ) {
        let Some(ref node) = root else {
            return;
        };

        if node.is_leaf() {
            for &index in &node.indices {
                let point = &self.points[index];
                let distance = self.metric.measure(point, query);
                if distance < nearest.distance {
                    nearest.index = index;
                    nearest.distance = distance;
                }
            }
            return;
        }

        let index = node.index();
        let point = &self.points[index];
        let distance = self.metric.measure(point, query);
        if distance < nearest.distance {
            nearest.index = index;
            nearest.distance = distance;
        }

        let axis = node.axis;
        let (near, far) = if query[axis] < point[axis] {
            (&node.left, &node.right)
        } else {
            (&node.right, &node.left)
        };

        self.search_nearest_recursive(near, query, nearest);
        if (query[axis] - point[axis]).abs() < nearest.distance {
            self.search_nearest_recursive(far, query, nearest);
        }
    }

    #[inline]
    fn search_radius_recursive(
        &self,
        root: &Option<Box<Node>>,
        query: &Point<N>,
        radius: f32,
        neighbors: &mut Vec<Neighbor>,
    ) {
        let Some(ref node) = root else {
            return;
        };

        if node.is_leaf() {
            for &index in &node.indices {
                let point = &self.points[index];
                let distance = self.metric.measure(point, query);
                if distance <= radius {
                    neighbors.push(Neighbor::new(index, distance));
                }
            }
            return;
        }

        let index = node.index();
        let point = &self.points[index];
        let distance = self.metric.measure(point, query);
        if distance <= radius {
            neighbors.push(Neighbor::new(index, distance));
        }

        let axis = node.axis;
        let (near, far) = if query[axis] < point[axis] {
            (&node.left, &node.right)
        } else {
            (&node.right, &node.left)
        };

        self.search_radius_recursive(near, query, radius, neighbors);
        if (query[axis] - point[axis]).abs() <= radius {
            self.search_radius_recursive(far, query, radius, neighbors);
        }
    }
}

impl<'a, const N: usize> NeighborSearch<N> for KDTreeSearch<'a, N> {
    #[must_use]
    fn search(&self, query: &Point<N>, k: usize) -> Vec<Neighbor> {
        if k == 0 {
            return Vec::new();
        }

        let mut neighbors = BinaryHeap::with_capacity(k);
        self.search_recursive(&self.root, query, k, &mut neighbors);
        neighbors.into_sorted_vec()
    }

    #[must_use]
    fn search_nearest(&self, query: &Point<N>) -> Option<Neighbor> {
        let mut nearest = Neighbor::new(0, f32::INFINITY);
        self.search_nearest_recursive(&self.root, query, &mut nearest);
        if nearest.distance == f32::INFINITY {
            None
        } else {
            Some(nearest)
        }
    }

    #[must_use]
    fn search_radius(&self, query: &Point<N>, radius: f32) -> Vec<Neighbor> {
        if radius < 0.0 {
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
    fn sample_points() -> Vec<Point<3>> {
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
    fn empty_points() -> Vec<Point<3>> {
        Vec::new()
    }

    #[test]
    fn test_build() {
        // Act
        let points = sample_points();
        let search = KDTreeSearch::build(&points, DistanceMetric::Euclidean, 2);

        // Assert
        assert!(search.root.is_some());
        assert_eq!(search.points.len(), points.len());
        assert_eq!(search.metric, DistanceMetric::Euclidean);
    }

    #[test]
    fn test_build_empty() {
        // Act
        let points = empty_points();
        let search: KDTreeSearch<3> = KDTreeSearch::build(&points, DistanceMetric::Euclidean, 2);

        // Assert
        assert!(search.root.is_none());
        assert_eq!(search.points.len(), 0);
        assert_eq!(search.metric, DistanceMetric::Euclidean);
    }

    #[test]
    fn test_search() {
        // Arrange
        let points = sample_points();
        let search = KDTreeSearch::build(&points, DistanceMetric::Euclidean, 2);

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
        let search = KDTreeSearch::build(&points, DistanceMetric::Euclidean, 2);

        // Act
        let query = [3.0, 5.0, 6.0];
        let neighbors = search.search(&query, 0);

        // Assert
        assert_eq!(neighbors.len(), 0);
    }

    #[test]
    fn test_search_nearest() {
        // Arrange
        let points = sample_points();
        let search = KDTreeSearch::build(&points, DistanceMetric::Euclidean, 2);

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
        let search = KDTreeSearch::build(&points, DistanceMetric::Euclidean, 2);

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
        let search = KDTreeSearch::build(&points, DistanceMetric::Euclidean, 2);

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
        let search = KDTreeSearch::build(&points, DistanceMetric::Euclidean, 2);

        // Act
        let query = [3.0, 5.0, 6.0];
        let neighbors = search.search_radius(&query, -1.0);

        // Assert
        assert_eq!(neighbors.len(), 0);
    }
}

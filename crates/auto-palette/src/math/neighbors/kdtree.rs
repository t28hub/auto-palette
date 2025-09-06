use std::{cmp::Ordering, collections::BinaryHeap};

use crate::math::{
    neighbors::{Neighbor, NeighborSearch},
    DistanceMetric,
    FloatNumber,
    Point,
};

/// Node in a KD-tree.
///
/// Each node is either an inner node that splits the space along one dimension,
/// or a leaf node that contains a collection of point indices.
#[derive(Debug)]
enum Node {
    /// Inner node that splits the space along one dimension.
    Inner {
        /// The axis (dimension) along which this node splits the space.
        split_axis: usize,

        /// Index of the pivot point in the original points array.
        pivot_index: usize,

        /// Index of the left child node (points with values less than or equal to the pivot).
        left_child: Option<usize>,

        /// Index of the right child node (points with values greater than the pivot).
        right_child: Option<usize>,
    },
    /// Leaf node containing a collection of point indices.
    Leaf {
        /// Indices of points in the original points array.
        point_indices: Vec<usize>,
    },
}

/// KD-tree data structure for efficient nearest neighbor search.
///
/// This implementation uses a balanced KD-tree with configurable leaf size
/// for optimal performance across different dataset sizes.
///
/// # Type Parameters
/// * `T` - The floating point type.
/// * `N` - The dimension of the points.
#[derive(Debug)]
pub struct KdTreeSearch<'a, T, const N: usize>
where
    T: FloatNumber,
{
    /// Reference to the points being indexed.
    points: &'a [Point<T, N>],

    /// Distance metric used for searches.
    metric: DistanceMetric,

    /// All nodes in the tree, stored in a flat array.
    nodes: Vec<Node>,

    /// Index of the root node, or None if the tree is empty.
    root: Option<usize>,
}

impl<'a, T, const N: usize> KdTreeSearch<'a, T, N>
where
    T: FloatNumber,
{
    /// Default maximum number of points in a leaf node.
    const DEFAULT_LEAF_SIZE: usize = 8;

    /// Builds a KD-tree with default leaf size.
    ///
    /// # Arguments
    /// * `points` - The points to index.
    /// * `metric` - The distance metric to use for searches.
    ///
    /// # Returns
    /// A new KD-tree instance.
    #[allow(unused)]
    #[must_use]
    pub fn build(points: &'a [Point<T, N>], metric: DistanceMetric) -> Self {
        Self::with_leaf_size(points, metric, Self::DEFAULT_LEAF_SIZE)
    }

    /// Builds a KD-tree with custom leaf size.
    ///
    /// # Arguments
    /// * `points` - The points to index.
    /// * `metric` - The distance metric to use for searches.
    /// * `leaf_size` - Maximum number of points in a leaf node.
    ///   - Smaller values (1-4): Deeper tree, faster searches, slower construction.
    ///   - Larger values (16-32): Shallower tree, slower searches, faster construction.
    ///   - Zero: Each point gets its own leaf node (not recommended).
    ///
    /// # Returns
    /// A new KD-tree instance.
    pub fn with_leaf_size(
        points: &'a [Point<T, N>],
        metric: DistanceMetric,
        leaf_size: usize,
    ) -> Self {
        if points.is_empty() {
            return Self {
                points,
                metric,
                nodes: Vec::new(),
                root: None,
            };
        }

        let mut tree = Self {
            points,
            metric,
            nodes: Vec::with_capacity(points.len()),
            root: None,
        };
        let mut indices: Vec<_> = (0..points.len()).collect();
        let root = tree.build_recursive(&mut indices, leaf_size, 0);
        tree.root = root;
        tree
    }

    /// Recursively builds the KD-tree.
    ///
    /// # Arguments
    /// * `indices` - Mutable slice of point indices to partition.
    /// * `leaf_size` - Maximum number of points in a leaf node.
    /// * `depth` - Current depth in the tree (used to determine split axis).
    ///
    /// # Returns
    /// Index of the created node, or None if indices is empty.
    #[inline]
    fn build_recursive(
        &mut self,
        indices: &mut [usize],
        leaf_size: usize,
        depth: usize,
    ) -> Option<usize> {
        match indices.len() {
            0 => None,
            n if n <= leaf_size => {
                let node_id = self.push_node(Node::Leaf {
                    point_indices: indices.to_vec(),
                });
                Some(node_id)
            }
            _ => {
                let split_axis = depth % N;
                let median_index = indices.len() / 2;
                let (left_indices, median, right_indices) =
                    indices.select_nth_unstable_by(median_index, |&a, &b| {
                        self.points[a][split_axis]
                            .partial_cmp(&self.points[b][split_axis])
                            .unwrap_or(Ordering::Less)
                    });

                let pivot_index = *median;
                let node_id = self.push_node(Node::Inner {
                    split_axis,
                    pivot_index,
                    left_child: None,
                    right_child: None,
                });

                let left_child_id = self.build_recursive(left_indices, leaf_size, depth + 1);
                let right_child_id = self.build_recursive(right_indices, leaf_size, depth + 1);
                if let Node::Inner {
                    left_child: l,
                    right_child: r,
                    ..
                } = &mut self.nodes[node_id]
                {
                    *l = left_child_id;
                    *r = right_child_id;
                }
                Some(node_id)
            }
        }
    }

    /// Adds a node to the tree and returns its index.
    ///
    /// # Arguments
    /// * `node` - The new node to add.
    ///
    /// # Returns
    /// Index of the newly added node.
    fn push_node(&mut self, node: Node) -> usize {
        let new_id = self.nodes.len();
        self.nodes.push(node);
        new_id
    }

    /// Traverses the KD-tree using a given search strategy.
    ///
    /// This method performs a depth-first traversal of the tree, visiting nodes
    /// and points based on the strategy's logic. The strategy controls which
    /// points to collect and when to prune branches.
    ///
    /// # Arguments
    /// * `node_index` - Index of the current node to visit.
    /// * `query` - The query point for distance calculations.
    /// * `strategy` - The search strategy that defines the search behavior.
    ///
    /// # Type Parameters
    /// * `S` - The search strategy type implementing [`SearchStrategy`].
    #[inline]
    fn traverse<S>(&self, node_index: usize, query: &Point<T, N>, strategy: &mut S)
    where
        S: SearchStrategy<T>,
    {
        let node = &self.nodes[node_index];
        match node {
            Node::Leaf { point_indices } => {
                for &point_index in point_indices {
                    let point = &self.points[point_index];
                    let distance = self.metric.measure(query, point);
                    strategy.visit_point(point_index, distance);
                }
            }
            Node::Inner {
                split_axis: axis,
                pivot_index: point_index,
                left_child: left,
                right_child: right,
            } => {
                let point = &self.points[*point_index];
                let distance = self.metric.measure(query, point);
                strategy.visit_point(*point_index, distance);

                let delta = query[*axis] - point[*axis];
                let (near, far) = if delta <= T::zero() {
                    (*left, *right)
                } else {
                    (*right, *left)
                };

                if let Some(near_index) = near {
                    self.traverse(near_index, query, strategy);
                }

                if strategy.should_continue(delta.abs()) {
                    if let Some(far_index) = far {
                        self.traverse(far_index, query, strategy);
                    }
                }
            }
        }
    }
}

impl<T, const N: usize> NeighborSearch<T, N> for KdTreeSearch<'_, T, N>
where
    T: FloatNumber,
{
    fn search(&self, query: &Point<T, N>, k: usize) -> Vec<Neighbor<T>> {
        let Some(root) = self.root else {
            return Vec::new();
        };

        let k = k.min(self.points.len());
        if k == 0 {
            return Vec::new();
        }

        let mut strategy = KNearestSearchStrategy::new(k);
        self.traverse(root, query, &mut strategy);
        strategy.into_result()
    }

    fn search_nearest(&self, query: &Point<T, N>) -> Option<Neighbor<T>> {
        let root = self.root?;
        let mut strategy = NearestSearchStrategy::new();
        self.traverse(root, query, &mut strategy);
        strategy.into_result()
    }

    fn search_radius(&self, query: &Point<T, N>, radius: T) -> Vec<Neighbor<T>> {
        let Some(root) = self.root else {
            return Vec::new();
        };

        if radius < T::zero() {
            return Vec::new();
        }

        let mut strategy = RadiusSearchStrategy::new(radius);
        self.traverse(root, query, &mut strategy);
        strategy.into_result()
    }
}

/// Strategy trait for customizing KD-tree traversal behavior.
///
/// This trait defines how points are collected during tree traversal and
/// when branches can be pruned. Different implementations enable various
/// search types (k-nearest, radius-based, etc.) without duplicating the
/// traversal logic.
///
/// # Type Parameters
/// * `T` - The floating point type used for distances.
trait SearchStrategy<T>
where
    T: FloatNumber,
{
    /// The type of result produced by this search strategy.
    type Result;

    /// Processes a point found during traversal.
    ///
    /// # Arguments
    /// * `point_index` - Index of the point in the original points array.
    /// * `distance` - Distance from the query point to this point.
    fn visit_point(&mut self, point_index: usize, distance: T);

    /// Determines whether to explore a subtree based on its distance from the query.
    ///
    /// # Arguments
    /// * `distance` - Distance from the query point to the splitting plane.
    ///
    /// # Returns
    /// `true` if the subtree should be explored, `false` to prune it.
    #[must_use]
    fn should_continue(&self, distance: T) -> bool;

    /// Converts the collected results into the final output format.
    ///
    /// # Returns
    /// The search results in the strategy-specific format.
    #[must_use]
    fn into_result(self) -> Self::Result;
}

/// Search strategy for finding k-nearest neighbors.
///
/// # Type Parameters
/// * `T` - The floating point type used for distances.
struct KNearestSearchStrategy<T>
where
    T: FloatNumber,
{
    /// Number of neighbors to find.
    k: usize,

    /// Max-heap storing the k closest neighbors found so far.
    neighbors: BinaryHeap<Neighbor<T>>,
}

impl<T> KNearestSearchStrategy<T>
where
    T: FloatNumber,
{
    /// Creates a new k-nearest neighbor search strategy.
    ///
    /// # Arguments
    /// * `k` - Number of nearest neighbors to find.
    ///
    /// # Returns
    /// A new strategy instance with pre-allocated capacity.
    #[must_use]
    fn new(k: usize) -> Self {
        Self {
            k,
            neighbors: BinaryHeap::with_capacity(k),
        }
    }
}

impl<T> SearchStrategy<T> for KNearestSearchStrategy<T>
where
    T: FloatNumber,
{
    type Result = Vec<Neighbor<T>>;

    fn visit_point(&mut self, point_index: usize, distance: T) {
        if self.neighbors.len() < self.k {
            self.neighbors.push(Neighbor::new(point_index, distance));
        } else if let Some(farthest) = self.neighbors.peek() {
            if distance < farthest.distance {
                self.neighbors.pop();
                self.neighbors.push(Neighbor::new(point_index, distance));
            }
        }
    }

    fn should_continue(&self, distance: T) -> bool {
        self.neighbors
            .peek()
            .is_none_or(|farthest| self.neighbors.len() < self.k || distance < farthest.distance)
    }

    fn into_result(self) -> Self::Result {
        self.neighbors.into_sorted_vec()
    }
}

/// Search strategy for finding the single nearest neighbor.
///
/// # Type Parameters
/// * `T` - The floating point type used for distances.
struct NearestSearchStrategy<T>
where
    T: FloatNumber,
{
    /// The closest neighbor found so far, if any.
    nearest: Option<Neighbor<T>>,
}

impl<T> NearestSearchStrategy<T>
where
    T: FloatNumber,
{
    /// Creates a new nearest neighbor search strategy.
    ///
    /// # Returns
    /// A new strategy instance with no initial neighbor.
    #[must_use]
    fn new() -> Self {
        Self { nearest: None }
    }
}

impl<T> SearchStrategy<T> for NearestSearchStrategy<T>
where
    T: FloatNumber,
{
    type Result = Option<Neighbor<T>>;

    fn visit_point(&mut self, point_index: usize, distance: T) {
        if self.nearest.as_ref().is_none_or(|n| distance < n.distance) {
            self.nearest = Some(Neighbor::new(point_index, distance));
        }
    }

    fn should_continue(&self, distance: T) -> bool {
        self.nearest
            .as_ref()
            .is_none_or(|nearest| distance < nearest.distance)
    }

    fn into_result(self) -> Self::Result {
        self.nearest
    }
}

/// Search strategy for finding all neighbors within a radius.
///
/// # Type Parameters
/// * `T` - The floating point type used for distances.
struct RadiusSearchStrategy<T>
where
    T: FloatNumber,
{
    /// Maximum distance from the query point.
    radius: T,

    /// All neighbors found within the radius.
    neighbors: Vec<Neighbor<T>>,
}

impl<T> RadiusSearchStrategy<T>
where
    T: FloatNumber,
{
    /// Initial capacity for the neighbors vector to minimize reallocations.
    const INITIAL_NEIGHBOR_CAPACITY: usize = 32;

    /// Creates a new radius search strategy.
    ///
    /// # Arguments
    /// * `radius` - Maximum distance from the query point.
    ///
    /// # Returns
    /// A new strategy instance with an empty results vector.
    #[must_use]
    fn new(radius: T) -> Self {
        Self {
            radius,
            neighbors: Vec::with_capacity(Self::INITIAL_NEIGHBOR_CAPACITY),
        }
    }
}

impl<T> SearchStrategy<T> for RadiusSearchStrategy<T>
where
    T: FloatNumber,
{
    type Result = Vec<Neighbor<T>>;

    fn visit_point(&mut self, point_index: usize, distance: T) {
        if distance <= self.radius {
            self.neighbors.push(Neighbor::new(point_index, distance));
        }
    }

    fn should_continue(&self, distance: T) -> bool {
        distance <= self.radius
    }

    fn into_result(self) -> Self::Result {
        self.neighbors
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
        let search = KdTreeSearch::build(&points, DistanceMetric::Euclidean);

        // Assert
        assert_eq!(search.points, &points);
        assert_eq!(search.metric, DistanceMetric::Euclidean);
        assert_eq!(search.root, Some(0));
        assert_eq!(search.nodes.len(), 3);
    }

    #[test]
    fn test_build_with_empty_points() {
        // Act
        let points = empty_points();
        let search: KdTreeSearch<f32, 3> =
            KdTreeSearch::with_leaf_size(&points, DistanceMetric::Euclidean, 8);

        // Assert
        assert_eq!(search.points, &points);
        assert_eq!(search.metric, DistanceMetric::Euclidean);
        assert_eq!(search.root, None);
        assert_eq!(search.nodes.len(), 0);
    }

    #[test]
    fn test_build_with_leaf_capacity() {
        // Act
        let points = sample_points();
        let search = KdTreeSearch::with_leaf_size(&points, DistanceMetric::Euclidean, 4);

        // Assert
        assert_eq!(search.points, &points);
        assert_eq!(search.metric, DistanceMetric::Euclidean);
        assert_eq!(search.root, Some(0));
        assert_eq!(search.nodes.len(), 7);
    }

    #[test]
    fn test_build_with_zero_leaf_capacity() {
        // Act
        let points = sample_points();
        let search = KdTreeSearch::with_leaf_size(&points, DistanceMetric::Euclidean, 0);

        // Assert
        assert_eq!(search.points, &points);
        assert_eq!(search.metric, DistanceMetric::Euclidean);
        assert_eq!(search.root, Some(0));
        assert_eq!(search.nodes.len(), points.len());
    }

    #[test]
    fn test_search() {
        // Arrange
        let points = sample_points();
        let search = KdTreeSearch::with_leaf_size(&points, DistanceMetric::Euclidean, 4);

        // Act
        let query = [3.0, 5.0, 6.0];
        let actual = search.search(&query, 3);

        // Assert
        assert_eq!(actual.len(), 3);
        assert_eq!(actual[0], Neighbor::new(8, 1.0_f32.sqrt()));
        assert_eq!(actual[1], Neighbor::new(4, 19.0_f32.sqrt()));
        assert_eq!(actual[2], Neighbor::new(0, 22.0_f32.sqrt()));
    }

    #[test]
    fn test_search_k_zero() {
        // Arrange
        let points = sample_points();
        let search = KdTreeSearch::with_leaf_size(&points, DistanceMetric::Euclidean, 4);

        // Act
        let query = [3.0, 5.0, 6.0];
        let actual = search.search(&query, 0);

        // Assert
        assert!(actual.is_empty());
    }

    #[test]
    fn test_search_k_greater_than_points() {
        // Arrange
        let points = sample_points();
        let search = KdTreeSearch::with_leaf_size(&points, DistanceMetric::Euclidean, 4);

        // Act
        let query = [3.0, 5.0, 6.0];
        let actual = search.search(&query, 20); // More than available points

        // Assert
        assert_eq!(actual.len(), points.len());
    }

    #[test]
    fn test_search_empty_tree() {
        // Arrange
        let points = empty_points();
        let search = KdTreeSearch::with_leaf_size(&points, DistanceMetric::Euclidean, 4);

        // Act
        let query = [3.0, 5.0, 6.0];
        let actual = search.search(&query, 3);

        // Assert
        assert!(actual.is_empty());
    }

    #[test]
    fn test_search_single_point() {
        // Arrange
        let points = vec![[1.0, 2.0, 3.0]];
        let search = KdTreeSearch::with_leaf_size(&points, DistanceMetric::Euclidean, 4);

        // Act
        let query = [2.0, 2.0, 3.0];
        let actual = search.search(&query, 1);

        // Assert
        assert_eq!(actual.len(), 1);
        assert_eq!(actual[0], Neighbor::new(0, 1.0));
    }

    #[test]
    fn test_search_exact_match() {
        // Arrange
        let points = sample_points();
        let search = KdTreeSearch::with_leaf_size(&points, DistanceMetric::Euclidean, 4);

        // Act
        let query = [3.0, 9.0, 1.0]; // Exact match with point at index 3
        let actual = search.search(&query, 1);

        // Assert
        assert_eq!(actual.len(), 1);
        assert_eq!(actual[0], Neighbor::new(3, 0.0));
    }

    #[test]
    fn test_search_all_points() {
        // Arrange
        let points = sample_points();
        let search = KdTreeSearch::with_leaf_size(&points, DistanceMetric::Euclidean, 4);

        // Act
        let query = [5.0, 5.0, 5.0];
        let actual = search.search(&query, points.len());

        // Assert
        assert_eq!(actual.len(), points.len());
    }

    #[test]
    fn test_search_nearest() {
        // Arrange
        let points = sample_points();
        let search = KdTreeSearch::with_leaf_size(&points, DistanceMetric::Euclidean, 4);

        // Act
        let query = [2.0, 2.0, 1.0];
        let actual = search.search_nearest(&query);

        // Assert
        assert!(actual.is_some());
        assert_eq!(actual.unwrap(), Neighbor::new(7, 1.0));
    }

    #[test]
    fn test_search_nearest_empty() {
        // Arrange
        let points = empty_points();
        let search = KdTreeSearch::with_leaf_size(&points, DistanceMetric::Euclidean, 4);

        // Act
        let query = [3.0, 2.0, 1.0];
        let actual = search.search_nearest(&query);

        // Assert
        assert!(actual.is_none());
    }

    #[test]
    fn test_search_nearest_single_point() {
        // Arrange
        let points = vec![[5.0, 5.0, 5.0]];
        let search = KdTreeSearch::with_leaf_size(&points, DistanceMetric::Euclidean, 4);

        // Act
        let query = [1.0, 1.0, 1.0];
        let actual = search.search_nearest(&query);

        // Assert
        assert!(actual.is_some());
        assert_eq!(actual.unwrap(), Neighbor::new(0, 48.0_f32.sqrt()));
    }

    #[test]
    fn test_search_nearest_exact_match() {
        // Arrange
        let points = sample_points();
        let search = KdTreeSearch::with_leaf_size(&points, DistanceMetric::Euclidean, 4);

        // Act
        let query = [9.0, 1.0, 1.0];
        let actual = search.search_nearest(&query).unwrap();

        // Assert
        assert_eq!(actual, Neighbor::new(5, 0.0));
    }

    #[test]
    fn test_search_nearest_boundary_case() {
        // Arrange
        let points = vec![
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ];
        let search = KdTreeSearch::with_leaf_size(&points, DistanceMetric::Euclidean, 0);

        // Act
        let query = [0.5, 0.5, 0.5];
        let actual = search.search_nearest(&query);

        // Assert
        assert!(actual.is_some());
        assert_eq!(actual.unwrap(), Neighbor::new(3, 0.75_f32.sqrt()));
    }

    #[test]
    fn test_search_radius() {
        // Arrange
        let points = sample_points();
        let search = KdTreeSearch::with_leaf_size(&points, DistanceMetric::Euclidean, 4);

        // Act
        let query = [3.0, 5.0, 6.0];
        let actual = search.search_radius(&query, 4.5);

        // Assert
        assert_eq!(actual.len(), 2);
        assert_eq!(actual[0], Neighbor::new(8, 1.0_f32.sqrt()));
        assert_eq!(actual[1], Neighbor::new(4, 19.0_f32.sqrt()));
    }

    #[test]
    fn test_search_radius_with_zero_radius() {
        // Arrange
        let points = sample_points();
        let search = KdTreeSearch::with_leaf_size(&points, DistanceMetric::Euclidean, 4);

        // Act
        let query = [3.0, 5.0, 6.0];
        let actual = search.search_radius(&query, 0.0);

        // Assert
        assert!(actual.is_empty());
    }

    #[test]
    fn test_search_radius_with_negative_radius() {
        // Arrange
        let points = sample_points();
        let search = KdTreeSearch::with_leaf_size(&points, DistanceMetric::Euclidean, 4);

        // Act
        let query = [3.0, 5.0, 6.0];
        let actual = search.search_radius(&query, -1.0);

        // Assert
        assert!(actual.is_empty());
    }

    #[test]
    fn test_search_radius_exact_match() {
        // Arrange
        let points = sample_points();
        let search = KdTreeSearch::with_leaf_size(&points, DistanceMetric::Euclidean, 4);

        // Act
        let query = [3.0, 9.0, 1.0]; // Exact match with point at index 3
        let actual = search.search_radius(&query, 0.001);

        // Assert
        assert_eq!(actual.len(), 1);
        assert_eq!(actual[0], Neighbor::new(3, 0.0));
    }

    #[test]
    fn test_search_radius_empty_points() {
        // Arrange
        let points = empty_points();
        let search = KdTreeSearch::with_leaf_size(&points, DistanceMetric::Euclidean, 4);

        // Act
        let query = [3.0, 5.0, 6.0];
        let actual = search.search_radius(&query, 1.0);

        // Assert
        assert!(actual.is_empty());
    }

    #[test]
    fn test_search_radius_large() {
        // Arrange
        let points = sample_points();
        let search = KdTreeSearch::with_leaf_size(&points, DistanceMetric::Euclidean, 4);

        // Act
        let query = [5.0, 5.0, 5.0];
        let actual = search.search_radius(&query, 100.0);

        // Assert
        assert_eq!(actual.len(), points.len());
    }

    #[test]
    fn test_search_radius_single_point() {
        // Arrange
        let points = vec![[3.0, 5.0, 6.0]];
        let search = KdTreeSearch::with_leaf_size(&points, DistanceMetric::Euclidean, 2);

        // Act
        let query = [1.0, 1.0, 1.0];
        let neighbors = search.search_radius(&query, 45.0);

        // Assert
        assert_eq!(neighbors.len(), 1);
        assert_eq!(neighbors[0], Neighbor::new(0, 45.0_f32.sqrt()));
    }

    #[test]
    fn test_search_radius_boundary() {
        // Arrange
        let points = sample_points();
        let search = KdTreeSearch::with_leaf_size(&points, DistanceMetric::Euclidean, 4);

        // Act
        let query = [3.0, 5.0, 6.0];
        let actual = search.search_radius(&query, 1.0);

        // Assert
        assert_eq!(actual.len(), 1);
        assert_eq!(actual[0], Neighbor::new(8, 1.0));
    }

    #[test]
    fn test_search_radius_no_matches() {
        // Arrange
        let points = sample_points();
        let search = KdTreeSearch::with_leaf_size(&points, DistanceMetric::Euclidean, 4);

        // Act
        let query = [100.0, 100.0, 100.0];
        let actual = search.search_radius(&query, 10.0);

        // Assert
        assert!(actual.is_empty());
    }
}

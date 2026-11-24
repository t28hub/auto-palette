use std::{cmp::Ordering, collections::BinaryHeap, mem};

use rustc_hash::FxHashMap;

use super::{grid::Grid, node::Node};
use crate::next::math::{DistanceMetric, Vector};

/// A formula for the distance between two clusters.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) enum Linkage {
    /// The largest distance between their points.
    #[default]
    Complete,
    /// The distance between the weighted means of their points.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "selected by the builder, which is not implemented yet"
        )
    )]
    Centroid,
}

impl Linkage {
    /// Merges the closest pair of nodes until every pair is at least the threshold apart.
    ///
    /// # Arguments
    /// * `nodes` - The nodes to merge, in the order of their first point.
    /// * `threshold` - The distance at or above which two nodes stay apart.
    ///
    /// # Returns
    /// The merged nodes, in the order of their first point.
    pub(super) fn merge(self, nodes: Vec<Node>, threshold: f64) -> Vec<Node> {
        match self {
            Self::Complete => merge_by_complete_linkage(nodes, threshold),
            Self::Centroid => merge_by_centroid_linkage(nodes, threshold),
        }
    }
}

/// Merges nodes by the largest distance between their points.
///
/// # Arguments
/// * `nodes` - The nodes to merge, in the order of their first point.
/// * `threshold` - The distance at or above which two nodes stay apart.
///
/// # Returns
/// The merged nodes, in the order of their first point.
fn merge_by_complete_linkage(nodes: Vec<Node>, threshold: f64) -> Vec<Node> {
    // Keeps only pairs under the threshold: complete linkage never shrinks a distance.
    let mut neighbors = find_neighbors(&nodes, threshold);
    let mut pairs: BinaryHeap<Pair> = neighbors
        .iter()
        .enumerate()
        .flat_map(|(index, near)| {
            near.iter()
                .filter(move |&(&other, _)| index < other)
                .map(move |(&other, &distance)| Pair::new(distance, index, other))
        })
        .collect();
    let mut nodes: Vec<Option<Node>> = nodes.into_iter().map(Some).collect();

    while let Some(pair) = pairs.pop() {
        let (lower, upper) = (pair.lower as usize, pair.upper as usize);
        // Skips a pair made stale by a later merge.
        if neighbors[lower].get(&upper) != Some(&pair.distance) {
            continue;
        }
        merge_pair(&mut nodes, lower, upper);

        // Removes the merged-away node from the neighbors of the others.
        let near_upper = mem::take(&mut neighbors[upper]);
        for other in near_upper.keys() {
            neighbors[*other].remove(&upper);
        }

        // Keeps only the neighbors of both nodes, at the larger of their two distances.
        let mut near_lower = mem::take(&mut neighbors[lower]);
        near_lower.retain(|&other, distance| {
            let Some(&through_upper) = near_upper.get(&other) else {
                neighbors[other].remove(&lower);
                return false;
            };
            if through_upper <= *distance {
                return true;
            }
            *distance = through_upper;
            neighbors[other].insert(lower, through_upper);
            pairs.push(Pair::new(through_upper, lower, other));
            true
        });
        neighbors[lower] = near_lower;
    }

    nodes.into_iter().flatten().collect()
}

/// Finds, for each node, the nodes with every pair of points closer than the threshold.
///
/// # Arguments
/// * `nodes` - The nodes to look through.
/// * `threshold` - The distance at or above which two nodes stay apart.
///
/// # Returns
/// The neighbors of each node, with the largest distance between their points.
fn find_neighbors(nodes: &[Node], threshold: f64) -> Vec<FxHashMap<usize, f64>> {
    let mut neighbors = vec![FxHashMap::default(); nodes.len()];
    let mut grid = Grid::new(threshold);
    for (index, node) in nodes.iter().enumerate() {
        // The first vectors of two neighbors are closer than the threshold, like all their vectors.
        let first_vector = node.vectors()[0];
        for other in grid.find_near(first_vector) {
            if let Some(distance) = measure_farthest(node, &nodes[other], threshold) {
                neighbors[index].insert(other, distance);
                neighbors[other].insert(index, distance);
            }
        }
        grid.insert(index, first_vector);
    }
    neighbors
}

/// Measures the largest distance between the points of two nodes, if it is under a limit.
///
/// # Arguments
/// * `first` - A node.
/// * `second` - The other node.
/// * `limit` - The distance at or above which the measurement stops.
///
/// # Returns
/// The largest distance, or `None` if a pair of points is at least `limit` apart.
fn measure_farthest(first: &Node, second: &Node, limit: f64) -> Option<f64> {
    let mut farthest: f64 = 0.0;
    for &first_vector in first.vectors() {
        for &second_vector in second.vectors() {
            let distance = measure(first_vector, second_vector);
            if distance >= limit {
                return None;
            }
            farthest = farthest.max(distance);
        }
    }
    Some(farthest)
}

/// Merges nodes by the distance between the weighted means of their points.
///
/// # Arguments
/// * `nodes` - The nodes to merge, in the order of their first point.
/// * `threshold` - The distance at or above which two nodes stay apart.
///
/// # Returns
/// The merged nodes, in the order of their first point.
fn merge_by_centroid_linkage(nodes: Vec<Node>, threshold: f64) -> Vec<Node> {
    let mut means: Vec<Vector<3>> = nodes.iter().map(Node::mean).collect();
    let mut nodes: Vec<Option<Node>> = nodes.into_iter().map(Some).collect();
    let mut grid = Grid::new(threshold);
    let mut pairs = BinaryHeap::new();
    for index in 0..nodes.len() {
        insert_mean(index, &means, threshold, &mut grid, &mut pairs);
    }

    while let Some(pair) = pairs.pop() {
        let (lower, upper) = (pair.lower as usize, pair.upper as usize);
        let merged_away = nodes[lower].is_none() || nodes[upper].is_none();
        if merged_away {
            continue;
        }
        let moved = measure(means[lower], means[upper]) != pair.distance;
        if moved {
            continue;
        }

        // A merged mean can move closer to other nodes, so the pairs around it are searched again.
        grid.remove(upper, means[upper]);
        grid.remove(lower, means[lower]);
        means[lower] = merge_pair(&mut nodes, lower, upper).mean();
        insert_mean(lower, &means, threshold, &mut grid, &mut pairs);
    }

    nodes.into_iter().flatten().collect()
}

/// Inserts a mean into the grid after pushing its pairs closer than the threshold.
///
/// # Arguments
/// * `index` - The index of the node.
/// * `means` - The weighted mean of the points of each node.
/// * `threshold` - The distance at or above which two nodes stay apart.
/// * `grid` - The means inserted so far.
/// * `pairs` - The pairs to push to.
fn insert_mean(
    index: usize,
    means: &[Vector<3>],
    threshold: f64,
    grid: &mut Grid,
    pairs: &mut BinaryHeap<Pair>,
) {
    for other in grid.find_near(means[index]) {
        let distance = measure(means[index], means[other]);
        if distance < threshold {
            pairs.push(Pair::new(distance, index, other));
        }
    }
    grid.insert(index, means[index]);
}

/// Merges the upper node of a pair into the lower one.
///
/// # Arguments
/// * `nodes` - The nodes, `None` for one merged away.
/// * `lower` - The index of the node to keep.
/// * `upper` - The index of the node to merge away.
///
/// # Returns
/// The merged node.
fn merge_pair(nodes: &mut [Option<Node>], lower: usize, upper: usize) -> &Node {
    let merged = nodes[upper].take().expect("a pair has two nodes");
    let kept = nodes[lower].as_mut().expect("a pair has two nodes");
    kept.absorb(merged);
    kept
}

/// A pair of nodes closer than the threshold, the closest first in a heap.
#[derive(Debug, Clone, Copy)]
struct Pair {
    /// The distance between the nodes.
    distance: f64,
    /// The index of the node to keep.
    lower: u32,
    /// The index of the node to merge away.
    upper: u32,
}

impl Pair {
    /// Pairs two nodes by their distance, the lower index first.
    ///
    /// # Arguments
    /// * `distance` - The distance between the nodes.
    /// * `first` - The index of a node.
    /// * `second` - The index of the other node.
    ///
    /// # Returns
    /// The pair of the nodes.
    fn new(distance: f64, first: usize, second: usize) -> Self {
        Self {
            distance,
            lower: first.min(second) as u32,
            upper: first.max(second) as u32,
        }
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        // Closest pair first, ties to the lowest indices. Bits sort like these floats, but faster.
        (other.distance.to_bits(), other.lower, other.upper).cmp(&(
            self.distance.to_bits(),
            self.lower,
            self.upper,
        ))
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Pair {}

/// Measures the distance between two vectors.
///
/// # Arguments
/// * `first` - A vector.
/// * `second` - The other vector.
///
/// # Returns
/// The Euclidean distance between the vectors.
fn measure(first: Vector<3>, second: Vector<3>) -> f64 {
    first.distance(second, DistanceMetric::Euclidean)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::next::extraction::{clustering::cluster::Cluster, point::Point};

    /// Creates a point on the first axis with weight 1.0.
    fn point(x: f64) -> Point<3> {
        Point::of_vector(Vector([x, 0.0, 0.0]), 1.0)
    }

    /// Creates a node of each point.
    fn nodes(points: &[Point<3>]) -> Vec<Node> {
        points
            .iter()
            .enumerate()
            .map(|(index, &point)| Node::of_point(index, point))
            .collect()
    }

    /// Merges the nodes of points and converts the results.
    fn merge(linkage: Linkage, points: &[Point<3>], threshold: f64) -> Vec<Cluster> {
        linkage
            .merge(nodes(points), threshold)
            .into_iter()
            .map(Node::into_cluster)
            .collect()
    }

    #[test]
    fn test_merge_by_complete_compares_farthest_points() {
        // Arrange: 0.3 is within the threshold of the mean of 0.0 and 0.15, but not of 0.0.
        let points = [
            point(0.0),  // 1st cluster
            point(0.15), // 1st cluster
            point(0.3),  // 2nd cluster
        ];

        // Act
        let actual = merge(Linkage::Complete, &points, 0.25);

        // Assert
        assert_eq!(actual, [Cluster::new(vec![0, 1]), Cluster::new(vec![2])]);
    }

    #[test]
    fn test_merge_by_complete_updates_distances_after_merge() {
        // Arrange: once 1 and 2 merge, 0 is 0.18 from them, so 0 and 3, 0.15 apart, merge first.
        let points = [
            point(0.15), // 1st cluster
            point(0.25), // 2nd cluster
            point(0.33), // 2nd cluster
            point(0.0),  // 1st cluster
        ];

        // Act
        let actual = merge(Linkage::Complete, &points, 0.2);

        // Assert
        assert_eq!(actual, [Cluster::new(vec![0, 3]), Cluster::new(vec![1, 2])]);
    }

    #[test]
    fn test_merge_by_centroid_compares_means() {
        // Arrange: 0.3 is within the threshold of the mean of 0.0 and 0.15, but not of 0.0.
        let points = [point(0.0), point(0.15), point(0.3)];

        // Act
        let actual = merge(Linkage::Centroid, &points, 0.25);

        // Assert
        assert_eq!(actual, [Cluster::new(vec![0, 1, 2])]);
    }

    #[test]
    fn test_merge_by_centroid_weighs_mean_by_weight() {
        // Arrange: the heavy middle point pulls the mean of the first pair toward 0.6.
        let points = [
            point(0.0),
            Point::of_vector(Vector([0.3, 0.0, 0.0]), 3.0),
            point(0.6),
        ];

        // Act
        let actual = merge(Linkage::Centroid, &points, 0.4);

        // Assert
        assert_eq!(actual, [Cluster::new(vec![0, 1, 2])]);
    }

    #[test]
    fn test_measure_farthest_finds_largest_distance() {
        // Arrange
        let mut first = Node::of_point(0, point(0.0));
        first.absorb(Node::of_point(1, point(0.1)));
        let second = Node::of_point(2, point(0.3));

        // Act
        let actual = measure_farthest(&first, &second, 0.5);

        // Assert
        assert_eq!(actual, Some(0.3));
    }

    #[test]
    fn test_measure_farthest_stops_at_limit() {
        // Arrange
        let first = Node::of_point(0, point(0.0));
        let second = Node::of_point(1, Point::of_vector(Vector([0.3, 0.4, 0.0]), 1.0));

        // Act
        let actual = measure_farthest(&first, &second, 0.5);

        // Assert
        assert_eq!(actual, None);
    }

    #[test]
    fn test_pair_new_puts_lower_index_first() {
        // Act
        let actual = Pair::new(0.1, 3, 1);

        // Assert
        assert_eq!((actual.lower, actual.upper), (1, 3));
    }

    #[test]
    fn test_pairs_pop_closest_first() {
        // Arrange
        let mut pairs = BinaryHeap::from([Pair::new(0.2, 0, 1), Pair::new(0.1, 2, 3)]);

        // Act
        let actual = pairs.pop().unwrap();

        // Assert
        assert_eq!(actual, Pair::new(0.1, 2, 3));
    }

    #[test]
    fn test_pairs_pop_lowest_indices_first_on_tie() {
        // Arrange
        let mut pairs = BinaryHeap::from([
            Pair::new(0.1, 1, 2),
            Pair::new(0.1, 0, 2),
            Pair::new(0.1, 0, 1),
        ]);

        // Act
        let actual = pairs.pop().unwrap();

        // Assert
        assert_eq!(actual, Pair::new(0.1, 0, 1));
    }
}

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
    pub(super) fn merge<const N: usize>(self, nodes: Vec<Node<N>>, threshold: f64) -> Vec<Node<N>> {
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
fn merge_by_complete_linkage<const N: usize>(nodes: Vec<Node<N>>, threshold: f64) -> Vec<Node<N>> {
    // Keeps only pairs under the threshold: complete linkage never shrinks a distance.
    let mut neighbors = find_neighbors(&nodes, threshold);
    let mut pairs = BinaryHeap::new();
    for (index, near) in neighbors.iter().enumerate() {
        for (&other, &distance) in near {
            // Each pair is in the neighbors of both nodes, so only the lower index pushes it.
            if other < index {
                continue;
            }
            pairs.push(Pair::new(index, other, distance));
        }
    }
    let mut nodes: Vec<Option<Node<N>>> = nodes.into_iter().map(Some).collect();

    while let Some(pair) = pairs.pop() {
        let lower = pair.lower as usize;
        let upper = pair.upper as usize;
        let current_distance = neighbors[lower].get(&upper);
        let stale = current_distance != Some(&pair.distance);
        if stale {
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
            pairs.push(Pair::new(lower, other, through_upper));
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
fn find_neighbors<const N: usize>(nodes: &[Node<N>], threshold: f64) -> Vec<FxHashMap<usize, f64>> {
    let mut neighbors = vec![FxHashMap::default(); nodes.len()];
    let first_vectors = nodes.iter().map(|node| node.vectors()[0]);
    let mut grid = Grid::new(threshold, first_vectors);
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
fn measure_farthest<const N: usize>(first: &Node<N>, second: &Node<N>, limit: f64) -> Option<f64> {
    let mut farthest: f64 = 0.0;
    for &first_vector in first.vectors() {
        for &second_vector in second.vectors() {
            let distance = first_vector.distance(second_vector, DistanceMetric::Euclidean);
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
fn merge_by_centroid_linkage<const N: usize>(nodes: Vec<Node<N>>, threshold: f64) -> Vec<Node<N>> {
    let mut means: Vec<Vector<N>> = nodes.iter().map(Node::mean).collect();
    let mut nodes: Vec<Option<Node<N>>> = nodes.into_iter().map(Some).collect();
    // Merged means stay within the range of these, as each is a weighted mean of them.
    let mut grid = Grid::new(threshold, means.iter().copied());
    let mut pairs = BinaryHeap::new();
    for index in 0..nodes.len() {
        insert_mean(index, &means, threshold, &mut grid, &mut pairs);
    }

    while let Some(pair) = pairs.pop() {
        let lower = pair.lower as usize;
        let upper = pair.upper as usize;
        let merged_away = nodes[lower].is_none() || nodes[upper].is_none();
        if merged_away {
            continue;
        }
        let current_distance = means[lower].distance(means[upper], DistanceMetric::Euclidean);
        let moved = current_distance != pair.distance;
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
fn insert_mean<const N: usize>(
    index: usize,
    means: &[Vector<N>],
    threshold: f64,
    grid: &mut Grid<N>,
    pairs: &mut BinaryHeap<Pair>,
) {
    for other in grid.find_near(means[index]) {
        let distance = means[index].distance(means[other], DistanceMetric::Euclidean);
        if distance < threshold {
            pairs.push(Pair::new(index, other, distance));
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
fn merge_pair<const N: usize>(
    nodes: &mut [Option<Node<N>>],
    lower: usize,
    upper: usize,
) -> &Node<N> {
    let merged = nodes[upper].take().expect("a pair has two nodes");
    let kept = nodes[lower].as_mut().expect("a pair has two nodes");
    kept.absorb(merged);
    kept
}

/// A pair of nodes closer than the threshold, the closest first in a heap.
#[derive(Debug, Clone, Copy)]
struct Pair {
    /// The index of the node to keep.
    lower: u32,
    /// The index of the node to merge away.
    upper: u32,
    /// The distance between the nodes.
    distance: f64,
}

impl Pair {
    /// Pairs two nodes with their distance, the lower index first.
    ///
    /// # Arguments
    /// * `first` - The index of a node.
    /// * `second` - The index of the other node.
    /// * `distance` - The distance between the nodes.
    ///
    /// # Returns
    /// The pair of the nodes.
    fn new(first: usize, second: usize, distance: f64) -> Self {
        Self {
            lower: first.min(second) as u32,
            upper: first.max(second) as u32,
            distance,
        }
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        // In priority order, not field order: the closest pair first, ties to the lowest indices.
        // Non-negative floats sort like their bits, which compare faster.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::next::extraction::{clustering::cluster::Cluster, point::Point};

    /// Creates a point on the first axis with weight 1.0.
    fn point(x: f64) -> Point<3> {
        Point::of_vector(Vector([x, 0.0, 0.0]), 1.0)
    }

    /// Creates a node of each point.
    fn nodes(points: &[Point<3>]) -> Vec<Node<3>> {
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
        let actual = Pair::new(3, 1, 0.1);

        // Assert
        assert_eq!(actual.lower, 1);
        assert_eq!(actual.upper, 3);
    }

    #[test]
    fn test_pairs_pop_closest_first() {
        // Arrange
        let mut pairs = BinaryHeap::from([Pair::new(0, 1, 0.2), Pair::new(2, 3, 0.1)]);

        // Act
        let actual = pairs.pop().unwrap();

        // Assert
        assert_eq!(actual, Pair::new(2, 3, 0.1));
    }

    #[test]
    fn test_pairs_pop_lowest_indices_first_on_tie() {
        // Arrange
        let mut pairs = BinaryHeap::from([
            Pair::new(1, 2, 0.1),
            Pair::new(0, 2, 0.1),
            Pair::new(0, 1, 0.1),
        ]);

        // Act
        let actual = pairs.pop().unwrap();

        // Assert
        assert_eq!(actual, Pair::new(0, 1, 0.1));
    }
}

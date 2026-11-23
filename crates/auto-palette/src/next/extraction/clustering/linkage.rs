use std::{cmp::Ordering, collections::BinaryHeap, mem};

use rustc_hash::FxHashMap;

use super::{grid::Grid, node::Node};
use crate::next::math::{DistanceMetric, Vector};

/// How the difference between two clusters is measured.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) enum Linkage {
    /// The largest difference between their points.
    #[default]
    Complete,
    /// The difference between the weighted means of their colors.
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
    /// Merges the closest pair of nodes until every pair differs by at least the threshold.
    ///
    /// # Arguments
    /// * `nodes` - The nodes to merge, in the order of their first point.
    /// * `threshold` - The difference at or above which two nodes stay apart.
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

/// Merges nodes by the largest difference between their points.
///
/// # Arguments
/// * `nodes` - The nodes to merge, in the order of their first point.
/// * `threshold` - The difference at or above which two nodes stay apart.
///
/// # Returns
/// The merged nodes, in the order of their first point.
fn merge_by_complete_linkage(nodes: Vec<Node>, threshold: f64) -> Vec<Node> {
    // Unlike the textbook method, which keeps the difference of every pair, this keeps only the
    // pairs closer than the threshold. Complete linkage never brings two nodes closer by a merge,
    // so the other pairs could never merge.
    let mut neighbors = find_neighbors(&nodes, threshold);
    let mut pairs: BinaryHeap<Pair> = neighbors
        .iter()
        .enumerate()
        .flat_map(|(index, near)| {
            near.iter()
                .filter(move |&(&other, _)| index < other)
                .map(move |(&other, &difference)| Pair::new(difference, index, other))
        })
        .collect();
    let mut nodes: Vec<Option<Node>> = nodes.into_iter().map(Some).collect();
    while let Some(pair) = pairs.pop() {
        let (lower, upper) = (pair.lower as usize, pair.upper as usize);
        if neighbors[lower].get(&upper) != Some(&pair.difference) {
            continue;
        }
        merge_pair(&mut nodes, lower, upper);
        let near_upper = mem::take(&mut neighbors[upper]);
        for other in near_upper.keys() {
            neighbors[*other].remove(&upper);
        }
        let mut near_lower = mem::take(&mut neighbors[lower]);
        near_lower.retain(|&other, difference| {
            let Some(&through_upper) = near_upper.get(&other) else {
                neighbors[other].remove(&lower);
                return false;
            };
            if through_upper > *difference {
                *difference = through_upper;
                neighbors[other].insert(lower, through_upper);
                pairs.push(Pair::new(through_upper, lower, other));
            }
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
/// * `threshold` - The difference at or above which two nodes stay apart.
///
/// # Returns
/// The neighbors of each node, with the largest difference between their points.
fn find_neighbors(nodes: &[Node], threshold: f64) -> Vec<FxHashMap<usize, f64>> {
    let mut neighbors = vec![FxHashMap::default(); nodes.len()];
    let mut grid = Grid::new(threshold);
    for (index, node) in nodes.iter().enumerate() {
        // The first colors of two neighbors are closer than the threshold, like all their colors.
        let first_color = node.colors()[0];
        for other in grid.find_near(first_color) {
            if let Some(difference) = measure_farthest(node, &nodes[other], threshold) {
                neighbors[index].insert(other, difference);
                neighbors[other].insert(index, difference);
            }
        }
        grid.insert(index, first_color);
    }
    neighbors
}

/// Measures the largest difference between the points of two nodes, if it is under a limit.
///
/// # Arguments
/// * `first` - A node.
/// * `second` - The other node.
/// * `limit` - The difference at or above which the measurement stops.
///
/// # Returns
/// The largest difference, or `None` if a pair of points differs by at least `limit`.
fn measure_farthest(first: &Node, second: &Node, limit: f64) -> Option<f64> {
    let mut farthest: f64 = 0.0;
    for &first_color in first.colors() {
        for &second_color in second.colors() {
            let difference = measure(first_color, second_color);
            if difference >= limit {
                return None;
            }
            farthest = farthest.max(difference);
        }
    }
    Some(farthest)
}

/// Merges nodes by the difference between the weighted means of their colors.
///
/// # Arguments
/// * `nodes` - The nodes to merge, in the order of their first point.
/// * `threshold` - The difference at or above which two nodes stay apart.
///
/// # Returns
/// The merged nodes, in the order of their first point.
fn merge_by_centroid_linkage(nodes: Vec<Node>, threshold: f64) -> Vec<Node> {
    // Centroid linkage can invert: a merged mean can lie closer to another node than the pair
    // was. So the pairs around each new mean are searched again, and merging stops only when no
    // pair is closer than the threshold, rather than by cutting a full hierarchy at it.
    let mut coordinates: Vec<Vector<3>> = nodes.iter().map(Node::mean).collect();
    let mut nodes: Vec<Option<Node>> = nodes.into_iter().map(Some).collect();
    let mut grid = Grid::new(threshold);
    let mut pairs = BinaryHeap::new();
    for index in 0..nodes.len() {
        insert_centroid(index, &coordinates, threshold, &mut grid, &mut pairs);
    }
    while let Some(pair) = pairs.pop() {
        let (lower, upper) = (pair.lower as usize, pair.upper as usize);
        let merged_away = nodes[lower].is_none() || nodes[upper].is_none();
        if merged_away {
            continue;
        }
        let moved = measure(coordinates[lower], coordinates[upper]) != pair.difference;
        if moved {
            continue;
        }
        grid.remove(upper, coordinates[upper]);
        grid.remove(lower, coordinates[lower]);
        coordinates[lower] = merge_pair(&mut nodes, lower, upper).mean();
        insert_centroid(lower, &coordinates, threshold, &mut grid, &mut pairs);
    }
    nodes.into_iter().flatten().collect()
}

/// Inserts a centroid into the grid after pushing its pairs closer than the threshold.
///
/// # Arguments
/// * `index` - The index of the node.
/// * `coordinates` - The mean of the colors of each node.
/// * `threshold` - The difference at or above which two nodes stay apart.
/// * `grid` - The centroids inserted so far, at their coordinates.
/// * `pairs` - The pairs to push to.
fn insert_centroid(
    index: usize,
    coordinates: &[Vector<3>],
    threshold: f64,
    grid: &mut Grid,
    pairs: &mut BinaryHeap<Pair>,
) {
    for other in grid.find_near(coordinates[index]) {
        let difference = measure(coordinates[index], coordinates[other]);
        if difference < threshold {
            pairs.push(Pair::new(difference, index, other));
        }
    }
    grid.insert(index, coordinates[index]);
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
    /// The difference between the nodes.
    difference: f64,
    /// The index of the node to keep.
    lower: u32,
    /// The index of the node to merge away.
    upper: u32,
}

impl Pair {
    /// Pairs two nodes by their difference, the lower index first.
    ///
    /// # Arguments
    /// * `difference` - The difference between the nodes.
    /// * `first` - The index of a node.
    /// * `second` - The index of the other node.
    ///
    /// # Returns
    /// The pair of the nodes.
    fn new(difference: f64, first: usize, second: usize) -> Self {
        Self {
            difference,
            lower: first.min(second) as u32,
            upper: first.max(second) as u32,
        }
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reversed, so the heap pops the closest pair first, ties to the lowest indices.
        // Non-negative floats order the same as their bits, which compare faster.
        (other.difference.to_bits(), other.lower, other.upper).cmp(&(
            self.difference.to_bits(),
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

/// Measures the difference between two coordinates.
///
/// # Arguments
/// * `first` - The coordinates of a color or a mean.
/// * `second` - The other coordinates.
///
/// # Returns
/// The Euclidean distance, which is the color difference between two colors.
fn measure(first: Vector<3>, second: Vector<3>) -> f64 {
    first.distance(second, DistanceMetric::Euclidean)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::next::{
        color::DeltaE,
        extraction::{
            clustering::cluster::Cluster,
            point::Point,
            position::Position,
            test_helpers::{gray, point},
        },
    };

    /// Creates a node of each point, embedded in OKLab.
    fn nodes(points: &[Point]) -> Vec<Node> {
        points
            .iter()
            .enumerate()
            .map(|(index, point)| {
                Node::of_point(index, DeltaE::Ok.embed(point.color()), point.weight())
            })
            .collect()
    }

    /// Merges the nodes of points and converts the results.
    fn merge(linkage: Linkage, points: &[Point], threshold: f64) -> Vec<Cluster> {
        linkage
            .merge(nodes(points), threshold)
            .into_iter()
            .map(Node::into_cluster)
            .collect()
    }

    #[test]
    fn test_merge_by_complete_compares_farthest_points() {
        // Arrange: 0.3 is within the threshold of the mean of 0.0 and 0.15, but not of 0.0.
        let points = [point(gray(0.0)), point(gray(0.15)), point(gray(0.3))];

        // Act
        let actual = merge(Linkage::Complete, &points, 0.25);

        // Assert
        assert_eq!(actual, [Cluster::new(vec![0, 1]), Cluster::new(vec![2])]);
    }

    #[test]
    fn test_merge_by_complete_updates_differences_after_merge() {
        // Arrange: once 1 and 2 merge, 0 is 0.18 from them, so 0 and 3, 0.15 apart, merge first.
        let points = [
            point(gray(0.15)),
            point(gray(0.25)),
            point(gray(0.33)),
            point(gray(0.0)),
        ];

        // Act
        let actual = merge(Linkage::Complete, &points, 0.2);

        // Assert
        assert_eq!(actual, [Cluster::new(vec![0, 3]), Cluster::new(vec![1, 2])]);
    }

    #[test]
    fn test_merge_by_centroid_compares_means() {
        // Arrange: 0.3 is within the threshold of the mean of 0.0 and 0.15, but not of 0.0.
        let points = [point(gray(0.0)), point(gray(0.15)), point(gray(0.3))];

        // Act
        let actual = merge(Linkage::Centroid, &points, 0.25);

        // Assert
        assert_eq!(actual, [Cluster::new(vec![0, 1, 2])]);
    }

    #[test]
    fn test_merge_by_centroid_weighs_mean_by_weight() {
        // Arrange: the heavy middle gray pulls the mean of the first pair toward 0.6.
        let points = [
            point(gray(0.0)),
            Point::new(gray(0.3), Position::new(0.0, 0.0), 3.0),
            point(gray(0.6)),
        ];

        // Act
        let actual = merge(Linkage::Centroid, &points, 0.4);

        // Assert
        assert_eq!(actual, [Cluster::new(vec![0, 1, 2])]);
    }

    #[test]
    fn test_measure_farthest_finds_largest_difference() {
        // Arrange
        let mut first = Node::of_point(0, Vector([0.0, 0.0, 0.0]), 1.0);
        first.absorb(Node::of_point(1, Vector([0.1, 0.0, 0.0]), 1.0));
        let second = Node::of_point(2, Vector([0.3, 0.0, 0.0]), 1.0);

        // Act
        let actual = measure_farthest(&first, &second, 0.5);

        // Assert
        assert_eq!(actual, Some(0.3));
    }

    #[test]
    fn test_measure_farthest_stops_at_limit() {
        // Arrange
        let first = Node::of_point(0, Vector([0.0, 0.0, 0.0]), 1.0);
        let second = Node::of_point(1, Vector([0.3, 0.4, 0.0]), 1.0);

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

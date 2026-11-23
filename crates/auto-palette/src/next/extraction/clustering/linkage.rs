use std::{cmp::Ordering, collections::BinaryHeap, mem};

use rustc_hash::FxHashMap;

use super::{grid::Grid, node::Node};
use crate::next::{
    color::DeltaE,
    math::{DistanceMetric, Vector},
};

/// How the difference between two clusters is measured.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) enum Linkage {
    /// The largest difference between their points.
    #[default]
    Complete,
    /// The difference between their weighted mean colors.
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
    /// * `difference` - The formula for the color difference between two points.
    ///
    /// # Returns
    /// The merged nodes, in the order of their first point.
    pub(super) fn merge(self, nodes: Vec<Node>, threshold: f64, difference: DeltaE) -> Vec<Node> {
        match self {
            Self::Complete => merge_by_complete_linkage(nodes, threshold),
            Self::Centroid => merge_by_centroid_linkage(nodes, threshold, difference),
        }
    }
}

/// Merges nodes by the difference between their farthest points.
///
/// # Arguments
/// * `nodes` - The nodes to merge, in the order of their first point.
/// * `threshold` - The difference at or above which two nodes stay apart.
///
/// # Returns
/// The merged nodes, in the order of their first point.
fn merge_by_complete_linkage(nodes: Vec<Node>, threshold: f64) -> Vec<Node> {
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

/// Finds, for each node, the nodes whose farthest points are closer than the threshold.
///
/// # Arguments
/// * `nodes` - The nodes to look through.
/// * `threshold` - The difference at or above which two nodes stay apart.
///
/// # Returns
/// The neighbors of each node, with the distance between their farthest points.
fn find_neighbors(nodes: &[Node], threshold: f64) -> Vec<FxHashMap<usize, f64>> {
    let mut neighbors = vec![FxHashMap::default(); nodes.len()];
    let mut grid = Grid::new(threshold);
    for (index, node) in nodes.iter().enumerate() {
        let center = node.bbox().center();
        for other in grid.find_near(center) {
            let difference = node.bbox().measure_farthest(nodes[other].bbox());
            if difference < threshold {
                neighbors[index].insert(other, difference);
                neighbors[other].insert(index, difference);
            }
        }
        grid.insert(index, center);
    }
    neighbors
}

/// Merges nodes by the difference between their weighted mean colors.
///
/// # Arguments
/// * `nodes` - The nodes to merge, in the order of their first point.
/// * `threshold` - The difference at or above which two nodes stay apart.
/// * `difference` - The formula for the color difference between two points.
///
/// # Returns
/// The merged nodes, in the order of their first point.
fn merge_by_centroid_linkage(nodes: Vec<Node>, threshold: f64, difference: DeltaE) -> Vec<Node> {
    let mut coordinates: Vec<Vector<3>> = nodes
        .iter()
        .map(|node| difference.embed(node.mean()))
        .collect();
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
        coordinates[lower] = difference.embed(merge_pair(&mut nodes, lower, upper).mean());
        insert_centroid(lower, &coordinates, threshold, &mut grid, &mut pairs);
    }
    nodes.into_iter().flatten().collect()
}

/// Inserts a centroid into the grid after pushing its pairs closer than the threshold.
///
/// # Arguments
/// * `index` - The index of the node.
/// * `coordinates` - The coordinates of the mean color of each node.
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

/// Measures the difference between two colors from their coordinates.
///
/// # Arguments
/// * `first` - The coordinates of a color.
/// * `second` - The coordinates of another color.
///
/// # Returns
/// The difference, the same as [`DeltaE::measure`] on the colors.
fn measure(first: Vector<3>, second: Vector<3>) -> f64 {
    first.distance(second, DistanceMetric::Euclidean)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::next::extraction::{
        clustering::cluster::Cluster,
        point::Point,
        position::Position,
        test_helpers::{gray, point},
    };

    /// Creates a node of each point, embedded in OKLab.
    fn nodes(points: &[Point]) -> Vec<Node> {
        points
            .iter()
            .enumerate()
            .map(|(index, point)| Node::of_point(index, *point, DeltaE::Ok.embed(point.color())))
            .collect()
    }

    /// Merges the nodes of points and converts the results.
    fn merge(linkage: Linkage, points: &[Point], threshold: f64) -> Vec<Cluster> {
        linkage
            .merge(nodes(points), threshold, DeltaE::Ok)
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

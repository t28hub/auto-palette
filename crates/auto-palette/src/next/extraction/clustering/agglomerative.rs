use std::collections::hash_map::Entry;

use rustc_hash::FxHashMap;

use super::{cluster::Cluster, linkage::Linkage, node::Node};
use crate::next::extraction::point::Point;

/// Agglomerative hierarchical clustering, which repeatedly merges the two closest clusters.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Agglomerative {
    /// The formula for the distance between two clusters.
    linkage: Linkage,
    /// The distance at or above which two clusters stay apart.
    threshold: f64,
}

impl Agglomerative {
    /// The threshold of the first pass relative to the threshold, which only saves time.
    const FIRST_PASS_RATIO: f64 = 0.125;

    /// Creates agglomerative clustering with the given options.
    ///
    /// # Arguments
    /// * `linkage` - The formula for the distance between two clusters.
    /// * `threshold` - The distance at or above which two clusters stay apart.
    ///
    /// # Returns
    /// Agglomerative clustering, or `None` if `threshold` is not a positive normal number.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "called by the builder, which is not implemented yet"
        )
    )]
    pub(crate) fn new(linkage: Linkage, threshold: f64) -> Option<Self> {
        if !threshold.is_normal() || threshold < 0.0 {
            return None;
        }
        Some(Self { linkage, threshold })
    }

    /// Clusters points closer than the threshold.
    ///
    /// # Arguments
    /// * `points` - The points to cluster, in a low dimension.
    ///
    /// # Returns
    /// The clusters, in the order of their first point. Weightless points belong to none.
    pub(crate) fn cluster<const N: usize>(&self, points: &[Point<N>]) -> Vec<Cluster> {
        // Merging duplicates and close pairs first is what the textbook does first, only faster.
        let nodes = merge_duplicates(points);
        let first_threshold = self.threshold * Self::FIRST_PASS_RATIO;
        let nodes = self.linkage.merge(nodes, first_threshold);
        let nodes = self.linkage.merge(nodes, self.threshold);
        nodes.into_iter().map(Node::into_cluster).collect()
    }
}

/// Merges the points at the same vector into one node.
///
/// # Arguments
/// * `points` - The points to merge.
///
/// # Returns
/// The nodes of the points with weight, in the order of their first point.
fn merge_duplicates<const N: usize>(points: &[Point<N>]) -> Vec<Node<N>> {
    let mut indices: FxHashMap<[u64; N], usize> = FxHashMap::default();
    let mut nodes: Vec<Node<N>> = Vec::new();
    for (index, &point) in points.iter().enumerate() {
        if point.is_weightless() {
            continue;
        }

        let coordinates = point.vector().0;
        let bits = coordinates.map(f64::to_bits);
        match indices.entry(bits) {
            // Another point at the same coordinates came first, so this one joins its node.
            Entry::Occupied(entry) => {
                let node = &mut nodes[*entry.get()];
                node.add_point(index, point);
            }
            // The first point at these coordinates starts a node of its own.
            Entry::Vacant(entry) => {
                entry.insert(nodes.len());
                nodes.push(Node::of_point(index, point));
            }
        }
    }
    nodes
}

#[cfg(test)]
mod tests {
    use std::array;

    use rstest::rstest;

    use super::*;
    use crate::next::{
        extraction::{centroid::Centroid, clustering::random::SplitMix64},
        math::{DistanceMetric, Vector},
    };

    /// Creates agglomerative clustering.
    fn agglomerative(linkage: Linkage, threshold: f64) -> Agglomerative {
        Agglomerative::new(linkage, threshold).unwrap()
    }

    /// Creates a point on the first axis with weight 1.0.
    fn point(x: f64) -> Point<3> {
        Point::of_vector(Vector([x, 0.0, 0.0]), 1.0)
    }

    /// Creates a point on the first axis with weight 0.0.
    fn weightless(x: f64) -> Point<3> {
        Point::of_vector(Vector([x, 0.0, 0.0]), 0.0)
    }

    /// Creates points at random in a cube with edge 0.05, with random weights.
    fn random_points(count: usize) -> Vec<Point<3>> {
        let mut random = SplitMix64::new(7);
        (0..count)
            .map(|_| {
                let vector = Vector(array::from_fn(|_| 0.05 * random.next_f64()));
                Point::of_vector(vector, 0.5 + random.next_f64())
            })
            .collect()
    }

    /// Clusters points the textbook way, comparing every pair of clusters before each merge.
    fn cluster_naively(linkage: Linkage, points: &[Point<3>], threshold: f64) -> Vec<Cluster> {
        let mut clusters: Vec<Vec<usize>> = (0..points.len()).map(|index| vec![index]).collect();
        while merge_closest_naively(linkage, points, threshold, &mut clusters) {}
        clusters.into_iter().map(Cluster::new).collect()
    }

    /// Merges the closest pair of clusters, if it is under the threshold.
    fn merge_closest_naively(
        linkage: Linkage,
        points: &[Point<3>],
        threshold: f64,
        clusters: &mut Vec<Vec<usize>>,
    ) -> bool {
        // The closest pair under the threshold, the first one on a tie.
        let mut smallest = threshold;
        let mut lower = 0;
        let mut upper = 0;
        for first in 0..clusters.len() {
            for second in first + 1..clusters.len() {
                let distance =
                    measure_naively(linkage, points, &clusters[first], &clusters[second]);
                if distance < smallest {
                    smallest = distance;
                    lower = first;
                    upper = second;
                }
            }
        }
        if smallest >= threshold {
            return false;
        }

        let merged = clusters.remove(upper);
        clusters[lower].extend(merged);
        clusters[lower].sort_unstable();
        true
    }

    /// Measures the distance between two clusters the textbook way.
    fn measure_naively(
        linkage: Linkage,
        points: &[Point<3>],
        first: &[usize],
        second: &[usize],
    ) -> f64 {
        match linkage {
            Linkage::Complete => {
                let mut farthest: f64 = 0.0;
                for &member in first {
                    for &other in second {
                        let distance =
                            points[member].distance(points[other], DistanceMetric::Euclidean);
                        farthest = farthest.max(distance);
                    }
                }
                farthest
            }
            Linkage::Centroid => {
                let first_mean = find_mean(points, first);
                let second_mean = find_mean(points, second);
                first_mean.distance(second_mean, DistanceMetric::Euclidean)
            }
        }
    }

    /// Finds the weighted mean of some points.
    fn find_mean(points: &[Point<3>], members: &[usize]) -> Vector<3> {
        let mut centroid = Centroid::default();
        for &member in members {
            centroid.add(points[member]);
        }
        let mean = centroid.point().expect("a cluster has weight");
        mean.vector()
    }

    #[rstest]
    #[case::tiny(1e-300)]
    #[case::small(0.02)]
    #[case::large(1.0)]
    fn test_new_accepts_threshold(#[case] threshold: f64) {
        // Act
        let actual = Agglomerative::new(Linkage::Centroid, threshold).unwrap();

        // Assert
        assert!(matches!(actual.linkage, Linkage::Centroid));
        assert_eq!(actual.threshold, threshold);
    }

    #[rstest]
    #[case::zero(0.0)]
    #[case::subnormal(1e-310)]
    #[case::negative(-0.02)]
    #[case::nan(f64::NAN)]
    #[case::infinity(f64::INFINITY)]
    fn test_new_rejects_threshold(#[case] threshold: f64) {
        // Act
        let actual = Agglomerative::new(Linkage::Complete, threshold);

        // Assert
        assert!(actual.is_none());
    }

    #[rstest]
    #[case::complete(Linkage::Complete)]
    #[case::centroid(Linkage::Centroid)]
    fn test_cluster_matches_naive_clustering(#[case] linkage: Linkage) {
        // Arrange
        let points = random_points(150);

        // Act
        let actual = agglomerative(linkage, 0.02).cluster(&points);

        // Assert
        assert_eq!(actual, cluster_naively(linkage, &points, 0.02));
    }

    #[rstest]
    #[case::complete(Linkage::Complete)]
    #[case::centroid(Linkage::Centroid)]
    fn test_cluster_groups_points_closer_than_threshold(#[case] linkage: Linkage) {
        // Arrange
        let points = [
            point(0.0),  // 1st cluster
            point(0.5),  // 2nd cluster
            point(0.01), // 1st cluster
        ];

        // Act
        let actual = agglomerative(linkage, 0.1).cluster(&points);

        // Assert
        assert_eq!(actual, [Cluster::new(vec![0, 2]), Cluster::new(vec![1])]);
    }

    #[test]
    fn test_cluster_by_complete_breaks_ties_by_lowest_indices() {
        // Arrange: points exactly 3/128 apart, so every merge ties and the lowest indices go first.
        let points: Vec<Point<3>> = (0..12)
            .map(|step| point(f64::from(1 + 3 * step) / 128.0))
            .collect();

        // Act
        let actual = agglomerative(Linkage::Complete, 0.25).cluster(&points);

        // Assert
        assert_eq!(
            actual,
            [
                Cluster::new((0..8).collect()),
                Cluster::new((8..12).collect())
            ]
        );
    }

    #[rstest]
    #[case::complete(Linkage::Complete)]
    #[case::centroid(Linkage::Centroid)]
    fn test_cluster_merges_duplicates(#[case] linkage: Linkage) {
        // Arrange
        let points = [
            point(0.3), // 1st cluster
            point(0.0), // 2nd cluster
            point(0.3), // 1st cluster
        ];

        // Act
        let actual = agglomerative(linkage, 0.1).cluster(&points);

        // Assert
        assert_eq!(actual, [Cluster::new(vec![0, 2]), Cluster::new(vec![1])]);
    }

    #[rstest]
    #[case::complete(Linkage::Complete)]
    #[case::centroid(Linkage::Centroid)]
    fn test_cluster_groups_points_across_cube_edges(#[case] linkage: Linkage) {
        // Arrange: 0.099 and 0.101 fall in different cubes of the grids of both passes.
        let points = [point(0.099), point(0.101)];

        // Act
        let actual = agglomerative(linkage, 0.1).cluster(&points);

        // Assert
        assert_eq!(actual, [Cluster::new(vec![0, 1])]);
    }

    #[rstest]
    #[case::complete(Linkage::Complete)]
    #[case::centroid(Linkage::Centroid)]
    fn test_cluster_keeps_points_at_threshold_apart(#[case] linkage: Linkage) {
        // Arrange
        let points = [
            point(0.0), // 1st cluster
            point(0.5), // 2nd cluster
        ];

        // Act
        let actual = agglomerative(linkage, 0.5).cluster(&points);

        // Assert
        assert_eq!(actual, [Cluster::new(vec![0]), Cluster::new(vec![1])]);
    }

    #[rstest]
    #[case::complete(Linkage::Complete)]
    #[case::centroid(Linkage::Centroid)]
    fn test_cluster_keeps_points_far_from_origin_apart(#[case] linkage: Linkage) {
        // Arrange
        let points = [
            point(2e9), // 1st cluster
            point(0.0), // 2nd cluster
        ];

        // Act
        let actual = agglomerative(linkage, 1e-9).cluster(&points);

        // Assert
        assert_eq!(actual, [Cluster::new(vec![0]), Cluster::new(vec![1])]);
    }

    #[rstest]
    #[case::complete(Linkage::Complete)]
    #[case::centroid(Linkage::Centroid)]
    fn test_cluster_groups_neighbors_far_from_origin(#[case] linkage: Linkage) {
        // Arrange
        let points = [point(2e9), point(2e9_f64.next_up())];

        // Act
        let actual = agglomerative(linkage, 1e-6).cluster(&points);

        // Assert
        assert_eq!(actual, [Cluster::new(vec![0, 1])]);
    }

    #[rstest]
    #[case::complete(Linkage::Complete)]
    #[case::centroid(Linkage::Centroid)]
    fn test_cluster_groups_points_in_two_dimensions(#[case] linkage: Linkage) {
        // Arrange
        let points = [
            Point::of_vector(Vector([0.0, 0.0]), 1.0),   // 1st cluster
            Point::of_vector(Vector([0.03, 0.03]), 1.0), // 1st cluster
            Point::of_vector(Vector([0.5, 0.5]), 1.0),   // 2nd cluster
        ];

        // Act
        let actual = agglomerative(linkage, 0.1).cluster(&points);

        // Assert
        assert_eq!(actual, [Cluster::new(vec![0, 1]), Cluster::new(vec![2])]);
    }

    #[rstest]
    #[case::complete(Linkage::Complete)]
    #[case::centroid(Linkage::Centroid)]
    fn test_cluster_merges_closest_pair_first(#[case] linkage: Linkage) {
        // Arrange: merging 0.0 and 0.25 first would bring 0.35 within the threshold.
        let points = [
            point(0.0),  // 1st cluster
            point(0.25), // 2nd cluster
            point(0.35), // 2nd cluster
        ];

        // Act
        let actual = agglomerative(linkage, 0.28).cluster(&points);

        // Assert
        assert_eq!(actual, [Cluster::new(vec![0]), Cluster::new(vec![1, 2])]);
    }

    #[rstest]
    #[case::complete(Linkage::Complete)]
    #[case::centroid(Linkage::Centroid)]
    fn test_cluster_does_not_chain(#[case] linkage: Linkage) {
        // Arrange: each neighbor is within the threshold, but the ends are not.
        let points = [
            point(0.0), // 1st cluster
            point(0.3), // 1st cluster
            point(0.6), // 2nd cluster
        ];

        // Act
        let actual = agglomerative(linkage, 0.4).cluster(&points);

        // Assert
        assert_eq!(actual, [Cluster::new(vec![0, 1]), Cluster::new(vec![2])]);
    }

    #[rstest]
    #[case::complete(Linkage::Complete)]
    #[case::centroid(Linkage::Centroid)]
    fn test_cluster_sorts_members(#[case] linkage: Linkage) {
        // Arrange: 0.0 and 0.01 merge in the first pass, and 0.3 joins them later.
        let points = [point(0.0), point(0.3), point(0.01)];

        // Act
        let actual = agglomerative(linkage, 0.4).cluster(&points);

        // Assert
        assert_eq!(actual, [Cluster::new(vec![0, 1, 2])]);
    }

    #[rstest]
    #[case::complete(Linkage::Complete)]
    #[case::centroid(Linkage::Centroid)]
    fn test_cluster_leaves_out_weightless_points(#[case] linkage: Linkage) {
        // Arrange
        let points = [
            point(0.0),      // 1st cluster
            weightless(0.0), // no cluster
        ];

        // Act
        let actual = agglomerative(linkage, 0.1).cluster(&points);

        // Assert
        assert_eq!(actual, [Cluster::new(vec![0])]);
    }

    #[rstest]
    #[case::complete(Linkage::Complete)]
    #[case::centroid(Linkage::Centroid)]
    fn test_cluster_returns_no_clusters_without_points(#[case] linkage: Linkage) {
        // Act
        let actual = agglomerative(linkage, 0.1).cluster::<3>(&[]);

        // Assert
        assert!(actual.is_empty());
    }
}

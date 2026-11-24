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

    /// The smallest threshold, so that cube positions stay exact for coordinates within ±10⁶.
    const MIN_THRESHOLD: f64 = 1e-9;

    /// Creates agglomerative clustering with the given options.
    ///
    /// # Arguments
    /// * `linkage` - The formula for the distance between two clusters.
    /// * `threshold` - The distance at or above which two clusters stay apart.
    ///
    /// # Returns
    /// Agglomerative clustering, or `None` if `threshold` is below `MIN_THRESHOLD` or not finite.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "called by the builder, which is not implemented yet"
        )
    )]
    pub(crate) fn new(linkage: Linkage, threshold: f64) -> Option<Self> {
        if !threshold.is_finite() || threshold < Self::MIN_THRESHOLD {
            return None;
        }
        Some(Self { linkage, threshold })
    }

    /// Clusters points closer than the threshold.
    ///
    /// # Arguments
    /// * `points` - The points to cluster.
    ///
    /// # Returns
    /// The clusters, in the order of their first point. Weightless points belong to none.
    pub(crate) fn cluster(&self, points: &[Point<3>]) -> Vec<Cluster> {
        // Merging duplicates and close pairs first is what the textbook does first, only faster.
        let nodes = merge_duplicates(points);
        let nodes = self
            .linkage
            .merge(nodes, self.threshold * Self::FIRST_PASS_RATIO);
        self.linkage
            .merge(nodes, self.threshold)
            .into_iter()
            .map(Node::into_cluster)
            .collect()
    }
}

/// Merges the points at the same vector into one node.
///
/// # Arguments
/// * `points` - The points to merge.
///
/// # Returns
/// The nodes of the points with weight, in the order of their first point.
fn merge_duplicates(points: &[Point<3>]) -> Vec<Node> {
    let mut vectors: FxHashMap<[u64; 3], usize> = FxHashMap::default();
    let mut nodes: Vec<Node> = Vec::new();
    for (index, &point) in points.iter().enumerate() {
        if point.is_weightless() {
            continue;
        }
        match vectors.entry(point.vector().0.map(f64::to_bits)) {
            Entry::Occupied(entry) => nodes[*entry.get()].add_duplicate(index, point),
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
    use rstest::rstest;

    use super::*;
    use crate::next::{
        extraction::clustering::random::SplitMix64,
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
                let vector = Vector([(); 3].map(|()| 0.05 * random.next_f64()));
                Point::of_vector(vector, 0.5 + random.next_f64())
            })
            .collect()
    }

    /// Clusters points the textbook way, comparing every pair of clusters before each merge.
    fn cluster_naively(linkage: Linkage, points: &[Point<3>], threshold: f64) -> Vec<Cluster> {
        let mean = |members: &[usize]| {
            let mut sum = Vector::default();
            let mut weight = 0.0;
            for &index in members {
                sum += points[index].vector() * points[index].weight();
                weight += points[index].weight();
            }
            sum / weight
        };
        let measure = |first: &[usize], second: &[usize]| match linkage {
            Linkage::Complete => first
                .iter()
                .flat_map(|&i| second.iter().map(move |&j| (i, j)))
                .map(|(i, j)| points[i].distance(points[j], DistanceMetric::Euclidean))
                .fold(0.0, f64::max),
            Linkage::Centroid => mean(first).distance(mean(second), DistanceMetric::Euclidean),
        };
        let mut clusters: Vec<Vec<usize>> = (0..points.len()).map(|index| vec![index]).collect();
        loop {
            let mut closest: Option<(f64, usize, usize)> = None;
            for first in 0..clusters.len() {
                for second in first + 1..clusters.len() {
                    let distance = measure(&clusters[first], &clusters[second]);
                    let closer = closest.is_none_or(|(smallest, _, _)| distance < smallest);
                    if distance < threshold && closer {
                        closest = Some((distance, first, second));
                    }
                }
            }
            let Some((_, first, second)) = closest else {
                break;
            };
            let merged = clusters.remove(second);
            clusters[first].extend(merged);
        }
        clusters
            .into_iter()
            .map(|mut members| {
                members.sort_unstable();
                Cluster::new(members)
            })
            .collect()
    }

    #[rstest]
    #[case::minimum(Agglomerative::MIN_THRESHOLD)]
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
    #[case::below_minimum(1e-20)]
    #[case::zero(0.0)]
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
    fn test_cluster_keeps_distant_points_apart_at_minimum_threshold(#[case] linkage: Linkage) {
        // Arrange
        let points = [
            point(0.1), // 1st cluster
            point(0.9), // 2nd cluster
        ];

        // Act
        let actual = agglomerative(linkage, Agglomerative::MIN_THRESHOLD).cluster(&points);

        // Assert
        assert_eq!(actual, [Cluster::new(vec![0]), Cluster::new(vec![1])]);
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
        let actual = agglomerative(linkage, 0.1).cluster(&[]);

        // Assert
        assert!(actual.is_empty());
    }
}

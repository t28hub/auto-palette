use std::collections::hash_map::Entry;

use rustc_hash::FxHashMap;

use super::{cluster::Cluster, linkage::Linkage, node::Node};
use crate::next::{color::DeltaE, extraction::point::Point};

/// Agglomerative hierarchical clustering, which repeatedly merges the two closest clusters.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Agglomerative {
    /// How the difference between two clusters is measured.
    linkage: Linkage,
    /// The difference at or above which two clusters stay apart, in the units of `difference`.
    threshold: f64,
    /// The formula for the color difference between two points.
    difference: DeltaE,
}

impl Agglomerative {
    /// The acceptability limit of [Paravina et al.](https://doi.org/10.1111/jerd.12149) in ΔEOKr2.
    const DEFAULT_THRESHOLD: f64 = 0.02;

    /// The threshold of the first pass relative to the threshold, which only saves time.
    const FIRST_PASS_RATIO: f64 = 0.125;

    /// The smallest threshold, so that the cube positions of colors stay exact in both passes.
    const MIN_THRESHOLD: f64 = 1e-9;

    /// Creates agglomerative clustering with the given options.
    ///
    /// # Arguments
    /// * `linkage` - How the difference between two clusters is measured.
    /// * `threshold` - The difference at or above which two clusters stay apart.
    /// * `difference` - The formula for the color difference between two points.
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
    pub(crate) fn new(linkage: Linkage, threshold: f64, difference: DeltaE) -> Option<Self> {
        if !threshold.is_finite() || threshold < Self::MIN_THRESHOLD {
            return None;
        }
        Some(Self {
            linkage,
            threshold,
            difference,
        })
    }

    /// Clusters points whose colors differ by less than the threshold.
    ///
    /// # Arguments
    /// * `points` - The points to cluster.
    ///
    /// # Returns
    /// The clusters, in the order of their first point. Weightless points belong to none.
    pub(crate) fn cluster(&self, points: &[Point]) -> Vec<Cluster> {
        // Unlike the textbook method, which starts from every point at once, this first merges
        // the points of the same color, then the pairs closer than a fraction of the threshold.
        // The closest pair always merges first, so these are the merges the textbook method makes
        // first, and the clusters stay the same.
        let nodes = self.merge_same_colors(points);
        let nodes = self
            .linkage
            .merge(nodes, self.threshold * Self::FIRST_PASS_RATIO);
        self.linkage
            .merge(nodes, self.threshold)
            .into_iter()
            .map(Node::into_cluster)
            .collect()
    }

    /// Merges the points of the same color into one node.
    ///
    /// # Arguments
    /// * `points` - The points to merge.
    ///
    /// # Returns
    /// The nodes of the points with weight, in the order of their first point.
    fn merge_same_colors(&self, points: &[Point]) -> Vec<Node> {
        let mut colors: FxHashMap<[u64; 3], usize> = FxHashMap::default();
        let mut nodes: Vec<Node> = Vec::new();
        for (index, point) in points.iter().enumerate() {
            if point.is_weightless() {
                continue;
            }
            let coordinates = self.difference.embed(point.color());
            match colors.entry(coordinates.0.map(f64::to_bits)) {
                Entry::Occupied(entry) => nodes[*entry.get()].add_point(index, point.weight()),
                Entry::Vacant(entry) => {
                    entry.insert(nodes.len());
                    nodes.push(Node::of_point(index, coordinates, point.weight()));
                }
            }
        }
        nodes
    }
}

impl Default for Agglomerative {
    fn default() -> Self {
        Self {
            linkage: Linkage::default(),
            threshold: Self::DEFAULT_THRESHOLD,
            difference: DeltaE::Okr2,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::next::{
        extraction::{
            clustering::random::SplitMix64,
            position::Position,
            test_helpers::{color, gray, point, weightless},
        },
        math::{DistanceMetric, Vector},
    };

    /// Creates agglomerative clustering by the difference in OKLab.
    fn agglomerative(linkage: Linkage, threshold: f64) -> Agglomerative {
        Agglomerative::new(linkage, threshold, DeltaE::Ok).unwrap()
    }

    /// Creates points of random colors around a middle gray, with random weights.
    fn random_points(count: usize) -> Vec<Point> {
        let mut random = SplitMix64::new(7);
        (0..count)
            .map(|_| {
                let l = 0.5 + 0.05 * random.next_f64();
                let a = 0.05 * random.next_f64() - 0.025;
                let b = 0.05 * random.next_f64() - 0.025;
                let weight = 0.5 + random.next_f64();
                Point::new(color(l, a, b), Position::new(0.0, 0.0), weight)
            })
            .collect()
    }

    /// Clusters points the textbook way, comparing every pair of clusters before each merge.
    fn cluster_naively(
        linkage: Linkage,
        difference: DeltaE,
        points: &[Point],
        threshold: f64,
    ) -> Vec<Cluster> {
        let coordinates: Vec<Vector<3>> = points
            .iter()
            .map(|point| difference.embed(point.color()))
            .collect();
        let mean = |members: &[usize]| {
            let mut sum = Vector::default();
            let mut weight = 0.0;
            for &index in members {
                sum += coordinates[index] * points[index].weight();
                weight += points[index].weight();
            }
            sum / weight
        };
        let measure = |first: &[usize], second: &[usize]| match linkage {
            Linkage::Complete => first
                .iter()
                .flat_map(|&i| second.iter().map(move |&j| (i, j)))
                .map(|(i, j)| difference.measure(points[i].color(), points[j].color()))
                .fold(0.0, f64::max),
            Linkage::Centroid => mean(first).distance(mean(second), DistanceMetric::Euclidean),
        };
        let mut clusters: Vec<Vec<usize>> = (0..points.len()).map(|index| vec![index]).collect();
        loop {
            let mut closest: Option<(f64, usize, usize)> = None;
            for first in 0..clusters.len() {
                for second in first + 1..clusters.len() {
                    let difference = measure(&clusters[first], &clusters[second]);
                    let closer = closest.is_none_or(|(smallest, _, _)| difference < smallest);
                    if difference < threshold && closer {
                        closest = Some((difference, first, second));
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
        let actual = Agglomerative::new(Linkage::Centroid, threshold, DeltaE::Ok).unwrap();

        // Assert
        assert!(matches!(actual.linkage, Linkage::Centroid));
        assert_eq!(actual.threshold, threshold);
        assert!(matches!(actual.difference, DeltaE::Ok));
    }

    #[rstest]
    #[case::below_minimum(1e-20)]
    #[case::zero(0.0)]
    #[case::negative(-0.02)]
    #[case::nan(f64::NAN)]
    #[case::infinity(f64::INFINITY)]
    fn test_new_rejects_threshold(#[case] threshold: f64) {
        // Act
        let actual = Agglomerative::new(Linkage::Complete, threshold, DeltaE::Okr2);

        // Assert
        assert!(actual.is_none());
    }

    #[rstest]
    #[case::complete(Linkage::Complete)]
    #[case::centroid(Linkage::Centroid)]
    fn test_cluster_groups_colors_closer_than_threshold(#[case] linkage: Linkage) {
        // Arrange
        let points = [
            point(gray(0.0)),  // 1st cluster
            point(gray(0.5)),  // 2nd cluster
            point(gray(0.01)), // 1st cluster
        ];

        // Act
        let actual = agglomerative(linkage, 0.1).cluster(&points);

        // Assert
        assert_eq!(actual, [Cluster::new(vec![0, 2]), Cluster::new(vec![1])]);
    }

    #[rstest]
    #[case::complete_ok(Linkage::Complete, DeltaE::Ok)]
    #[case::complete_okr2(Linkage::Complete, DeltaE::Okr2)]
    #[case::centroid_ok(Linkage::Centroid, DeltaE::Ok)]
    #[case::centroid_okr2(Linkage::Centroid, DeltaE::Okr2)]
    fn test_cluster_matches_naive_clustering(#[case] linkage: Linkage, #[case] difference: DeltaE) {
        // Arrange
        let points = random_points(150);
        let agglomerative =
            Agglomerative::new(linkage, Agglomerative::DEFAULT_THRESHOLD, difference).unwrap();

        // Act
        let actual = agglomerative.cluster(&points);

        // Assert
        let expected = cluster_naively(
            linkage,
            difference,
            &points,
            Agglomerative::DEFAULT_THRESHOLD,
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_cluster_by_complete_breaks_ties_by_lowest_indices() {
        // Arrange: grays exactly 3/128 apart, so every merge ties. Taking the lowest indices first
        // in both passes ends in 0..8 and 8..12.
        let points: Vec<Point> = (0..12)
            .map(|step| point(gray(f64::from(1 + 3 * step) / 128.0)))
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
    fn test_cluster_merges_same_colors(#[case] linkage: Linkage) {
        // Arrange
        let points = [
            point(gray(0.3)), // 1st cluster
            point(gray(0.0)), // 2nd cluster
            point(gray(0.3)), // 1st cluster
        ];

        // Act
        let actual = agglomerative(linkage, 0.1).cluster(&points);

        // Assert
        assert_eq!(actual, [Cluster::new(vec![0, 2]), Cluster::new(vec![1])]);
    }

    #[rstest]
    #[case::complete(Linkage::Complete)]
    #[case::centroid(Linkage::Centroid)]
    fn test_cluster_groups_colors_across_cube_edges(#[case] linkage: Linkage) {
        // Arrange: 0.099 and 0.101 fall in different cubes of the grids of both passes.
        let points = [point(gray(0.099)), point(gray(0.101))];

        // Act
        let actual = agglomerative(linkage, 0.1).cluster(&points);

        // Assert
        assert_eq!(actual, [Cluster::new(vec![0, 1])]);
    }

    #[rstest]
    #[case::complete(Linkage::Complete)]
    #[case::centroid(Linkage::Centroid)]
    fn test_cluster_keeps_colors_at_threshold_apart(#[case] linkage: Linkage) {
        // Arrange
        let points = [
            point(gray(0.0)), // 1st cluster
            point(gray(0.5)), // 2nd cluster
        ];

        // Act
        let actual = agglomerative(linkage, 0.5).cluster(&points);

        // Assert
        assert_eq!(actual, [Cluster::new(vec![0]), Cluster::new(vec![1])]);
    }

    #[rstest]
    #[case::complete(Linkage::Complete)]
    #[case::centroid(Linkage::Centroid)]
    fn test_cluster_keeps_distant_colors_apart_at_minimum_threshold(#[case] linkage: Linkage) {
        // Arrange
        let points = [
            point(gray(0.1)), // 1st cluster
            point(gray(0.9)), // 2nd cluster
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
            point(gray(0.0)),  // 1st cluster
            point(gray(0.25)), // 2nd cluster
            point(gray(0.35)), // 2nd cluster
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
            point(gray(0.0)), // 1st cluster
            point(gray(0.3)), // 1st cluster
            point(gray(0.6)), // 2nd cluster
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
        let points = [point(gray(0.0)), point(gray(0.3)), point(gray(0.01))];

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
            point(gray(0.0)),      // 1st cluster
            weightless(gray(0.0)), // no cluster
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

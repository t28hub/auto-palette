use std::collections::hash_map::Entry;

use rustc_hash::FxHashMap;

use super::{
    cluster::Cluster,
    grid::{locate_cube, Cube},
    linkage::Linkage,
    node::Node,
};
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

    /// The cube edge in `merge_cubes` relative to the threshold, so a diagonal is under a quarter.
    const CUBE_RATIO: f64 = 0.125;

    /// The smallest threshold, so that the cube positions of colors stay exact integers.
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
        let nodes = self.merge_cubes(points);
        self.linkage
            .merge(nodes, self.threshold, self.difference)
            .into_iter()
            .map(Node::into_cluster)
            .collect()
    }

    /// Merges the points in each cube of a fine grid into one node.
    ///
    /// # Arguments
    /// * `points` - The points to merge.
    ///
    /// # Returns
    /// The nodes of the points with weight, in the order of their first point.
    fn merge_cubes(&self, points: &[Point]) -> Vec<Node> {
        let size = self.threshold * Self::CUBE_RATIO;
        let mut cubes: FxHashMap<Cube, usize> = FxHashMap::default();
        let mut nodes: Vec<Node> = Vec::new();
        for (index, point) in points.iter().enumerate() {
            if point.is_weightless() {
                continue;
            }
            let coordinates = self.difference.embed(point.color());
            let node = Node::of_point(index, *point, coordinates);
            match cubes.entry(locate_cube(coordinates, size)) {
                Entry::Occupied(entry) => nodes[*entry.get()].absorb(node),
                Entry::Vacant(entry) => {
                    entry.insert(nodes.len());
                    nodes.push(node);
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
    use crate::next::extraction::test_helpers::{gray, point, weightless};

    /// Creates agglomerative clustering by the difference in OKLab.
    fn agglomerative(linkage: Linkage, threshold: f64) -> Agglomerative {
        Agglomerative::new(linkage, threshold, DeltaE::Ok).unwrap()
    }

    #[rstest]
    #[case::minimum(Agglomerative::MIN_THRESHOLD)]
    #[case::small(0.02)]
    #[case::large(1.0)]
    fn test_new_accepts_threshold(#[case] threshold: f64) {
        // Act
        let actual = Agglomerative::new(Linkage::Complete, threshold, DeltaE::Okr2);

        // Assert
        assert!(actual.is_some());
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
        let points = [point(gray(0.0)), point(gray(0.5)), point(gray(0.01))];

        // Act
        let actual = agglomerative(linkage, 0.1).cluster(&points);

        // Assert
        assert_eq!(actual, [Cluster::new(vec![0, 2]), Cluster::new(vec![1])]);
    }

    #[rstest]
    #[case::complete(Linkage::Complete)]
    #[case::centroid(Linkage::Centroid)]
    fn test_cluster_groups_colors_across_cube_edges(#[case] linkage: Linkage) {
        // Arrange: 0.099 and 0.101 fall in different cubes of both grids.
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
        let points = [point(gray(0.0)), point(gray(0.5))];

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
        let points = [point(gray(0.1)), point(gray(0.9))];

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
        let points = [point(gray(0.0)), point(gray(0.25)), point(gray(0.35))];

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
        let points = [point(gray(0.0)), point(gray(0.3)), point(gray(0.6))];

        // Act
        let actual = agglomerative(linkage, 0.4).cluster(&points);

        // Assert
        assert_eq!(actual, [Cluster::new(vec![0, 1]), Cluster::new(vec![2])]);
    }

    #[rstest]
    #[case::complete(Linkage::Complete)]
    #[case::centroid(Linkage::Centroid)]
    fn test_cluster_sorts_members(#[case] linkage: Linkage) {
        // Arrange: 0.0 and 0.01 share a cube, and 0.3 joins them later.
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
        let points = [point(gray(0.0)), weightless(gray(0.0))];

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

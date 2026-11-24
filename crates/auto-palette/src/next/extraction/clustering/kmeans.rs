use std::num::NonZeroU32;

use super::{cluster::Cluster, initialization::Initialization, lloyd::Lloyd};
use crate::next::extraction::point::Point;

/// K-means, which picks initial centers and then moves them with Lloyd's algorithm.
#[derive(Debug, Clone, Copy)]
pub(crate) struct KMeans {
    /// The number of clusters to aim for.
    count: NonZeroU32,
    /// The method that picks the initial centers.
    initialization: Initialization,
    /// The algorithm that moves the centers until they settle.
    lloyd: Lloyd,
}

impl KMeans {
    /// Creates k-means with the given options.
    ///
    /// # Arguments
    /// * `count` - The number of clusters to aim for.
    /// * `initialization` - The method that picks the initial centers.
    /// * `lloyd` - The algorithm that moves the centers until they settle.
    ///
    /// # Returns
    /// K-means with the given options.
    #[inline]
    pub(crate) const fn new(
        count: NonZeroU32,
        initialization: Initialization,
        lloyd: Lloyd,
    ) -> Self {
        Self {
            count,
            initialization,
            lloyd,
        }
    }

    /// Returns the number of clusters to aim for.
    ///
    /// # Returns
    /// The number of clusters to aim for.
    #[inline]
    pub(crate) const fn count(&self) -> NonZeroU32 {
        self.count
    }

    /// Clusters points.
    ///
    /// # Arguments
    /// * `points` - The points to cluster.
    ///
    /// # Returns
    /// The clusters that have points, in a fixed order. Weightless points belong to none.
    pub(crate) fn cluster<const N: usize>(&self, points: &[Point<N>]) -> Vec<Cluster> {
        if points.iter().all(|point| point.is_weightless()) {
            return Vec::new();
        }
        let count = self.count.get() as usize;
        // Weightless points never become centers.
        let mut centers = self.initialization.centers(points, count);
        let assigned = self.lloyd.move_centers(points, &mut centers);
        group(points, &assigned, centers.len())
    }
}

/// Groups the points with weight by their centers, leaving out the centers without points.
///
/// # Arguments
/// * `points` - The points that were clustered.
/// * `assigned` - The center of each point.
/// * `count` - The number of centers.
///
/// # Returns
/// The cluster of each center that has points, in center order.
fn group<const N: usize>(points: &[Point<N>], assigned: &[usize], count: usize) -> Vec<Cluster> {
    let mut members = vec![Vec::new(); count];
    for (index, (point, &center)) in points.iter().zip(assigned).enumerate() {
        if !point.is_weightless() {
            members[center].push(index);
        }
    }
    members
        .into_iter()
        .filter(|members| !members.is_empty())
        .map(Cluster::new)
        .collect()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::next::extraction::test_helpers::{point, weightless, BLACK, WHITE};

    #[rstest]
    #[case::empty(vec![])]
    #[case::weightless(vec![weightless(BLACK), weightless(WHITE)])]
    fn test_cluster_returns_no_clusters_without_weight(#[case] points: Vec<Point<5>>) {
        // Arrange
        let kmeans = KMeans::new(NonZeroU32::MIN, Initialization::Farthest, Lloyd::default());

        // Act
        let actual = kmeans.cluster(&points);

        // Assert
        assert!(actual.is_empty());
    }

    #[test]
    fn test_cluster_groups_similar_colors() {
        // Arrange
        let points = [point(BLACK), point(WHITE), point(BLACK)];
        let kmeans = KMeans::new(
            NonZeroU32::new(2).unwrap(),
            Initialization::Farthest,
            Lloyd::default(),
        );

        // Act
        let actual = kmeans.cluster(&points);

        // Assert
        assert_eq!(actual, [Cluster::new(vec![0, 2]), Cluster::new(vec![1])]);
    }

    #[test]
    fn test_group_leaves_out_weightless_points() {
        // Arrange
        let points = [point(BLACK), weightless(BLACK)];

        // Act
        let actual = group(&points, &[0, 0], 1);

        // Assert
        assert_eq!(actual, [Cluster::new(vec![0])]);
    }

    #[test]
    fn test_group_leaves_out_centers_without_points() {
        // Arrange: no point is assigned to center 1.
        let points = [point(BLACK), point(WHITE)];

        // Act
        let actual = group(&points, &[2, 0], 3);

        // Assert
        assert_eq!(actual, [Cluster::new(vec![1]), Cluster::new(vec![0])]);
    }
}

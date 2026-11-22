use super::{cluster::Cluster, kmeans::KMeans};
use crate::next::extraction::point::Point;

/// The clustering algorithm to use.
#[derive(Debug, Clone, Copy)]
pub(crate) enum ClusteringAlgorithm {
    /// K-means ([`KMeans`]).
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "selected by the candidates, which are not implemented yet"
        )
    )]
    KMeans(KMeans),
}

impl ClusteringAlgorithm {
    /// Clusters points.
    ///
    /// # Arguments
    /// * `points` - The points to cluster.
    ///
    /// # Returns
    /// The clusters that have points, in a fixed order. Weightless points belong to none.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "called by the candidates, which are not implemented yet"
        )
    )]
    pub(crate) fn cluster(&self, points: &[Point]) -> Vec<Cluster> {
        match self {
            Self::KMeans(kmeans) => kmeans.cluster(points),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use super::*;
    use crate::next::extraction::{
        clustering::{Initialization, Lloyd},
        test_helpers::{point, BLACK, WHITE},
    };

    #[test]
    fn test_cluster() {
        // Arrange
        let points = [point(BLACK), point(WHITE)];
        let kmeans = KMeans::new(
            NonZeroU32::new(2).unwrap(),
            Initialization::default(),
            Lloyd::default(),
        );

        // Act
        let actual = ClusteringAlgorithm::KMeans(kmeans).cluster(&points);

        // Assert
        assert_eq!(actual, kmeans.cluster(&points));
    }
}

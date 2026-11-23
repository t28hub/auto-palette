use super::{agglomerative::Agglomerative, cluster::Cluster, kmeans::KMeans};
use crate::next::extraction::point::Point;

/// The clustering algorithm to use.
#[derive(Debug, Clone, Copy)]
pub(crate) enum ClusteringAlgorithm {
    /// K-means ([`KMeans`]), by color and position.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "selected by the builder, which is not implemented yet"
        )
    )]
    KMeans(KMeans),
    /// Agglomerative hierarchical clustering ([`Agglomerative`]), by color alone.
    Agglomerative(Agglomerative),
}

impl Default for ClusteringAlgorithm {
    fn default() -> Self {
        Self::Agglomerative(Agglomerative::default())
    }
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
            Self::Agglomerative(agglomerative) => agglomerative.cluster(points),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use rstest::rstest;

    use super::*;
    use crate::next::extraction::{
        clustering::{Initialization, Lloyd},
        test_helpers::{point, BLACK, WHITE},
    };

    /// A black point and a white point.
    fn points() -> [Point; 2] {
        [point(BLACK), point(WHITE)]
    }

    /// Creates k-means into two clusters.
    fn kmeans() -> KMeans {
        KMeans::new(
            NonZeroU32::new(2).unwrap(),
            Initialization::default(),
            Lloyd::default(),
        )
    }

    #[rstest]
    #[case::kmeans(ClusteringAlgorithm::KMeans(kmeans()), kmeans().cluster(&points()))]
    #[case::agglomerative(
        ClusteringAlgorithm::Agglomerative(Agglomerative::default()),
        Agglomerative::default().cluster(&points())
    )]
    fn test_cluster(#[case] clustering: ClusteringAlgorithm, #[case] expected: Vec<Cluster>) {
        // Act
        let actual = clustering.cluster(&points());

        // Assert
        assert_eq!(actual, expected);
    }
}

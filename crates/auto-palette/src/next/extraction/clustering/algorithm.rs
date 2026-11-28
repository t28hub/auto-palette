use super::{agglomerative::Agglomerative, cluster::Cluster, kmeans::KMeans};
use crate::next::extraction::point::Point;

/// The clustering algorithm to use.
#[derive(Debug, Clone, Copy)]
pub(crate) enum ClusteringAlgorithm {
    /// Agglomerative hierarchical clustering ([`Agglomerative`]), up to a distance threshold.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "selected by the builder, which is not implemented yet"
        )
    )]
    Agglomerative(Agglomerative),
    /// K-means ([`KMeans`]), into a fixed number of clusters.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "selected by the builder, which is not implemented yet"
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
    pub(crate) fn cluster<const N: usize>(&self, points: &[Point<N>]) -> Vec<Cluster> {
        match self {
            Self::Agglomerative(agglomerative) => agglomerative.cluster(points),
            Self::KMeans(kmeans) => kmeans.cluster(points),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use rstest::rstest;

    use super::*;
    use crate::next::{
        extraction::clustering::{linkage::Linkage, Initialization, Lloyd},
        math::Vector,
    };

    /// Two points a unit apart, in five dimensions.
    fn points() -> [Point<5>; 2] {
        [
            Point::of_vector(Vector([0.0; 5]), 1.0),
            Point::of_vector(Vector([1.0, 0.0, 0.0, 0.0, 0.0]), 1.0),
        ]
    }

    /// Creates agglomerative clustering by complete linkage.
    fn agglomerative() -> Agglomerative {
        Agglomerative::new(Linkage::Complete, 0.02).unwrap()
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
    #[case::agglomerative(
        ClusteringAlgorithm::Agglomerative(agglomerative()),
        agglomerative().cluster(&points())
    )]
    #[case::kmeans(ClusteringAlgorithm::KMeans(kmeans()), kmeans().cluster(&points()))]
    fn test_cluster(#[case] clustering: ClusteringAlgorithm, #[case] expected: Vec<Cluster>) {
        // Act
        let actual = clustering.cluster(&points());

        // Assert
        assert_eq!(actual, expected);
    }
}

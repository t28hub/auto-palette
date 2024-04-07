use crate::errors::PaletteError;
use crate::math::clustering::dbscan::DBSCAN;
use crate::math::clustering::kmeans::InitializationStrategy::KmeansPlusPlus;
use crate::math::clustering::kmeans::KMeans;
use crate::math::clustering::{Cluster, ClusteringAlgorithm};
use crate::math::{DistanceMetric, Point};
use rand::thread_rng;
use std::str::FromStr;

/// Clustering algorithm.
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, PartialEq)]
pub enum Algorithm {
    /// K-means clustering algorithm.
    KMeans,
    /// Density-based spatial clustering of applications with noise (DBSCAN) algorithm.
    DBSCAN,
}

impl Algorithm {
    /// Clusters the points using the specified algorithm.
    ///
    /// # Type Parameters
    /// * `N` - The number of dimensions.
    ///
    /// # Arguments
    /// * `points` - The points to cluster.
    ///
    /// # Returns
    /// The clusters of the points using the specified algorithm.
    #[must_use]
    pub(crate) fn cluster<const N: usize>(&self, points: &[Point<N>]) -> Vec<Cluster<N>> {
        match self {
            Self::KMeans => {
                let strategy = KmeansPlusPlus(thread_rng(), DistanceMetric::SquaredEuclidean);
                let clustering =
                    KMeans::new(16, 100, 1e-3, DistanceMetric::SquaredEuclidean, strategy).unwrap();
                clustering.fit(points)
            }
            Self::DBSCAN => {
                let clustering = DBSCAN::new(16, 2.5, DistanceMetric::Euclidean).unwrap();
                clustering.fit(points)
            }
        }
    }
}

impl FromStr for Algorithm {
    type Err = PaletteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "kmeans" => Ok(Self::KMeans),
            "dbscan" => Ok(Self::DBSCAN),
            _ => Err(PaletteError::InvalidAlgorithm),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algorithm_from_str() {
        assert_eq!(Algorithm::from_str("kmeans").unwrap(), Algorithm::KMeans);
        assert_eq!(Algorithm::from_str("dbscan").unwrap(), Algorithm::DBSCAN);
        assert_eq!(
            Algorithm::from_str("foo").unwrap_err(),
            PaletteError::InvalidAlgorithm
        );
    }
}

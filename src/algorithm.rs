use crate::errors::PaletteError;
use crate::math::clustering::{
    Cluster, ClusteringAlgorithm, DBSCANpp, InitializationStrategy, KMeans, DBSCAN,
};
use crate::math::{DistanceMetric, FloatNumber, Point};
use rand::thread_rng;
use std::str::FromStr;

/// Clustering algorithm.
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, PartialEq)]
pub enum Algorithm {
    /// K-means clustering algorithm.
    KMeans,
    /// DBSCAN clustering algorithm.
    DBSCAN,
    /// DBSCAN++ clustering algorithm.
    DBSCANpp,
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
    pub(crate) fn cluster<T, const N: usize>(&self, points: &[Point<T, N>]) -> Vec<Cluster<T, N>>
    where
        T: FloatNumber,
    {
        match self {
            Self::KMeans => {
                let strategy = InitializationStrategy::KmeansPlusPlus(
                    thread_rng(),
                    DistanceMetric::SquaredEuclidean,
                );
                let clustering = KMeans::new(
                    32,
                    100,
                    T::from_f32(1e-3),
                    DistanceMetric::SquaredEuclidean,
                    strategy,
                )
                .unwrap();
                clustering.fit(points)
            }
            Self::DBSCAN => {
                let clustering =
                    DBSCAN::new(16, T::from_f32(16e-4), DistanceMetric::SquaredEuclidean).unwrap();
                clustering.fit(points)
            }
            Self::DBSCANpp => {
                let clustering = DBSCANpp::new(
                    T::from_f32(0.1),
                    16,
                    T::from_f32(16e-4),
                    DistanceMetric::SquaredEuclidean,
                )
                .unwrap();
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
            "dbscan++" => Ok(Self::DBSCANpp),
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
            Algorithm::from_str("dbscan++").unwrap(),
            Algorithm::DBSCANpp
        );
        assert_eq!(
            Algorithm::from_str("foo").unwrap_err(),
            PaletteError::InvalidAlgorithm
        );
    }
}

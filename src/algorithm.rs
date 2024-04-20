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
        match s.to_lowercase().as_str() {
            "kmeans" => Ok(Self::KMeans),
            "dbscan" => Ok(Self::DBSCAN),
            "dbscan++" => Ok(Self::DBSCANpp),
            _ => Err(PaletteError::InvalidAlgorithm(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::kmeans("kmeans", Algorithm::KMeans)]
    #[case::dbscan("dbscan", Algorithm::DBSCAN)]
    #[case::dbscanpp("dbscan++", Algorithm::DBSCANpp)]
    #[case::kmeans_upper("KMEANS", Algorithm::KMeans)]
    #[case::dbscan_upper("DBSCAN", Algorithm::DBSCAN)]
    #[case::dbscanpp_upper("DBSCAN++", Algorithm::DBSCANpp)]
    #[case::kmeans_capitalized("Kmeans", Algorithm::KMeans)]
    #[case::dbscan_capitalized("Dbscan", Algorithm::DBSCAN)]
    #[case::dbscanpp_capitalized("Dbscan++", Algorithm::DBSCANpp)]
    fn test_from_str(#[case] input: &str, #[case] expected: Algorithm) {
        // Act
        let actual = Algorithm::from_str(input).unwrap();

        // Assert
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case::empty("")]
    #[case::invalid("invalid")]
    fn test_from_str_error(#[case] input: &str) {
        // Act
        let actual = Algorithm::from_str(input);

        // Assert
        assert!(actual.is_err());
        assert_eq!(
            actual.unwrap_err(),
            PaletteError::InvalidAlgorithm(input.to_string())
        );
    }
}

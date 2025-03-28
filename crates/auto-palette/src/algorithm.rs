use std::str::FromStr;

use rand_distr::weighted::AliasableWeight;

use crate::{
    error::Error,
    math::{
        clustering::{Cluster, ClusteringAlgorithm, DBSCANPlusPlus, KMeans, DBSCAN},
        DistanceMetric,
        FloatNumber,
        Point,
    },
};

/// The clustering algorithm to use for color palette extraction.
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Default, Clone, PartialEq)]
pub enum Algorithm {
    /// K-means clustering algorithm.
    KMeans,
    /// DBSCAN clustering algorithm.
    #[default]
    DBSCAN,
    /// DBSCAN++ clustering algorithm.
    DBSCANpp,
}

impl Algorithm {
    /// Clusters the given pixels using the algorithm.
    ///
    /// # Arguments
    /// * `pixels` - The pixels to cluster.
    ///
    /// # Returns
    /// The clusters found by the algorithm.
    #[must_use]
    pub(crate) fn cluster<T>(&self, pixels: &[Point<T, 5>]) -> Vec<Cluster<T, 5>>
    where
        T: FloatNumber + AliasableWeight,
    {
        match self {
            Self::KMeans => cluster_with_kmeans(pixels),
            Self::DBSCAN => cluster_with_dbscan(pixels),
            Self::DBSCANpp => cluster_with_dbscanpp(pixels),
        }
    }
}

impl FromStr for Algorithm {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "kmeans" => Ok(Self::KMeans),
            "dbscan" => Ok(Self::DBSCAN),
            "dbscan++" => Ok(Self::DBSCANpp),
            _ => Err(Error::UnsupportedAlgorithm {
                name: s.to_string(),
            }),
        }
    }
}

fn cluster_with_kmeans<T>(pixels: &[Point<T, 5>]) -> Vec<Cluster<T, 5>>
where
    T: FloatNumber + AliasableWeight,
{
    let clustering = KMeans::new(
        32,
        100,
        T::from_f32(1e-3),
        DistanceMetric::SquaredEuclidean,
        rand::rng(),
    )
    .unwrap();
    clustering.fit(pixels)
}

#[must_use]
fn cluster_with_dbscan<T>(pixels: &[Point<T, 5>]) -> Vec<Cluster<T, 5>>
where
    T: FloatNumber,
{
    let clustering = DBSCAN::new(16, T::from_f32(16e-4), DistanceMetric::SquaredEuclidean).unwrap();
    clustering.fit(pixels)
}

#[must_use]
fn cluster_with_dbscanpp<T>(pixels: &[Point<T, 5>]) -> Vec<Cluster<T, 5>>
where
    T: FloatNumber,
{
    let clustering = DBSCANPlusPlus::new(
        T::from_f32(0.1),
        16,
        T::from_f32(16e-4),
        DistanceMetric::SquaredEuclidean,
    )
    .unwrap();
    clustering.fit(pixels)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

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
    #[case::invalid("unknown")]
    fn test_from_str_error(#[case] input: &str) {
        // Act
        let actual = Algorithm::from_str(input);

        // Assert
        assert!(actual.is_err());
        assert_eq!(
            actual.unwrap_err().to_string(),
            format!("Unsupported algorithm specified: '{}'", input)
        );
    }
}

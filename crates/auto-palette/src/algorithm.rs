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
    pub(crate) fn cluster<T>(&self, pixels: &[Point<T, 5>]) -> Result<Vec<Cluster<T, 5>>, Error>
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

const KMEANS_CLUSTER_COUNT: usize = 32;
const KMEANS_MAX_ITER: usize = 100;
const KMEANS_TOLERANCE: f64 = 1e-3;

fn cluster_with_kmeans<T>(pixels: &[Point<T, 5>]) -> Result<Vec<Cluster<T, 5>>, Error>
where
    T: FloatNumber + AliasableWeight,
{
    let clustering = KMeans::new(
        KMEANS_CLUSTER_COUNT,
        KMEANS_MAX_ITER,
        T::from_f64(KMEANS_TOLERANCE),
        DistanceMetric::SquaredEuclidean,
        rand::rng(),
    )
    .map_err(|e| Error::PaletteExtractionError {
        details: e.to_string(),
    })?;
    clustering
        .fit(pixels)
        .map_err(|e| Error::PaletteExtractionError {
            details: e.to_string(),
        })
}

const DBSCAN_MIN_POINTS: usize = 16;
const DBSCAN_EPSILON: f64 = 16e-4;

fn cluster_with_dbscan<T>(pixels: &[Point<T, 5>]) -> Result<Vec<Cluster<T, 5>>, Error>
where
    T: FloatNumber,
{
    let clustering = DBSCAN::new(
        DBSCAN_MIN_POINTS,
        T::from_f64(DBSCAN_EPSILON),
        DistanceMetric::SquaredEuclidean,
    )
    .map_err(|e| Error::PaletteExtractionError {
        details: e.to_string(),
    })?;
    clustering
        .fit(pixels)
        .map_err(|e| Error::PaletteExtractionError {
            details: e.to_string(),
        })
}

const DBSCANPP_PROBABILITY: f64 = 0.1;
const DBSCANPP_MIN_POINTS: usize = 16;
const DBSCANPP_EPSILON: f64 = 16e-4;

fn cluster_with_dbscanpp<T>(pixels: &[Point<T, 5>]) -> Result<Vec<Cluster<T, 5>>, Error>
where
    T: FloatNumber,
{
    let clustering = DBSCANPlusPlus::new(
        T::from_f64(DBSCANPP_PROBABILITY),
        DBSCANPP_MIN_POINTS,
        T::from_f64(DBSCANPP_EPSILON),
        DistanceMetric::SquaredEuclidean,
    )
    .map_err(|e| Error::PaletteExtractionError {
        details: e.to_string(),
    })?;
    clustering
        .fit(pixels)
        .map_err(|e| Error::PaletteExtractionError {
            details: e.to_string(),
        })
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

    #[test]
    fn test_cluster_with_dbscan_empty() {
        // Arrange
        let pixels: Vec<Point<f32, 5>> = vec![];

        // Act
        let actual = cluster_with_dbscan(&pixels);

        // Assert
        assert!(actual.is_err());
    }

    #[test]
    fn test_cluster_with_dbscanpp_empty() {
        // Arrange
        let pixels: Vec<Point<f32, 5>> = vec![];

        // Act
        let actual = cluster_with_dbscanpp(&pixels);

        // Assert
        assert!(actual.is_err());
    }

    #[test]
    fn test_cluster_with_kmeans_empty() {
        // Arrange
        let pixels: Vec<Point<f32, 5>> = vec![];

        // Act
        let actual = cluster_with_kmeans(&pixels);

        // Assert
        assert!(actual.is_err());
    }
}

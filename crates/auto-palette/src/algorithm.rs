use std::{fmt::Display, str::FromStr};

use rand_distr::weighted::AliasableWeight;

use crate::{
    error::Error,
    math::{
        clustering::{Cluster, ClusteringAlgorithm, DBSCANPlusPlus, KMeans, DBSCAN, SLIC, SNIC},
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

    /// SLIC algorithm.
    SLIC,

    /// SNIC algorithm.
    SNIC,
}

impl Algorithm {
    /// Clusters the given pixels using the algorithm.
    ///
    /// # Arguments
    /// * `width` - The width of the image.
    /// * `height` - The height of the image.
    /// * `pixels` - The pixels to cluster.
    ///
    /// # Returns
    /// The clusters found by the algorithm.
    pub(crate) fn cluster<T>(
        &self,
        width: u32,
        height: u32,
        pixels: &[Point<T, 5>],
    ) -> Result<Vec<Cluster<T, 5>>, Error>
    where
        T: FloatNumber + AliasableWeight,
    {
        match self {
            Self::KMeans => cluster_with_kmeans(pixels),
            Self::DBSCAN => cluster_with_dbscan(pixels),
            Self::DBSCANpp => cluster_with_dbscanpp(pixels),
            Self::SLIC => cluster_with_slic(width as usize, height as usize, pixels),
            Self::SNIC => cluster_with_snic(width as usize, height as usize, pixels),
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
            "slic" => Ok(Self::SLIC),
            "snic" => Ok(Self::SNIC),
            _ => Err(Error::UnsupportedAlgorithm {
                name: s.to_string(),
            }),
        }
    }
}

impl Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KMeans => write!(f, "kmeans"),
            Self::DBSCAN => write!(f, "dbscan"),
            Self::DBSCANpp => write!(f, "dbscan++"),
            Self::SLIC => write!(f, "slic"),
            Self::SNIC => write!(f, "snic"),
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

const SLIC_SEGMENTS: usize = 128;
const SLIC_COMPACTNESS: f64 = 0.0225; // 0.15^2
const SLIC_MAX_ITER: usize = 10;
const SLIC_TOLERANCE: f64 = 1e-3;

fn cluster_with_slic<T>(
    width: usize,
    height: usize,
    pixels: &[Point<T, 5>],
) -> Result<Vec<Cluster<T, 5>>, Error>
where
    T: FloatNumber,
{
    let clustering = SLIC::new(
        (width, height),
        SLIC_SEGMENTS,
        T::from_f64(SLIC_COMPACTNESS),
        SLIC_MAX_ITER,
        T::from_f64(SLIC_TOLERANCE),
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

const SNIC_SEGMENTS: usize = 128;

fn cluster_with_snic<T>(
    width: usize,
    height: usize,
    pixels: &[Point<T, 5>],
) -> Result<Vec<Cluster<T, 5>>, Error>
where
    T: FloatNumber,
{
    let clustering = SNIC::new(
        (width, height),
        SNIC_SEGMENTS,
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
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use rstest::rstest;

    use super::*;

    #[must_use]
    fn empty_points() -> Vec<Point<f64, 5>> {
        Vec::new()
    }

    #[rstest]
    #[case::kmeans("kmeans", Algorithm::KMeans)]
    #[case::dbscan("dbscan", Algorithm::DBSCAN)]
    #[case::dbscanpp("dbscan++", Algorithm::DBSCANpp)]
    #[case::slic("slic", Algorithm::SLIC)]
    #[case::kmeans_upper("KMEANS", Algorithm::KMeans)]
    #[case::dbscan_upper("DBSCAN", Algorithm::DBSCAN)]
    #[case::dbscanpp_upper("DBSCAN++", Algorithm::DBSCANpp)]
    #[case::slic_upper("SLIC", Algorithm::SLIC)]
    #[case::kmeans_capitalized("Kmeans", Algorithm::KMeans)]
    #[case::dbscan_capitalized("Dbscan", Algorithm::DBSCAN)]
    #[case::dbscanpp_capitalized("Dbscan++", Algorithm::DBSCANpp)]
    #[case::slic_capitalized("Slic", Algorithm::SLIC)]
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

    #[rstest]
    #[case::kmeans(Algorithm::KMeans, "kmeans")]
    #[case::dbscan(Algorithm::DBSCAN, "dbscan")]
    #[case::dbscanpp(Algorithm::DBSCANpp, "dbscan++")]
    #[case::slic(Algorithm::SLIC, "slic")]
    #[case::snic(Algorithm::SNIC, "snic")]
    fn test_fmt(#[case] algorithm: Algorithm, #[case] expected: &str) {
        // Act
        let actual = format!("{}", algorithm);

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_cluster_with_dbscan_empty() {
        // Arrange
        let pixels = empty_points();

        // Act
        let actual = cluster_with_dbscan(&pixels);

        // Assert
        assert!(actual.is_err());
    }

    #[test]
    fn test_cluster_with_dbscanpp_empty() {
        // Arrange
        let pixels = empty_points();

        // Act
        let actual = cluster_with_dbscanpp(&pixels);

        // Assert
        assert!(actual.is_err());
    }

    #[test]
    fn test_cluster_with_kmeans_empty() {
        // Arrange
        let pixels = empty_points();

        // Act
        let actual = cluster_with_kmeans(&pixels);

        // Assert
        assert!(actual.is_err());
    }

    #[test]
    fn test_cluster_with_slic_empty() {
        // Arrange
        let pixels = empty_points();

        // Act
        let actual = cluster_with_slic(0, 0, &pixels);

        // Assert
        assert!(actual.is_err());
    }
}

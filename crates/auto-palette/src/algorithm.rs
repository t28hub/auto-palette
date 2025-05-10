use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use crate::{
    error::Error,
    image::{
        segmentation::{
            KmeansSegmentation,
            Segment,
            Segmentation,
            SlicSegmentation,
            SnicSegmentation,
        },
        Pixel,
    },
    math::{
        clustering::{ClusteringAlgorithm, DBSCANPlusPlus, DBSCAN},
        DistanceMetric,
        FloatNumber,
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
    /// * `width` - The width of the image.
    /// * `height` - The height of the image.
    /// * `pixels` - The pixels to cluster.
    ///
    /// # Returns
    /// The clusters found by the algorithm.
    pub(crate) fn segment<T>(
        &self,
        width: u32,
        height: u32,
        pixels: &[Pixel<T>],
    ) -> Result<Vec<Segment<T>>, Error>
    where
        T: FloatNumber,
    {
        match self {
            Self::KMeans => kmeans(width as usize, height as usize, pixels),
            Self::DBSCAN => dbscan(pixels),
            Self::DBSCANpp => dbscanpp(pixels),
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

impl Display for Algorithm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KMeans => write!(f, "kmeans"),
            Self::DBSCAN => write!(f, "dbscan"),
            Self::DBSCANpp => write!(f, "dbscan++"),
        }
    }
}

const KMEANS_CLUSTER_COUNT: usize = 128;
const KMEANS_MAX_ITER: usize = 50;
const KMEANS_TOLERANCE: f64 = 1e-3;

fn kmeans<T>(width: usize, height: usize, pixels: &[Pixel<T>]) -> Result<Vec<Segment<T>>, Error>
where
    T: FloatNumber,
{
    let segmentation = KmeansSegmentation::builder()
        .segments(KMEANS_CLUSTER_COUNT)
        .max_iter(KMEANS_MAX_ITER)
        .tolerance(T::from_f64(KMEANS_TOLERANCE))
        .metric(DistanceMetric::SquaredEuclidean)
        .build()
        .map_err(|e| Error::PaletteExtractionError {
            details: e.to_string(),
        })?;

    segmentation
        .segment(width, height, pixels)
        .map_err(|e| Error::PaletteExtractionError {
            details: e.to_string(),
        })
}

const DBSCAN_MIN_POINTS: usize = 16;
const DBSCAN_EPSILON: f64 = 16e-4;

fn dbscan<T>(pixels: &[Pixel<T>]) -> Result<Vec<Segment<T>>, Error>
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

    let clusters = clustering
        .fit(pixels)
        .map_err(|e| Error::PaletteExtractionError {
            details: e.to_string(),
        })?;

    let segments = clusters
        .iter()
        .filter_map(|cluster| {
            if cluster.is_empty() {
                None
            } else {
                Some(Segment::from(cluster))
            }
        })
        .collect();
    Ok(segments)
}

const DBSCANPP_PROBABILITY: f64 = 0.1;
const DBSCANPP_MIN_POINTS: usize = 16;
const DBSCANPP_EPSILON: f64 = 16e-4;

fn dbscanpp<T>(pixels: &[Pixel<T>]) -> Result<Vec<Segment<T>>, Error>
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

    let clusters = clustering
        .fit(pixels)
        .map_err(|e| Error::PaletteExtractionError {
            details: e.to_string(),
        })?;

    let segments = clusters
        .iter()
        .filter_map(|cluster| {
            if cluster.is_empty() {
                None
            } else {
                Some(Segment::from(cluster))
            }
        })
        .collect();
    Ok(segments)
}

const SLIC_SEGMENTS: usize = 128;
const SLIC_COMPACTNESS: f64 = 0.0225; // 0.15^2
const SLIC_MAX_ITER: usize = 10;
const SLIC_TOLERANCE: f64 = 1e-3;

#[allow(dead_code)]
fn slic<T>(width: usize, height: usize, pixels: &[Pixel<T>]) -> Result<Vec<Segment<T>>, Error>
where
    T: FloatNumber,
{
    let segmentation = SlicSegmentation::builder()
        .segments(SLIC_SEGMENTS)
        .compactness(T::from_f64(SLIC_COMPACTNESS))
        .max_iter(SLIC_MAX_ITER)
        .tolerance(T::from_f64(SLIC_TOLERANCE))
        .metric(DistanceMetric::SquaredEuclidean)
        .build()
        .map_err(|e| Error::PaletteExtractionError {
            details: e.to_string(),
        })?;

    segmentation
        .segment(width, height, pixels)
        .map_err(|e| Error::PaletteExtractionError {
            details: e.to_string(),
        })
}

const SNIC_SEGMENTS: usize = 128;

#[allow(dead_code)]
fn snic<T>(width: usize, height: usize, pixels: &[Pixel<T>]) -> Result<Vec<Segment<T>>, Error>
where
    T: FloatNumber,
{
    let segmentation = SnicSegmentation::<T>::builder()
        .segments(SNIC_SEGMENTS)
        .metric(DistanceMetric::Euclidean)
        .build()
        .map_err(|e| Error::PaletteExtractionError {
            details: e.to_string(),
        })?;

    segmentation
        .segment(width, height, pixels)
        .map_err(|e| Error::PaletteExtractionError {
            details: e.to_string(),
        })
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[must_use]
    fn empty_pixels<T>() -> Vec<Pixel<T>>
    where
        T: FloatNumber,
    {
        Vec::new()
    }

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

    #[rstest]
    #[case::kmeans(Algorithm::KMeans, "kmeans")]
    #[case::dbscan(Algorithm::DBSCAN, "dbscan")]
    #[case::dbscanpp(Algorithm::DBSCANpp, "dbscan++")]
    fn test_fmt(#[case] algorithm: Algorithm, #[case] expected: &str) {
        // Act
        let actual = format!("{}", algorithm);

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_dbscan_empty() {
        // Act
        let actual = dbscan(&empty_pixels::<f64>());

        // Assert
        assert!(actual.is_err());
    }

    #[test]
    fn test_dbscanpp_empty() {
        // Act
        let actual = dbscanpp(&empty_pixels::<f64>());

        // Assert
        assert!(actual.is_err());
    }

    #[test]
    fn test_kmeans_empty() {
        // Act
        let actual = kmeans(192, 128, &empty_pixels::<f64>());

        // Assert
        assert!(actual.is_err());
    }

    #[test]
    fn test_slic_empty() {
        // Act
        let actual = slic(192, 128, &empty_pixels::<f64>());

        // Assert
        assert!(actual.is_err());
    }

    #[test]
    fn test_snic_empty() {
        // Act
        let actual = snic(192, 128, &empty_pixels::<f64>());

        // Assert
        assert!(actual.is_err());
    }
}

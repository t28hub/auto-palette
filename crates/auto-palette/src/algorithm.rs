use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use crate::{
    error::Error,
    image::{
        segmentation::{
            DbscanSegmentation,
            FastDbscanSegmentation,
            KmeansSegmentation,
            LabelImage,
            Segmentation,
            SlicSegmentation,
            SnicConfig,
            SnicSegmentation,
        },
        Pixel,
    },
    math::{DistanceMetric, FloatNumber},
    Filter,
    ImageData,
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

    /// SLIC (Simple Linear Iterative Clustering) algorithm.
    SLIC,

    /// SNIC (Simple Non-Iterative Clustering) algorithm.
    SNIC,
}

impl Algorithm {
    /// The number of segments to use for segmentation.
    const SEGMENTS: usize = 128;

    /// The maximum number of iterations for the K-means algorithm.
    const KMEANS_MAX_ITER: usize = 50;

    /// The tolerance for convergence conditions in the K-means algorithm.
    const KMEANS_TOLERANCE: f64 = 1e-3;

    /// The minimum number of points for the DBSCAN algorithm.
    const DBSCAN_MIN_POINTS: usize = 10;

    /// The epsilon value for the DBSCAN algorithm.
    const DBSCAN_EPSILON: f64 = 0.03;

    /// The probability for the Fast DBSCAN (DBSCAN++) algorithm.
    const FASTDBSCAN_PROBABILITY: f64 = 0.1;

    /// The minimum number of points for the Fast DBSCAN (DBSCAN++) algorithm.
    const FASTDBSCAN_MIN_POINTS: usize = 10;

    /// The epsilon value for the Fast DBSCAN (DBSCAN++) algorithm.
    const FASTDBSCAN_EPSILON: f64 = 0.04;

    /// The compactness value for the SLIC algorithm.
    const SLIC_COMPACTNESS: f64 = 0.0225; // 0.15^2

    /// The maximum number of iterations for the SLIC algorithm.
    const SLIC_MAX_ITER: usize = 10;

    /// The tolerance for convergence conditions in the SLIC algorithm.
    const SLIC_TOLERANCE: f64 = 1e-3;

    /// Clusters the given pixels using the algorithm.
    ///
    /// # Arguments
    /// * `width` - The width of the image.
    /// * `height` - The height of the image.
    /// * `pixels` - The pixels to cluster.
    ///
    /// # Returns
    /// The clusters found by the algorithm.
    pub(crate) fn segment<T, F>(
        &self,
        image_data: &ImageData,
        filter: &F,
    ) -> Result<LabelImage<T>, Error>
    where
        T: FloatNumber,
        F: Filter,
    {
        match self {
            Self::KMeans => segment_internal(image_data, filter, || {
                KmeansSegmentation::builder()
                    .segments(Self::SEGMENTS)
                    .max_iter(Self::KMEANS_MAX_ITER)
                    .tolerance(T::from_f64(Self::KMEANS_TOLERANCE))
                    .metric(DistanceMetric::SquaredEuclidean)
                    .build()
            }),
            Self::DBSCAN => segment_internal(image_data, filter, || {
                DbscanSegmentation::builder()
                    .segments(Self::SEGMENTS)
                    .min_pixels(Self::DBSCAN_MIN_POINTS)
                    .epsilon(T::from_f64(Self::DBSCAN_EPSILON.powi(2))) // Squared epsilon for squared euclidean distance
                    .metric(DistanceMetric::SquaredEuclidean)
                    .build()
            }),
            Self::DBSCANpp => segment_internal(image_data, filter, || {
                FastDbscanSegmentation::builder()
                    .min_pixels(Self::FASTDBSCAN_MIN_POINTS)
                    .probability(T::from_f64(Self::FASTDBSCAN_PROBABILITY))
                    .epsilon(T::from_f64(Self::FASTDBSCAN_EPSILON).powi(2))
                    .metric(DistanceMetric::SquaredEuclidean)
                    .build()
            }),
            Self::SLIC => segment_internal(image_data, filter, || {
                SlicSegmentation::builder()
                    .segments(Self::SEGMENTS)
                    .max_iter(Self::SLIC_MAX_ITER)
                    .compactness(T::from_f64(Self::SLIC_COMPACTNESS))
                    .tolerance(T::from_f64(Self::SLIC_TOLERANCE))
                    .metric(DistanceMetric::SquaredEuclidean)
                    .build()
            }),
            Self::SNIC => segment_internal(image_data, filter, || {
                SnicSegmentation::try_from(
                    SnicConfig::<T>::default()
                        .segments(Self::SEGMENTS)
                        .metric(DistanceMetric::SquaredEuclidean),
                )
            }),
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KMeans => write!(f, "kmeans"),
            Self::DBSCAN => write!(f, "dbscan"),
            Self::DBSCANpp => write!(f, "dbscan++"),
            Self::SLIC => write!(f, "slic"),
            Self::SNIC => write!(f, "snic"),
        }
    }
}

/// Segments the image data using the specified filter and algorithm.
///
/// # Type Parameters
/// * `T` - The floating point type.
/// * `F` - The filter function.
/// * `B` - The builder function.
/// * `S` - The segmentation algorithm.
/// * `E` - The error type for the segmentation algorithm.
///
/// # Arguments
/// * `image_data` - The image data to segment.
/// * `filter` - The filter to apply to the image data.
/// * `builder` - The builder function to create the segmentation algorithm.
///
/// # Returns
/// A vector of segments found by the segmentation algorithm.
fn segment_internal<T, F, B, S, E>(
    image_data: &ImageData,
    filter: &F,
    builder: B,
) -> Result<LabelImage<T>, Error>
where
    T: FloatNumber,
    F: Filter,
    B: FnOnce() -> Result<S, E>,
    S: Segmentation<T, Err = E>,
    E: Display,
{
    let segmentation = builder().map_err(|e| Error::PaletteExtractionError {
        details: e.to_string(),
    })?;

    let width = image_data.width() as usize;
    let height = image_data.height() as usize;
    let (pixels, mask) = collect_pixels_and_mask(image_data, filter);
    segmentation
        .segment_with_mask(width, height, &pixels, &mask)
        .map_err(|e| Error::PaletteExtractionError {
            details: e.to_string(),
        })
}

/// Collects the pixels and mask from the image data.
///
/// # Type Parameters
/// * `T` - The floating point type.
/// * `F` - The filter type.
///
/// # Arguments
/// * `image_data` - The image data to collect pixels from.
/// * `filter` - The filter to apply to the pixels.
///
/// # Returns
/// A tuple containing a vector of pixels and a vector of masks.
#[must_use]
fn collect_pixels_and_mask<T, F>(image_data: &ImageData, filter: &F) -> (Vec<Pixel<T>>, Vec<bool>)
where
    T: FloatNumber,
    F: Filter,
{
    let width = image_data.width() as usize;
    let height = image_data.height() as usize;
    let (pixels, mask) = image_data.pixels_with_filter(filter).fold(
        (
            Vec::with_capacity(width * height),
            Vec::with_capacity(width * height),
        ),
        |(mut pixels, mut mask), (p, m)| {
            pixels.push(p);
            mask.push(m);
            (pixels, mask)
        },
    );
    (pixels, mask)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::Rgba;

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

    #[rstest]
    #[case::kmeans(Algorithm::KMeans)]
    #[case::dbscan(Algorithm::DBSCAN)]
    #[case::dbscanpp(Algorithm::DBSCANpp)]
    #[case::slic(Algorithm::SLIC)]
    #[case::snic(Algorithm::SNIC)]
    fn test_segment_empty(#[case] algorithm: Algorithm) {
        // Arrange
        let pixels: Vec<_> = Vec::new();
        let image_data = ImageData::new(0, 0, &pixels).expect("Failed to create empty image data");

        // Act
        let actual = algorithm.segment(&image_data, &|rgba: &Rgba| rgba[0] != 0);

        // Assert
        assert!(actual.is_ok());

        let label_image: LabelImage<f64> = actual.unwrap();
        assert_eq!(label_image.width(), 0);
        assert_eq!(label_image.height(), 0);
    }
}

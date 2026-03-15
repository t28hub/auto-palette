use crate::{
    error::{Error, ExtractionError, ExtractionErrorKind},
    image::Pixel,
    math::FloatNumber,
    segmentation::{
        error::SegmentationError,
        input::SegmentationInput,
        DbscanConfig,
        DbscanSegmentation,
        FastDbscanConfig,
        FastDbscanSegmentation,
        KmeansConfig,
        KmeansSegmentation,
        Segmentation,
        SegmentationResult,
        SlicConfig,
        SlicSegmentation,
        SnicConfig,
        SnicSegmentation,
    },
    Filter,
    ImageData,
};

/// The segmentation method to use for splitting an image into regions.
///
/// Each variant holds the configuration for a specific segmentation algorithm.
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug, Clone, PartialEq)]
pub enum SegmentationMethod<T>
where
    T: FloatNumber,
{
    /// DBSCAN-based segmentation.
    Dbscan(DbscanConfig<T>),

    /// DBSCAN++ (fast DBSCAN) segmentation.
    FastDbscan(FastDbscanConfig<T>),

    /// K-means-based segmentation.
    Kmeans(KmeansConfig<T>),

    /// SLIC (Simple Linear Iterative Clustering) segmentation.
    Slic(SlicConfig<T>),

    /// SNIC (Simple Non-Iterative Clustering) segmentation.
    Snic(SnicConfig<T>),
}

impl<T> SegmentationMethod<T>
where
    T: FloatNumber,
{
    /// Segments the image into regions using the configured algorithm.
    ///
    /// # Arguments
    /// * `image` - The image data to segment.
    /// * `filter` - The filter to apply to the image pixels.
    ///
    /// # Returns
    /// A `SegmentationResult` representing the segmented regions, or an error if segmentation fails.
    pub fn segment<F>(&self, image: &ImageData, filter: &F) -> Result<SegmentationResult<T>, Error>
    where
        F: Filter,
    {
        let width = image.width() as usize;
        let height = image.height() as usize;
        let (pixels, mask) = collect_pixels_and_mask(image, filter);
        let input = SegmentationInput::new(width, height, &pixels, &mask)
            .map_err(|_| ExtractionError::from(ExtractionErrorKind::DimensionMismatch))?;
        match self {
            Self::Dbscan(config) => segment_with(DbscanSegmentation::try_from(*config), &input),
            Self::FastDbscan(config) => {
                segment_with(FastDbscanSegmentation::try_from(*config), &input)
            }
            Self::Kmeans(config) => segment_with(KmeansSegmentation::try_from(*config), &input),
            Self::Slic(config) => segment_with(SlicSegmentation::try_from(*config), &input),
            Self::Snic(config) => segment_with(SnicSegmentation::try_from(*config), &input),
        }
    }
}

impl<T> Default for SegmentationMethod<T>
where
    T: FloatNumber,
{
    fn default() -> Self {
        Self::Dbscan(DbscanConfig::default())
    }
}

impl<T> From<DbscanConfig<T>> for SegmentationMethod<T>
where
    T: FloatNumber,
{
    fn from(config: DbscanConfig<T>) -> Self {
        Self::Dbscan(config)
    }
}

impl<T> From<FastDbscanConfig<T>> for SegmentationMethod<T>
where
    T: FloatNumber,
{
    fn from(config: FastDbscanConfig<T>) -> Self {
        Self::FastDbscan(config)
    }
}

impl<T> From<KmeansConfig<T>> for SegmentationMethod<T>
where
    T: FloatNumber,
{
    fn from(config: KmeansConfig<T>) -> Self {
        Self::Kmeans(config)
    }
}

impl<T> From<SlicConfig<T>> for SegmentationMethod<T>
where
    T: FloatNumber,
{
    fn from(config: SlicConfig<T>) -> Self {
        Self::Slic(config)
    }
}

impl<T> From<SnicConfig<T>> for SegmentationMethod<T>
where
    T: FloatNumber,
{
    fn from(config: SnicConfig<T>) -> Self {
        Self::Snic(config)
    }
}

/// Runs a segmentation algorithm on the given input, converting any errors.
///
/// # Type Parameters
/// * `T` - The floating point type.
/// * `S` - The segmentation algorithm type.
///
/// # Arguments
/// * `segmentation` - The result of creating a segmentation algorithm (e.g., via `TryFrom`).
/// * `input` - The segmentation input to process.
///
/// # Returns
/// A `SegmentationResult`, or an `Error` if creation or segmentation fails.
fn segment_with<T, S>(
    segmentation: Result<S, SegmentationError>,
    input: &SegmentationInput<'_, T>,
) -> Result<SegmentationResult<T>, Error>
where
    T: FloatNumber,
    S: Segmentation<T>,
{
    segmentation
        .and_then(|s| s.segment(input))
        .map_err(|e: SegmentationError| -> Error {
            match e {
                SegmentationError::InvalidArgument(_) => {
                    ExtractionError::from(ExtractionErrorKind::InvalidParameter).into()
                }
                SegmentationError::UnexpectedLength(_) => {
                    ExtractionError::from(ExtractionErrorKind::DimensionMismatch).into()
                }
            }
        })
}

/// Collects pixels and a mask from the image data.
///
/// # Type Parameters
/// * `T` - The floating point type.
/// * `F` - The filter type.
///
/// # Arguments
/// * `image` - The image data to collect pixels from.
/// * `filter` - The filter to apply to each pixel.
///
/// # Returns
/// A tuple of `(pixels, mask)` where `mask[i]` indicates whether `pixels[i]` passed the filter.
#[must_use]
fn collect_pixels_and_mask<T, F>(image: &ImageData, filter: &F) -> (Vec<Pixel<T>>, Vec<bool>)
where
    T: FloatNumber,
    F: Filter,
{
    let width = image.width() as usize;
    let height = image.height() as usize;
    let (pixels, mask) = image.pixels_with_filter(filter).fold(
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
    use crate::{error::ExtractionErrorKind, Rgba};

    #[rstest]
    #[case::dbscan(SegmentationMethod::Dbscan(DbscanConfig::default()))]
    #[case::fast_dbscan(SegmentationMethod::FastDbscan(FastDbscanConfig::default()))]
    #[case::kmeans(SegmentationMethod::Kmeans(KmeansConfig::default()))]
    #[case::slic(SegmentationMethod::Slic(SlicConfig::default()))]
    #[case::snic(SegmentationMethod::Snic(SnicConfig::default()))]
    fn test_segment_empty(#[case] method: SegmentationMethod<f64>) {
        // Arrange
        let pixels: Vec<_> = Vec::new();
        let image_data = ImageData::new(0, 0, &pixels).expect("Failed to create empty image data");

        // Act
        let actual = method.segment(&image_data, &|rgba: &Rgba| rgba[0] != 0);

        // Assert
        assert!(actual.is_ok());

        let result = actual.unwrap();
        assert!(result.is_empty());
    }

    #[rstest]
    #[case::dbscan(SegmentationMethod::Dbscan(DbscanConfig::default().segments(32)))]
    #[case::fast_dbscan(SegmentationMethod::FastDbscan(FastDbscanConfig::default()))]
    #[case::kmeans(SegmentationMethod::Kmeans(KmeansConfig::default().segments(32)))]
    #[case::slic(SegmentationMethod::Slic(SlicConfig::default().segments(32)))]
    #[case::snic(SegmentationMethod::Snic(SnicConfig::default().segments(32)))]
    #[cfg(feature = "image")]
    fn test_segment(#[case] method: SegmentationMethod<f64>) {
        // Arrange
        let image_data = ImageData::load("../../gfx/flags/za.png").unwrap();

        // Act
        let actual = method.segment(&image_data, &|_: &Rgba| true);

        // Assert
        assert!(actual.is_ok());

        let result = actual.unwrap();
        assert!(!result.is_empty());
        for segment in result.segments() {
            assert!(!segment.is_empty());
        }
    }

    #[rstest]
    #[case::dbscan(SegmentationMethod::Dbscan(DbscanConfig::default().segments(32)))]
    #[case::fast_dbscan(SegmentationMethod::FastDbscan(FastDbscanConfig::default()))]
    #[case::kmeans(SegmentationMethod::Kmeans(KmeansConfig::default().segments(32)))]
    #[case::slic(SegmentationMethod::Slic(SlicConfig::default().segments(32)))]
    #[case::snic(SegmentationMethod::Snic(SnicConfig::default().segments(32)))]
    #[cfg(feature = "image")]
    fn test_segment_with_filter(#[case] method: SegmentationMethod<f64>) {
        // Arrange
        let image_data = ImageData::load("../../gfx/flags/np.png").unwrap();

        // Act
        let actual = method.segment(&image_data, &|rgba: &Rgba| rgba[3] != 0);

        // Assert
        assert!(actual.is_ok());

        let result = actual.unwrap();
        assert!(!result.is_empty());
    }

    #[test]
    fn test_segment_with_invalid_argument_error() {
        // Arrange
        let pixels: Vec<_> = Vec::new();
        let image_data = ImageData::new(0, 0, &pixels).unwrap();
        let (pixels, mask) = collect_pixels_and_mask::<f64, _>(&image_data, &|_: &Rgba| true);
        let input = SegmentationInput::new(0, 0, &pixels, &mask).unwrap();

        // Act
        let actual = segment_with::<f64, DbscanSegmentation<f64>>(
            Err(SegmentationError::InvalidArgument("test".to_string())),
            &input,
        );

        // Assert
        let Error::Extraction(e) = actual.unwrap_err() else {
            panic!("expected Error::Extraction");
        };
        assert_eq!(e.kind(), ExtractionErrorKind::InvalidParameter);
    }

    #[test]
    fn test_segment_with_unexpected_length_error() {
        // Arrange
        let pixels: Vec<_> = Vec::new();
        let image_data = ImageData::new(0, 0, &pixels).unwrap();
        let (pixels, mask) = collect_pixels_and_mask::<f64, _>(&image_data, &|_: &Rgba| true);
        let input = SegmentationInput::new(0, 0, &pixels, &mask).unwrap();
        let matrix_error = crate::math::matrix::MatrixError::DimensionMismatch {
            cols: 2,
            rows: 3,
            expected: 6,
            actual: 5,
        };

        // Act
        let actual = segment_with::<f64, DbscanSegmentation<f64>>(
            Err(SegmentationError::UnexpectedLength(matrix_error)),
            &input,
        );

        // Assert
        let Error::Extraction(e) = actual.unwrap_err() else {
            panic!("expected Error::Extraction");
        };
        assert_eq!(e.kind(), ExtractionErrorKind::DimensionMismatch);
    }
}

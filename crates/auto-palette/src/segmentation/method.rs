use crate::{
    error::Error,
    image::Pixel,
    math::FloatNumber,
    segmentation::{
        DbscanConfig,
        DbscanSegmentation,
        FastDbscanConfig,
        FastDbscanSegmentation,
        KmeansConfig,
        KmeansSegmentation,
        LabelImage,
        Segmentation,
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
    /// A `LabelImage` containing the segmented regions, or an error if segmentation fails.
    pub fn segment<F>(&self, image: &ImageData, filter: &F) -> Result<LabelImage<T>, Error>
    where
        F: Filter,
    {
        let width = image.width() as usize;
        let height = image.height() as usize;
        let (pixels, mask) = collect_pixels_and_mask(image, filter);
        match self {
            Self::Dbscan(config) => DbscanSegmentation::try_from(*config)
                .and_then(|s| s.segment_with_mask(width, height, &pixels, &mask))
                .map_err(|e| Error::PaletteExtractionError {
                    details: e.to_string(),
                }),
            Self::FastDbscan(config) => FastDbscanSegmentation::try_from(*config)
                .and_then(|s| s.segment_with_mask(width, height, &pixels, &mask))
                .map_err(|e| Error::PaletteExtractionError {
                    details: e.to_string(),
                }),
            Self::Kmeans(config) => KmeansSegmentation::try_from(*config)
                .and_then(|s| s.segment_with_mask(width, height, &pixels, &mask))
                .map_err(|e| Error::PaletteExtractionError {
                    details: e.to_string(),
                }),
            Self::Slic(config) => SlicSegmentation::try_from(*config)
                .and_then(|s| s.segment_with_mask(width, height, &pixels, &mask))
                .map_err(|e| Error::PaletteExtractionError {
                    details: e.to_string(),
                }),
            Self::Snic(config) => SnicSegmentation::try_from(*config)
                .and_then(|s| s.segment_with_mask(width, height, &pixels, &mask))
                .map_err(|e| Error::PaletteExtractionError {
                    details: e.to_string(),
                }),
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
    use crate::Rgba;

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

        let label_image: LabelImage<f64> = actual.unwrap();
        assert_eq!(label_image.width(), 0);
        assert_eq!(label_image.height(), 0);
    }
}

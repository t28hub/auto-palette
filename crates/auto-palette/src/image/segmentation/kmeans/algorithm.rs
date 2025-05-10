use crate::{
    image::{
        segmentation::{kmeans::KmeansError, seed::SeedGenerator, Segment, Segmentation, Segments},
        Pixel,
    },
    math::{
        clustering::{CentroidInit, ClusteringAlgorithm, Kmeans},
        DistanceMetric,
        FloatNumber,
    },
};

/// K-means segmentation algorithm.
///
/// This algorithm is used to segment an image into k clusters.
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug, PartialEq)]
pub struct KmeansSegmentation<T>
where
    T: FloatNumber,
{
    kmeans: Kmeans<T>,
}

impl<T> KmeansSegmentation<T>
where
    T: FloatNumber,
{
    /// Default number of segments for the segmentation.
    const DEFAULT_SEGMENTS: usize = 64;

    /// Default maximum number of iterations for the K-means algorithm.
    const DEFAULT_MAX_ITER: usize = 100;

    /// Default tolerance for convergence conditions.
    const DEFAULT_TOLERANCE: f64 = 1e-4;

    /// Creates a new `KmeansSegmentationBuilder` instance.
    ///
    /// # Returns
    /// A new `KmeansSegmentationBuilder` instance with default parameters.
    #[must_use]
    pub fn builder() -> KmeansSegmentationBuilder<T> {
        KmeansSegmentationBuilder::default()
    }
}

impl<T> Segmentation<T> for KmeansSegmentation<T>
where
    T: FloatNumber,
{
    type Err = KmeansError<T>;

    fn segment(
        &self,
        _width: usize,
        _height: usize,
        pixels: &[Pixel<T>],
    ) -> Result<Segments<T>, Self::Err> {
        if pixels.is_empty() {
            return Err(KmeansError::EmptyPixels);
        }

        let clusters = self.kmeans.fit(pixels)?;
        let segments = clusters.iter().map(Segment::from).collect::<Vec<_>>();
        Ok(segments)
    }
}

/// Builder for `KmeansSegmentation`.
///
/// This struct allows for the configuration of the K-means segmentation algorithm.
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug, PartialEq)]
pub struct KmeansSegmentationBuilder<T>
where
    T: FloatNumber,
{
    segments: usize,
    max_iter: usize,
    tolerance: T,
    generator: SeedGenerator,
    metric: DistanceMetric,
}

impl<T> KmeansSegmentationBuilder<T>
where
    T: FloatNumber,
{
    /// Sets the number of segments for the segmentation.
    ///
    /// # Arguments
    /// * `segments` - The number of segments to create.
    ///
    /// # Returns
    /// A new `KmeansSegmentationBuilder` instance with the specified number of segments.
    #[must_use]
    pub fn segments(mut self, segments: usize) -> Self {
        self.segments = segments;
        self
    }

    /// Sets the maximum number of iterations for the K-means algorithm.
    ///
    /// # Arguments
    /// * `max_iter` - The maximum number of iterations to perform.
    ///
    /// # Returns
    /// A new `KmeansSegmentationBuilder` instance with the specified maximum number of iterations.
    #[must_use]
    pub fn max_iter(mut self, max_iter: usize) -> Self {
        self.max_iter = max_iter;
        self
    }

    /// Sets the tolerance for convergence conditions.
    ///
    /// # Arguments
    /// * `tolerance` - The tolerance value for convergence conditions.
    ///
    /// # Returns
    /// The `KmeansSegmentationBuilder` instance with the specified tolerance.
    #[must_use]
    pub fn tolerance(mut self, tolerance: T) -> Self {
        self.tolerance = tolerance;
        self
    }

    /// Sets the seed generator for the segmentation.
    ///
    /// # Arguments
    /// * `generator` - The seed generator to use for the segmentation.
    ///
    /// # Returns
    /// The `KmeansSegmentationBuilder` instance with the specified seed generator.
    #[allow(dead_code)]
    #[must_use]
    pub fn generator(mut self, generator: SeedGenerator) -> Self {
        self.generator = generator;
        self
    }

    /// Sets the distance metric for the segmentation.
    ///
    /// # Arguments
    /// * `metric` - The distance metric to use for the segmentation.
    ///
    /// # Returns
    /// The `KmeansSegmentationBuilder` instance with the specified distance metric.
    #[must_use]
    pub fn metric(mut self, metric: DistanceMetric) -> Self {
        self.metric = metric;
        self
    }

    /// Builds the KmeansSegmentation instance.
    ///
    /// # Returns
    /// A new KmeansSegmentation instance with the specified parameters.
    pub fn build(self) -> Result<KmeansSegmentation<T>, KmeansError<T>> {
        if self.segments == 0 {
            return Err(KmeansError::InvalidSegments);
        }

        let kmeans = Kmeans::new(
            self.segments,
            self.max_iter,
            self.tolerance,
            self.metric,
            CentroidInit::RegularInterval,
        )?;
        Ok(KmeansSegmentation { kmeans })
    }
}

impl<T> Default for KmeansSegmentationBuilder<T>
where
    T: FloatNumber,
{
    fn default() -> Self {
        Self {
            segments: KmeansSegmentation::<T>::DEFAULT_SEGMENTS,
            max_iter: KmeansSegmentation::<T>::DEFAULT_MAX_ITER,
            tolerance: T::from_f64(KmeansSegmentation::<T>::DEFAULT_TOLERANCE),
            generator: SeedGenerator::default(),
            metric: DistanceMetric::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::{math::clustering::KmeansError as KmeansClusteringError, ImageData, RgbaPixel};

    #[test]
    fn test_builder() {
        // Act
        let actual = KmeansSegmentation::<f64>::builder();

        // Assert
        assert_eq!(actual, KmeansSegmentationBuilder::default());
    }

    #[test]
    fn test_builder_build() {
        // Act
        let actual = KmeansSegmentation::<f64>::builder()
            .segments(10)
            .max_iter(100)
            .tolerance(1e-4)
            .generator(SeedGenerator::RegularGrid)
            .metric(DistanceMetric::SquaredEuclidean)
            .build();

        // Assert
        assert!(actual.is_ok());

        let segmentation = actual.unwrap();
        assert_eq!(
            segmentation.kmeans,
            Kmeans::new(
                10,
                100,
                1e-4,
                DistanceMetric::SquaredEuclidean,
                CentroidInit::RegularInterval
            )
            .unwrap(),
        );
    }

    #[test]
    fn test_builder_build_invalid_segment() {
        // Act
        let actual = KmeansSegmentation::<f64>::builder().segments(0).build();

        // Assert
        assert!(actual.is_err());

        let error = actual.unwrap_err();
        assert_eq!(error, KmeansError::InvalidSegments);
    }

    #[rstest]
    #[case(0, 1e-4, KmeansClusteringError::InvalidIterations(0))]
    #[case(100, -1.0, KmeansClusteringError::InvalidTolerance(-1.0))]
    fn test_builder_build_invalid_parameters(
        #[case] max_iter: usize,
        #[case] tolerance: f64,
        #[case] cause: KmeansClusteringError<f64>,
    ) {
        // Act
        let actual = KmeansSegmentation::builder()
            .max_iter(max_iter)
            .tolerance(tolerance)
            .build();

        // Assert
        assert!(actual.is_err());

        let error = actual.unwrap_err();
        assert_eq!(error, KmeansError::InvalidParameters(cause));
    }

    #[test]
    #[cfg(feature = "image")]
    fn test_segment() {
        // Arrange
        let image_data = ImageData::load("../../gfx/baboon.jpg").unwrap();
        let segmentation = KmeansSegmentation::builder()
            .segments(16)
            .max_iter(5)
            .tolerance(1e-4)
            .build()
            .unwrap();

        // Act
        let width = image_data.width() as usize;
        let height = image_data.height() as usize;
        let pixels = image_data.pixels(|pixel: &RgbaPixel| pixel[3] != 0);
        let actual = segmentation.segment(width, height, &pixels);

        // Assert
        assert!(actual.is_ok());

        let segments = actual.unwrap();
        assert_eq!(segments.len(), 16);
    }

    #[test]
    fn test_segment_empty_pixels() {
        // Arrange
        let segmentation = KmeansSegmentation::builder()
            .segments(16)
            .max_iter(5)
            .tolerance(1e-4)
            .build()
            .unwrap();

        // Act
        let pixels: Vec<Pixel<f64>> = vec![];
        let actual = segmentation.segment(192, 128, &pixels);

        // Assert
        assert!(actual.is_err());

        let error = actual.unwrap_err();
        assert_eq!(error, KmeansError::EmptyPixels);
    }
}

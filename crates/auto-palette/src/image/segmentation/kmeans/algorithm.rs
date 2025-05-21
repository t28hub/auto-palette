use crate::{
    image::{
        segmentation::{kmeans::KmeansError, seed::SeedGenerator, Segment, Segmentation, Segments},
        Pixel,
    },
    math::{
        neighbors::{kdtree::KDTreeSearch, NeighborSearch},
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
    segments: usize,
    max_iter: usize,
    tolerance: T,
    generator: SeedGenerator,
    metric: DistanceMetric,
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
    pub fn builder() -> Builder<T> {
        Builder::default()
    }

    #[must_use]
    fn iterate(
        &self,
        pixels: &[Pixel<T>],
        mask: &[bool],
        centers: &mut [Pixel<T>],
        segments: &mut [Segment<T>],
    ) -> bool {
        segments.iter_mut().for_each(Segment::reset);

        let center_search = KDTreeSearch::build(centers, self.metric, 16);
        for (index, pixel) in pixels.iter().enumerate() {
            if !mask[index] {
                continue;
            }

            if let Some(nearest) = center_search.search_nearest(pixel) {
                segments[nearest.index].assign(index, pixel);
            }
        }

        let mut converged = true;
        for (label, segment) in segments.iter_mut().enumerate() {
            let Some(old_center) = centers.get_mut(label) else {
                continue;
            };

            let new_center = segment.center();
            let distance = self.metric.measure(old_center, new_center);
            if distance > self.tolerance {
                converged = false;
            }

            *old_center = *new_center;
        }
        converged
    }
}

impl<T> Segmentation<T> for KmeansSegmentation<T>
where
    T: FloatNumber,
{
    type Err = KmeansError<T>;

    fn segment_with_mask(
        &self,
        width: usize,
        height: usize,
        pixels: &[Pixel<T>],
        mask: &[bool],
    ) -> Result<Segments<T>, Self::Err> {
        if width * height != pixels.len() {
            return Err(KmeansError::UnexpectedLength {
                actual: pixels.len(),
                expected: width * height,
            });
        }

        let mut centers: Vec<_> = self
            .generator
            .generate(width, height, pixels, mask, self.segments)
            .iter()
            .map(|&seed| pixels[seed])
            .collect();
        let mut segments = vec![Segment::default(); centers.len()];

        for _ in 0..self.max_iter {
            if self.iterate(pixels, mask, &mut centers, &mut segments) {
                break;
            }
        }
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
pub struct Builder<T>
where
    T: FloatNumber,
{
    segments: usize,
    max_iter: usize,
    tolerance: T,
    generator: SeedGenerator,
    metric: DistanceMetric,
}

impl<T> Builder<T>
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
        if self.max_iter == 0 {
            return Err(KmeansError::InvalidIterations);
        }
        if self.tolerance <= T::zero() || self.tolerance.is_nan() {
            return Err(KmeansError::InvalidTolerance(self.tolerance));
        }

        Ok(KmeansSegmentation {
            segments: self.segments,
            max_iter: self.max_iter,
            tolerance: self.tolerance,
            generator: self.generator,
            metric: self.metric,
        })
    }
}

impl<T> Default for Builder<T>
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
    use crate::ImageData;

    #[test]
    fn test_builder() {
        // Act
        let actual = KmeansSegmentation::<f64>::builder();

        // Assert
        assert_eq!(actual, Builder::default());
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
            segmentation,
            KmeansSegmentation {
                segments: 10,
                max_iter: 100,
                tolerance: 1e-4,
                generator: SeedGenerator::RegularGrid,
                metric: DistanceMetric::SquaredEuclidean,
            }
        );
    }

    #[rstest]
    #[case(0, 25, 1e-4, KmeansError::InvalidSegments)]
    #[case(48, 0, 1e-4, KmeansError::InvalidIterations)]
    #[case(48, 25, -1e-4, KmeansError::InvalidTolerance(-1e-4))]
    fn test_builder_build_invalid_parameters(
        #[case] segments: usize,
        #[case] max_iter: usize,
        #[case] tolerance: f64,
        #[case] expected: KmeansError<f64>,
    ) {
        // Act
        let actual = KmeansSegmentation::builder()
            .segments(segments)
            .max_iter(max_iter)
            .tolerance(tolerance)
            .build();

        // Assert
        assert!(actual.is_err());

        let error = actual.unwrap_err();
        assert_eq!(error, expected);
    }

    #[test]
    fn test_builder_build_invalid_tolerance_nan() {
        // Act
        let actual = KmeansSegmentation::<f64>::builder()
            .tolerance(f64::NAN)
            .build();

        // Assert
        assert!(actual.is_err());

        let error = actual.unwrap_err();
        assert_eq!(
            error.to_string(),
            "Tolerance must be greater than zero and not NaN: NaN"
        );
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
        let pixels: Vec<_> = image_data.pixels().collect();
        let actual = segmentation.segment(width, height, &pixels);

        // Assert
        assert!(actual.is_ok());

        let segments = actual.unwrap();
        assert_eq!(segments.len(), 16);
    }
}

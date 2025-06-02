use std::collections::HashMap;

use crate::{
    image::{
        segmentation::{
            helper::gradient,
            label::{Builder as SegmentBuilder, LabelImage},
            seed::SeedGenerator,
            segment::SegmentMetadata,
            slic::error::SlicError,
            Segmentation,
        },
        Pixel,
        LABXY_CHANNELS,
    },
    math::{matrix::MatrixView, DistanceMetric, FloatNumber},
};

/// SLIC (Simple Linear Iterative Clustering) segmentation algorithm.
/// The algorithm is based on the following paper:
/// [SLIC Superpixels Compared to State-of-the-art Superpixel Methods](https://core.ac.uk/download/pdf/147983593.pdf)
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug, PartialEq)]
pub struct SlicSegmentation<T>
where
    T: FloatNumber,
{
    segments: usize,
    compactness: T,
    max_iter: usize,
    tolerance: T,
    generator: SeedGenerator,
    metric: DistanceMetric,
}

impl<T> SlicSegmentation<T>
where
    T: FloatNumber,
{
    /// Default number of segments to create.
    const DEFAULT_SEGMENTS: usize = 64;

    /// Default compactness of the segments.
    const DEFAULT_COMPACTNESS: f64 = 1.0;

    /// Default maximum number of iterations.
    const DEFAULT_MAX_ITER: usize = 10;

    /// Default tolerance for convergence conditions.
    const DEFAULT_TOLERANCE: f64 = 1e-4;

    /// Creates a new `SlicSegmentationBuilder` instance.
    ///
    /// # Returns
    /// A new `SlicSegmentationBuilder` instance with default values.
    #[must_use]
    pub fn builder() -> Builder<T> {
        Builder::default()
    }

    /// Finds the lowest gradient pixel in the neighborhood of the given index.
    ///
    /// # Arguments
    /// * `matrix` - The matrix of pixels.
    /// * `index` - The index of the pixel to find the lowest gradient for.
    /// * `mask` - The mask for ignoring certain pixels.
    ///
    /// # Returns
    /// The index of the lowest gradient pixel in the neighborhood, or `None` if no such pixel exists.
    #[inline]
    #[must_use]
    fn find_lowest_gradient_index(
        &self,
        matrix: &MatrixView<'_, T, LABXY_CHANNELS>,
        index: usize,
        mask: &[bool],
    ) -> Option<usize> {
        let col = index % matrix.cols;
        let row = index / matrix.cols;

        let (_, lowest_index) = matrix.neighbors(col, row).fold(
            (T::max_value(), None),
            |(lowest_score, lowest_index), (neighbor_index, _)| {
                if !mask[neighbor_index] {
                    return (lowest_score, lowest_index);
                }

                let neighbor_col = neighbor_index % matrix.cols;
                let neighbor_row = neighbor_index / matrix.cols;
                let score = gradient(matrix, neighbor_col, neighbor_row, self.metric);
                if score < lowest_score {
                    (score, Some(neighbor_index))
                } else {
                    (lowest_score, lowest_index)
                }
            },
        );
        lowest_index
    }

    /// Iterates over the points and updates the centroids and clusters.
    ///
    /// # Type Parameters
    /// * `N` - The number of dimensions.
    ///
    /// # Arguments
    /// * `matrix` - The matrix of pixels.
    /// * `pixels` - The pixels to segment.
    /// * `mask` - The mask for ignoring certain pixels.
    /// * `centers` - The current centroids of the segments.
    /// * `builder` - The segment builder to use for creating segments.
    ///
    /// # Returns
    /// `true` if the centroids have converged, `false` otherwise.
    #[inline]
    fn iterate(
        &self,
        matrix: &MatrixView<'_, T, LABXY_CHANNELS>,
        pixels: &[Pixel<T>],
        mask: &[bool],
        centers: &mut HashMap<usize, Pixel<T>>,
        builder: &mut SegmentBuilder<T>,
    ) -> bool {
        builder.iter_mut().for_each(SegmentMetadata::clear);

        let s = (T::from_usize(matrix.size()) / T::from_usize(self.segments)).sqrt();
        let radius = (T::from_u8(2) * s).ceil().trunc_to_usize();

        let mut labels = vec![usize::MAX; pixels.len()];
        let mut distances = vec![T::max_value(); pixels.len()];

        centers.iter().for_each(|(&center_index, center_pixel)| {
            let col = center_index % matrix.cols;
            let row = center_index / matrix.cols;

            matrix.neighbors_with_size(col, row, radius).for_each(
                |(neighbor_index, neighbor_pixel)| {
                    if !mask[neighbor_index] {
                        return;
                    }

                    let distance = self.metric.measure(center_pixel, neighbor_pixel);
                    if distance < distances[neighbor_index] {
                        distances[neighbor_index] = distance;
                        labels[neighbor_index] = center_index;
                    }
                },
            );
        });

        for (index, label) in labels.iter().enumerate() {
            builder.get_mut(label).insert(index, &pixels[index]);
        }

        let mut converged = true;
        builder.iter().for_each(|segment| {
            if segment.is_empty() {
                return;
            }

            let new_center = segment.center();
            let Some(old_center) = centers.get_mut(&segment.label()) else {
                return;
            };

            let diff = self.metric.measure(old_center, new_center);
            if diff > self.tolerance {
                converged = false;
            }

            *old_center = *new_center;
        });
        converged
    }
}

impl<T> Segmentation<T> for SlicSegmentation<T>
where
    T: FloatNumber,
{
    type Err = SlicError<T>;

    fn segment_with_mask(
        &self,
        width: usize,
        height: usize,
        pixels: &[Pixel<T>],
        mask: &[bool],
    ) -> Result<LabelImage<T>, Self::Err> {
        let matrix = MatrixView::new(width, height, pixels)?;
        let seeds = self
            .generator
            .generate(width, height, pixels, mask, self.segments);
        let mut centers: HashMap<_, _> = seeds
            .into_iter()
            .map(|seed_index| {
                let found = self.find_lowest_gradient_index(&matrix, seed_index, mask);
                let index = found.unwrap_or(seed_index);
                (index, pixels[index])
            })
            .collect();

        let mut builder = LabelImage::builder(width, height);
        for _ in 0..self.max_iter {
            if self.iterate(&matrix, pixels, mask, &mut centers, &mut builder) {
                break;
            }
        }
        Ok(builder.build())
    }
}

/// Builder for `SlicSegmentation`.
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug, PartialEq)]
pub struct Builder<T>
where
    T: FloatNumber,
{
    segments: usize,
    compactness: T,
    max_iter: usize,
    tolerance: T,
    generator: SeedGenerator,
    metric: DistanceMetric,
}

impl<T> Builder<T>
where
    T: FloatNumber,
{
    /// Sets the number of segments.
    ///
    /// # Arguments
    /// * `segments` - The number of segments.
    ///
    /// # Returns
    /// The `SlicSegmentationBuilder` instance with the specified number of segments.
    #[must_use]
    pub fn segments(mut self, segments: usize) -> Self {
        self.segments = segments;
        self
    }

    /// Sets the compactness of the segments.
    ///
    /// # Arguments
    /// * `compactness` - The compactness of the segments.
    ///
    /// # Returns
    /// The `SlicSegmentationBuilder` instance with the specified compactness.
    #[must_use]
    pub fn compactness(mut self, compactness: T) -> Self {
        self.compactness = compactness;
        self
    }

    /// Sets the maximum number of iterations.
    ///
    /// # Arguments
    /// * `max_iter` - The maximum number of iterations.
    ///
    /// # Returns
    /// The `SlicSegmentationBuilder` instance with the specified maximum iterations.
    #[must_use]
    pub fn max_iter(mut self, max_iter: usize) -> Self {
        self.max_iter = max_iter;
        self
    }

    /// Sets the tolerance for convergence conditions.
    ///
    /// # Arguments
    /// * `tolerance` - The tolerance for convergence conditions.
    ///
    /// # Returns
    /// The `SlicSegmentationBuilder` instance with the specified tolerance.
    #[must_use]
    pub fn tolerance(mut self, tolerance: T) -> Self {
        self.tolerance = tolerance;
        self
    }

    /// Sets the seed generator for the segmentation.
    ///
    /// # Arguments
    /// * `generator` - The seed generator to use.
    ///
    /// # Returns
    /// The `SlicSegmentationBuilder` instance with the specified generator.
    #[allow(dead_code)]
    #[must_use]
    pub fn generator(mut self, generator: SeedGenerator) -> Builder<T> {
        self.generator = generator;
        self
    }

    /// Sets the distance metric to use.
    ///
    /// # Arguments
    /// * `metric` - The distance metric to use.
    ///
    /// # Returns
    /// The `SlicSegmentationBuilder` instance with the specified metric.
    #[must_use]
    pub fn metric(mut self, metric: DistanceMetric) -> Self {
        self.metric = metric;
        self
    }

    /// Builds the `SlicSegmentation` instance.
    ///
    /// # Returns
    /// A `Result` containing the `SlicSegmentation` instance or an error if the parameters are invalid.
    pub fn build(self) -> Result<SlicSegmentation<T>, SlicError<T>> {
        if self.segments == 0 {
            return Err(SlicError::InvalidSegments(self.segments));
        }
        if self.compactness <= T::zero() || self.compactness.is_nan() {
            return Err(SlicError::InvalidCompactness(self.compactness));
        }
        if self.max_iter == 0 {
            return Err(SlicError::InvalidIterations(self.max_iter));
        }
        if self.tolerance <= T::zero() || self.tolerance.is_nan() {
            return Err(SlicError::InvalidTolerance(self.tolerance));
        }
        Ok(SlicSegmentation {
            segments: self.segments,
            compactness: self.compactness,
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
            segments: SlicSegmentation::<T>::DEFAULT_SEGMENTS,
            compactness: T::from_f64(SlicSegmentation::<T>::DEFAULT_COMPACTNESS),
            max_iter: SlicSegmentation::<T>::DEFAULT_MAX_ITER,
            tolerance: T::from_f64(SlicSegmentation::<T>::DEFAULT_TOLERANCE),
            generator: SeedGenerator::default(),
            metric: DistanceMetric::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::{math::matrix::MatrixError, ImageData};

    #[must_use]
    fn sample_pixels<T>(width: usize, height: usize) -> Vec<Pixel<T>>
    where
        T: FloatNumber,
    {
        vec![[T::zero(); 5]; width * height]
    }

    #[test]
    fn test_builder() {
        // Act
        let actual = SlicSegmentation::builder();

        // Assert
        assert_eq!(
            actual,
            Builder {
                segments: 64,
                compactness: 1.0,
                max_iter: 10,
                tolerance: 1e-4,
                generator: SeedGenerator::default(),
                metric: DistanceMetric::Euclidean,
            }
        );
    }

    #[test]
    fn test_builder_build() {
        // Act
        let actual = SlicSegmentation::builder()
            .segments(128)
            .compactness(10.0)
            .max_iter(25)
            .tolerance(1e-8)
            .generator(SeedGenerator::RegularGrid)
            .metric(DistanceMetric::SquaredEuclidean)
            .build();

        // Assert
        assert!(actual.is_ok());

        let slic = actual.unwrap();
        assert_eq!(
            slic,
            SlicSegmentation {
                segments: 128,
                compactness: 10.0,
                max_iter: 25,
                tolerance: 1e-8,
                generator: SeedGenerator::RegularGrid,
                metric: DistanceMetric::SquaredEuclidean,
            }
        );
    }

    #[rstest]
    #[case::invalid_segments(
        0,
        1.0,
        10,
        1e-3,
        DistanceMetric::Euclidean,
        SlicError::InvalidSegments(0)
    )]
    #[case::invalid_compactness(
        64,
        0.0,
        10,
        1e-3,
        DistanceMetric::Euclidean,
        SlicError::InvalidCompactness(0.0)
    )]
    #[case::invalid_iterations(
        64,
        1.0,
        0,
        1e-3,
        DistanceMetric::Euclidean,
        SlicError::InvalidIterations(0)
    )]
    #[case::invalid_tolerance(
        64,
        1.0,
        10,
        0.0,
        DistanceMetric::Euclidean,
        SlicError::InvalidTolerance(0.0)
    )]
    fn test_build_error(
        #[case] segments: usize,
        #[case] compactness: f64,
        #[case] max_iter: usize,
        #[case] tolerance: f64,
        #[case] metric: DistanceMetric,
        #[case] expected: SlicError<f64>,
    ) {
        // Act
        let actual = SlicSegmentation::builder()
            .segments(segments)
            .compactness(compactness)
            .max_iter(max_iter)
            .tolerance(tolerance)
            .metric(metric)
            .build();

        // Assert
        assert!(actual.is_err());

        let error = actual.unwrap_err();
        assert_eq!(error, expected);
    }

    #[test]
    fn test_build_error_compactness_nan() {
        // Act
        let actual = SlicSegmentation::builder().compactness(f64::NAN).build();

        // Assert
        assert!(actual.is_err());

        let error = actual.unwrap_err();
        assert_eq!(
            error.to_string(),
            "Compactness must be greater than zero: NaN"
        );
    }

    #[test]
    fn test_build_error_tolerance_nan() {
        // Act
        let actual = SlicSegmentation::builder().tolerance(f64::NAN).build();

        // Assert
        assert!(actual.is_err());

        let error = actual.unwrap_err();
        assert_eq!(
            error.to_string(),
            "Tolerance must be greater than zero: NaN"
        );
    }

    #[test]
    #[cfg(feature = "image")]
    fn test_segment() {
        // Arrange
        let image_data = ImageData::load("../../gfx/flags/za.png").unwrap();
        let slic = SlicSegmentation::<f64>::builder()
            .segments(32)
            .build()
            .unwrap();

        // Act
        let width = image_data.width() as usize;
        let height = image_data.height() as usize;
        let pixels: Vec<_> = image_data.pixels().collect();
        let actual = slic.segment(width, height, &pixels);

        // Assert
        assert!(actual.is_ok());

        let label_image = actual.unwrap();
        let segments: Vec<_> = label_image.segments().collect();
        assert_eq!(segments.len(), 32);
    }

    #[test]
    fn test_segment_unexpected_length() {
        // Arrange
        let slic = SlicSegmentation::builder().segments(32).build().unwrap();

        // Act
        let pixels = sample_pixels::<f64>(48, 26);
        let actual = slic.segment(48, 27, &pixels);

        // Assert
        assert!(actual.is_err());

        let error = actual.unwrap_err();
        assert_eq!(
            error,
            SlicError::UnexpectedLength(MatrixError::InvalidPoints(48, 27))
        );
    }
}

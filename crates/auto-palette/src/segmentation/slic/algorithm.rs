use rustc_hash::FxHashMap;

use crate::{
    image::{Pixel, LABXY_CHANNELS},
    math::{matrix::MatrixView, DistanceMetric, FloatNumber},
    segmentation::{
        helper::gradient,
        label::{Builder as SegmentBuilder, LabelImage},
        seed::SeedGenerator,
        segment::SegmentMetadata,
        slic::{config::SlicConfig, error::SlicError},
        Segmentation,
    },
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

impl<T> TryFrom<SlicConfig<T>> for SlicSegmentation<T>
where
    T: FloatNumber,
{
    type Error = SlicError<T>;

    fn try_from(config: SlicConfig<T>) -> Result<Self, Self::Error> {
        if config.segments == 0 {
            return Err(SlicError::InvalidSegments(config.segments));
        }
        if config.compactness <= T::zero() || config.compactness.is_nan() {
            return Err(SlicError::InvalidCompactness(config.compactness));
        }
        if config.max_iter == 0 {
            return Err(SlicError::InvalidIterations(config.max_iter));
        }
        if config.tolerance <= T::zero() || config.tolerance.is_nan() {
            return Err(SlicError::InvalidTolerance(config.tolerance));
        }
        Ok(Self {
            segments: config.segments,
            compactness: config.compactness,
            max_iter: config.max_iter,
            tolerance: config.tolerance,
            generator: config.generator,
            metric: config.metric,
        })
    }
}

impl<T> SlicSegmentation<T>
where
    T: FloatNumber,
{
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
        centers: &mut FxHashMap<usize, Pixel<T>>,
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

            matrix.neighbors_within(col, row, radius).for_each(
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
        let mut centers: FxHashMap<_, _> = seeds
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

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::{math::matrix::MatrixError, segmentation::seed::SeedGenerator, ImageData};

    #[must_use]
    fn sample_pixels<T>(width: usize, height: usize) -> Vec<Pixel<T>>
    where
        T: FloatNumber,
    {
        vec![[T::zero(); 5]; width * height]
    }

    #[test]
    fn test_try_from() {
        // Act
        let config = SlicConfig::<f64>::default()
            .segments(128)
            .compactness(10.0)
            .max_iter(25)
            .tolerance(1e-8)
            .generator(SeedGenerator::RegularGrid)
            .metric(DistanceMetric::SquaredEuclidean);
        let actual = SlicSegmentation::try_from(config);

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
    fn test_try_from_error(
        #[case] segments: usize,
        #[case] compactness: f64,
        #[case] max_iter: usize,
        #[case] tolerance: f64,
        #[case] metric: DistanceMetric,
        #[case] expected: SlicError<f64>,
    ) {
        // Act
        let config = SlicConfig::default()
            .segments(segments)
            .compactness(compactness)
            .max_iter(max_iter)
            .tolerance(tolerance)
            .metric(metric);
        let actual = SlicSegmentation::try_from(config);

        // Assert
        assert!(actual.is_err());

        let error = actual.unwrap_err();
        assert_eq!(error, expected);
    }

    #[test]
    fn test_try_from_compactness_nan() {
        // Act
        let config = SlicConfig::<f64>::default().compactness(f64::NAN);
        let actual = SlicSegmentation::try_from(config);

        // Assert
        assert!(actual.is_err());

        let error = actual.unwrap_err();
        assert_eq!(
            error.to_string(),
            "Compactness must be greater than zero: NaN"
        );
    }

    #[test]
    fn test_try_from_tolerance_nan() {
        // Act
        let config = SlicConfig::<f64>::default().tolerance(f64::NAN);
        let actual = SlicSegmentation::try_from(config);

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
        let config = SlicConfig::default().segments(32);
        let slic = SlicSegmentation::<f64>::try_from(config).unwrap();

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
        let config = SlicConfig::default().segments(32);
        let slic = SlicSegmentation::<f64>::try_from(config).unwrap();

        // Act
        let pixels = sample_pixels::<f64>(48, 26);
        let actual = slic.segment(48, 27, &pixels);

        // Assert
        assert!(actual.is_err());

        let error = actual.unwrap_err();
        assert_eq!(
            error,
            SlicError::UnexpectedLength(MatrixError::DimensionMismatch {
                cols: 48,
                rows: 27,
                expected: 48 * 27,
                actual: 48 * 26,
            })
        );
    }
}

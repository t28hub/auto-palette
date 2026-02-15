use crate::{
    image::{
        segmentation::{
            kmeans::{config::KmeansConfig, KmeansError},
            label::{Builder as SegmentBuilder, LabelImage},
            seed::SeedGenerator,
            segment::SegmentMetadata,
            Segmentation,
        },
        Pixel,
    },
    math::{
        neighbors::{kdtree::KdTreeSearch, NeighborSearch},
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

impl<T> TryFrom<KmeansConfig<T>> for KmeansSegmentation<T>
where
    T: FloatNumber,
{
    type Error = KmeansError<T>;

    fn try_from(config: KmeansConfig<T>) -> Result<Self, Self::Error> {
        if config.segments == 0 {
            return Err(KmeansError::InvalidSegments);
        }
        if config.max_iter == 0 {
            return Err(KmeansError::InvalidIterations);
        }
        if config.tolerance <= T::zero() || config.tolerance.is_nan() {
            return Err(KmeansError::InvalidTolerance(config.tolerance));
        }
        Ok(Self {
            segments: config.segments,
            max_iter: config.max_iter,
            tolerance: config.tolerance,
            generator: config.generator,
            metric: config.metric,
        })
    }
}

impl<T> KmeansSegmentation<T>
where
    T: FloatNumber,
{
    #[must_use]
    fn iterate(
        &self,
        pixels: &[Pixel<T>],
        mask: &[bool],
        centers: &mut [Pixel<T>],
        builder: &mut SegmentBuilder<T>,
    ) -> bool {
        builder.iter_mut().for_each(SegmentMetadata::clear);

        let center_search = KdTreeSearch::with_leaf_size(centers, self.metric, 16);
        for (index, pixel) in pixels.iter().enumerate() {
            if !mask[index] {
                continue;
            }

            if let Some(nearest) = center_search.search_nearest(pixel) {
                builder.get_mut(&nearest.index()).insert(index, pixel);
            }
        }

        let mut converged = true;
        for segment in builder.iter() {
            let Some(old_center) = centers.get_mut(segment.label()) else {
                continue;
            };

            let new_center = segment.center();
            let diff = self.metric.measure(old_center, new_center);
            if diff > self.tolerance {
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
    ) -> Result<LabelImage<T>, Self::Err> {
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
        let mut builder = LabelImage::builder(width, height);
        for _ in 0..self.max_iter {
            if self.iterate(pixels, mask, &mut centers, &mut builder) {
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
    use crate::{image::segmentation::seed::SeedGenerator, ImageData};

    #[test]
    fn test_try_from() {
        // Act
        let config = KmeansConfig::<f64>::default()
            .segments(10)
            .max_iter(100)
            .tolerance(1e-4)
            .generator(SeedGenerator::RegularGrid)
            .metric(DistanceMetric::SquaredEuclidean);
        let actual = KmeansSegmentation::try_from(config);

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
    fn test_try_from_error(
        #[case] segments: usize,
        #[case] max_iter: usize,
        #[case] tolerance: f64,
        #[case] expected: KmeansError<f64>,
    ) {
        // Act
        let config = KmeansConfig::default()
            .segments(segments)
            .max_iter(max_iter)
            .tolerance(tolerance);
        let actual = KmeansSegmentation::try_from(config);

        // Assert
        assert!(actual.is_err());

        let error = actual.unwrap_err();
        assert_eq!(error, expected);
    }

    #[test]
    fn test_try_from_tolerance_nan() {
        // Act
        let config = KmeansConfig::<f64>::default().tolerance(f64::NAN);
        let actual = KmeansSegmentation::try_from(config);

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
        let image_data = ImageData::load("../../gfx/flags/za.png").unwrap();
        let config = KmeansConfig::default()
            .segments(24)
            .max_iter(5)
            .tolerance(1e-4);
        let segmentation = KmeansSegmentation::<f64>::try_from(config).unwrap();

        // Act
        let width = image_data.width() as usize;
        let height = image_data.height() as usize;
        let pixels: Vec<_> = image_data.pixels().collect();
        let actual = segmentation.segment(width, height, &pixels);

        // Assert
        assert!(actual.is_ok());

        let label_image = actual.unwrap();
        let segments: Vec<_> = label_image.segments().collect();
        assert_eq!(segments.len(), 24);
    }
}

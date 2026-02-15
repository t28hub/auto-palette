use std::collections::VecDeque;

use crate::{
    image::{
        segmentation::{
            fastdbscan::{config::FastDbscanConfig, error::FastDbscanError},
            label::{Builder as LabelImageBuilder, LabelImage},
            Segmentation,
        },
        Pixel,
    },
    math::{
        neighbors::{kdtree::KdTreeSearch, Neighbor, NeighborSearch},
        DistanceMetric,
    },
    FloatNumber,
};

/// Image segmentation algorithm using DBSCAN++ clustering algorithm.
#[derive(Debug, PartialEq)]
pub struct FastDbscanSegmentation<T>
where
    T: FloatNumber,
{
    min_pixels: usize,
    epsilon: T,
    probability: T,
    metric: DistanceMetric,
}

impl<T> TryFrom<FastDbscanConfig<T>> for FastDbscanSegmentation<T>
where
    T: FloatNumber,
{
    type Error = FastDbscanError<T>;

    fn try_from(config: FastDbscanConfig<T>) -> Result<Self, Self::Error> {
        if config.min_pixels == 0 {
            return Err(FastDbscanError::InvalidMinPixels);
        }
        if config.epsilon <= T::zero() || config.epsilon.is_nan() {
            return Err(FastDbscanError::InvalidEpsilon(config.epsilon));
        }
        if !(T::zero()..=T::one()).contains(&config.probability) {
            return Err(FastDbscanError::OutOfRangeProbability(config.probability));
        }
        Ok(Self {
            min_pixels: config.min_pixels,
            epsilon: config.epsilon,
            probability: config.probability,
            metric: config.metric,
        })
    }
}

impl<T> FastDbscanSegmentation<T>
where
    T: FloatNumber,
{
    /// Maximum number of leaf nodes in the KdTree.
    const MAX_LEAF_SIZE: usize = 16;

    /// Label for unlabelled pixels.
    const LABEL_UNLABELED: usize = usize::MAX;

    /// Label for ignored pixels.
    const LABEL_IGNORED: usize = usize::MAX - 1;

    /// Label for noise pixels.
    const LABEL_NOISE: usize = usize::MAX - 2;

    #[must_use]
    fn select_core_pixels(&self, pixels: &[Pixel<T>], mask: &[bool]) -> Vec<Pixel<T>> {
        let sampling_step = (T::one() / self.probability)
            .round()
            .trunc_to_usize()
            .max(1);

        let pixel_search = KdTreeSearch::with_leaf_size(pixels, self.metric, Self::MAX_LEAF_SIZE);
        pixels
            .iter()
            .zip(mask)
            .step_by(sampling_step)
            .filter_map(|(pixel, &is_candidate)| {
                if !is_candidate {
                    return None;
                }

                let neighbors = pixel_search.search_within_radius(pixel, self.epsilon);
                (neighbors.len() >= self.min_pixels).then_some(*pixel)
            })
            .collect()
    }

    #[must_use]
    fn label_core_pixels<S>(&self, core_pixels: &[Pixel<T>], core_pixel_search: &S) -> Vec<usize>
    where
        S: NeighborSearch<T, 5>,
    {
        let mut labels = vec![Self::LABEL_UNLABELED; core_pixels.len()];
        let mut current_label = 0;
        for (index, core_pixel) in core_pixels.iter().enumerate() {
            // Skip already labeled core pixels
            if labels[index] != Self::LABEL_UNLABELED {
                continue;
            }

            let neighbors = core_pixel_search.search_within_radius(core_pixel, self.epsilon);
            // Not enough neighbors to form a segment
            if neighbors.len() < self.min_pixels {
                labels[index] = Self::LABEL_NOISE;
                continue;
            }

            // Label the core pixel and its neighbors if they are not labeled
            let mut queue: VecDeque<_> = neighbors.into();
            self.expand_segment(
                core_pixels,
                core_pixel_search,
                &mut queue,
                &mut labels,
                current_label,
            );

            current_label += 1;
        }
        labels
    }

    #[inline]
    fn expand_segment<S>(
        &self,
        pixels: &[Pixel<T>],
        pixel_search: &S,
        queue: &mut VecDeque<Neighbor<T>>,
        labels: &mut [usize],
        current_label: usize,
    ) where
        S: NeighborSearch<T, 5>,
    {
        while let Some(neighbor) = queue.pop_front() {
            let neighbor_index = neighbor.index();
            // Label the neighbor with the current segment label
            if labels[neighbor_index] == Self::LABEL_NOISE {
                labels[neighbor_index] = current_label;
                continue;
            }

            // Skip if the neighbor is already labeled
            if labels[neighbor_index] != Self::LABEL_UNLABELED
                || labels[neighbor_index] == Self::LABEL_IGNORED
            {
                continue;
            }

            labels[neighbor_index] = current_label;

            let neighbor_pixel = &pixels[neighbor_index];
            let secondary_neighbors =
                pixel_search.search_within_radius(neighbor_pixel, self.epsilon);
            if secondary_neighbors.len() >= self.min_pixels {
                queue.extend(secondary_neighbors);
            }
        }
    }

    fn build_segments<S>(
        &self,
        builder: &mut LabelImageBuilder<T>,
        pixels: &[Pixel<T>],
        mask: &[bool],
        core_pixel_search: &S,
        core_labels: &[usize],
    ) where
        S: NeighborSearch<T, 5>,
    {
        for (index, pixel) in pixels.iter().enumerate() {
            if !mask[index] {
                continue;
            }

            let Some(nearest) = core_pixel_search.search_nearest(pixel) else {
                continue;
            };

            if nearest.distance() > self.epsilon {
                continue;
            }

            let core_label = core_labels[nearest.index()];
            // Skip unlabelled, noise, or ignored pixels
            if core_label == Self::LABEL_UNLABELED
                || core_label == Self::LABEL_NOISE
                || core_label == Self::LABEL_IGNORED
            {
                continue;
            }
            builder.get_mut(&core_label).insert(index, pixel);
        }
    }
}

impl<T> Segmentation<T> for FastDbscanSegmentation<T>
where
    T: FloatNumber,
{
    type Err = FastDbscanError<T>;

    fn segment_with_mask(
        &self,
        width: usize,
        height: usize,
        pixels: &[Pixel<T>],
        mask: &[bool],
    ) -> Result<LabelImage<T>, Self::Err> {
        if pixels.len() != width * height {
            return Err(FastDbscanError::UnexpectedLength {
                actual: pixels.len(),
                expected: width * height,
            });
        }

        let mut builder = LabelImage::builder(width, height);
        let core_pixels = self.select_core_pixels(pixels, mask);
        if core_pixels.is_empty() {
            return Ok(builder.build());
        }

        let core_pixel_search =
            KdTreeSearch::with_leaf_size(&core_pixels, self.metric, Self::MAX_LEAF_SIZE);
        let core_labels = self.label_core_pixels(&core_pixels, &core_pixel_search);
        self.build_segments(&mut builder, pixels, mask, &core_pixel_search, &core_labels);
        Ok(builder.build())
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::{ImageData, Rgba};

    #[test]
    fn test_try_from() {
        // Act
        let config = FastDbscanConfig::<f64>::default()
            .min_pixels(8)
            .epsilon(0.05)
            .probability(0.25)
            .metric(DistanceMetric::Euclidean);
        let actual = FastDbscanSegmentation::try_from(config);

        // Assert
        assert!(actual.is_ok());

        let segmentation = actual.unwrap();
        assert_eq!(
            segmentation,
            FastDbscanSegmentation {
                min_pixels: 8,
                epsilon: 0.05,
                probability: 0.25,
                metric: DistanceMetric::Euclidean,
            }
        );
    }

    #[rstest]
    #[case::invalid_min_pixels(0, 0.02, 0.1, FastDbscanError::InvalidMinPixels)]
    #[case::invalid_epsilon(5, -0.01, 0.1, FastDbscanError::InvalidEpsilon(-0.01))]
    #[case::invalid_probability_more(5, 0.02, 1.1, FastDbscanError::OutOfRangeProbability(1.1))]
    #[case::invalid_probability_less(5, 0.02, -0.1, FastDbscanError::OutOfRangeProbability(-0.1))]
    fn test_try_from_error(
        #[case] min_pixels: usize,
        #[case] epsilon: f64,
        #[case] probability: f64,
        #[case] expected: FastDbscanError<f64>,
    ) {
        // Act
        let config = FastDbscanConfig::<f64>::default()
            .min_pixels(min_pixels)
            .epsilon(f64::from_f64(epsilon))
            .probability(f64::from_f64(probability));
        let actual = FastDbscanSegmentation::try_from(config);

        // Assert
        assert!(actual.is_err());

        let error = actual.unwrap_err();
        assert_eq!(error, expected);
    }

    #[test]
    fn test_try_from_epsilon_nan() {
        // Act
        let config = FastDbscanConfig::<f64>::default()
            .min_pixels(5)
            .epsilon(f64::NAN)
            .probability(0.1);
        let actual = FastDbscanSegmentation::try_from(config);

        // Assert
        assert!(actual.is_err());

        let error = actual.unwrap_err();
        assert_eq!(
            error.to_string(),
            "The epsilon value must be greater than zero and not NaN: NaN"
        );
    }

    #[test]
    fn test_try_from_probability_nan() {
        // Act
        let config = FastDbscanConfig::<f64>::default()
            .min_pixels(5)
            .epsilon(0.02)
            .probability(f64::NAN);
        let actual = FastDbscanSegmentation::try_from(config);

        // Assert
        assert!(actual.is_err());

        let error = actual.unwrap_err();
        assert_eq!(
            error.to_string(),
            "The probability value must be in the range (0, 1]: NaN"
        );
    }

    #[test]
    #[cfg(feature = "image")]
    fn test_segment() {
        // Arrange
        let image_data = ImageData::load("../../gfx/flags/za.png").unwrap();
        let config = FastDbscanConfig::default()
            .min_pixels(10)
            .epsilon(0.03)
            .probability(0.1)
            .metric(DistanceMetric::Euclidean);
        let segmentation = FastDbscanSegmentation::<f64>::try_from(config).unwrap();

        // Act
        let width = image_data.width() as usize;
        let height = image_data.height() as usize;
        let pixels: Vec<_> = image_data.pixels().collect();
        let actual = segmentation.segment(width, height, &pixels);

        // Assert
        assert!(actual.is_ok());

        let label_image = actual.unwrap();
        let segments: Vec<_> = label_image.segments().collect();
        assert!(!segments.is_empty());
        assert!(segments.len() >= 64);
        for segment in segments {
            assert!(segment.len() >= 10);
        }
    }

    #[test]
    fn test_segment_empty_image() {
        // Arrange
        let segmentation =
            FastDbscanSegmentation::<f64>::try_from(FastDbscanConfig::default()).unwrap();

        // Act
        let width = 0;
        let height = 0;
        let pixels = Vec::new();
        let actual = segmentation.segment(width, height, &pixels);

        // Assert
        assert!(actual.is_ok());

        let label_image = actual.unwrap();
        assert_eq!(label_image.width(), 0);
        assert_eq!(label_image.height(), 0);
    }

    #[test]
    fn test_segment_with_mask() {
        // Arrange
        let image_data = ImageData::load("../../gfx/flags/np.png").unwrap();
        let config = FastDbscanConfig::default()
            .min_pixels(10)
            .epsilon(0.03)
            .probability(0.1)
            .metric(DistanceMetric::Euclidean);
        let segmentation = FastDbscanSegmentation::<f64>::try_from(config).unwrap();

        let width = image_data.width() as usize;
        let height = image_data.height() as usize;
        let (pixels, mask) = image_data
            .pixels_with_filter::<f64, _>(&|rgba: &Rgba| rgba[3] != 0)
            .fold(
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

        // Act
        let actual = segmentation.segment_with_mask(width, height, &pixels, &mask);

        // Assert
        assert!(actual.is_ok());

        let label_image = actual.unwrap();
        let segments: Vec<_> = label_image.segments().collect();
        assert!(!segments.is_empty());
        assert!(segments.len() >= 6);
        for segment in segments {
            assert!(segment.len() >= 10);
        }
    }

    #[test]
    fn test_segment_with_mask_empty_image() {
        // Arrange
        let segmentation =
            FastDbscanSegmentation::<f64>::try_from(FastDbscanConfig::default()).unwrap();

        // Act
        let width = 0;
        let height = 0;
        let pixels = Vec::new();
        let mask = Vec::new();
        let actual = segmentation.segment_with_mask(width, height, &pixels, &mask);

        // Assert
        assert!(actual.is_ok());

        let label_image = actual.unwrap();
        let segments: Vec<_> = label_image.segments().collect();
        assert!(segments.is_empty());
        assert_eq!(segments.len(), 0);
    }

    #[test]
    fn test_segment_with_mask_unexpected_length() {
        // Arrange
        let segmentation =
            FastDbscanSegmentation::<f64>::try_from(FastDbscanConfig::default()).unwrap();

        // Act
        let width = 16;
        let height = 9;
        let pixels = vec![Pixel::default(); width * height - 1];
        let mask = vec![true; width * height - 1];
        let actual = segmentation.segment_with_mask(width, height, &pixels, &mask);

        // Assert
        assert!(actual.is_err());
        assert_eq!(
            actual.unwrap_err(),
            FastDbscanError::UnexpectedLength {
                actual: pixels.len(),
                expected: width * height
            }
        );
    }
}

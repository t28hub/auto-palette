use std::collections::VecDeque;

use rustc_hash::FxHashMap;

use crate::{
    image::Pixel,
    math::{
        neighbors::{kdtree::KdTreeSearch, Neighbor, NeighborSearch},
        DistanceMetric,
        FloatNumber,
    },
    segmentation::{
        dbscan::{config::DbscanConfig, error::DbscanError},
        label::{Builder as LabelImageBuilder, LabelImage},
        Segmentation,
    },
};

/// Image segmentation algorithm using DBSCAN (Density-Based Spatial Clustering of Applications with Noise).
///
/// This algorithm is used to group pixels in an image into segments based on their spatial density.
/// The algorithm is based on the following paper:
/// [Real-Time Superpixel Segmentation by DBSCAN Clustering Algorithm](https://core.ac.uk/download/pdf/79609243.pdf)
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug, PartialEq)]
pub struct DbscanSegmentation<T>
where
    T: FloatNumber,
{
    segments: usize,
    min_pixels: usize,
    epsilon: T,
    metric: DistanceMetric,
}

impl<T> TryFrom<DbscanConfig<T>> for DbscanSegmentation<T>
where
    T: FloatNumber,
{
    type Error = DbscanError<T>;

    fn try_from(config: DbscanConfig<T>) -> Result<Self, Self::Error> {
        if config.segments == 0 {
            return Err(DbscanError::InvalidSegments);
        }
        if config.min_pixels == 0 {
            return Err(DbscanError::InvalidMinPixels);
        }
        if config.epsilon <= T::zero() || config.epsilon.is_nan() {
            return Err(DbscanError::InvalidEpsilon(config.epsilon));
        }
        Ok(Self {
            segments: config.segments,
            min_pixels: config.min_pixels,
            epsilon: config.epsilon,
            metric: config.metric,
        })
    }
}

impl<T> DbscanSegmentation<T>
where
    T: FloatNumber,
{
    /// Maximum number of leaf nodes in the KdTree.
    const MAX_LEAF_SIZE: usize = 16;

    const LABEL_UNLABELLED: usize = usize::MAX;

    const LABEL_IGNORED: usize = usize::MAX - 1;

    const LABEL_NOISE: usize = usize::MAX - 2;

    /// Merges small segments into their nearest large segment.
    ///
    /// # Arguments
    /// * `builder` - The `LabelImageBuilder` to build the label image.
    /// * `min_size` - The minimum size for a segment to be considered large.
    fn merge_segments(&self, builder: &mut LabelImageBuilder<T>, min_size: usize) {
        let (labels, centers): (Vec<_>, Vec<_>) = builder
            .iter()
            .map(|segment| (segment.label(), segment.center()))
            .unzip();
        let center_search =
            KdTreeSearch::with_leaf_size(&centers, self.metric, Self::MAX_LEAF_SIZE);

        // Merge small segments into their nearest large segment
        let relocation_table: FxHashMap<_, _> = builder
            .iter()
            .filter(|s| s.len() < min_size)
            .filter_map(|s| {
                center_search
                    .search(s.center(), 2)
                    .into_iter()
                    .find_map(|n| {
                        if labels[n.index()] != s.label() {
                            Some((s.label(), labels[n.index()]))
                        } else {
                            None
                        }
                    })
            })
            .collect();

        for (small_label, large_label) in relocation_table {
            builder.merge(&small_label, &large_label);
        }

        // Remove segments that are still below the minimum size
        let labels: Vec<_> = builder
            .iter()
            .filter_map(|s| {
                if s.len() < self.min_pixels {
                    Some(s.label())
                } else {
                    None
                }
            })
            .collect();
        for label in labels {
            builder.remove(&label);
        }
    }

    /// Converts a linear index to 2D coordinates.
    ///
    /// # Arguments
    /// * `index` - The linear index to convert.
    /// * `width` - The width of the image.
    ///
    /// # Returns
    /// A tuple containing the x and y coordinates.
    #[inline(always)]
    #[must_use]
    fn index_to_coords(index: usize, width: usize) -> (usize, usize) {
        (index % width + 1, index / width + 1)
    }
}

impl<T> Segmentation<T> for DbscanSegmentation<T>
where
    T: FloatNumber,
{
    type Err = DbscanError<T>;

    fn segment_with_mask(
        &self,
        width: usize,
        height: usize,
        pixels: &[Pixel<T>],
        mask: &[bool],
    ) -> Result<LabelImage<T>, Self::Err> {
        if width * height != pixels.len() {
            return Err(DbscanError::UnexpectedLength {
                actual: pixels.len(),
                expected: width * height,
            });
        }

        let spatial_radius = (T::from_usize(pixels.len()) / T::from_usize(self.segments))
            .sqrt()
            .round()
            .trunc_to_usize()
            .max(1);
        let segment_capacity = (width * height) / self.segments;

        let pixel_search = KdTreeSearch::with_leaf_size(pixels, self.metric, Self::MAX_LEAF_SIZE);
        let find_neighbors = |index: usize| -> Vec<Neighbor<T>> {
            let seed = &pixels[index];
            let (sx, sy) = Self::index_to_coords(index, width);
            pixel_search
                .search_within_radius(seed, self.epsilon)
                .into_iter()
                .filter(|neighbor| {
                    let neighbor_index = neighbor.index();
                    let (nx, xy) = Self::index_to_coords(neighbor_index, width);
                    nx.abs_diff(sx) + xy.abs_diff(sy) <= spatial_radius
                })
                .collect()
        };

        let mut builder = LabelImage::builder(width, height);
        let mut labels = vec![Self::LABEL_UNLABELLED; pixels.len()];

        let mut current_label = 0;
        let mut next_seed_index = 0;
        while let Some(seed_index) = labels
            .iter()
            .skip(next_seed_index)
            .position(|&label| {
                label == Self::LABEL_UNLABELLED
                    || label == Self::LABEL_IGNORED
                    || label == Self::LABEL_NOISE
            })
            .map(|offset| offset + next_seed_index)
        {
            if !mask[seed_index] {
                labels[seed_index] = Self::LABEL_IGNORED;
                next_seed_index = seed_index + 1;
                continue;
            }

            let neighbors: Vec<_> = find_neighbors(seed_index);
            if neighbors.len() < self.min_pixels {
                labels[seed_index] = Self::LABEL_NOISE;
                next_seed_index = seed_index + 1;
                continue;
            }

            let segment = builder.get_mut(&current_label);
            segment.insert(seed_index, &pixels[seed_index]);
            labels[seed_index] = current_label;

            // Expand the segment using a queue
            let mut queue: VecDeque<_> = neighbors.into();
            while let Some(neighbor) = queue.pop_front() {
                // Check if the segment is full for performance improvement
                if segment.len() >= segment_capacity {
                    break;
                }

                let neighbor_index = neighbor.index();
                if !mask[neighbor_index] {
                    labels[neighbor_index] = Self::LABEL_IGNORED;
                    continue;
                }

                if labels[neighbor_index] == Self::LABEL_NOISE {
                    labels[neighbor_index] = current_label;
                    segment.insert(neighbor_index, &pixels[neighbor_index]);
                }

                // Check if the neighbor is already labeled or ignored
                if labels[neighbor_index] != Self::LABEL_UNLABELLED {
                    continue;
                }

                labels[neighbor_index] = current_label;
                segment.insert(neighbor_index, &pixels[neighbor_index]);

                let secondary_neighbors = find_neighbors(neighbor_index);
                if secondary_neighbors.len() >= self.min_pixels {
                    queue.extend(secondary_neighbors);
                }
            }

            current_label += 1;
            next_seed_index = seed_index + 1;
        }

        let min_segment_size = (T::from_usize(pixels.len()) / T::from_usize(self.segments)
            * T::from_f64(0.5))
        .trunc_to_usize()
        .max(1);
        self.merge_segments(&mut builder, min_segment_size);
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
        let config = DbscanConfig::<f64>::default()
            .segments(32)
            .min_pixels(12)
            .epsilon(0.01)
            .metric(DistanceMetric::Euclidean);
        let actual = DbscanSegmentation::try_from(config);

        // Assert
        assert!(actual.is_ok());

        let segmentation = actual.unwrap();
        assert_eq!(
            segmentation,
            DbscanSegmentation {
                segments: 32,
                min_pixels: 12,
                epsilon: 0.01,
                metric: DistanceMetric::Euclidean,
            }
        );
    }

    #[rstest]
    #[case::invalid_segments(0, 6, 0.01, DbscanError::InvalidSegments)]
    #[case::invalid_min_pixels(32, 0, 0.01, DbscanError::InvalidMinPixels)]
    #[case::invalid_epsilon(32, 6, -0.01, DbscanError::InvalidEpsilon(-0.01))]
    fn test_try_from_error(
        #[case] segments: usize,
        #[case] min_pixels: usize,
        #[case] epsilon: f64,
        #[case] expected: DbscanError<f64>,
    ) {
        // Act
        let config = DbscanConfig::<f64>::default()
            .segments(segments)
            .min_pixels(min_pixels)
            .epsilon(epsilon);
        let actual = DbscanSegmentation::try_from(config);

        // Assert
        assert!(actual.is_err());

        let error = actual.unwrap_err();
        assert_eq!(error, expected);
    }

    #[test]
    fn test_try_from_epsilon_nan() {
        // Act
        let config = DbscanConfig::<f64>::default()
            .segments(32)
            .min_pixels(6)
            .epsilon(f64::NAN);
        let actual = DbscanSegmentation::try_from(config);

        // Assert
        assert!(actual.is_err());

        let error = actual.unwrap_err();
        assert_eq!(
            error.to_string(),
            "Epsilon must be greater than zero and not NaN: NaN"
        );
    }

    #[test]
    #[cfg(feature = "image")]
    fn test_segment() {
        // Arrange
        let image_data = ImageData::load("../../gfx/flags/za.png").unwrap();
        let config = DbscanConfig::default()
            .segments(32)
            .min_pixels(10)
            .epsilon(0.01)
            .metric(DistanceMetric::Euclidean);
        let segmentation = DbscanSegmentation::<f64>::try_from(config).unwrap();

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
        assert_eq!(segments.len(), 31);
    }

    #[test]
    fn test_segment_with_mask() {
        // Arrange
        let image_data = ImageData::load("../../gfx/flags/np.png").unwrap();
        let config = DbscanConfig::default()
            .segments(32)
            .min_pixels(10)
            .epsilon(0.01)
            .metric(DistanceMetric::Euclidean);
        let segmentation = DbscanSegmentation::<f64>::try_from(config).unwrap();

        // Create a mask that includes all pixels
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
        assert!(segments.len() >= 16);
    }

    #[test]
    fn test_segment_with_mask_unexpected_length() {
        // Arrange
        let segmentation = DbscanSegmentation::<f64>::try_from(DbscanConfig::default()).unwrap();

        // Act
        let width = 9;
        let height = 4;
        let pixels: Vec<Pixel<f64>> = vec![Pixel::default(); width * height - 1];
        let mask: Vec<bool> = vec![true; width * height - 1];

        let actual = segmentation.segment_with_mask(width, height, &pixels, &mask);

        // Assert
        assert!(actual.is_err());
        assert_eq!(
            actual.unwrap_err(),
            DbscanError::UnexpectedLength {
                actual: pixels.len(),
                expected: width * height,
            }
        );
    }
}

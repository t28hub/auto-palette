use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
    marker::PhantomData,
};

use crate::{
    image::{
        segmentation::{
            helper::gradient,
            label::LabelImage,
            seed::SeedGenerator,
            snic::error::SnicError,
            Segmentation,
        },
        Pixel,
    },
    math::{matrix::MatrixView, DistanceMetric, FloatNumber},
};

/// SNIC (Simple Non-Iterative Clustering) segmentation algorithm.
///
/// This implementation is based on the following paper:
/// [Superpixels and Polygons using Simple Non-Iterative Clustering](https://openaccess.thecvf.com/content_cvpr_2017/papers/Achanta_Superpixels_and_Polygons_CVPR_2017_paper.pdf)
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug, PartialEq)]
pub struct SnicSegmentation<T>
where
    T: FloatNumber,
{
    segments: usize,
    generator: SeedGenerator,
    metric: DistanceMetric,
    _marker: PhantomData<T>,
}

impl<T> SnicSegmentation<T>
where
    T: FloatNumber,
{
    /// Default number of segments to create.
    const DEFAULT_SEGMENTS: usize = 64;

    /// Label for unlabelled pixels.
    const LABEL_UNLABELLED: usize = usize::MAX;

    /// Label for ignored pixels.
    const LABEL_IGNORED: usize = usize::MAX - 1;

    /// Creates a new `SnicSegmentationBuilder` instance.
    ///
    /// # Returns
    /// A new `SnicSegmentationBuilder` instance with default values.
    #[must_use]
    pub fn builder() -> Builder<T> {
        Builder::default()
    }

    /// Finds the index of the lowest gradient point in the matrix.
    ///
    /// # Type Parameters
    /// * `N` - The number of dimensions.
    ///
    /// # Arguments
    /// * `matrix` - The matrix of points.
    /// * `index` - The index of the point to check.
    /// * `mask` - A mask indicating which points are valid.
    ///
    /// # Returns
    /// The index of the point with the lowest gradient in the neighborhood if it exists.
    #[must_use]
    fn find_lowest_gradient_index<const N: usize>(
        &self,
        matrix: &MatrixView<T, N>,
        index: usize,
        mask: &[bool],
    ) -> Option<usize> {
        let col = index % matrix.cols;
        let row = index / matrix.cols;

        let mut lowest_score = T::max_value();
        let mut lowest_index = None;
        matrix.neighbors(col, row).for_each(|(neighbor_index, _)| {
            if !mask[neighbor_index] {
                return;
            }

            let neighbor_col = neighbor_index % matrix.cols;
            let neighbor_row = neighbor_index / matrix.cols;
            let score = gradient(matrix, neighbor_col, neighbor_row, self.metric);
            if score < lowest_score {
                lowest_score = score;
                lowest_index = Some(neighbor_index);
            }
        });
        lowest_index
    }
}

impl<T> Segmentation<T> for SnicSegmentation<T>
where
    T: FloatNumber,
{
    type Err = SnicError;

    fn segment_with_mask(
        &self,
        width: usize,
        height: usize,
        pixels: &[Pixel<T>],
        mask: &[bool],
    ) -> Result<LabelImage<T>, Self::Err> {
        let matrix = MatrixView::new(width, height, pixels)?;

        // Initialize the seeds using a grid pattern.
        let seeds: Vec<_> = self
            .generator
            .generate(width, height, pixels, mask, self.segments)
            .into_iter()
            .map(|seed_index| {
                let found = self.find_lowest_gradient_index(&matrix, seed_index, mask);
                found.unwrap_or(seed_index)
            })
            .collect();

        // Initialize the priority queue with the seeds.
        let mut builder = LabelImage::builder(width, height);
        let mut queue = seeds.into_iter().enumerate().fold(
            BinaryHeap::with_capacity(matrix.size()),
            |mut heap, (segment_label, pixel_index)| {
                let element = Element {
                    col: pixel_index % width,
                    row: pixel_index / width,
                    distance: T::zero(),
                    segment_label,
                };
                heap.push(Reverse(element));
                heap
            },
        );

        let mut labels = vec![Self::LABEL_UNLABELLED; width * height];
        while let Some(Reverse(element)) = queue.pop() {
            let pixel_index = element.col + element.row * width;
            if !mask[pixel_index] {
                labels[pixel_index] = Self::LABEL_IGNORED;
                continue;
            }

            // Skip if the point is already assigned to a cluster.
            if labels[pixel_index] != Self::LABEL_UNLABELLED
                && labels[pixel_index] != Self::LABEL_IGNORED
            {
                continue;
            }

            // Assign the nearest cluster label to the point.
            let segment_label = element.segment_label;
            let segment = builder.get_mut(&segment_label);
            segment.insert(pixel_index, &pixels[pixel_index]);
            labels[pixel_index] = segment_label;

            let center_pixel = segment.center();
            // Traverse the neighbors of the point and add them as candidates for clustering.
            matrix
                .neighbors(element.col, element.row)
                .filter(|(neighbor_index, _)| labels[*neighbor_index] == Self::LABEL_UNLABELLED)
                .for_each(|(neighbor_index, neighbor_pixel)| {
                    let distance = self.metric.measure(center_pixel, neighbor_pixel);
                    let element = Element {
                        col: neighbor_index % width,
                        row: neighbor_index / width,
                        distance,
                        segment_label,
                    };
                    queue.push(Reverse(element));
                });
        }
        Ok(builder.build())
    }
}

/// Builder for `SnicSegmentation`.
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug, PartialEq)]
pub struct Builder<T>
where
    T: FloatNumber,
{
    segments: usize,
    generator: SeedGenerator,
    metric: DistanceMetric,
    _marker: PhantomData<T>,
}

impl<T> Builder<T>
where
    T: FloatNumber,
{
    /// Sets the number of segments to create.
    ///
    /// # Arguments
    /// * `segments` - The number of segments to create.
    ///
    /// # Returns
    /// The `SnicSegmentationBuilder` instance with the specified number of segments.
    #[must_use]
    pub fn segments(mut self, segments: usize) -> Self {
        self.segments = segments;
        self
    }

    /// Sets the seed generator to use.
    ///
    /// # Arguments
    /// * `generator` - The seed generator to use.
    ///
    /// # Returns
    /// The `SnicSegmentationBuilder` instance with the specified seed generator.
    #[allow(dead_code)]
    #[must_use]
    pub fn generator(mut self, generator: SeedGenerator) -> Self {
        self.generator = generator;
        self
    }

    /// Sets the distance metric to use.
    ///
    /// # Arguments
    /// * `metric` - The distance metric to use.
    ///
    /// # Returns
    /// The `SnicSegmentationBuilder` instance with the specified distance metric.
    #[must_use]
    pub fn metric(mut self, metric: DistanceMetric) -> Self {
        self.metric = metric;
        self
    }

    /// Builds a new `SnicSegmentation` instance.
    ///
    /// # Returns
    /// A `Result` containing the `SnicSegmentation` instance or an error if the parameters are invalid.
    pub fn build(self) -> Result<SnicSegmentation<T>, SnicError> {
        if self.segments == 0 {
            return Err(SnicError::InvalidSegments(self.segments));
        }

        Ok(SnicSegmentation {
            segments: self.segments,
            generator: self.generator,
            metric: self.metric,
            _marker: PhantomData,
        })
    }
}

impl<T> Default for Builder<T>
where
    T: FloatNumber,
{
    fn default() -> Self {
        Self {
            segments: SnicSegmentation::<T>::DEFAULT_SEGMENTS,
            generator: SeedGenerator::default(),
            metric: DistanceMetric::default(),
            _marker: PhantomData,
        }
    }
}

/// An element representing a candidate for clustering.
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug)]
struct Element<T>
where
    T: FloatNumber,
{
    /// The column index of the element.
    col: usize,

    /// The row index of the element.
    row: usize,

    /// The distance of the element from the cluster centroid.
    distance: T,

    /// The cluster label of the element.
    segment_label: usize,
}

impl<T> PartialEq for Element<T>
where
    T: FloatNumber,
{
    fn eq(&self, other: &Self) -> bool {
        self.segment_label == other.segment_label
            && self.distance == other.distance
            && self.col == other.col
            && self.row == other.row
    }
}

impl<T> Eq for Element<T> where T: FloatNumber {}

impl<T> PartialOrd for Element<T>
where
    T: FloatNumber,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Element<T>
where
    T: FloatNumber,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance
            .partial_cmp(&other.distance)
            .unwrap_or(Ordering::Less)
    }
}

#[cfg(test)]
mod tests {
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
        let actual = SnicSegmentation::<f64>::builder();

        // Assert
        assert_eq!(actual, Builder::<f64>::default());
    }

    #[test]
    fn test_builder_build() {
        // Act
        let actual = SnicSegmentation::<f64>::builder()
            .segments(128)
            .generator(SeedGenerator::RegularGrid)
            .metric(DistanceMetric::SquaredEuclidean)
            .build();

        // Assert
        assert!(actual.is_ok());

        let snic = actual.unwrap();
        assert_eq!(
            snic,
            SnicSegmentation {
                segments: 128,
                generator: SeedGenerator::RegularGrid,
                metric: DistanceMetric::SquaredEuclidean,
                _marker: PhantomData,
            }
        );
    }

    #[test]
    fn test_builder_build_error() {
        // Act
        let actual = SnicSegmentation::<f64>::builder().segments(0).build();

        // Assert
        assert!(actual.is_err());

        let error = actual.unwrap_err();
        assert_eq!(error, SnicError::InvalidSegments(0));
    }

    #[test]
    #[cfg(feature = "image")]
    fn test_segment() {
        // Arrange
        let image_data = ImageData::load("../../gfx/flags/za.png").unwrap();
        let snic = SnicSegmentation::<f64>::builder()
            .segments(32)
            .build()
            .unwrap();

        // Act
        let width = image_data.width() as usize;
        let height = image_data.height() as usize;
        let pixels = image_data.pixels().collect::<Vec<_>>();
        let actual = snic.segment(width, height, &pixels);

        // Assert
        assert!(actual.is_ok());

        let label_image = actual.unwrap();
        let segments: Vec<_> = label_image.segments().collect();
        assert_eq!(segments.len(), 32);
    }

    #[test]
    fn test_segment_unexpected_length() {
        // Arrange
        let snic = SnicSegmentation::<f64>::builder()
            .segments(12)
            .generator(SeedGenerator::RegularGrid)
            .metric(DistanceMetric::SquaredEuclidean)
            .build()
            .unwrap();

        // Act
        let width = 32;
        let height = 18;
        let pixels = sample_pixels(width - 1, height);
        let actual = snic.segment(width, height, &pixels);

        // Assert
        assert!(actual.is_err());

        let error = actual.unwrap_err();
        assert_eq!(
            error,
            SnicError::UnexpectedLength(MatrixError::DimensionMismatch {
                cols: width,
                rows: height,
                expected: width * height,
                actual: (width - 1) * height,
            })
        );
    }

    #[test]
    fn test_element_eq() {
        // Arrange
        let element1 = Element {
            col: 1,
            row: 2,
            distance: 3.0,
            segment_label: 4,
        };
        let element2 = Element {
            col: 1,
            row: 2,
            distance: 3.0,
            segment_label: 4,
        };

        // Act & Assert
        assert_eq!(element1, element2);
    }

    #[test]
    fn test_element_cmp() {
        // Arrange
        let element1 = Element {
            col: 1,
            row: 2,
            distance: 3.0,
            segment_label: 4,
        };
        let element2 = Element {
            col: 1,
            row: 2,
            distance: 4.0,
            segment_label: 4,
        };

        // Act & Assert
        assert_eq!(element1.cmp(&element1), Ordering::Equal);
        assert_eq!(element2.cmp(&element2), Ordering::Equal);
        assert_eq!(element1.cmp(&element2), Ordering::Less);
        assert_eq!(element2.cmp(&element1), Ordering::Greater);
    }

    #[test]
    fn test_element_partial_cmp() {
        // Arrange
        let element1 = Element {
            col: 1,
            row: 2,
            distance: 3.0,
            segment_label: 4,
        };
        let element2 = Element {
            col: 1,
            row: 2,
            distance: 4.0,
            segment_label: 4,
        };

        // Act & Assert
        assert_eq!(element1.partial_cmp(&element1), Some(Ordering::Equal));
        assert_eq!(element2.partial_cmp(&element2), Some(Ordering::Equal));
        assert_eq!(element1.partial_cmp(&element2), Some(Ordering::Less));
        assert_eq!(element2.partial_cmp(&element1), Some(Ordering::Greater));
    }

    #[test]
    fn test_cmp_nan() {
        // Arrange
        let element1 = Element {
            col: 1,
            row: 2,
            distance: 3.0,
            segment_label: 4,
        };
        let element2 = Element {
            col: 1,
            row: 2,
            distance: f64::NAN,
            segment_label: 4,
        };

        // Act & Assert
        assert_eq!(element1.cmp(&element2), Ordering::Less);
        assert_eq!(element2.cmp(&element1), Ordering::Less);
    }
}

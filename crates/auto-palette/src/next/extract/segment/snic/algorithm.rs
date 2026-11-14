use std::{cmp::Reverse, collections::BinaryHeap, num::NonZeroU32};

use crate::next::{
    color::{AlphaColor, Oklab},
    extract::{
        nearest::nearest,
        segment::{
            centroid::Centroid,
            point::Point,
            position::Position,
            segments::SegmentId,
            Segmenter,
        },
    },
    image::{ImageBuf, Size},
    math::DistanceMetric,
};

/// A pixel waiting to join a segment: its distance as bits, its index, and the segment.
type Candidate = Reverse<(u64, u32, u32)>;

/// Simple Non-Iterative Clustering, which grows segments from seeds on a grid.
///
/// From "Superpixels and Polygons using Simple Non-Iterative Clustering" (Achanta and
/// Süsstrunk, CVPR 2017).
#[derive(Debug, Clone, Copy)]
pub(crate) struct Snic {
    /// The number of segments to aim for.
    segments: NonZeroU32,
    /// How much distance counts against color difference.
    compactness: f64,
}

impl Snic {
    /// The default for `segments`.
    const DEFAULT_SEGMENTS: NonZeroU32 = NonZeroU32::new(128).unwrap();

    /// The default for `compactness`.
    const DEFAULT_COMPACTNESS: f64 = 0.1;

    /// Creates SNIC with the given options.
    ///
    /// # Arguments
    /// * `segments` - The number of segments to aim for.
    /// * `compactness` - How much distance counts against color difference.
    ///
    /// # Returns
    /// SNIC, or `None` if `compactness` is negative or not finite.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "called by the builder, which is not implemented yet"
        )
    )]
    pub(crate) fn new(segments: NonZeroU32, compactness: f64) -> Option<Self> {
        if !compactness.is_finite() || compactness < 0.0 {
            return None;
        }
        Some(Self {
            segments,
            compactness,
        })
    }
}

impl Default for Snic {
    fn default() -> Self {
        Self {
            segments: Self::DEFAULT_SEGMENTS,
            compactness: Self::DEFAULT_COMPACTNESS,
        }
    }
}

impl Segmenter for Snic {
    fn label(&self, image: &ImageBuf<AlphaColor<Oklab>>) -> Vec<Option<SegmentId>> {
        let size = image.size();
        let grid = size.fit(self.segments);
        let scale = self.compactness / Position::segment_length(size, grid.pixel_count());
        let points = Point::of_pixels(image, scale);
        let mut labels = vec![None; points.len()];
        let mut centroids = Vec::new();
        let mut queue = BinaryHeap::new();
        for index in grid_centers(size, grid) {
            if points[index].is_weightless() {
                continue;
            }
            add_seed(index, &mut queue, &mut centroids);
        }
        grow_segments(&points, size, &mut queue, &mut centroids, &mut labels);

        // Pixels cut off from every seed by transparent pixels start segments of their own.
        for index in 0..points.len() {
            if labels[index].is_some() || points[index].is_weightless() {
                continue;
            }
            add_seed(index, &mut queue, &mut centroids);
            grow_segments(&points, size, &mut queue, &mut centroids, &mut labels);
        }
        labels
    }
}

/// Adds a pixel as the seed of a new segment.
///
/// # Arguments
/// * `index` - The index of the pixel, row by row.
/// * `queue` - The pixels waiting to join a segment.
/// * `centroids` - The centroid of each segment.
fn add_seed(index: usize, queue: &mut BinaryHeap<Candidate>, centroids: &mut Vec<Centroid>) {
    queue.push(Reverse((0, index as u32, centroids.len() as u32)));
    centroids.push(Centroid::default());
}

/// Returns the pixels at the centers of the cells of a grid.
///
/// # Arguments
/// * `size` - The size of the image, with at least one pixel.
/// * `grid` - The number of columns and rows, each from 1 to the matching side of `size`.
///
/// # Returns
/// The index of each seed pixel, row by row.
fn grid_centers(size: Size, grid: Size) -> impl Iterator<Item = usize> {
    let width = size.width as usize;
    (0..grid.height).flat_map(move |row| {
        let y = nearest(row, grid.height, size.height) as usize;
        (0..grid.width)
            .map(move |column| y * width + nearest(column, grid.width, size.width) as usize)
    })
}

/// Adds the closest waiting pixel to its segment until no pixel is waiting.
///
/// # Arguments
/// * `points` - The point of each pixel, with positions scaled.
/// * `size` - The size of the image.
/// * `queue` - The pixels waiting to join a segment.
/// * `centroids` - The centroid of each segment.
/// * `labels` - The segment of each pixel, or `None` if it has not joined one yet.
fn grow_segments(
    points: &[Point],
    size: Size,
    queue: &mut BinaryHeap<Candidate>,
    centroids: &mut [Centroid],
    labels: &mut [Option<SegmentId>],
) {
    while let Some(Reverse((_, index, segment))) = queue.pop() {
        let index = index as usize;
        if labels[index].is_some() {
            continue;
        }
        labels[index] = Some(SegmentId::new(segment));
        let centroid = &mut centroids[segment as usize];
        centroid.add(points[index]);
        let center = centroid
            .point()
            .expect("a segment grows only from opaque pixels");
        for neighbor in neighbors(index, size) {
            let point = points[neighbor];
            if labels[neighbor].is_some() || point.is_weightless() {
                continue;
            }
            let distance = center.distance(point, DistanceMetric::SquaredEuclidean);
            // Non-negative floats order the same as their bits. Adding 1 puts candidates behind
            // the seeds, whose key is 0, even at distance 0.
            queue.push(Reverse((distance.to_bits() + 1, neighbor as u32, segment)));
        }
    }
}

/// Returns the pixels above, below, left, and right of a pixel.
///
/// # Arguments
/// * `index` - The index of the pixel, row by row.
/// * `size` - The size of the image.
///
/// # Returns
/// The indices of the neighbors inside the image.
fn neighbors(index: usize, size: Size) -> impl Iterator<Item = usize> {
    let (width, height) = (size.width as usize, size.height as usize);
    let (column, row) = (index % width, index / width);
    [
        (column > 0).then(|| index - 1),
        (column + 1 < width).then(|| index + 1),
        (row > 0).then(|| index - width),
        (row + 1 < height).then(|| index + width),
    ]
    .into_iter()
    .flatten()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::next::extract::segment::test_helpers::{image, BLACK, WHITE};

    /// Creates SNIC that aims for `segments` segments.
    fn snic(segments: u32) -> Snic {
        Snic {
            segments: NonZeroU32::new(segments).unwrap(),
            ..Snic::default()
        }
    }

    #[rstest]
    #[case::zero(0.0)]
    #[case::positive(0.1)]
    fn test_new_accepts_compactness(#[case] compactness: f64) {
        // Act
        let actual = Snic::new(Snic::DEFAULT_SEGMENTS, compactness);

        // Assert
        assert!(actual.is_some());
    }

    #[rstest]
    #[case::negative(-0.1)]
    #[case::nan(f64::NAN)]
    #[case::infinity(f64::INFINITY)]
    fn test_new_rejects_compactness(#[case] compactness: f64) {
        // Act
        let actual = Snic::new(Snic::DEFAULT_SEGMENTS, compactness);

        // Assert
        assert!(actual.is_none());
    }

    #[test]
    fn test_label_every_opaque_pixel() {
        // Arrange
        let image = image(&[[Some(BLACK), Some(WHITE), Some(BLACK)]; 3]);

        // Act
        let actual = snic(2).label(&image);

        // Assert
        assert!(actual.iter().all(Option::is_some));
    }

    #[test]
    fn test_label_skips_transparent_pixels() {
        // Arrange
        let image = image(&[[Some(BLACK), None]]);

        // Act
        let actual = snic(1).label(&image);

        // Assert
        assert_eq!(actual, [Some(SegmentId::new(0)), None]);
    }

    #[test]
    fn test_label_splits_by_color() {
        // Arrange: black on the left and white on the right.
        let image = image(&[[Some(BLACK), Some(BLACK), Some(WHITE), Some(WHITE)]; 2]);

        // Act
        let actual = snic(2).label(&image);

        // Assert
        let (left, right) = (actual[0], actual[3]);
        assert_ne!(left, right);
        assert_eq!(actual, [left, left, right, right, left, left, right, right]);
    }

    #[test]
    fn test_label_keeps_seeds_without_compactness() {
        // Arrange: one color, so every candidate is as close as a seed. The seeds are pixels 1
        // and 3.
        let image = image(&[[Some(BLACK); 4]]);
        let snic = Snic {
            compactness: 0.0,
            ..snic(2)
        };

        // Act
        let actual = snic.label(&image);

        // Assert
        assert_eq!(
            (actual[1], actual[3]),
            (Some(SegmentId::new(0)), Some(SegmentId::new(1)))
        );
    }

    #[test]
    fn test_label_isolated_pixels() {
        // Arrange: the only seed is the transparent pixel in the middle, and transparent pixels
        // cut the corners off from each other.
        let image = image(&[
            [Some(BLACK), None, Some(BLACK)],
            [None, None, None],
            [Some(BLACK), None, Some(BLACK)],
        ]);

        // Act
        let actual = snic(1).label(&image);

        // Assert: each corner starts a segment of its own.
        let corners = [actual[0], actual[2], actual[6], actual[8]];
        assert_eq!(corners, [0, 1, 2, 3].map(|id| Some(SegmentId::new(id))));
    }

    #[rstest]
    #[case::every_pixel(Size::new(4, 4), Size::new(4, 4), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15])]
    #[case::half(Size::new(4, 4), Size::new(2, 2), vec![5, 7, 13, 15])]
    #[case::one_row(Size::new(4, 1), Size::new(1, 1), vec![2])]
    fn test_grid_centers(#[case] size: Size, #[case] grid: Size, #[case] expected: Vec<usize>) {
        // Act
        let actual: Vec<usize> = grid_centers(size, grid).collect();

        // Assert
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case::corner(0, vec![1, 3])]
    #[case::edge(1, vec![0, 2, 4])]
    #[case::center(4, vec![3, 5, 1, 7])]
    #[case::last(8, vec![7, 5])]
    fn test_neighbors(#[case] index: usize, #[case] expected: Vec<usize>) {
        // Act
        let actual: Vec<usize> = neighbors(index, Size::new(3, 3)).collect();

        // Assert
        assert_eq!(actual, expected);
    }
}

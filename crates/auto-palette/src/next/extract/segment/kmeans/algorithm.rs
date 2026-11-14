use std::num::NonZeroU32;

use super::initialization::Initialization;
use crate::next::{
    color::{AlphaColor, Oklab},
    extract::segment::{
        centroid::Centroid,
        point::Point,
        position::Position,
        segments::SegmentId,
        Segmenter,
    },
    image::ImageBuf,
    math::DistanceMetric,
};

/// Lloyd's k-means over color and position, which groups nearby pixels of similar colors.
#[derive(Debug, Clone, Copy)]
pub(crate) struct KMeans {
    /// The number of segments to aim for.
    segments: NonZeroU32,
    /// The largest number of iterations.
    max_iterations: NonZeroU32,
    /// The squared distance at or below which a moving center counts as settled.
    tolerance: f64,
    /// How much distance counts against color difference.
    compactness: f64,
    /// How to pick the initial centers.
    initialization: Initialization,
}

impl KMeans {
    /// The default for `segments`.
    const DEFAULT_SEGMENTS: NonZeroU32 = NonZeroU32::new(128).unwrap();

    /// The default for `max_iterations`.
    const DEFAULT_MAX_ITERATIONS: NonZeroU32 = NonZeroU32::new(50).unwrap();

    /// The default for `tolerance`.
    const DEFAULT_TOLERANCE: f64 = 1e-5;

    /// The default for `compactness`.
    const DEFAULT_COMPACTNESS: f64 = 0.1;

    /// Creates k-means with the given options.
    ///
    /// # Arguments
    /// * `segments` - The number of segments to aim for.
    /// * `max_iterations` - The largest number of iterations.
    /// * `tolerance` - The squared distance at or below which a moving center counts as settled.
    /// * `compactness` - How much distance counts against color difference.
    /// * `initialization` - How to pick the initial centers.
    ///
    /// # Returns
    /// K-means, or `None` if `tolerance` or `compactness` is negative or not finite.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "called by the builder, which is not implemented yet"
        )
    )]
    pub(crate) fn new(
        segments: NonZeroU32,
        max_iterations: NonZeroU32,
        tolerance: f64,
        compactness: f64,
        initialization: Initialization,
    ) -> Option<Self> {
        if !tolerance.is_finite() || tolerance < 0.0 {
            return None;
        }
        if !compactness.is_finite() || compactness < 0.0 {
            return None;
        }
        Some(Self {
            segments,
            max_iterations,
            tolerance,
            compactness,
            initialization,
        })
    }
}

impl Default for KMeans {
    fn default() -> Self {
        Self {
            segments: Self::DEFAULT_SEGMENTS,
            max_iterations: Self::DEFAULT_MAX_ITERATIONS,
            tolerance: Self::DEFAULT_TOLERANCE,
            compactness: Self::DEFAULT_COMPACTNESS,
            initialization: Initialization::default(),
        }
    }
}

impl Segmenter for KMeans {
    fn label(&self, image: &ImageBuf<AlphaColor<Oklab>>) -> Vec<Option<SegmentId>> {
        let count = self.segments.get() as usize;
        let scale = self.compactness / Position::segment_length(image.size(), count);
        let points = Point::of_pixels(image, scale);
        // Weightless points never become centers.
        let mut centers = self.initialization.centers(&points, count);
        let mut assigned = vec![0; points.len()];
        for _ in 0..self.max_iterations.get() {
            let centroids = assign(&points, &centers, &mut assigned);
            let shift = update(&mut centers, &centroids);
            if shift <= self.tolerance {
                break;
            }
        }

        segment_ids(&points, &assigned, centers.len())
    }
}

/// Numbers the centers that have points, from 0 in center order.
///
/// # Arguments
/// * `points` - The point of each pixel.
/// * `assigned` - The center of each point.
/// * `count` - The number of centers.
///
/// # Returns
/// The segment of each point, or `None` for a weightless point.
fn segment_ids(points: &[Point], assigned: &[usize], count: usize) -> Vec<Option<SegmentId>> {
    let mut occupied = vec![false; count];
    for (point, &center) in points.iter().zip(assigned) {
        if !point.is_weightless() {
            occupied[center] = true;
        }
    }
    let ids: Vec<u32> = occupied
        .iter()
        .scan(0, |next, &is_occupied| {
            let id = *next;
            *next += u32::from(is_occupied);
            Some(id)
        })
        .collect();
    points
        .iter()
        .zip(assigned)
        .map(|(point, &center)| (!point.is_weightless()).then(|| SegmentId::new(ids[center])))
        .collect()
}

/// Assigns each point to its nearest center.
///
/// # Arguments
/// * `points` - The point of each pixel.
/// * `centers` - The centers, at least one.
/// * `assigned` - The center of each point, updated for the points with weight.
///
/// # Returns
/// The centroid of the points assigned to each center.
fn assign(points: &[Point], centers: &[Point], assigned: &mut [usize]) -> Vec<Centroid> {
    let mut centroids = vec![Centroid::default(); centers.len()];
    for (point, assigned) in points.iter().zip(assigned) {
        if point.is_weightless() {
            continue;
        }
        *assigned = nearest_center(*point, centers);
        centroids[*assigned].add(*point);
    }
    centroids
}

/// Moves each center to its centroid.
///
/// # Arguments
/// * `centers` - The centers to move. A center with no points assigned stays where it is.
/// * `centroids` - The centroid of the points assigned to each center.
///
/// # Returns
/// The squared distance that the center moving the most moved.
fn update(centers: &mut [Point], centroids: &[Centroid]) -> f64 {
    let mut shift: f64 = 0.0;
    for (center, centroid) in centers.iter_mut().zip(centroids) {
        let Some(moved) = centroid.point() else {
            continue;
        };
        shift = shift.max(center.distance(moved, DistanceMetric::SquaredEuclidean));
        *center = moved;
    }
    shift
}

/// Returns the index of the center closest to a point.
///
/// # Arguments
/// * `point` - The point to match.
/// * `centers` - The centers, at least one.
///
/// # Returns
/// The index of the closest center, the first one on a tie.
fn nearest_center(point: Point, centers: &[Point]) -> usize {
    let (index, _) = centers
        .iter()
        .map(|center| point.distance(*center, DistanceMetric::SquaredEuclidean))
        .enumerate()
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .expect("there is at least one center");
    index
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::next::extract::segment::test_helpers::{gradient, image, BLACK, WHITE};

    /// Creates k-means with the default options except `tolerance` and `compactness`.
    fn new(tolerance: f64, compactness: f64) -> Option<KMeans> {
        KMeans::new(
            KMeans::DEFAULT_SEGMENTS,
            KMeans::DEFAULT_MAX_ITERATIONS,
            tolerance,
            compactness,
            Initialization::default(),
        )
    }

    /// Creates k-means that aims for `segments` segments.
    fn kmeans(segments: u32) -> KMeans {
        KMeans {
            segments: NonZeroU32::new(segments).unwrap(),
            ..KMeans::default()
        }
    }

    #[rstest]
    #[case::zero(0.0)]
    #[case::positive(1e-6)]
    fn test_new_accepts_tolerance(#[case] tolerance: f64) {
        // Act
        let actual = new(tolerance, KMeans::DEFAULT_COMPACTNESS);

        // Assert
        assert!(actual.is_some());
    }

    #[rstest]
    #[case::negative(-1e-6)]
    #[case::nan(f64::NAN)]
    #[case::infinity(f64::INFINITY)]
    fn test_new_rejects_tolerance(#[case] tolerance: f64) {
        // Act
        let actual = new(tolerance, KMeans::DEFAULT_COMPACTNESS);

        // Assert
        assert!(actual.is_none());
    }

    #[rstest]
    #[case::zero(0.0)]
    #[case::positive(0.1)]
    fn test_new_accepts_compactness(#[case] compactness: f64) {
        // Act
        let actual = new(KMeans::DEFAULT_TOLERANCE, compactness);

        // Assert
        assert!(actual.is_some());
    }

    #[rstest]
    #[case::negative(-0.1)]
    #[case::nan(f64::NAN)]
    #[case::infinity(f64::INFINITY)]
    fn test_new_rejects_compactness(#[case] compactness: f64) {
        // Act
        let actual = new(KMeans::DEFAULT_TOLERANCE, compactness);

        // Assert
        assert!(actual.is_none());
    }

    #[rstest]
    #[case::plus_plus(Initialization::default())]
    #[case::farthest(Initialization::Farthest)]
    fn test_label_groups_colors_across_the_image(#[case] initialization: Initialization) {
        // Arrange: black and white in a checkerboard, so neither color is connected.
        let image = image(&[[Some(BLACK), Some(WHITE)], [Some(WHITE), Some(BLACK)]]);
        let kmeans = KMeans {
            initialization,
            ..kmeans(2)
        };

        // Act
        let actual = kmeans.label(&image);

        // Assert
        assert_ne!(actual[0], actual[1]);
        assert_eq!(actual, [actual[0], actual[1], actual[1], actual[0]]);
    }

    #[test]
    fn test_label_splits_one_color_by_position() {
        // Arrange
        let image = image(&[[Some(BLACK), Some(BLACK)]]);

        // Act
        let actual = kmeans(2).label(&image);

        // Assert
        assert_ne!(actual[0], actual[1]);
    }

    #[test]
    fn test_label_ignores_position_without_compactness() {
        // Arrange
        let image = image(&[[Some(BLACK), Some(BLACK)]]);
        let kmeans = KMeans {
            compactness: 0.0,
            ..kmeans(2)
        };

        // Act
        let actual = kmeans.label(&image);

        // Assert
        assert_eq!(actual[0], actual[1]);
    }

    #[test]
    fn test_label_skips_transparent_pixels() {
        // Arrange
        let image = image(&[[Some(BLACK), None]]);

        // Act
        let actual = kmeans(1).label(&image);

        // Assert
        assert_eq!(actual, [Some(SegmentId::new(0)), None]);
    }

    #[rstest]
    #[case::plus_plus(Initialization::default())]
    #[case::farthest(Initialization::Farthest)]
    fn test_label_fewer_pixels_than_segments(#[case] initialization: Initialization) {
        // Arrange
        let image = image(&[[Some(BLACK), Some(WHITE), Some(BLACK)]]);
        let kmeans = KMeans {
            initialization,
            ..kmeans(8)
        };

        // Act
        let actual = kmeans.label(&image);

        // Assert: one segment per pixel.
        let mut ids: Vec<usize> = actual.iter().map(|id| id.unwrap().index()).collect();
        ids.sort_unstable();
        assert_eq!(ids, [0, 1, 2]);
    }

    #[test]
    fn test_label_is_reproducible() {
        // Arrange: a gradient, so the initial centers depend on the random draws.
        let image = image(&[gradient::<16>().map(Some)]);

        // Act
        let first = kmeans(4).label(&image);
        let second = kmeans(4).label(&image);

        // Assert
        assert_eq!(first, second);
    }

    /// Creates a point of `color` at the origin with weight 1.0.
    fn point(color: Oklab) -> Point {
        Point::new(color, Position::new(0.0, 0.0), 1.0)
    }

    #[test]
    fn test_segment_ids_skips_centers_without_points() {
        // Arrange: no point is assigned to center 0.
        let points = [point(BLACK), point(WHITE)];

        // Act
        let actual = segment_ids(&points, &[2, 1], 3);

        // Assert
        assert_eq!(actual, [Some(SegmentId::new(1)), Some(SegmentId::new(0))]);
    }

    #[test]
    fn test_assign() {
        // Arrange
        let points = [point(BLACK), point(WHITE)];
        let centers = [point(WHITE), point(BLACK)];
        let mut assigned = [0; 2];

        // Act
        assign(&points, &centers, &mut assigned);

        // Assert
        assert_eq!(assigned, [1, 0]);
    }

    #[test]
    fn test_assign_skips_weightless_points() {
        // Arrange
        let points = [Point::new(BLACK, Position::new(0.0, 0.0), 0.0)];
        let centers = [point(BLACK)];
        let mut assigned = [0; 1];

        // Act
        let actual = assign(&points, &centers, &mut assigned);

        // Assert
        assert!(actual[0].point().is_none());
    }

    #[test]
    fn test_update_moves_center_to_mean() {
        // Arrange
        let mut centers = [point(BLACK)];
        let mut centroid = Centroid::default();
        centroid.add(point(WHITE));

        // Act
        update(&mut centers, &[centroid]);

        // Assert
        assert_eq!(centers[0].color().l, 1.0);
    }

    #[test]
    fn test_update_keeps_center_without_points() {
        // Arrange
        let mut centers = [point(BLACK)];

        // Act
        update(&mut centers, &[Centroid::default()]);

        // Assert
        assert_eq!(centers[0].color().l, 0.0);
    }

    #[test]
    fn test_update_returns_largest_shift() {
        // Arrange: the first center moves by 0.5 in lightness and the second by 1.0.
        let mut centers = [point(BLACK), point(BLACK)];
        let mut gray = Centroid::default();
        gray.add(point(Oklab::new(0.5, 0.0, 0.0)));
        let mut white = Centroid::default();
        white.add(point(WHITE));

        // Act
        let actual = update(&mut centers, &[gray, white]);

        // Assert: the squared distance of the second.
        assert_eq!(actual, 1.0);
    }

    #[test]
    fn test_nearest_center() {
        // Arrange
        let gray = point(Oklab::new(0.6, 0.0, 0.0));
        let centers = [point(BLACK), point(WHITE)];

        // Act
        let actual = nearest_center(gray, &centers);

        // Assert
        assert_eq!(actual, 1);
    }
}

use std::num::NonZeroU32;

use super::{assignment::Assignment, hamerly::Hamerly};
use crate::next::{
    extraction::{centroid::Centroid, point::Point},
    math::DistanceMetric,
};

/// Lloyd's algorithm, which moves each center to the mean of its points until the centers settle.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Lloyd {
    /// The largest number of iterations.
    max_iterations: NonZeroU32,
    /// The squared distance at or below which a moving center counts as settled.
    tolerance: f64,
}

impl Lloyd {
    /// The default for `max_iterations`.
    const DEFAULT_MAX_ITERATIONS: NonZeroU32 = NonZeroU32::new(50).unwrap();

    /// The default for `tolerance`.
    const DEFAULT_TOLERANCE: f64 = 1e-5;

    /// Creates Lloyd's algorithm with the given options.
    ///
    /// # Arguments
    /// * `max_iterations` - The largest number of iterations.
    /// * `tolerance` - The squared distance at or below which a moving center counts as settled.
    ///
    /// # Returns
    /// Lloyd's algorithm, or `None` if `tolerance` is negative or not finite.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "called by the builder, which is not implemented yet"
        )
    )]
    pub(crate) fn new(max_iterations: NonZeroU32, tolerance: f64) -> Option<Self> {
        if !tolerance.is_finite() || tolerance < 0.0 {
            return None;
        }
        Some(Self {
            max_iterations,
            tolerance,
        })
    }

    /// Moves each center to the mean of its points until the centers settle.
    ///
    /// # Arguments
    /// * `points` - The points to cluster.
    /// * `centers` - The initial centers, at least one, moved in place.
    ///
    /// # Returns
    /// The center of each point.
    pub(super) fn move_centers(&self, points: &[Point], centers: &mut [Point]) -> Vec<usize> {
        self.move_centers_with::<Hamerly>(points, centers)
    }

    /// Moves the centers like [`Self::move_centers`], finding the nearest centers with `A`.
    ///
    /// # Arguments
    /// * `points` - The points to cluster.
    /// * `centers` - The initial centers, at least one, moved in place.
    ///
    /// # Returns
    /// The center of each point.
    fn move_centers_with<A: Assignment>(
        &self,
        points: &[Point],
        centers: &mut [Point],
    ) -> Vec<usize> {
        let mut assignment = A::new(points, centers);
        let mut assigned = vec![0; points.len()];
        let mut moves = vec![0.0; centers.len()];
        for _ in 0..self.max_iterations.get() {
            let centroids = assignment.assign(points, centers, &moves, &mut assigned);
            moves = update(centers, &centroids);
            let shift = moves.iter().copied().fold(0.0, f64::max);
            if shift.powi(2) <= self.tolerance {
                break;
            }
        }
        assigned
    }
}

impl Default for Lloyd {
    fn default() -> Self {
        Self {
            max_iterations: Self::DEFAULT_MAX_ITERATIONS,
            tolerance: Self::DEFAULT_TOLERANCE,
        }
    }
}

/// Moves each center to its centroid.
///
/// # Arguments
/// * `centers` - The centers to move. A center with no points assigned stays where it is.
/// * `centroids` - The centroid of the points assigned to each center.
///
/// # Returns
/// How far each center moved.
fn update(centers: &mut [Point], centroids: &[Centroid]) -> Vec<f64> {
    let mut moves = vec![0.0; centers.len()];
    for ((center, centroid), moved) in centers.iter_mut().zip(centroids).zip(&mut moves) {
        let Some(mean) = centroid.point() else {
            continue;
        };
        *moved = center.distance(mean, DistanceMetric::Euclidean);
        *center = mean;
    }
    moves
}

#[cfg(test)]
mod tests {
    use std::array;

    use rstest::rstest;

    use super::*;
    use crate::next::{
        extraction::{
            clustering::{naive::Naive, random::SplitMix64},
            test_helpers::{gray, point, BLACK, WHITE},
        },
        math::Vector,
    };

    #[rstest]
    #[case::zero(0.0)]
    #[case::positive(1e-6)]
    fn test_new_accepts_tolerance(#[case] tolerance: f64) {
        // Act
        let actual = Lloyd::new(Lloyd::DEFAULT_MAX_ITERATIONS, tolerance);

        // Assert
        assert!(actual.is_some());
    }

    #[rstest]
    #[case::negative(-1e-6)]
    #[case::nan(f64::NAN)]
    #[case::infinity(f64::INFINITY)]
    fn test_new_rejects_tolerance(#[case] tolerance: f64) {
        // Act
        let actual = Lloyd::new(Lloyd::DEFAULT_MAX_ITERATIONS, tolerance);

        // Assert
        assert!(actual.is_none());
    }

    #[test]
    fn test_move_centers_hamerly_matches_naive() {
        // Arrange: random points, so that many sit near the border of two centers.
        let mut random = SplitMix64::new(0);
        let points: Vec<Point> = (0..256)
            .map(|_| Point::of_vector(Vector(array::from_fn(|_| random.next_f64())), 1.0))
            .collect();
        let initial = &points[..8];
        let mut expected_centers = initial.to_vec();
        let expected = Lloyd::default().move_centers_with::<Naive>(&points, &mut expected_centers);
        let mut centers = initial.to_vec();

        // Act
        let actual = Lloyd::default().move_centers_with::<Hamerly>(&points, &mut centers);

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_move_centers_hamerly_matches_naive_on_tie() {
        // Arrange: after the first move, gray 0.25 is as near the center at 0.0 as the one at 0.5.
        let points = [0.0, 0.25, 0.75].map(gray).map(point);
        let lloyd = Lloyd {
            tolerance: 0.0,
            ..Lloyd::default()
        };
        let mut expected_centers = [0.0, 0.25].map(gray).map(point);
        let expected = lloyd.move_centers_with::<Naive>(&points, &mut expected_centers);
        let mut centers = [0.0, 0.25].map(gray).map(point);

        // Act
        let actual = lloyd.move_centers_with::<Hamerly>(&points, &mut centers);

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_move_centers_stops_when_centers_settle() {
        // Arrange: any move counts as settled, so gray 0.2 stays with the center at 0.3.
        let points = [0.0, 0.2, 1.0].map(gray).map(point);
        let mut centers = [0.0, 0.3].map(gray).map(point);
        let lloyd = Lloyd {
            tolerance: f64::MAX,
            ..Lloyd::default()
        };

        // Act
        let actual = lloyd.move_centers_with::<Naive>(&points, &mut centers);

        // Assert
        assert_eq!(actual, [0, 1, 1]);
    }

    #[test]
    fn test_move_centers_stops_at_max_iterations() {
        // Arrange: one iteration leaves gray 0.2 with the center at 0.3.
        let points = [0.0, 0.2, 1.0].map(gray).map(point);
        let mut centers = [0.0, 0.3].map(gray).map(point);
        let lloyd = Lloyd {
            max_iterations: NonZeroU32::MIN,
            tolerance: 0.0,
        };

        // Act
        let actual = lloyd.move_centers_with::<Naive>(&points, &mut centers);

        // Assert
        assert_eq!(actual, [0, 1, 1]);
    }

    #[test]
    fn test_move_centers_repeats_until_centers_settle() {
        // Arrange: gray 0.2 moves to the center at 0.0 once the other center moves to 0.6.
        let points = [0.0, 0.2, 1.0].map(gray).map(point);
        let mut centers = [0.0, 0.3].map(gray).map(point);
        let lloyd = Lloyd {
            tolerance: 0.0,
            ..Lloyd::default()
        };

        // Act
        let actual = lloyd.move_centers_with::<Naive>(&points, &mut centers);

        // Assert
        assert_eq!(actual, [0, 0, 1]);
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
        assert_eq!(centers[0].color().to_oklab().l, 1.0);
    }

    #[test]
    fn test_update_keeps_center_without_points() {
        // Arrange
        let mut centers = [point(BLACK)];

        // Act
        update(&mut centers, &[Centroid::default()]);

        // Assert
        assert_eq!(centers[0].color().to_oklab().l, 0.0);
    }

    #[test]
    fn test_update_returns_moves() {
        // Arrange: the first center moves by 0.5 in lightness, and the second has no points.
        let mut centers = [point(BLACK), point(BLACK)];
        let mut centroid = Centroid::default();
        centroid.add(point(gray(0.5)));

        // Act
        let actual = update(&mut centers, &[centroid, Centroid::default()]);

        // Assert
        assert_eq!(actual, [0.5, 0.0]);
    }
}

use crate::next::{extract::point::Point, math::DistanceMetric};

/// The nearest and the second nearest centers of a point.
#[derive(Debug)]
pub(super) struct Nearest {
    /// The index of the nearest center, the first one on a tie.
    pub(super) center: usize,
    /// The distance to the nearest center.
    pub(super) distance: f64,
    /// The distance to the second nearest center, infinite if there is one center.
    pub(super) runner_up: f64,
}

/// Finds the nearest centers of a point by measuring every center.
///
/// # Arguments
/// * `point` - The point to match.
/// * `centers` - The centers, at least one.
///
/// # Returns
/// The nearest center and the distances to the two nearest centers.
pub(super) fn nearest(point: Point, centers: &[Point]) -> Nearest {
    let mut center = 0;
    let mut squared = f64::INFINITY;
    let mut runner_up_squared = f64::INFINITY;
    for (index, &other) in centers.iter().enumerate() {
        let distance = point.distance(other, DistanceMetric::SquaredEuclidean);
        if distance < squared {
            runner_up_squared = squared;
            squared = distance;
            center = index;
        } else if distance < runner_up_squared {
            runner_up_squared = distance;
        }
    }
    Nearest {
        center,
        distance: squared.sqrt(),
        runner_up: runner_up_squared.sqrt(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        assert_approx_eq,
        next::extract::test_helpers::{gray, point, BLACK, WHITE},
    };

    #[test]
    fn test_nearest() {
        // Arrange
        let centers = [point(BLACK), point(WHITE)];

        // Act
        let actual = nearest(point(gray(0.6)), &centers);

        // Assert: 0.4 to white and 0.6 to black.
        assert_eq!(actual.center, 1);
        assert_approx_eq!(actual.distance, 0.4);
        assert_approx_eq!(actual.runner_up, 0.6);
    }

    #[test]
    fn test_nearest_first_on_tie() {
        // Arrange
        let centers = [point(BLACK), point(WHITE)];

        // Act
        let actual = nearest(point(gray(0.5)), &centers);

        // Assert
        assert_eq!(actual.center, 0);
        assert_eq!(actual.distance, actual.runner_up);
    }

    #[test]
    fn test_nearest_one_center() {
        // Act
        let actual = nearest(point(BLACK), &[point(WHITE)]);

        // Assert
        assert_eq!(actual.runner_up, f64::INFINITY);
    }
}

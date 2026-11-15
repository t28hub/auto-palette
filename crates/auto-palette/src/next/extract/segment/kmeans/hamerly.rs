use super::{assignment::Assignment, nearest::nearest};
use crate::next::{
    extract::segment::{centroid::Centroid, point::Point},
    math::DistanceMetric,
};

/// Hamerly's assignment, which skips the points whose bounds keep them on their center.
///
/// From ["Making k-means Even Faster"](https://doi.org/10.1137/1.9781611972801.12) (SDM 2010).
#[derive(Debug)]
pub(super) struct Hamerly {
    /// The bounds of each point.
    bounds: Vec<Bounds>,
}

impl Hamerly {
    /// Widens the bounds of every point by how far the centers moved.
    ///
    /// # Arguments
    /// * `moves` - How far each center moved.
    /// * `assigned` - The center of each point.
    fn loosen(&mut self, moves: &[f64], assigned: &[usize]) {
        let others = largest_other_moves(moves);
        for (bounds, &center) in self.bounds.iter_mut().zip(assigned) {
            bounds.loosen(moves[center], others[center]);
        }
    }
}

impl Assignment for Hamerly {
    fn new(points: &[Point], _centers: &[Point]) -> Self {
        Self {
            bounds: vec![Bounds::UNKNOWN; points.len()],
        }
    }

    fn assign(
        &mut self,
        points: &[Point],
        centers: &[Point],
        moves: &[f64],
        assigned: &mut [usize],
    ) -> Vec<Centroid> {
        self.loosen(moves, assigned);
        let separations = separations(centers);
        let mut centroids = vec![Centroid::default(); centers.len()];
        for ((&point, center), bounds) in points.iter().zip(assigned).zip(&mut self.bounds) {
            if point.is_weightless() {
                continue;
            }
            bounds.reassign(point, centers, separations[*center], center);
            centroids[*center].add(point);
        }
        centroids
    }
}

/// Bounds on the distances from a point to the centers.
#[derive(Debug, Clone, Copy)]
struct Bounds {
    /// An upper bound on the distance to the center of the point.
    upper: f64,
    /// A lower bound on the distance to the other centers.
    lower: f64,
}

impl Bounds {
    /// Bounds that rule out nothing, so the point is measured.
    const UNKNOWN: Self = Self {
        upper: f64::INFINITY,
        lower: 0.0,
    };

    /// Widens the bounds by how far the centers moved.
    ///
    /// # Arguments
    /// * `own` - How far the center of the point moved.
    /// * `other` - The largest move among the other centers.
    fn loosen(&mut self, own: f64, other: f64) {
        self.upper += own;
        self.lower -= other;
    }

    /// Moves a point to its nearest center unless the bounds prove its center strictly nearest.
    ///
    /// # Arguments
    /// * `point` - The point to match.
    /// * `centers` - The centers, at least one.
    /// * `separation` - The distance from the center of the point to the nearest other center.
    /// * `center` - The center of the point, updated if another center is nearer.
    fn reassign(&mut self, point: Point, centers: &[Point], separation: f64, center: &mut usize) {
        // Strict, unlike the paper, so that a tie goes to the first center as in `nearest`.
        let bound = (separation / 2.0).max(self.lower);
        if self.upper < bound {
            return;
        }

        self.upper = point.distance(centers[*center], DistanceMetric::Euclidean);
        if self.upper < bound {
            return;
        }

        let nearest = nearest(point, centers);
        *center = nearest.center;
        self.upper = nearest.distance;
        self.lower = nearest.runner_up;
    }
}

/// Returns the distance from each center to its nearest other center.
///
/// # Arguments
/// * `centers` - The centers.
///
/// # Returns
/// The distance for each center, infinite if there is one center.
fn separations(centers: &[Point]) -> Vec<f64> {
    centers
        .iter()
        .enumerate()
        .map(|(index, &center)| {
            let squared = centers
                .iter()
                .enumerate()
                .filter(|&(other, _)| other != index)
                .map(|(_, &other)| center.distance(other, DistanceMetric::SquaredEuclidean))
                .fold(f64::INFINITY, f64::min);
            squared.sqrt()
        })
        .collect()
}

/// Returns the largest move among the other centers for each center.
///
/// # Arguments
/// * `moves` - How far each center moved.
///
/// # Returns
/// The largest move of the centers other than each center, zero if there is one center.
fn largest_other_moves(moves: &[f64]) -> Vec<f64> {
    let mut most_moved = 0;
    let mut largest = 0.0;
    let mut second = 0.0;
    for (index, &moved) in moves.iter().enumerate() {
        if moved > largest {
            second = largest;
            largest = moved;
            most_moved = index;
        } else if moved > second {
            second = moved;
        }
    }
    (0..moves.len())
        .map(|index| if index == most_moved { second } else { largest })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        assert_approx_eq,
        next::extract::segment::test_helpers::{gray, point, weightless, BLACK, WHITE},
    };

    #[test]
    fn test_new_rules_out_nothing() {
        // Arrange
        let points = [point(BLACK), point(WHITE)];

        // Act
        let actual = Hamerly::new(&points, &[point(BLACK)]);

        // Assert: one bound per point, which every distance satisfies.
        assert_eq!(actual.bounds.len(), 2);
        for bounds in actual.bounds {
            assert_eq!(bounds.upper, f64::INFINITY);
            assert_eq!(bounds.lower, 0.0);
        }
    }

    #[test]
    fn test_assign_measures_every_point_first() {
        // Arrange
        let points = [point(BLACK), point(WHITE)];
        let centers = [point(WHITE), point(BLACK)];
        let mut assigned = [0; 2];

        // Act
        Hamerly::new(&points, &centers).assign(&points, &centers, &[0.0; 2], &mut assigned);

        // Assert
        assert_eq!(assigned, [1, 0]);
    }

    #[test]
    fn test_assign_follows_moved_center() {
        // Arrange: gray 0.4 starts nearer black, then white moves to 0.5.
        let points = [gray(0.4)];
        let before = [point(BLACK), point(WHITE)];
        let after = [point(BLACK), gray(0.5)];
        let mut hamerly = Hamerly::new(&points, &before);
        let mut assigned = [0];
        hamerly.assign(&points, &before, &[0.0; 2], &mut assigned);

        // Act
        hamerly.assign(&points, &after, &[0.0, 0.5], &mut assigned);

        // Assert
        assert_eq!(assigned, [1]);
    }

    #[test]
    fn test_assign_skips_weightless_points() {
        // Arrange
        let points = [weightless(BLACK)];
        let centers = [point(BLACK)];
        let mut hamerly = Hamerly::new(&points, &centers);

        // Act
        let actual = hamerly.assign(&points, &centers, &[0.0], &mut [0]);

        // Assert
        assert!(actual[0].point().is_none());
    }

    #[test]
    fn test_loosen() {
        // Arrange
        let mut hamerly = Hamerly {
            bounds: vec![Bounds {
                upper: 1.0,
                lower: 1.0,
            }],
        };

        // Act
        hamerly.loosen(&[0.1, 0.3], &[0]);

        // Assert: the own move widens the upper bound, the other move the lower bound.
        assert_approx_eq!(hamerly.bounds[0].upper, 1.1);
        assert_approx_eq!(hamerly.bounds[0].lower, 0.7);
    }

    #[test]
    fn test_loosen_center_that_moved_most() {
        // Arrange
        let mut hamerly = Hamerly {
            bounds: vec![Bounds {
                upper: 1.0,
                lower: 1.0,
            }],
        };

        // Act
        hamerly.loosen(&[0.1, 0.3], &[1]);

        // Assert: the lower bound falls by the second largest move.
        assert_approx_eq!(hamerly.bounds[0].upper, 1.3);
        assert_approx_eq!(hamerly.bounds[0].lower, 0.9);
    }

    #[test]
    fn test_reassign_skips_point_inside_bounds() {
        // Arrange: half the separation of black and white is 0.5.
        let mut bounds = Bounds {
            upper: 0.45,
            lower: 0.0,
        };
        let mut center = 0;

        // Act
        bounds.reassign(gray(0.4), &[point(BLACK), point(WHITE)], 1.0, &mut center);

        // Assert: the upper bound is not measured again.
        assert_eq!(center, 0);
        assert_eq!(bounds.upper, 0.45);
    }

    #[test]
    fn test_reassign_skips_point_inside_lower_bound() {
        // Arrange: the lower bound is above half the separation.
        let mut bounds = Bounds {
            upper: 0.6,
            lower: 0.7,
        };
        let mut center = 0;

        // Act
        bounds.reassign(gray(0.4), &[point(BLACK), point(WHITE)], 1.0, &mut center);

        // Assert
        assert_eq!(center, 0);
        assert_eq!(bounds.upper, 0.6);
    }

    #[test]
    fn test_reassign_tightens_upper_bound() {
        // Arrange: the upper bound is loose, but the true distance 0.4 is inside.
        let mut bounds = Bounds {
            upper: 0.7,
            lower: 0.0,
        };
        let mut center = 0;

        // Act
        bounds.reassign(gray(0.4), &[point(BLACK), point(WHITE)], 1.0, &mut center);

        // Assert: no search, so the lower bound stays.
        assert_eq!(center, 0);
        assert_approx_eq!(bounds.upper, 0.4);
        assert_eq!(bounds.lower, 0.0);
    }

    #[test]
    fn test_reassign_searches_point_outside_bounds() {
        // Arrange: gray 0.4 is assigned to white, 0.6 away.
        let mut bounds = Bounds::UNKNOWN;
        let mut center = 1;

        // Act
        bounds.reassign(gray(0.4), &[point(BLACK), point(WHITE)], 1.0, &mut center);

        // Assert
        assert_eq!(center, 0);
        assert_approx_eq!(bounds.upper, 0.4);
        assert_approx_eq!(bounds.lower, 0.6);
    }

    #[test]
    fn test_reassign_searches_point_on_bound() {
        // Arrange: gray 0.5 is as near black as white, and both bounds equal 0.5.
        let mut bounds = Bounds {
            upper: 0.5,
            lower: 0.5,
        };
        let mut center = 1;

        // Act
        bounds.reassign(gray(0.5), &[point(BLACK), point(WHITE)], 1.0, &mut center);

        // Assert: the first center wins the tie, as in `nearest`.
        assert_eq!(center, 0);
    }

    #[test]
    fn test_reassign_one_center() {
        // Arrange: no other center, so the separation is infinite.
        let mut bounds = Bounds::UNKNOWN;
        let mut center = 0;

        // Act
        bounds.reassign(point(WHITE), &[point(BLACK)], f64::INFINITY, &mut center);

        // Assert: the point stays after the upper bound is measured.
        assert_eq!(center, 0);
        assert_approx_eq!(bounds.upper, 1.0);
        assert_eq!(bounds.lower, 0.0);
    }

    #[test]
    fn test_separations() {
        // Act
        let actual = separations(&[gray(0.0), gray(0.2), gray(1.0)]);

        // Assert
        assert_approx_eq!(actual[0], 0.2);
        assert_approx_eq!(actual[1], 0.2);
        assert_approx_eq!(actual[2], 0.8);
    }

    #[test]
    fn test_separations_one_center() {
        // Act
        let actual = separations(&[point(BLACK)]);

        // Assert
        assert_eq!(actual, [f64::INFINITY]);
    }

    #[test]
    fn test_largest_other_moves() {
        // Act
        let actual = largest_other_moves(&[0.1, 0.3, 0.2]);

        // Assert
        assert_eq!(actual, [0.3, 0.2, 0.3]);
    }

    #[test]
    fn test_largest_other_moves_tied_largest() {
        // Act
        let actual = largest_other_moves(&[0.3, 0.3, 0.1]);

        // Assert: each of the two largest sees the other.
        assert_eq!(actual, [0.3, 0.3, 0.3]);
    }

    #[test]
    fn test_largest_other_moves_one_center() {
        // Act
        let actual = largest_other_moves(&[0.2]);

        // Assert
        assert_eq!(actual, [0.0]);
    }
}

use super::{assignment::Assignment, nearest::nearest};
use crate::next::extraction::{centroid::Centroid, point::Point};

/// An assignment that measures every center for every point.
#[derive(Debug)]
pub(super) struct Naive;

impl<const N: usize> Assignment<N> for Naive {
    fn new(_points: &[Point<N>], _centers: &[Point<N>]) -> Self {
        Self
    }

    fn assign(
        &mut self,
        points: &[Point<N>],
        centers: &[Point<N>],
        _moves: &[f64],
        assigned: &mut [usize],
    ) -> Vec<Centroid<N>> {
        let mut centroids = vec![Centroid::default(); centers.len()];
        for (point, assigned) in points.iter().zip(assigned) {
            if point.is_weightless() {
                continue;
            }
            let nearest = nearest(*point, centers);
            *assigned = nearest.center;
            centroids[*assigned].add(*point);
        }
        centroids
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::next::extraction::test_helpers::{point, weightless, BLACK, WHITE};

    #[test]
    fn test_assign() {
        // Arrange
        let points = [point(BLACK), point(WHITE)];
        let centers = [point(WHITE), point(BLACK)];
        let mut assigned = [0; 2];

        // Act
        Naive.assign(&points, &centers, &[0.0; 2], &mut assigned);

        // Assert
        assert_eq!(assigned, [1, 0]);
    }

    #[test]
    fn test_assign_skips_weightless_points() {
        // Arrange
        let points = [weightless(BLACK)];
        let centers = [point(BLACK)];
        let mut assigned = [0; 1];

        // Act
        let actual = Naive.assign(&points, &centers, &[0.0], &mut assigned);

        // Assert
        assert!(actual[0].point().is_none());
    }
}

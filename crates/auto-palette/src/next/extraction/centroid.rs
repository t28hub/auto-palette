use super::point::Point;
use crate::next::math::Vector;

/// The weighted mean of a group of points, kept as running sums.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Centroid<const N: usize> {
    /// The weighted sum of the coordinates.
    sum: Vector<N>,
    /// The sum of the weights.
    weight: f64,
}

impl<const N: usize> Centroid<N> {
    /// Adds a point to the group, weighted by its weight.
    ///
    /// # Arguments
    /// * `point` - The point to add, counted by its weight.
    #[inline]
    pub(crate) fn add(&mut self, point: Point<N>) {
        self.sum += point.vector() * point.weight();
        self.weight += point.weight();
    }

    /// Merges the points of another group into this one.
    ///
    /// # Arguments
    /// * `other` - The centroid of the other group.
    #[inline]
    pub(crate) fn merge(&mut self, other: Self) {
        self.sum += other.sum;
        self.weight += other.weight;
    }

    /// Returns the weighted mean as a point.
    ///
    /// # Returns
    /// The mean of the points with their total weight, or `None` if the group has no weight.
    #[inline]
    pub(crate) fn point(&self) -> Option<Point<N>> {
        if self.weight == 0.0 {
            return None;
        }
        Some(Point::of_vector(self.sum / self.weight, self.weight))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        assert_approx_eq,
        next::extraction::{position::Position, test_helpers::gray},
    };

    #[test]
    fn test_point() {
        // Arrange: a dark point with weight 0.25 and a light point with weight 0.75.
        let mut centroid = Centroid::default();
        centroid.add(Point::new(gray(0.2), Position::new(0.0, 0.0), 0.25));
        centroid.add(Point::new(gray(0.6), Position::new(0.4, 0.8), 0.75));

        // Act
        let actual = centroid.point().unwrap();

        // Assert
        assert_approx_eq!(actual.color().to_oklab().l, 0.5);
        assert_approx_eq!(actual.position().x, 0.3);
        assert_approx_eq!(actual.position().y, 0.6);
        assert_eq!(actual.weight(), 1.0);
    }

    #[test]
    fn test_point_without_weight() {
        // Arrange
        let mut centroid = Centroid::default();
        centroid.add(Point::new(gray(0.2), Position::new(0.0, 0.0), 0.0));

        // Act
        let actual = centroid.point();

        // Assert
        assert!(actual.is_none());
    }
}

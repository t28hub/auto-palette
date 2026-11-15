use super::point::Point;
use crate::next::math::Vector;

/// The weighted mean color and position of a group of points, kept as running sums.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Centroid {
    /// The weighted sums of L, a, b, x, and y.
    sum: Vector<5>,
    /// The sum of the weights.
    weight: f64,
}

impl Centroid {
    /// Adds a point to the group, weighted by its weight.
    ///
    /// # Arguments
    /// * `point` - The point to add, counted by its weight.
    #[inline]
    pub(crate) fn add(&mut self, point: Point) {
        self.sum += point.vector() * point.weight();
        self.weight += point.weight();
    }

    /// Returns the weighted mean as a point.
    ///
    /// # Returns
    /// The mean color at the mean position, with the total weight, or `None` if the group has no
    /// weight.
    #[inline]
    pub(crate) fn point(&self) -> Option<Point> {
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
        next::{color::Oklab, extract::segment::position::Position},
    };

    #[test]
    fn test_point() {
        // Arrange: a dark point with weight 0.25 and a light point with weight 0.75.
        let mut centroid = Centroid::default();
        centroid.add(Point::new(
            Oklab::new(0.2, 0.0, 0.0),
            Position::new(0.0, 0.0),
            0.25,
        ));
        centroid.add(Point::new(
            Oklab::new(0.6, 0.0, 0.0),
            Position::new(0.4, 0.8),
            0.75,
        ));

        // Act
        let actual = centroid.point().unwrap();

        // Assert
        assert_approx_eq!(actual.color().l, 0.5);
        assert_approx_eq!(actual.position().x, 0.3);
        assert_approx_eq!(actual.position().y, 0.6);
        assert_eq!(actual.weight(), 1.0);
    }

    #[test]
    fn test_point_without_weight() {
        // Arrange
        let mut centroid = Centroid::default();
        centroid.add(Point::new(
            Oklab::new(0.2, 0.0, 0.0),
            Position::new(0.0, 0.0),
            0.0,
        ));

        // Act
        let actual = centroid.point();

        // Assert
        assert!(actual.is_none());
    }
}

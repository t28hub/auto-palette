use std::array;

use crate::next::math::{DistanceMetric, Vector};

/// The smallest axis-aligned box around some coordinates.
#[derive(Debug, Clone, Copy)]
pub(super) struct Bbox {
    /// The smallest value along each axis.
    min: Vector<3>,
    /// The largest value along each axis.
    max: Vector<3>,
}

impl Bbox {
    /// Creates the box of one point.
    ///
    /// # Arguments
    /// * `coordinates` - The coordinates of the point.
    ///
    /// # Returns
    /// The box that holds only `coordinates`.
    #[inline]
    pub(super) const fn of(coordinates: Vector<3>) -> Self {
        Self {
            min: coordinates,
            max: coordinates,
        }
    }

    /// Extends the box to cover another box.
    ///
    /// # Arguments
    /// * `other` - The box to cover.
    #[inline]
    pub(super) fn extend(&mut self, other: Self) {
        self.min = Vector(array::from_fn(|axis| {
            self.min.0[axis].min(other.min.0[axis])
        }));
        self.max = Vector(array::from_fn(|axis| {
            self.max.0[axis].max(other.max.0[axis])
        }));
    }

    /// Returns the center of the box.
    ///
    /// # Returns
    /// The point halfway between the smallest and largest values along each axis.
    #[inline]
    pub(super) fn center(&self) -> Vector<3> {
        let mut sum = self.min;
        sum += self.max;
        sum * 0.5
    }

    /// Measures the distance between the farthest corners of two boxes.
    ///
    /// # Arguments
    /// * `other` - The other box.
    ///
    /// # Returns
    /// The distance, at least the largest distance between their points.
    #[inline]
    pub(super) fn measure_farthest(&self, other: &Self) -> f64 {
        let spans: Vector<3> = Vector(array::from_fn(|axis| {
            (self.max.0[axis] - other.min.0[axis]).max(other.max.0[axis] - self.min.0[axis])
        }));
        spans.distance(Vector::default(), DistanceMetric::Euclidean)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_approx_eq;

    #[test]
    fn test_extend_covers_both_boxes() {
        // Arrange
        let mut bbox = Bbox::of(Vector([0.0, 0.5, 0.2]));

        // Act
        bbox.extend(Bbox::of(Vector([0.4, 0.1, 0.2])));

        // Assert
        assert_eq!(bbox.min, Vector([0.0, 0.1, 0.2]));
        assert_eq!(bbox.max, Vector([0.4, 0.5, 0.2]));
    }

    #[test]
    fn test_center_is_halfway() {
        // Arrange
        let mut bbox = Bbox::of(Vector([0.0, 0.0, 0.0]));
        bbox.extend(Bbox::of(Vector([0.4, 0.2, 0.0])));

        // Act
        let actual = bbox.center();

        // Assert
        assert_eq!(actual, Vector([0.2, 0.1, 0.0]));
    }

    #[test]
    fn test_measure_farthest_of_points_is_their_distance() {
        // Arrange
        let first = Bbox::of(Vector([0.0, 0.0, 0.0]));
        let second = Bbox::of(Vector([0.3, 0.4, 0.0]));

        // Act
        let actual = first.measure_farthest(&second);

        // Assert
        assert_approx_eq!(actual, 0.5);
    }

    #[test]
    fn test_measure_farthest_spans_both_boxes() {
        // Arrange
        let mut first = Bbox::of(Vector([0.0, 0.0, 0.0]));
        first.extend(Bbox::of(Vector([0.1, 0.0, 0.0])));
        let mut second = Bbox::of(Vector([0.2, 0.0, 0.0]));
        second.extend(Bbox::of(Vector([0.3, 0.0, 0.0])));

        // Act
        let actual = first.measure_farthest(&second);

        // Assert
        assert_approx_eq!(actual, 0.3);
    }
}

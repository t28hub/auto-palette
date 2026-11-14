use std::ops::{AddAssign, Div, Mul};

use super::DistanceMetric;

/// An `N`-dimensional vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Vector<const N: usize>(pub(crate) [f64; N]);

impl<const N: usize> Vector<N> {
    /// Returns the distance to another vector.
    ///
    /// # Arguments
    /// * `other` - The vector to measure the distance to.
    /// * `metric` - How to measure the distance.
    ///
    /// # Returns
    /// The distance between the two vectors.
    #[inline]
    pub(crate) fn distance(self, other: Self, metric: DistanceMetric) -> f64 {
        metric.measure(self, other)
    }
}

impl<const N: usize> Default for Vector<N> {
    #[inline]
    fn default() -> Self {
        Self([0.0; N])
    }
}

impl<const N: usize> AddAssign for Vector<N> {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        for i in 0..N {
            self.0[i] += other.0[i];
        }
    }
}

impl<const N: usize> Mul<f64> for Vector<N> {
    type Output = Self;

    #[inline]
    fn mul(self, scale: f64) -> Self {
        Self(self.0.map(|component| component * scale))
    }
}

impl<const N: usize> Div<f64> for Vector<N> {
    type Output = Self;

    #[inline]
    fn div(self, divisor: f64) -> Self {
        Self(self.0.map(|component| component / divisor))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_approx_eq;

    #[test]
    fn test_distance() {
        // Act
        let actual = Vector([0.0, 0.0]).distance(Vector([0.3, 0.4]), DistanceMetric::Euclidean);

        // Assert
        assert_approx_eq!(actual, 0.5);
    }

    #[test]
    fn test_default() {
        // Act
        let actual = Vector::<3>::default();

        // Assert
        assert_eq!(actual, Vector([0.0; 3]));
    }

    #[test]
    fn test_add_assign() {
        // Arrange
        let mut vector = Vector([1.0, 2.0]);

        // Act
        vector += Vector([0.5, -1.0]);

        // Assert
        assert_eq!(vector, Vector([1.5, 1.0]));
    }

    #[test]
    fn test_mul() {
        // Act
        let actual = Vector([1.0, -2.0]) * 0.5;

        // Assert
        assert_eq!(actual, Vector([0.5, -1.0]));
    }

    #[test]
    fn test_div() {
        // Act
        let actual = Vector([1.0, -2.0]) / 2.0;

        // Assert
        assert_eq!(actual, Vector([0.5, -1.0]));
    }
}

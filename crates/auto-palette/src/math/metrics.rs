use std::iter::zip;

use crate::math::{point::Point, FloatNumber};

/// Distance metrics for measuring the distance between points in N-dimensional space.
///
/// This enum provides different methods to calculate distance between points,
/// commonly used in clustering algorithms and nearest neighbor searches.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DistanceMetric {
    /// Euclidean distance: √(∑(x₁ᵢ - x₂ᵢ)²)
    ///
    /// The standard straight-line distance between two points.
    /// This is the most common distance metric used in clustering.
    #[default]
    Euclidean,

    /// Squared Euclidean distance: ∑(x₁ᵢ - x₂ᵢ)²
    ///
    /// Avoids the square root computation, making it faster for distance comparisons
    /// where relative ordering matters more than the actual distance value.
    SquaredEuclidean,
}

impl DistanceMetric {
    /// Measures the distance between two points using the specified distance metric.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type.
    /// * `N` - The dimension of the points.
    ///
    /// # Arguments
    /// * `point1` - The first point.
    /// * `point2` - The second point.
    ///
    /// # Returns
    /// The distance between the two points.
    #[inline(always)]
    #[must_use]
    pub fn measure<T, const N: usize>(&self, point1: &Point<T, N>, point2: &Point<T, N>) -> T
    where
        T: FloatNumber,
    {
        let squared_distance: T = zip(point1.iter(), point2.iter())
            .map(|(value1, value2)| {
                let diff = *value1 - *value2;
                diff * diff
            })
            .sum();

        match self {
            DistanceMetric::Euclidean => squared_distance.sqrt(),
            DistanceMetric::SquaredEuclidean => squared_distance,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean_distance() {
        // Act
        let point1 = [0.0, 1.0];
        let point2 = [1.0, 0.0];
        let distance = DistanceMetric::Euclidean.measure(&point1, &point2);

        // Assert
        assert_eq!(distance, 2.0_f32.sqrt());
    }

    #[test]
    fn test_square_euclidean_distance() {
        // Act
        let point1 = [0.0, 1.0];
        let point2 = [1.0, 0.0];
        let distance = DistanceMetric::SquaredEuclidean.measure(&point1, &point2);

        // Assert
        assert_eq!(distance, 2.0);
    }

    #[test]
    fn test_identical_points() {
        // Act & Assert
        let point = [1.0, 2.0];
        assert_eq!(DistanceMetric::Euclidean.measure(&point, &point), 0.0);
        assert_eq!(
            DistanceMetric::SquaredEuclidean.measure(&point, &point),
            0.0
        );
    }

    #[test]
    fn test_with_nan_values() {
        // Arrange
        let point1 = [0.0, f32::NAN];
        let point2 = [0.0, 0.0];

        // Act & Assert
        let euclidean = DistanceMetric::Euclidean.measure(&point1, &point2);
        assert!(euclidean.is_nan());

        let squared = DistanceMetric::SquaredEuclidean.measure(&point1, &point2);
        assert!(squared.is_nan());
    }

    #[test]
    fn test_three_dimensional_points() {
        // Act
        let point1 = [1.0, 2.0, 3.0];
        let point2 = [4.0, 5.0, 6.0];

        // Assert
        assert_eq!(
            DistanceMetric::Euclidean.measure(&point1, &point2),
            27.0_f32.sqrt()
        );
        assert_eq!(
            DistanceMetric::SquaredEuclidean.measure(&point1, &point2),
            27.0_f32
        );
    }

    #[test]
    fn test_single_dimensional_points() {
        // Arrange
        let point1: Point<f32, 1> = [3.0];
        let point2: Point<f32, 1> = [7.0];

        // Act & Assert
        let euclidean = DistanceMetric::Euclidean.measure(&point1, &point2);
        assert_eq!(euclidean, 4.0);

        let squared = DistanceMetric::SquaredEuclidean.measure(&point1, &point2);
        assert_eq!(squared, 16.0);
    }

    #[test]
    fn test_high_dimensional_points() {
        let point1: Point<f32, 10> = [1.0; 10];
        let point2: Point<f32, 10> = [2.0; 10];

        // Each dimension contributes 1.0 to squared distance
        let euclidean = DistanceMetric::Euclidean.measure(&point1, &point2);
        assert_eq!(euclidean, 10.0_f32.sqrt());

        let squared = DistanceMetric::SquaredEuclidean.measure(&point1, &point2);
        assert_eq!(squared, 10.0);
    }

    #[test]
    fn test_infinity_values() {
        // Arrange
        let point1 = [f32::INFINITY, 0.0];
        let point2 = [0.0, 0.0];

        // Act & Assert
        let euclidean = DistanceMetric::Euclidean.measure(&point1, &point2);
        assert_eq!(euclidean, f32::INFINITY);

        let squared = DistanceMetric::SquaredEuclidean.measure(&point1, &point2);
        assert_eq!(squared, f32::INFINITY);
    }
}

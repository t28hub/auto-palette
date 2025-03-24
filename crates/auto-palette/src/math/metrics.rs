use std::iter::zip;

use crate::math::{point::Point, FloatNumber};

/// DistanceMetric enum used to measure the distance between two points.
///
/// This enum provides different methods to calculate distance between points in an N-dimensional space.
#[derive(Debug, Clone, Default, PartialEq)]
pub enum DistanceMetric {
    /// The Euclidean distance, used to measure the straight-line distance between two points.
    #[default]
    Euclidean,
    /// The squared Euclidean distance, used to measure the squared straight-line distance between two points.
    SquaredEuclidean,
}

impl DistanceMetric {
    /// Measures the distance between two points using the specified distance metric.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type used for calculating the distance.
    /// * `N` - The number of dimensions.
    ///
    /// # Arguments
    /// * `point1` - The first point.
    /// * `point2` - The second point.
    ///
    /// # Returns
    /// The distance between the two points.
    #[inline]
    #[must_use]
    pub fn measure<T, const N: usize>(&self, point1: &Point<T, N>, point2: &Point<T, N>) -> T
    where
        T: FloatNumber,
    {
        match self {
            DistanceMetric::Euclidean => measure_squared_euclidean(point1, point2).sqrt(),
            DistanceMetric::SquaredEuclidean => measure_squared_euclidean(point1, point2),
        }
    }
}

/// Measures the squared Euclidean distance between two points.
///
/// This is a helper function used by both Euclidean and SquaredEuclidean metrics.
///
/// # Type Parameters
/// * `T` - The floating point type used for calculating the distance.
/// * `N` - The number of dimensions.
///
/// # Arguments
/// * `point1` - The first point.
/// * `point2` - The second point.
///
/// # Returns
/// The squared Euclidean distance between the two points.
#[inline]
#[must_use]
fn measure_squared_euclidean<T, const N: usize>(point1: &Point<T, N>, point2: &Point<T, N>) -> T
where
    T: FloatNumber,
{
    zip(point1.iter(), point2.iter())
        .map(|(value1, value2)| {
            let diff = *value1 - *value2;
            diff * diff
        })
        .sum()
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
        let point = [1.0, 2.0];
        assert_eq!(DistanceMetric::Euclidean.measure(&point, &point), 0.0);
        assert_eq!(
            DistanceMetric::SquaredEuclidean.measure(&point, &point),
            0.0
        );
    }

    #[test]
    fn test_with_nan_values() {
        let point1 = [0.0, f32::NAN];
        let point2 = [0.0, 0.0];
        let distance = DistanceMetric::Euclidean.measure(&point1, &point2);
        assert!(distance.is_nan());
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
}

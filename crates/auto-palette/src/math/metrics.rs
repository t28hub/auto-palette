use crate::math::{point::Point, FloatNumber};

/// DistanceMetric enum used to measure the distance between two points.
#[derive(Debug, Clone, Default, PartialEq)]
pub enum DistanceMetric {
    /// The Euclidean distance.
    #[default]
    Euclidean,
    /// The squared Euclidean distance.
    SquaredEuclidean,
}

impl DistanceMetric {
    /// Measures the distance between two points.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type.
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
            DistanceMetric::Euclidean => square_euclidean(point1, point2).sqrt(),
            DistanceMetric::SquaredEuclidean => square_euclidean(point1, point2),
        }
    }
}

/// Measures the squared Euclidean distance between two points.
///
/// # Type Parameters
/// * `T` - The floating point type.
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
fn square_euclidean<T, const N: usize>(point1: &Point<T, N>, point2: &Point<T, N>) -> T
where
    T: FloatNumber,
{
    point1
        .iter()
        .zip(point2.iter())
        .map(|(value1, value2)| (*value1 - *value2).powi(2))
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
}

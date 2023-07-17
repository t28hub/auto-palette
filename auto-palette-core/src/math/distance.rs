use crate::math::number::Float;
use crate::math::point::Point;

/// Enum representing distance metric.
#[derive(Debug, PartialEq, Eq)]
pub enum DistanceMetric {
    Euclidean,
    SquaredEuclidean,
}

impl DistanceMetric {
    /// Measures the distance between two points.
    ///
    /// # Type Parameters
    /// * `F` - The float type used for calculations.
    /// * `P` - The point type used for calculations.
    ///
    /// # Arguments
    /// * `point1` - The first point.
    /// * `point2` - The second point.
    ///
    /// # Returns
    /// The distance between `point1` and `point2`.
    pub fn measure<F: Float, P: Point<F>>(&self, point1: &P, point2: &P) -> F {
        match *self {
            DistanceMetric::Euclidean => squared_euclidean(point1, point2).sqrt(),
            DistanceMetric::SquaredEuclidean => squared_euclidean(point1, point2),
        }
    }
}

#[inline]
#[must_use]
fn squared_euclidean<F: Float, P: Point<F>>(point1: &P, point2: &P) -> F {
    point1
        .iter()
        .zip(point2.iter())
        .fold(F::zero(), |mut total, (value1, value2)| {
            let delta = value1 - value2;
            total += delta * delta;
            total
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::point::{Point2, Point3};

    #[test]
    fn test_euclidean() {
        let metric = DistanceMetric::Euclidean;
        assert_eq!(
            metric.measure(&Point2(0.0, 1.0), &Point2(1.0, 0.0)),
            2.0_f32.sqrt()
        );
        assert_eq!(
            metric.measure(&Point3(0.0, 1.0, 2.0), &Point3(1.0, 2.0, 3.0)),
            3.0_f32.sqrt()
        );
    }

    #[test]
    fn test_squared_euclidean() {
        let metric = DistanceMetric::SquaredEuclidean;
        assert_eq!(metric.measure(&Point2(0.0, 1.0), &Point2(1.0, 0.0)), 2.0);
        assert_eq!(
            metric.measure(&Point3(0.0, 1.0, 2.0), &Point3(1.0, 2.0, 3.0)),
            3.0
        );
    }
}

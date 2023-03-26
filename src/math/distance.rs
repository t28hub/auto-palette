use crate::math::number::Float;
use crate::math::point::Point;

/// Enum representing distance metric.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Distance {
    Euclidean,
    SquaredEuclidean,
}

impl Distance {
    /// Measures the distance between two points.
    ///
    /// # Arguments
    /// * `point1` - The first point.
    /// * `point2` - The second point.
    ///
    /// # Returns
    /// The distance between `point1` and `point2`.
    pub fn measure<F: Float, P: Point<F>>(&self, point1: &P, point2: &P) -> F {
        match *self {
            Distance::Euclidean => Distance::SquaredEuclidean.measure(point1, point2).sqrt(),
            Distance::SquaredEuclidean => point1
                .sub(*point2)
                .to_vec()
                .iter()
                .fold(F::zero(), |total, delta| total + delta.powi(2)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::point::{Point2, Point3};

    #[test]
    fn compute_should_compute_euclidean_distance() {
        let metric = Distance::Euclidean;
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
    fn compute_should_compute_squared_euclidean_distance() {
        let metric = Distance::SquaredEuclidean;
        assert_eq!(metric.measure(&Point2(0.0, 1.0), &Point2(1.0, 0.0)), 2.0);
        assert_eq!(
            metric.measure(&Point3(0.0, 1.0, 2.0), &Point3(1.0, 2.0, 3.0)),
            3.0
        );
    }
}

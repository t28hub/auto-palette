use crate::number::Float;
use ndarray::ArrayView1;
use std::fmt::Debug;

/// Enum representing a distance measure.
#[derive(Debug, PartialEq, Eq)]
pub enum DistanceMetric {
    #[allow(unused)]
    Euclidean,
    #[allow(unused)]
    SquaredEuclidean,
}

impl DistanceMetric {
    /// Measures the distance between two points.
    ///
    /// # Type parameters
    /// * `F` - The float type used for the distance.
    ///
    /// # Arguments
    /// * `point1` - The first point.
    /// * `point2` - The second point.
    ///
    /// # Returns
    /// The distance between `point1` and `point2`.
    #[must_use]
    #[allow(unused)]
    pub fn measure<F>(&self, point1: &ArrayView1<F>, point2: &ArrayView1<F>) -> F
    where
        F: Float,
    {
        assert_eq!(
            point1.len(),
            point2.len(),
            "Points must have the same dimensionality"
        );

        match *self {
            DistanceMetric::Euclidean => DistanceMetric::SquaredEuclidean
                .measure(point1, point2)
                .sqrt(),
            DistanceMetric::SquaredEuclidean => {
                point1
                    .iter()
                    .zip(point2.iter())
                    .fold(F::zero(), |mut total, (&v1, &v2)| {
                        let delta = v1 - v2;
                        total += delta * delta;
                        total
                    })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::aview1;

    #[test]
    fn test_euclidean() {
        let metric = DistanceMetric::Euclidean;

        let point1 = aview1(&[0.0, 1.0]);
        let point2 = aview1(&[1.0, 0.0]);
        assert_eq!(metric.measure(&point1, &point2), 2.0_f64.sqrt());

        let point1 = aview1(&[0.0, 1.0, 2.0]);
        let point2 = aview1(&[1.0, 2.0, 3.0]);
        assert_eq!(metric.measure(&point1, &point2), 3.0_f64.sqrt());
    }

    #[test]
    fn test_squared_euclidean() {
        let metric = DistanceMetric::SquaredEuclidean;

        let point1 = aview1(&[0.0, 1.0]);
        let point2 = aview1(&[1.0, 0.0]);
        assert_eq!(metric.measure(&point1, &point2), 2.0_f64);

        let point1 = aview1(&[0.0, 1.0, 2.0]);
        let point2 = aview1(&[1.0, 2.0, 3.0]);
        assert_eq!(metric.measure(&point1, &point2), 3.0_f64);
    }
}

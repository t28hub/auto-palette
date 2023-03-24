use crate::math::distance::metric::DistanceMetric;
use crate::math::number::Float;

/// Parameters of DBSCAN clustering algorithm.
#[derive(Debug, PartialEq)]
pub struct DBSCANParams<F>
where
    F: Float,
{
    min_points: usize,
    epsilon: F,
    metric: DistanceMetric,
}

impl<F> DBSCANParams<F>
where
    F: Float,
{
    /// Create a new Params with required parameters.
    #[must_use]
    pub fn new(min_points: usize, epsilon: F, metric: DistanceMetric) -> Self {
        Self {
            min_points,
            epsilon,
            metric,
        }
    }

    /// Return the minimum number of points.
    #[must_use]
    pub fn min_points(&self) -> usize {
        self.min_points
    }

    /// Return the epsilon value.
    #[must_use]
    pub fn epsilon(&self) -> F {
        self.epsilon
    }

    /// Return the distance metric.
    #[must_use]
    pub fn metric(&self) -> &DistanceMetric {
        &self.metric
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_create_params() {
        let params = DBSCANParams::new(16, 5.0, DistanceMetric::SquaredEuclidean);
        assert_eq!(params.min_points(), 16);
        assert_eq!(params.epsilon(), 5.0);
        assert_eq!(params.metric(), &DistanceMetric::SquaredEuclidean);
    }
}

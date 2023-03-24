use crate::math::clustering::kmeans::init::Initializer;
use crate::math::distance::metric::DistanceMetric;
use crate::math::number::Float;
use rand::Rng;

/// A struct representing the parameters of Kmeans.
#[derive(Debug, PartialEq)]
pub struct KmeansParams<F, R>
where
    F: Float,
    R: Rng + Clone,
{
    k: usize,
    max_iterations: usize,
    tolerance: F,
    metric: DistanceMetric,
    initializer: Initializer<R>,
}

impl<F, R> KmeansParams<F, R>
where
    F: Float,
    R: Rng + Clone,
{
    pub fn new(k: usize, metric: DistanceMetric, initializer: Initializer<R>) -> Self {
        Self {
            k,
            max_iterations: 10,
            tolerance: F::from_f32(0.0001),
            metric,
            initializer,
        }
    }

    pub fn with_max_iterations(mut self, max_iterations: usize) -> Self {
        self.max_iterations = max_iterations;
        self
    }

    pub fn with_tolerance(mut self, tolerance: F) -> Self {
        self.tolerance = tolerance;
        self
    }

    pub fn k(&self) -> usize {
        self.k
    }

    pub fn max_iterations(&self) -> usize {
        self.max_iterations
    }

    pub fn tolerance(&self) -> F {
        self.tolerance
    }

    pub fn metric(&self) -> &DistanceMetric {
        &self.metric
    }

    pub fn initializer(&self) -> &Initializer<R> {
        &self.initializer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;

    #[test]
    fn should_create_params() {
        let params = KmeansParams::new(
            5,
            DistanceMetric::SquaredEuclidean,
            Initializer::KmeansPlusPlus(thread_rng()),
        )
        .with_tolerance(0.025)
        .with_max_iterations(25);
        assert_eq!(params.k(), 5);
        assert_eq!(params.tolerance(), 0.025);
        assert_eq!(params.max_iterations(), 25);
    }
}

use crate::{image::segmentation::seed::SeedGenerator, math::DistanceMetric, FloatNumber};

/// Configuration for the K-means segmentation algorithm.
///
/// Use this to customize parameters before creating a [`KmeansSegmentation`] via [`TryFrom`].
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct KmeansConfig<T>
where
    T: FloatNumber,
{
    /// The number of segments to generate.
    pub(crate) segments: usize,

    /// The maximum number of iterations.
    pub(crate) max_iter: usize,

    /// The tolerance for convergence conditions.
    pub(crate) tolerance: T,

    /// The seed generator to use for the initial seeds.
    pub(crate) generator: SeedGenerator,

    /// The distance metric to use for calculating distances between pixels and seeds.
    pub(crate) metric: DistanceMetric,
}

impl<T> KmeansConfig<T>
where
    T: FloatNumber,
{
    /// Default number of segments to generate.
    const DEFAULT_SEGMENTS: usize = 64;

    /// Default maximum number of iterations.
    const DEFAULT_MAX_ITER: usize = 100;

    /// Default tolerance for convergence conditions.
    const DEFAULT_TOLERANCE: f64 = 1e-4;

    /// Sets the number of segments to generate.
    ///
    /// # Arguments
    /// * `segments` - The number of segments to generate.
    ///
    /// # Returns
    /// A new `KmeansConfig` with the specified number of segments.
    pub fn segments(mut self, segments: usize) -> Self {
        self.segments = segments;
        self
    }

    /// Sets the maximum number of iterations.
    ///
    /// # Arguments
    /// * `max_iter` - The maximum number of iterations.
    ///
    /// # Returns
    /// A new `KmeansConfig` with the specified maximum iterations.
    pub fn max_iter(mut self, max_iter: usize) -> Self {
        self.max_iter = max_iter;
        self
    }

    /// Sets the tolerance for convergence conditions.
    ///
    /// # Arguments
    /// * `tolerance` - The tolerance for convergence conditions.
    ///
    /// # Returns
    /// A new `KmeansConfig` with the specified tolerance.
    pub fn tolerance(mut self, tolerance: T) -> Self {
        self.tolerance = tolerance;
        self
    }

    /// Sets the seed generator to use for the initial seeds.
    ///
    /// # Arguments
    /// * `generator` - The seed generator to use for the initial seeds.
    ///
    /// # Returns
    /// A new `KmeansConfig` with the specified seed generator.
    pub(crate) fn generator(mut self, generator: SeedGenerator) -> Self {
        self.generator = generator;
        self
    }

    /// Sets the distance metric to use for calculating distances between pixels and seeds.
    ///
    /// # Arguments
    /// * `metric` - The distance metric to use for calculating distances between pixels and seeds.
    ///
    /// # Returns
    /// A new `KmeansConfig` with the specified distance metric.
    pub(crate) fn metric(mut self, metric: DistanceMetric) -> Self {
        self.metric = metric;
        self
    }
}

impl<T> Default for KmeansConfig<T>
where
    T: FloatNumber,
{
    fn default() -> Self {
        Self {
            segments: Self::DEFAULT_SEGMENTS,
            max_iter: Self::DEFAULT_MAX_ITER,
            tolerance: T::from_f64(Self::DEFAULT_TOLERANCE),
            generator: SeedGenerator::default(),
            metric: DistanceMetric::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        // Act
        let actual = KmeansConfig::<f64>::default();

        // Assert
        assert_eq!(
            actual,
            KmeansConfig {
                segments: KmeansConfig::<f64>::DEFAULT_SEGMENTS,
                max_iter: KmeansConfig::<f64>::DEFAULT_MAX_ITER,
                tolerance: KmeansConfig::<f64>::DEFAULT_TOLERANCE,
                generator: SeedGenerator::default(),
                metric: DistanceMetric::default(),
            }
        );
    }

    #[test]
    fn test_with_custom_values() {
        // Act
        let actual = KmeansConfig::<f64>::default()
            .segments(128)
            .max_iter(50)
            .tolerance(1e-8)
            .generator(SeedGenerator::RegularGrid)
            .metric(DistanceMetric::SquaredEuclidean);

        // Assert
        assert_eq!(
            actual,
            KmeansConfig {
                segments: 128,
                max_iter: 50,
                tolerance: 1e-8,
                generator: SeedGenerator::RegularGrid,
                metric: DistanceMetric::SquaredEuclidean,
            }
        );
    }
}

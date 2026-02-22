use std::marker::PhantomData;

use crate::{image::segmentation::seed::SeedGenerator, math::DistanceMetric, FloatNumber};

/// Configuration for the SNIC segmentation algorithm.
///
/// Use this to customize parameters before creating a [`SnicSegmentation`] via [`TryFrom`].
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SnicConfig<T>
where
    T: FloatNumber,
{
    /// The number of segments to generate.
    pub(crate) segments: usize,

    /// The seed generator to use for the initial seeds.
    pub(crate) generator: SeedGenerator,

    /// The distance metric to use for calculating distances between pixels and seeds.
    pub(crate) metric: DistanceMetric,

    _marker: PhantomData<T>,
}

impl<T> SnicConfig<T>
where
    T: FloatNumber,
{
    /// Default number of segments to generate.
    const DEFAULT_SEGMENTS: usize = 128;

    /// Sets the number of segments to generate.
    ///
    /// # Arguments
    /// * `segments` - The number of segments to generate.
    ///
    /// # Returns
    /// A new `SnicConfig` with the specified number of segments.
    pub fn segments(mut self, segments: usize) -> Self {
        self.segments = segments;
        self
    }

    /// Sets the seed generator to use for the initial seeds.
    ///
    /// # Arguments
    /// * `generator` - The seed generator to use for the initial seeds.
    ///
    /// # Returns
    /// A new `SnicConfig` with the specified seed generator.
    pub(crate) fn generator(mut self, generator: SeedGenerator) -> Self {
        self.generator = generator;
        self
    }

    /// Sets the distance metric to use for calculating distances between pixels and seeds.
    ///
    /// # Arguments
    /// * `metric` - The distance metric to use for calculating distances between pixels and seeds
    ///
    /// # Returns
    /// A new `SnicConfig` with the specified distance metric.
    pub(crate) fn metric(mut self, metric: DistanceMetric) -> Self {
        self.metric = metric;
        self
    }
}

impl<T> Default for SnicConfig<T>
where
    T: FloatNumber,
{
    fn default() -> Self {
        Self {
            segments: Self::DEFAULT_SEGMENTS,
            generator: SeedGenerator::default(),
            metric: DistanceMetric::SquaredEuclidean,
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        // Act
        let actual = SnicConfig::<f64>::default();

        // Assert
        assert_eq!(
            actual,
            SnicConfig {
                segments: SnicConfig::<f64>::DEFAULT_SEGMENTS,
                generator: SeedGenerator::default(),
                metric: DistanceMetric::SquaredEuclidean,
                _marker: PhantomData,
            }
        );
    }

    #[test]
    fn test_with_custom_values() {
        // Act
        let actual = SnicConfig::<f64>::default()
            .segments(128)
            .generator(SeedGenerator::RegularGrid)
            .metric(DistanceMetric::Euclidean);

        // Assert
        assert_eq!(
            actual,
            SnicConfig {
                segments: 128,
                generator: SeedGenerator::RegularGrid,
                metric: DistanceMetric::Euclidean,
                _marker: PhantomData,
            }
        );
    }
}

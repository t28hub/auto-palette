use crate::{math::DistanceMetric, FloatNumber};

/// Configuration for the Fast DBSCAN (DBSCAN++) segmentation algorithm.
///
/// Use this to customize parameters before creating a [`FastDbscanSegmentation`] via [`TryFrom`].
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FastDbscanConfig<T>
where
    T: FloatNumber,
{
    /// The minimum number of pixels required to form a segment.
    pub(crate) min_pixels: usize,

    /// The epsilon value that defines the radius for neighborhood search.
    pub(crate) epsilon: T,

    /// The probability value that defines the likelihood of selecting a core pixel.
    pub(crate) probability: T,

    /// The distance metric to use for calculating distances between pixels.
    pub(crate) metric: DistanceMetric,
}

impl<T> FastDbscanConfig<T>
where
    T: FloatNumber,
{
    /// Default minimum number of pixels in a segment.
    const DEFAULT_MIN_PIXELS: usize = 10;

    /// Default epsilon value for the segmentation.
    /// The epsilon value is squared because the default distance metric is SquaredEuclidean.
    const DEFAULT_EPSILON: f64 = 0.04 * 0.04;

    /// Default probability for the segmentation.
    const DEFAULT_PROBABILITY: f64 = 0.1;

    /// Sets the minimum number of pixels required to form a segment.
    ///
    /// # Arguments
    /// * `min_pixels` - The minimum number of pixels required to form a segment.
    ///
    /// # Returns
    /// A new `FastDbscanConfig` with the specified minimum pixels.
    #[must_use]
    pub fn min_pixels(mut self, min_pixels: usize) -> Self {
        self.min_pixels = min_pixels;
        self
    }

    /// Sets the epsilon value that defines the radius for neighborhood search.
    ///
    /// # Arguments
    /// * `epsilon` - The epsilon value that defines the radius for neighborhood search.
    ///
    /// # Returns
    /// A new `FastDbscanConfig` with the specified epsilon.
    #[must_use]
    pub fn epsilon(mut self, epsilon: T) -> Self {
        self.epsilon = epsilon;
        self
    }

    /// Sets the probability value that defines the likelihood of selecting a core pixel.
    ///
    /// # Arguments
    /// * `probability` - The probability value that defines the likelihood of selecting a core pixel.
    ///
    /// # Returns
    /// A new `FastDbscanConfig` with the specified probability.
    #[must_use]
    pub fn probability(mut self, probability: T) -> Self {
        self.probability = probability;
        self
    }

    /// Sets the distance metric to use for calculating distances between pixels.
    ///
    /// # Arguments
    /// * `metric` - The distance metric to use for calculating distances between pixels.
    ///
    /// # Returns
    /// A new `FastDbscanConfig` with the specified distance metric.
    #[allow(unused)]
    #[must_use]
    pub(crate) fn metric(mut self, metric: DistanceMetric) -> Self {
        self.metric = metric;
        self
    }
}

impl<T> Default for FastDbscanConfig<T>
where
    T: FloatNumber,
{
    fn default() -> Self {
        Self {
            min_pixels: Self::DEFAULT_MIN_PIXELS,
            epsilon: T::from_f64(Self::DEFAULT_EPSILON),
            probability: T::from_f64(Self::DEFAULT_PROBABILITY),
            metric: DistanceMetric::SquaredEuclidean,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        // Act
        let actual = FastDbscanConfig::<f64>::default();

        // Assert
        assert_eq!(
            actual,
            FastDbscanConfig {
                min_pixels: FastDbscanConfig::<f64>::DEFAULT_MIN_PIXELS,
                epsilon: FastDbscanConfig::<f64>::DEFAULT_EPSILON,
                probability: FastDbscanConfig::<f64>::DEFAULT_PROBABILITY,
                metric: DistanceMetric::SquaredEuclidean,
            }
        );
    }

    #[test]
    fn test_with_custom_values() {
        // Act
        let actual = FastDbscanConfig::<f64>::default()
            .min_pixels(10)
            .epsilon(0.05)
            .probability(0.25)
            .metric(DistanceMetric::Euclidean);

        // Assert
        assert_eq!(
            actual,
            FastDbscanConfig {
                min_pixels: 10,
                epsilon: 0.05,
                probability: 0.25,
                metric: DistanceMetric::Euclidean,
            }
        );
    }
}

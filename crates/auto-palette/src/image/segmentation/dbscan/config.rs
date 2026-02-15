use crate::{math::DistanceMetric, FloatNumber};

/// Configuration for the DBSCAN segmentation algorithm.
///
/// Use this to customize parameters before creating a [`DbscanSegmentation`] via [`TryFrom`].
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DbscanConfig<T>
where
    T: FloatNumber,
{
    /// The number of segments to generate.
    pub(crate) segments: usize,

    /// The minimum number of pixels required to form a segment.
    pub(crate) min_pixels: usize,

    /// The epsilon value that defines the radius for neighborhood search.
    pub(crate) epsilon: T,

    /// The distance metric to use for calculating distances between pixels.
    pub(crate) metric: DistanceMetric,
}

impl<T> DbscanConfig<T>
where
    T: FloatNumber,
{
    /// Default number of segments to generate.
    const DEFAULT_SEGMENTS: usize = 64;

    /// Default minimum number of pixels for a segment.
    const DEFAULT_MIN_PIXELS: usize = 6;

    /// Default epsilon value for the segmentation.
    const DEFAULT_EPSILON: f64 = 1e-3;

    /// Sets the number of segments to generate.
    ///
    /// # Arguments
    /// * `segments` - The number of segments to generate.
    ///
    /// # Returns
    /// A new `DbscanConfig` with the specified number of segments.
    pub fn segments(mut self, segments: usize) -> Self {
        self.segments = segments;
        self
    }

    /// Sets the minimum number of pixels required to form a segment.
    ///
    /// # Arguments
    /// * `min_pixels` - The minimum number of pixels required to form a segment.
    ///
    /// # Returns
    /// A new `DbscanConfig` with the specified minimum pixels.
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
    /// A new `DbscanConfig` with the specified epsilon.
    pub fn epsilon(mut self, epsilon: T) -> Self {
        self.epsilon = epsilon;
        self
    }

    /// Sets the distance metric to use for calculating distances between pixels.
    ///
    /// # Arguments
    /// * `metric` - The distance metric to use for calculating distances between pixels.
    ///
    /// # Returns
    /// A new `DbscanConfig` with the specified distance metric.
    pub(crate) fn metric(mut self, metric: DistanceMetric) -> Self {
        self.metric = metric;
        self
    }
}

impl<T> Default for DbscanConfig<T>
where
    T: FloatNumber,
{
    fn default() -> Self {
        Self {
            segments: Self::DEFAULT_SEGMENTS,
            min_pixels: Self::DEFAULT_MIN_PIXELS,
            epsilon: T::from_f64(Self::DEFAULT_EPSILON),
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
        let actual = DbscanConfig::<f64>::default();

        // Assert
        assert_eq!(
            actual,
            DbscanConfig {
                segments: DbscanConfig::<f64>::DEFAULT_SEGMENTS,
                min_pixels: DbscanConfig::<f64>::DEFAULT_MIN_PIXELS,
                epsilon: DbscanConfig::<f64>::DEFAULT_EPSILON,
                metric: DistanceMetric::default(),
            }
        );
    }

    #[test]
    fn test_with_custom_values() {
        // Act
        let actual = DbscanConfig::<f64>::default()
            .segments(128)
            .min_pixels(10)
            .epsilon(0.05)
            .metric(DistanceMetric::SquaredEuclidean);

        // Assert
        assert_eq!(
            actual,
            DbscanConfig {
                segments: 128,
                min_pixels: 10,
                epsilon: 0.05,
                metric: DistanceMetric::SquaredEuclidean,
            }
        );
    }
}

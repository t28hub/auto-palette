use crate::math::Point;
use std::collections::HashSet;

/// Sampling strategy.
///
/// # Type Parameters
/// * `N` - The number of dimensions.
pub trait SamplingStrategy<const N: usize> {
    /// Samples points from the given list of points.
    ///
    /// # Arguments
    /// * `points` - The points to sample from.
    /// * `n` - The number of points to sample.
    ///
    /// # Returns
    /// The indices of the sampled points.
    #[must_use]
    fn sample(&self, points: &[Point<N>], n: usize) -> HashSet<usize>;
}

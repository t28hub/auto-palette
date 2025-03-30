use std::collections::HashSet;

use crate::{
    math::{sampling::error::SamplingError, Point},
    FloatNumber,
};

/// Trait for sampling algorithms.
///
/// # Type Parameters
/// * `T` - The floating point type.
pub trait SamplingAlgorithm<T>
where
    T: FloatNumber,
{
    /// Samples points from the given data.
    ///
    /// # Arguments
    /// * `points` - The points to sample from.
    /// * `num_samples` - The number of samples to generate.
    ///
    /// # Returns
    /// A vector containing the sampled points.
    #[allow(dead_code)]
    fn sample<const N: usize>(
        &self,
        points: &[Point<T, N>],
        num_samples: usize,
    ) -> Result<HashSet<usize>, SamplingError<T>>;
}

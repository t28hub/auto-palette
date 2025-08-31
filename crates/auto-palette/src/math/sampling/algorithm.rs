use std::collections::HashSet;

use crate::{
    math::{sampling::error::SamplingError, Point},
    FloatNumber,
};

/// Trait defining the interface for point sampling algorithms.
///
/// This trait provides a common interface for algorithms that select a subset of points
/// from a larger dataset. Sampling algorithms are commonly used in clustering, data
/// visualization, and dimensionality reduction to work with representative subsets
/// of large datasets.
///
/// # Type Parameters
/// * `T` - The floating point type used for calculations.
pub trait SamplingAlgorithm<T>
where
    T: FloatNumber,
{
    /// Selects the initial index for the sampling process.
    ///
    /// This method allows each sampling algorithm to define its own strategy
    /// for selecting the initial point's index. The initial point selection can significantly
    /// impact the final sample distribution.
    ///
    /// # Default Implementation
    /// The default implementation selects the first point's index (0) in the dataset.
    ///
    /// # Arguments
    /// * `points` - The dataset to select the initial index from.
    ///
    /// # Returns
    /// The index of the selected initial point.
    ///
    /// # Errors
    /// Returns [`SamplingError::EmptyPoints`] if the points array is empty.
    ///
    /// # Examples
    /// ```rust,ignore
    /// let initial_index = algorithm.select_initial_index(&points)?;
    /// ```
    fn select_initial_index<const N: usize>(
        &self,
        points: &[Point<T, N>],
    ) -> Result<usize, SamplingError> {
        if points.is_empty() {
            return Err(SamplingError::EmptyPoints);
        }
        Ok(0)
    }

    /// Samples a subset of points from the given dataset.
    ///
    /// This method is the core operation of the sampling algorithm. It selects
    /// `num_samples` points from the input dataset according to the algorithm's
    /// specific strategy.
    ///
    /// # Arguments
    /// * `points` - The dataset to sample from.
    /// * `num_samples` - The number of points to sample.
    ///
    /// # Returns
    /// A `HashSet` containing the indices of the sampled points.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The input points array is empty ([`SamplingError::EmptyPoints`])
    /// - Algorithm-specific constraints are violated
    ///
    /// # Performance
    /// The time complexity depends on the specific algorithm implementation.
    fn sample<const N: usize>(
        &self,
        points: &[Point<T, N>],
        num_samples: usize,
    ) -> Result<HashSet<usize>, SamplingError>;
}

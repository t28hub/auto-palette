use crate::math::{point::Point, FloatNumber};

/// A generic trait for clustering algorithm.
///
/// # Type Parameters
/// * `T` - The floating point type.
/// * `N` - The number of dimensions.
pub trait ClusteringAlgorithm<T, const N: usize>
where
    T: FloatNumber,
{
    /// The concrete output type of the clustering algorithm.
    type Output;

    /// The algorithm specific error type.
    type Error;

    /// Runs the clustering algorithm on the given points.
    ///
    /// # Arguments
    /// * `points` - A slice of points to cluster.
    ///
    /// # Returns
    /// A result containing the clustering output or an error.
    ///
    /// # Note
    /// *Time complexity* and *space complexity* should be documented in the individual implementations.
    fn run(&self, points: &[Point<T, N>]) -> Result<Self::Output, Self::Error>;
}

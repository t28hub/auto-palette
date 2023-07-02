use crate::math::number::Float;
use crate::math::point::Point;

/// Trait representing a clustering algorithm.
///
/// # Type Parameters
/// * `F` - The float type used for calculations.
/// * `P` - The point type used for calculations.
pub trait ClusteringAlgorithm<F, P>
where
    F: Float,
    P: Point<F>,
{
    type Output;

    /// Fits the clustering algorithm to the given points.
    ///
    /// # Arguments
    /// * `points` - A slice of data points to cluster.
    ///
    /// # Returns
    /// A fitted output.
    #[must_use]
    fn fit(&self, points: &[P]) -> Self::Output;
}

use crate::math::number::Float;
use crate::math::point::Point;

/// Trait representing a clustering algorithm.
pub trait Algorithm<F, P, T>
where
    F: Float,
    P: Point<F>,
{
    /// Fits the algorithm to the given dataset with the given parameters.
    ///
    /// # Arguments
    /// * `dataset` - The dataset to fit the algorithm to.
    /// * `params` - The parameters of this algorithm.
    ///
    /// # Returns
    /// The fitted algorithm.
    ///
    #[must_use]
    fn fit(dataset: &[P], params: &T) -> Self;

    /// Returns the indices of outliers.
    ///
    /// # Returns
    /// A slice containing the indices of the outliers.
    ///
    #[must_use]
    fn outliers(&self) -> &[usize];
}

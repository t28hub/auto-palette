use crate::math::clustering::cluster::Cluster;
use crate::math::number::Float;
use crate::math::point::Point;

/// Trait representing a clustering algorithm.
pub trait Clustering<F, P>
where
    F: Float,
    P: Point<F>,
{
    type Params;

    /// Fit the algorithm to the given dataset with the given parameters.
    ///
    /// # Arguments
    /// * `dataset` - The dataset to fit the algorithm to.
    /// * `params` - The parameters of this algorithm.
    ///
    /// # Returns
    /// The fitted algorithm.
    #[must_use]
    fn fit(dataset: &[P], params: &Self::Params) -> Self;

    /// Return the clusters.
    ///
    /// # Returns
    /// A reference to a slice of clusters.
    #[must_use]
    fn clusters(&self) -> &[Cluster<F, P>];

    /// Return the indices of outliers.
    ///
    /// # Returns
    /// A slice containing the indices of the outliers.
    #[must_use]
    fn outliers(&self) -> &[usize];
}

use crate::math::clustering::cluster::Cluster;
use crate::math::point::Point;
use crate::math::FloatNumber;

/// Trait for clustering algorithms.
///
/// # Type Parameters
/// * `T` - The floating point type.
/// * `N` - The number of dimensions.
pub trait ClusteringAlgorithm<T, const N: usize>
where
    T: FloatNumber,
{
    /// Fits the clustering algorithm to the given points.
    ///
    /// # Arguments
    /// * `points` - The points to cluster.
    ///
    /// # Returns
    /// The clusters of the points.
    fn fit(&self, points: &[Point<T, N>]) -> Vec<Cluster<T, N>>;
}

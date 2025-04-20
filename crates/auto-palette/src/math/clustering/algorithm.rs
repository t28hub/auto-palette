use crate::math::{clustering::cluster::Cluster, point::Point, FloatNumber};

/// Trait for clustering algorithms.
///
/// # Type Parameters
/// * `T` - The floating point type.
/// * `N` - The number of dimensions.
pub trait ClusteringAlgorithm<T, const N: usize>
where
    T: FloatNumber,
{
    /// The error type for the clustering algorithm.
    type Err;

    /// Fits the clustering algorithm to the given points.
    ///
    /// # Arguments
    /// * `points` - The points to cluster.
    ///
    /// # Returns
    /// The clusters of the points.
    fn fit(&self, points: &[Point<T, N>]) -> Result<Vec<Cluster<T, N>>, Self::Err>;
}

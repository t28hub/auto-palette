use crate::math::clustering::cluster::Cluster;
use crate::math::point::Point;

/// Trait for clustering algorithms.
pub trait ClusteringAlgorithm {
    /// Fits the clustering algorithm to the given points.
    ///
    /// # Type Parameters
    /// * `N` - The number of dimensions.
    ///
    /// # Arguments
    /// * `points` - The points to cluster.
    ///
    /// # Returns
    /// The clusters of the points.
    fn fit<const N: usize>(&self, points: &[Point<N>]) -> Vec<Cluster<N>>;
}

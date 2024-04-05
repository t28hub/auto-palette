use crate::math::neighbors::neighbor::Neighbor;
use crate::math::point::Point;

/// Trait for neighbor search algorithms.
///
/// # Type Parameters
/// * `N` - The dimension of the points.
pub trait NeighborSearch<const N: usize> {
    /// Searches for the k nearest neighbors of a point.
    ///
    /// # Arguments
    /// * `query` - The query point to search for neighbors.
    /// * `k` - The number of neighbors to search for.
    ///
    /// # Returns
    /// The k nearest neighbors.
    fn search(&self, query: &Point<N>, k: usize) -> Vec<Neighbor>;

    /// Searches for the nearest neighbor of a point.
    ///
    /// # Arguments
    /// * `query` - The query point to search for a neighbor.
    ///
    /// # Returns
    /// The nearest neighbor.
    fn search_nearest(&self, query: &Point<N>) -> Option<Neighbor>;
}

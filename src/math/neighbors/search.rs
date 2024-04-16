use crate::math::neighbors::neighbor::Neighbor;
use crate::math::point::Point;
use crate::math::FloatNumber;

/// Trait for neighbor search algorithms.
///
/// # Type Parameters
/// * `T` - The floating point type.
/// * `N` - The dimension of the points.
pub trait NeighborSearch<T, const N: usize>
where
    T: FloatNumber,
{
    /// Searches for the k nearest neighbors of a point.
    ///
    /// # Arguments
    /// * `query` - The query point to search for neighbors.
    /// * `k` - The number of neighbors to search for.
    ///
    /// # Returns
    /// The k nearest neighbors.
    fn search(&self, query: &Point<T, N>, k: usize) -> Vec<Neighbor<T>>;

    /// Searches for the nearest neighbor of a point.
    ///
    /// # Arguments
    /// * `query` - The query point to search for a neighbor.
    ///
    /// # Returns
    /// The nearest neighbor.
    fn search_nearest(&self, query: &Point<T, N>) -> Option<Neighbor<T>>;

    /// Searches for the neighbors within a given radius of a point.
    ///
    /// # Arguments
    /// * `query` - The query point to search for neighbors.
    /// * `radius` - The radius within which to search for neighbors.
    ///
    /// # Returns
    /// The neighbors within the given radius.
    fn search_radius(&self, query: &Point<T, N>, radius: T) -> Vec<Neighbor<T>>;
}

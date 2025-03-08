use crate::math::{neighbors::neighbor::Neighbor, point::Point, FloatNumber};

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
    /// * `k` - The number of neighbors to search for. If `k == 0`, an empty vector is returned.
    ///
    /// # Returns
    /// A vector containing the k nearest neighbors in ascending order of distance.
    #[allow(dead_code)]
    fn search(&self, query: &Point<T, N>, k: usize) -> Vec<Neighbor<T>>;

    /// Searches for the nearest neighbor of a point.
    ///
    /// # Arguments
    /// * `query` - The query point to search for a neighbor.
    ///
    /// # Returns
    /// The nearest neighbor, or `None` if no neighbors are found.
    fn search_nearest(&self, query: &Point<T, N>) -> Option<Neighbor<T>>;

    /// Searches for the neighbors within a given radius of a point.
    ///
    /// # Arguments
    /// * `query` - The query point to search for neighbors.
    /// * `radius` - The radius within which to search for neighbors. If `radius < 0`, an empty vector is returned.
    ///
    /// # Returns
    /// A vector containing the neighbors within the specified radius.
    fn search_radius(&self, query: &Point<T, N>, radius: T) -> Vec<Neighbor<T>>;
}

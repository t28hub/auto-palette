use crate::math::{neighbors::neighbor::Neighbor, point::Point, FloatNumber};

/// Trait for neighbor search algorithms.
///
/// # Type Parameters
/// * `T` - The floating point type used for distances (e.g., `f32`, `f64`).
/// * `N` - The dimension of the points.
pub trait NeighborSearch<T, const N: usize>
where
    T: FloatNumber,
{
    /// Finds the k nearest neighbors to q query point.
    ///
    /// # Arguments
    /// * `query` - The point to search from.
    /// * `k` - Maximum number of neighbors to return.
    ///
    /// # Returns
    /// A vector of up to `k` nearest neighbors. If `k == 0`, an empty vector is returned.
    #[allow(dead_code)]
    fn search(&self, query: &Point<T, N>, k: usize) -> Vec<Neighbor<T>>;

    /// Finds the nearest neighbor to a query point.
    ///
    /// # Arguments
    /// * `query` - The point to search from.
    ///
    /// # Returns
    /// The nearest neighbor, or `None` if no neighbors exist.
    fn search_nearest(&self, query: &Point<T, N>) -> Option<Neighbor<T>>;

    /// Finds all neighbors within a specified radius from a query point.
    ///
    /// # Arguments
    /// * `query` - The point to search from.
    /// * `radius` - Maximum distance from the query point.
    ///
    /// # Returns
    /// A vector containing all neighbors within the specified radius.
    /// If `radius <= 0`, an empty vector is returned.
    fn search_within_radius(&self, query: &Point<T, N>, radius: T) -> Vec<Neighbor<T>>;
}

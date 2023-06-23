use crate::math2::neighbors::neighbor::Neighbor;
use crate::number::Float;
use ndarray::ArrayView1;

/// Trait representing neighbor search algorithms.
///
/// # Type parameters
/// * `F` - The float type for calculations.
pub trait NeighborSearch<F>
where
    F: Float,
{
    /// Searches for the k-nearest neighbors of the given point.
    ///
    /// # Arguments
    /// * `query` - The reference point of the neighbors are searched.
    /// * `k` - The number of nearest neighbors.
    ///
    /// # Returns
    /// A `Vec` of the k-nearest neighbors.
    #[must_use]
    fn search(&self, query: &ArrayView1<F>, k: usize) -> Vec<Neighbor<F>>;

    /// Search for the nearest neighbor of the given point.
    ///
    /// # Arguments
    /// * `query` - The reference point of the neighbor is searched.
    ///
    /// # Returns
    /// An `Option` of the nearest neighbor.
    #[must_use]
    fn search_nearest(&self, query: &ArrayView1<F>) -> Option<Neighbor<F>>;

    /// Searches for all neighbors within the given radius of a given point.
    ///
    /// # Arguments
    /// * `query` - The reference point of the neighbors are searched.
    ///
    /// # Returns
    /// A `Vec` of all neighbors within the given radius.
    #[must_use]
    fn search_radius(&self, query: &ArrayView1<F>, radius: F) -> Vec<Neighbor<F>>;
}

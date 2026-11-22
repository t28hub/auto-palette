use crate::next::extraction::{centroid::Centroid, point::Point};

/// A way to find the nearest center of each point, iteration after iteration.
pub(super) trait Assignment {
    /// Creates the state for clustering points from initial centers.
    ///
    /// # Arguments
    /// * `points` - The points to cluster.
    /// * `centers` - The initial centers, at least one.
    ///
    /// # Returns
    /// The state before the first assignment.
    fn new(points: &[Point], centers: &[Point]) -> Self;

    /// Assigns each point to its nearest center.
    ///
    /// # Arguments
    /// * `points` - The points to cluster.
    /// * `centers` - The centers, at least one.
    /// * `moves` - How far each center moved since the last call, all zero on the first call.
    /// * `assigned` - The center of each point, updated for the points with weight.
    ///
    /// # Returns
    /// The centroid of the points assigned to each center.
    fn assign(
        &mut self,
        points: &[Point],
        centers: &[Point],
        moves: &[f64],
        assigned: &mut [usize],
    ) -> Vec<Centroid>;
}

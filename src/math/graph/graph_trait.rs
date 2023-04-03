use crate::math::graph::edge::Edge;

/// Trait representing a graph.
///
/// # Type Parameters
/// * `V` - The type of the vertices in this graph.
/// * `E` - The type of the edges in this graph.
pub trait Graph<V, E: Edge> {
    #[must_use]
    fn vertex_at(&self, index: usize) -> Option<&V>;

    /// Return a list of the vertices in this graph.
    ///
    /// # Returns
    /// A slice containing the vertices in this graph.
    #[must_use]
    fn vertices(&self) -> &[V];

    /// Returns a list of the edges starting from the vertex corresponding to the given index.
    ///
    /// # Arguments
    /// * `index` - The index corresponding to vertex.
    ///
    /// # Returns
    /// A slice containing the edges starting from the vertex.
    #[must_use]
    fn edges_at(&self, index: usize) -> Vec<E>;
}

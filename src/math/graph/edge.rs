/// Trait representing an edge in a `Graph`.
pub trait Edge {
    /// Return the index of the starting vertex of this edge.
    ///
    /// # Returns
    /// The index of the starting vertex.
    #[must_use]
    fn u(&self) -> usize;

    /// Return the index of the ending vertex of this edge.
    ///
    /// # Returns
    /// The index of the ending vertex.
    #[must_use]
    fn v(&self) -> usize;
}

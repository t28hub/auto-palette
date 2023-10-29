use crate::math::graph::edge::Edge;
use crate::math::number::Float;
use std::cmp::Ordering;

/// Struct representing a weighted edge in a `WeightedGraph`.
///
/// # Type Parameters
/// * `F` - The type of the weight of this edge.
#[derive(Debug, Clone, PartialEq)]
pub struct WeightedEdge<F: Float> {
    u: usize,
    v: usize,
    weight: F,
}

impl<F> WeightedEdge<F>
where
    F: Float,
{
    /// Creates a new `WeightedEdge` instance.
    ///
    /// # Arguments
    /// * `u` - The index of the starting vertex of this edge.
    /// * `v` - The index of the ending vertex of this edge.
    /// * `weight` - The weight of this edge.
    ///
    /// # Returns
    /// A new `WeightedEdge` instance.
    pub fn new(u: usize, v: usize, weight: F) -> Self {
        Self { u, v, weight }
    }
}

impl<F> Eq for WeightedEdge<F> where F: Float {}

impl<F> PartialOrd<Self> for WeightedEdge<F>
where
    F: Float,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl<F> Ord for WeightedEdge<F>
where
    F: Float,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl<F> Edge for WeightedEdge<F>
where
    F: Float,
{
    #[must_use]
    fn u(&self) -> usize {
        self.u
    }

    #[must_use]
    fn v(&self) -> usize {
        self.v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weighted_edge() {
        let edge = WeightedEdge::new(0, 1, 5.0);
        assert_eq!(edge.u(), 0);
        assert_eq!(edge.v(), 1);
        assert_eq!(edge.weight, 5.0);
    }

    #[test]
    fn test_eq() {
        let edge1 = WeightedEdge::new(0, 1, 5.0);
        let edge2 = WeightedEdge::new(0, 1, 5.0);
        assert_eq!(edge1.eq(&edge2), true);

        let edge1 = WeightedEdge::new(0, 1, 5.0);
        let edge2 = WeightedEdge::new(0, 2, 5.0);
        assert_eq!(edge1.eq(&edge2), false);
    }

    #[test]
    fn test_cmp() {
        let edge1 = WeightedEdge::new(0, 1, 5.0);
        let edge2 = WeightedEdge::new(1, 2, 2.5);
        assert_eq!(edge1.cmp(&edge2), Ordering::Greater);

        let edge1 = WeightedEdge::new(0, 1, 5.0);
        let edge2 = WeightedEdge::new(1, 2, 7.5);
        assert_eq!(edge1.cmp(&edge2), Ordering::Less);

        let edge1 = WeightedEdge::new(0, 1, 5.0);
        let edge2 = WeightedEdge::new(1, 2, 5.0);
        assert_eq!(edge1.cmp(&edge2), Ordering::Equal);
    }
}

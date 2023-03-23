use crate::math::number::Float;
use std::cmp::Ordering;
use std::marker::PhantomData;

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

/// Trait representing a graph.
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

/// Struct representing a weighted edge in a `WeightedGraph`.
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
    /// Creates a new weighted edge.
    ///
    /// # Arguments
    /// * `u` - The index of the starting vertex of this edge.
    /// * `v` - The index of the ending vertex of this edge.
    /// * `weight` - The weight of this edge.
    ///
    /// # Returns
    /// A new `WeightedEdge`.
    pub fn new(u: usize, v: usize, weight: F) -> Self {
        Self { u, v, weight }
    }

    /// Return the weight of this edge.
    ///
    /// # Returns
    /// The weight of this edge.
    #[must_use]
    pub fn weight(&self) -> F {
        self.weight
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

impl<F> Eq for WeightedEdge<F> where F: Float {}

impl<F> PartialOrd<Self> for WeightedEdge<F>
where
    F: Float,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.weight.partial_cmp(&self.weight)
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

/// Struct representing a weighted graph.
#[derive(Debug)]
pub struct WeightedGraph<'a, V, W: Fn(usize, usize) -> F, F: Float> {
    vertices: &'a [V],
    weight_fn: W,
    _phantom: PhantomData<F>,
}

impl<'a, V, W, F> WeightedGraph<'a, V, W, F>
where
    W: Fn(usize, usize) -> F,
    F: Float,
{
    /// Creates a new `WeightedGraph`.
    ///
    /// # Arguments
    /// * `vertices` - The slice containing the vertices of this graph.
    /// * `weight_fn` - The function that takes the indices of two vertices and returns the weight of the edge.
    ///
    /// # Returns
    /// A new `WeightedGraph`.
    pub fn new(vertices: &'a [V], weight_fn: W) -> Self {
        Self {
            vertices,
            weight_fn,
            _phantom: PhantomData::default(),
        }
    }
}

impl<V, W, F> Graph<V, WeightedEdge<F>> for WeightedGraph<'_, V, W, F>
where
    W: Fn(usize, usize) -> F,
    F: Float,
{
    #[must_use]
    fn vertex_at(&self, index: usize) -> Option<&V> {
        self.vertices.get(index)
    }

    #[must_use]
    fn vertices(&self) -> &[V] {
        &self.vertices
    }

    #[must_use]
    fn edges_at(&self, index: usize) -> Vec<WeightedEdge<F>> {
        let mut edges = Vec::new();
        if index >= self.vertices.len() {
            return edges;
        }

        for i in 0..self.vertices.len() {
            if i == index {
                continue;
            }

            let weight = (self.weight_fn)(index, i);
            edges.push(WeightedEdge::<F>::new(index, i, weight));
        }
        edges
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_create_weighted_edge() {
        let edge = WeightedEdge::new(0, 1, 5.0);
        assert_eq!(edge.u(), 0);
        assert_eq!(edge.v(), 1);
        assert_eq!(edge.weight(), 5.0);
    }

    #[test]
    fn eq_should_return_true_if_each_edge_is_equal() {
        let edge1 = WeightedEdge::new(0, 1, 5.0);
        let edge2 = WeightedEdge::new(0, 1, 5.0);
        assert_eq!(edge1.eq(&edge2), true);

        let edge1 = WeightedEdge::new(0, 1, 5.0);
        let edge2 = WeightedEdge::new(0, 2, 5.0);
        assert_eq!(edge1.eq(&edge2), false);
    }

    #[test]
    fn cmp_should_return_reversed_ordering() {
        let edge1 = WeightedEdge::new(0, 1, 5.0);
        let edge2 = WeightedEdge::new(1, 2, 2.5);
        assert_eq!(edge1.cmp(&edge2), Ordering::Less);

        let edge1 = WeightedEdge::new(0, 1, 2.5);
        let edge2 = WeightedEdge::new(1, 2, 2.5);
        assert_eq!(edge1.cmp(&edge2), Ordering::Equal);

        let edge1 = WeightedEdge::new(0, 1, 2.0);
        let edge2 = WeightedEdge::new(1, 2, 2.5);
        assert_eq!(edge1.cmp(&edge2), Ordering::Greater);

        let edge1 = WeightedEdge::new(0, 1, f64::NAN);
        let edge2 = WeightedEdge::new(1, 2, 2.5);
        assert_eq!(edge1.cmp(&edge2), Ordering::Equal);
    }

    #[test]
    fn new_should_create_weighted_graph() {
        let vertices = ["Alice", "Bob", "Charlie"];
        let graph = WeightedGraph::new(&vertices, |u: usize, v: usize| {
            let matrix = [
                [f64::NAN, 1.0, 4.0],
                [1.0, f64::NAN, 7.0],
                [4.0, 9.0, f64::NAN],
            ];
            matrix[u][v]
        });
        assert_eq!(graph.vertices(), vertices);
        assert_eq!(graph.vertex_at(0), Some(&"Alice"));
        assert_eq!(graph.vertex_at(1), Some(&"Bob"));
        assert_eq!(graph.vertex_at(2), Some(&"Charlie"));
        assert_eq!(graph.vertex_at(3), None);
        assert_eq!(
            graph.edges_at(0),
            vec![WeightedEdge::new(0, 1, 1.0), WeightedEdge::new(0, 2, 4.0)]
        );
        assert_eq!(
            graph.edges_at(1),
            vec![WeightedEdge::new(1, 0, 1.0), WeightedEdge::new(1, 2, 7.0)]
        );
        assert_eq!(
            graph.edges_at(2),
            vec![WeightedEdge::new(2, 0, 4.0), WeightedEdge::new(2, 1, 9.0)]
        );
        assert_eq!(graph.edges_at(3), vec![]);
    }
}

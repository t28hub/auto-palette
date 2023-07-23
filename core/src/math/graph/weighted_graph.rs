use crate::math::graph::graph_trait::Graph;
use crate::math::graph::weighted_edge::WeightedEdge;
use crate::math::number::Float;
use std::marker::PhantomData;

/// Struct representing a weighted graph.
///
/// # Type Parameters
/// * `V` - The type of the vertices of this graph.
/// * `W` - The type of the function that computes the weight between 2 edgew.
#[derive(Debug)]
pub struct WeightedGraph<'a, V, W: Fn(usize, usize) -> F, F: Float> {
    vertices: &'a [V],
    weight_fn: W,
    _marker: PhantomData<F>,
}

impl<'a, V, W, F> WeightedGraph<'a, V, W, F>
where
    W: Fn(usize, usize) -> F,
    F: Float,
{
    /// Creates a new `WeightedGraph` instance.
    ///
    /// # Arguments
    /// * `vertices` - The slice containing the vertices of this graph.
    /// * `weight_fn` - The function that takes the indices of two vertices and returns the weight of the edge.
    ///
    /// # Returns
    /// A new `WeightedGraph` instance.
    pub fn new(vertices: &'a [V], weight_fn: W) -> Self {
        Self {
            vertices,
            weight_fn,
            _marker: PhantomData,
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
        self.vertices
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
    fn test_weighted_graph() {
        let vertices = vec![1, 2, 3, 4, 5];
        let graph = WeightedGraph::new(&vertices, |u, v| (u + v) as f64);
        assert_eq!(graph.vertices(), &vertices);
        assert_eq!(graph.vertex_at(0), Some(&1));
        assert_eq!(graph.vertex_at(4), Some(&5));
        assert_eq!(graph.vertex_at(5), None);
        assert_eq!(
            graph.edges_at(0),
            vec![
                WeightedEdge::new(0, 1, 1.0),
                WeightedEdge::new(0, 2, 2.0),
                WeightedEdge::new(0, 3, 3.0),
                WeightedEdge::new(0, 4, 4.0)
            ]
        );
        assert_eq!(graph.edges_at(5), Vec::new());
    }
}

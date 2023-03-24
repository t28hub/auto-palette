use crate::math::graph::graph::{Edge, Graph, WeightedEdge};
use crate::math::number::Float;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

/// Trait representing a spanning tree of a graph.
pub trait SpanningTree<F: Float, E: Edge> {
    /// Returns the weight of this spanning tree.
    ///
    /// # Returns
    /// The weight of this spanning tree.
    #[must_use]
    fn weight(&self) -> F;

    /// Return a list of the edges in this spanning tree.
    ///
    /// # Returns
    /// A slice containing the edges in this spanning tree.
    #[must_use]
    fn edges(&self) -> &[E];
}

/// Struct representing a minimum spanning tree.
#[derive(Debug)]
pub struct MinimumSpanningTree<F: Float> {
    weight: F,
    edges: Vec<WeightedEdge<F>>,
}

impl<F> MinimumSpanningTree<F>
where
    F: Float,
{
    /// Builds a minimum spanning tree using Prim's algorithm.
    ///
    /// # Arguments
    /// * `graph` - The source graph to create a minimum spanning tree.
    ///
    /// # Returns
    /// A new `MinimumSpanningTree`.
    pub fn build<G, V>(graph: &G) -> Self
    where
        G: Graph<V, WeightedEdge<F>>,
    {
        let vertices = graph.vertices();
        if vertices.is_empty() {
            return Self::default();
        }

        let mut heap = BinaryHeap::new();
        let mut edges = Vec::new();
        let mut visited = HashSet::new();

        let mut weight = F::zero();
        let mut index = vertices.len() - 1;
        visited.insert(index);
        while visited.len() < vertices.len() {
            for edge in graph.edges_at(index).into_iter() {
                if visited.contains(&edge.v()) {
                    continue;
                }
                heap.push(Reverse(edge));
            }

            while let Some(Reverse(edge)) = heap.pop() {
                if !visited.contains(&edge.v()) {
                    index = edge.v();
                    weight += edge.weight();
                    edges.push(edge);
                    visited.insert(index);
                    break;
                }
            }
        }

        Self { edges, weight }
    }
}

impl<F> Default for MinimumSpanningTree<F>
where
    F: Float,
{
    fn default() -> Self {
        Self {
            weight: F::zero(),
            edges: Vec::new(),
        }
    }
}

impl<F> SpanningTree<F, WeightedEdge<F>> for MinimumSpanningTree<F>
where
    F: Float,
{
    #[must_use]
    fn weight(&self) -> F {
        self.weight
    }

    #[must_use]
    fn edges(&self) -> &[WeightedEdge<F>] {
        &self.edges
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_create_weighted_edge() {
        let vertices = [0, 1, 2, 3];
        let graph = WeightedGraph::new(&vertices, |u: usize, v: usize| {
            let incidence_matrix = [
                [f64::NAN, 1.0, 4.0, 3.0],
                [1.0, f64::NAN, 7.0, 2.0],
                [4.0, 9.0, f64::NAN, 5.0],
                [3.0, 2.0, 5.0, f64::NAN],
            ];
            incidence_matrix[u][v]
        });
        let mst = MinimumSpanningTree::build(&graph);
        assert_eq!(mst.weight(), 7.0);
        assert_eq!(
            mst.edges(),
            &[
                WeightedEdge::new(3, 1, 2.0),
                WeightedEdge::new(1, 0, 1.0),
                WeightedEdge::new(0, 2, 4.0),
            ]
        );
    }
}

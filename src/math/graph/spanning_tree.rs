use crate::math::graph::graph::{Edge, Graph, WeightedEdge, WeightedGraph};
use crate::math::number::Float;
use std::collections::{BinaryHeap, HashSet};

/// Trait for spanning tree.
pub trait SpanningTree<F: Float, E: Edge> {
    /// Return total weight of this spanning tree.
    fn weight(&self) -> F;

    /// Return all edges of this graph.
    fn edges(&self) -> &[E];
}

/// Minimum spanning tree struct.
#[derive(Debug, Clone)]
pub struct MinimumSpanningTree<F: Float> {
    weight: F,
    edges: Vec<WeightedEdge<F>>,
}

impl<F> MinimumSpanningTree<F>
where
    F: Float,
{
    /// Build a minimum spanning tree.
    pub fn build<V, W>(graph: &WeightedGraph<V, W, F>) -> Self
    where
        W: Fn(usize, usize) -> F,
    {
        let vertices = graph.vertices();
        if vertices.is_empty() {
            return Self {
                weight: F::zero(),
                edges: Vec::new(),
            };
        }

        let n_vertices = vertices.len();
        let mut edges = Vec::new();
        let mut attached = HashSet::with_capacity(n_vertices);
        let mut candidates: BinaryHeap<WeightedEdge<F>> = BinaryHeap::new();
        let mut total_weight = F::zero();
        let mut current_index = n_vertices - 1;
        attached.insert(current_index);
        while attached.len() < n_vertices {
            graph
                .edges_at(current_index)
                .into_iter()
                .filter(|edge| !attached.contains(&edge.v()))
                .for_each(|edge| {
                    candidates.push(edge);
                });

            while let Some(edge) = candidates.pop() {
                if !attached.contains(&edge.v()) {
                    current_index = edge.v();
                    total_weight += edge.weight();
                    edges.push(edge);
                    attached.insert(current_index);
                    break;
                }
            }
        }
        Self {
            edges,
            weight: total_weight,
        }
    }
}

impl<F> SpanningTree<F, WeightedEdge<F>> for MinimumSpanningTree<F>
where
    F: Float,
{
    fn weight(&self) -> F {
        self.weight
    }

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

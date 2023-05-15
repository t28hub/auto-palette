use crate::math::clustering::hierarchical::node::HierarchicalNode;
use crate::math::clustering::hierarchical::union_find::UnionFind;
use crate::math::graph::edge::Edge;
use crate::math::graph::spanning_tree::{MinimumSpanningTree, SpanningTree};
use crate::math::graph::weighted_edge::WeightedEdge;
use crate::math::graph::weighted_graph::WeightedGraph;
use crate::math::number::Float;
use std::cmp::Ordering;

/// Struct representing a hierarchical clustering algorithm.
///
/// # Type parameters
/// * `F` - The type of distance calculation.
#[derive(Debug, PartialEq)]
pub struct HierarchicalClustering<F: Float> {
    hierarchy: Vec<HierarchicalNode<F>>,
}

impl<F> HierarchicalClustering<F>
where
    F: Float,
{
    /// Fits a hierarchical clustering algorithm to the given dataset.
    ///
    /// # Arguments
    /// * `dataset` - The dataset to fit the algorithm to.
    /// * `weight_fn` - The function used to calculate the weight of an edge between two data.
    ///
    /// # Returns
    /// A new `HierarchicalClustering` instance.
    #[must_use]
    pub fn fit<T, WF>(dataset: &[T], weight_fn: WF) -> Self
    where
        WF: Fn(usize, usize) -> F,
    {
        if dataset.is_empty() {
            return Self::default();
        }

        let graph = WeightedGraph::new(dataset, weight_fn);
        let spanning_tree = MinimumSpanningTree::build(&graph);
        let mut edges: Vec<WeightedEdge<F>> = spanning_tree.edges().to_vec();
        edges.sort_unstable_by(|edge1, edge2| {
            edge1
                .weight()
                .partial_cmp(&edge2.weight())
                .unwrap_or(Ordering::Greater)
        });

        let n_edge = edges.len();
        let n_node = n_edge + 1;
        let mut union_find = UnionFind::new(n_node);
        let nodes = edges
            .iter()
            .map(|edge| -> HierarchicalNode<F> {
                let root_u = union_find.find(edge.u());
                let root_v = union_find.find(edge.v());
                let size = union_find.union(root_u, root_v);
                HierarchicalNode {
                    left: root_u,
                    right: root_v,
                    weight: edge.weight(),
                    size,
                }
            })
            .collect();
        Self { hierarchy: nodes }
    }

    /// Returns the hierarchical nodes.
    ///
    /// # Returns
    /// A reference to the hierarchical nodes.
    #[must_use]
    pub fn nodes(&self) -> &Vec<HierarchicalNode<F>> {
        &self.hierarchy
    }
}

impl<F> Default for HierarchicalClustering<F>
where
    F: Float,
{
    #[must_use]
    fn default() -> Self {
        Self {
            hierarchy: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::distance::Distance::SquaredEuclidean;
    use crate::math::point::Point2;

    #[must_use]
    fn sample_dataset() -> Vec<Point2<f64>> {
        vec![
            Point2(0.0, 0.0),
            Point2(1.0, 1.0),
            Point2(2.0, 1.5),
            Point2(1.0, 0.0),
            Point2(2.0, 2.0),
            Point2(2.5, 3.0),
        ]
    }

    #[test]
    fn test_fit() {
        let dataset = sample_dataset();
        let actual = HierarchicalClustering::fit(&dataset, |u, v| {
            let point_u = &dataset[u];
            let point_v = &dataset[v];
            SquaredEuclidean.measure(point_u, point_v)
        });
        assert_eq!(
            actual.hierarchy,
            vec![
                HierarchicalNode {
                    left: 4,
                    right: 2,
                    weight: 0.25,
                    size: 2,
                },
                HierarchicalNode {
                    left: 1,
                    right: 3,
                    weight: 1.0,
                    size: 2,
                },
                HierarchicalNode {
                    left: 7,
                    right: 0,
                    weight: 1.0,
                    size: 3,
                },
                HierarchicalNode {
                    left: 5,
                    right: 6,
                    weight: 1.25,
                    size: 3,
                },
                HierarchicalNode {
                    left: 9,
                    right: 8,
                    weight: 1.25,
                    size: 6,
                },
            ]
        );
    }
}

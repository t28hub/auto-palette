use crate::math::clustering::hierarchical::node::HierarchicalNode;
use crate::math::clustering::hierarchical::union_find::UnionFind;
use crate::math::graph::edge::Edge;
use crate::math::graph::spanning_tree;
use crate::math::graph::spanning_tree::{MinimumSpanningTree, SpanningTree};
use crate::math::number::Float;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};

#[derive(Debug, PartialEq)]
pub struct HierarchicalClustering<F: Float> {
    hierarchy: Vec<HierarchicalNode<F>>,
}

impl<F> HierarchicalClustering<F>
where
    F: Float,
{
    #[must_use]
    pub fn fit<T, WF>(dataset: &[T], weight_fn: WF) -> Self
    where
        WF: Fn(usize, usize) -> F,
    {
        if dataset.is_empty() {
            return Self {
                hierarchy: Vec::new(),
            };
        }

        let spanning_tree = MinimumSpanningTree::build(dataset, weight_fn);
        let mut edges = spanning_tree.edges().to_vec();
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

    #[must_use]
    pub fn nodes(&self) -> &Vec<HierarchicalNode<F>> {
        &self.hierarchy
    }

    #[must_use]
    pub fn partition(&self, k: usize) -> Vec<usize> {
        let n_edge = self.hierarchy.len();
        let n_data = n_edge + 1;
        assert!(k <= n_data);

        let mut labels = vec![0; n_data];
        if k < 2 {
            return labels;
        }

        let mut node_ids = BinaryHeap::new();
        let root_node_id = n_edge * 2;
        node_ids.push(root_node_id);
        while node_ids.len() < k {
            let Some(node_id) = node_ids.pop() else {
                break;
            };

            let node = &self.hierarchy[node_id - n_data];
            node_ids.push(node.left);
            node_ids.push(node.right);
        }

        let mut cluster_id = 0;
        while let Some(node_id) = node_ids.pop() {
            if node_id >= n_data {
                self.bfs(node_id, cluster_id, &mut labels);
            } else {
                labels[node_id] = cluster_id;
            }
            cluster_id += 1;
        }
        labels
    }

    fn bfs(&self, root_node_id: usize, cluster_id: usize, labels: &mut [usize]) {
        let n_edge = self.hierarchy.len();
        let n_data = n_edge + 1;
        let root_node = &self.hierarchy[root_node_id - n_data];

        let mut queue = VecDeque::new();
        queue.push_back(root_node);
        while let Some(node) = queue.pop_front() {
            let left_id = node.left;
            if left_id >= n_data {
                queue.push_back(&self.hierarchy[left_id - n_data]);
            } else {
                labels[left_id] = cluster_id;
            }

            let right_id = node.right;
            if right_id >= n_data {
                queue.push_back(&self.hierarchy[right_id - n_data]);
            } else {
                labels[right_id] = cluster_id;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::distance::metric::DistanceMetric::{Euclidean, SquaredEuclidean};
    use crate::math::point::Point2;

    #[test]
    fn fit_should_return_hierarchical_clustering() {
        let dataset = vec![
            Point2::new(0.0, 0.1), // 0
            Point2::new(0.1, 0.1), // 1
            Point2::new(0.1, 0.0), // 2
            Point2::new(2.1, 2.0), // 3
            Point2::new(2.0, 2.0), // 4
            Point2::new(1.9, 2.1), // 5
            Point2::new(5.0, 5.1), // 6
        ];
        let hierarchical_clustering = HierarchicalClustering::fit(&dataset, |u, v| {
            let point_u = &dataset[u];
            let point_v = &dataset[v];
            Euclidean.measure(point_u, point_v)
        });
        assert_eq!(
            hierarchical_clustering.hierarchy,
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

    #[test]
    fn partition_should_partition_dataset() {
        let dataset = vec![
            Point2::new(0.0, 0.0), // 0
            Point2::new(1.0, 1.0), // 1
            Point2::new(2.0, 1.5), // 2
            Point2::new(1.0, 0.0), // 3
            Point2::new(2.0, 2.0), // 4
            Point2::new(2.5, 3.0), // 5
        ];
        let hierarchical_clustering = HierarchicalClustering::fit(&dataset, |u, v| {
            let point_u = &dataset[u];
            let point_v = &dataset[v];
            SquaredEuclidean.measure(point_u, point_v)
        });
        assert_eq!(hierarchical_clustering.partition(1), vec![0, 0, 0, 0, 0, 0]);
        assert_eq!(hierarchical_clustering.partition(2), vec![1, 1, 0, 1, 0, 0]);
        assert_eq!(hierarchical_clustering.partition(4), vec![3, 0, 1, 0, 1, 2]);
        assert_eq!(hierarchical_clustering.partition(6), vec![5, 4, 3, 2, 1, 0]);
    }
}

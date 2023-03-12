use crate::math::clustering::hdbscan::core_distance::CoreDistance;
use crate::math::clustering::hdbscan::params::Params;
use crate::math::clustering::hierarchical::algorithm::HierarchicalClustering;
use crate::math::clustering::hierarchical::node::HierarchicalNode;
use crate::math::clustering::traits::Fit;
use crate::math::number::Float;
use crate::math::point::Point;

/// HDBSCAN clustering algorithm.
#[derive(Debug, Clone)]
struct HDBSCAN {}

impl HDBSCAN {
    /// Create an HDBSCAN.
    fn new() -> Self {
        Self {}
    }

    fn condense_tree<F>(
        hierarchy: &[HierarchicalNode<F>],
        min_cluster_size: usize,
    ) -> Vec<CondensedNode<F>>
    where
        F: Float,
    {
        let n_edge = hierarchy.len();
        let n_data = n_edge + 1;
        let root_node_id = 2 * n_edge;

        let mut relabel = vec![0; root_node_id + 1];
        relabel[root_node_id] = n_data;

        let node_ids = Self::bfs_hierarchy(hierarchy, root_node_id);
        let mut next_label = n_data + 1;
        let mut visited = vec![false; root_node_id];
        let mut condensed = vec![];
        for node_id in node_ids {
            if visited[node_id] || node_id < n_data {
                continue;
            }

            let node = &hierarchy[node_id - n_data];
            let lambda = if node.weight > F::zero() {
                node.weight.recip()
            } else {
                F::max_value()
            };

            let left_node_id = node.left;
            let left_node_size = if let Some(left_node) = hierarchy.get(left_node_id - n_data) {
                left_node.size
            } else {
                1
            };

            let right_node_id = node.right;
            let right_node_size = if let Some(right_node) = hierarchy.get(right_node_id - n_data) {
                right_node.size
            } else {
                1
            };

            match (
                left_node_size >= min_cluster_size,
                right_node_size >= min_cluster_size,
            ) {
                (true, true) => {
                    relabel[left_node_id] = next_label;
                    condensed.push(CondensedNode {
                        parent_id: relabel[node_id],
                        child_id: relabel[left_node_id],
                        size: left_node_size,
                        lambda,
                    });
                    next_label += 1;

                    relabel[right_node_id] = next_label;
                    condensed.push(CondensedNode {
                        parent_id: relabel[node_id],
                        child_id: relabel[right_node_id],
                        size: right_node_size,
                        lambda,
                    });
                    next_label += 1;
                }
                (true, false) => {
                    relabel[left_node_id] = next_label;
                    Self::bfs_hierarchy(hierarchy, left_node_id)
                        .into_iter()
                        .for_each(|child_id| {
                            if child_id < n_data {
                                condensed.push(CondensedNode {
                                    parent_id: relabel[node_id],
                                    child_id,
                                    size: 1,
                                    lambda,
                                });
                            }
                            visited[child_id] = true;
                        });
                    next_label += 1;
                }
                (false, true) => {
                    relabel[right_node_id] = next_label;
                    Self::bfs_hierarchy(hierarchy, left_node_id)
                        .into_iter()
                        .for_each(|child_id| {
                            if child_id < n_data {
                                condensed.push(CondensedNode {
                                    parent_id: relabel[node_id],
                                    child_id,
                                    size: 1,
                                    lambda,
                                });
                            }
                            visited[child_id] = true;
                        });
                    next_label += 1;
                }
                (false, false) => {
                    for child_id in Self::bfs_hierarchy(hierarchy, node_id) {
                        if child_id == node_id {
                            continue;
                        }

                        if child_id < n_data {
                            condensed.push(CondensedNode {
                                parent_id: relabel[node_id],
                                child_id,
                                size: 1,
                                lambda,
                            });
                        }
                        visited[child_id] = true;
                    }
                }
            }
        }
        condensed
    }

    fn bfs_hierarchy<F>(hierarchy: &[HierarchicalNode<F>], root_node_id: usize) -> Vec<usize>
    where
        F: Float,
    {
        let n_edge = hierarchy.len();
        let mut to_process = vec![root_node_id];
        let mut node_ids = vec![];
        while !to_process.is_empty() {
            node_ids.extend_from_slice(to_process.as_slice());
            to_process = to_process
                .into_iter()
                .filter_map(|node_id| {
                    if node_id >= n_edge {
                        let node = &hierarchy[node_id - n_edge - 1];
                        Some([node.left, node.right])
                    } else {
                        None
                    }
                })
                .flatten()
                .collect()
        }
        node_ids
    }
}

impl<F, P> Fit<F, P, Params> for HDBSCAN
where
    F: Float,
    P: Point<F>,
{
    fn fit(dataset: &[P], params: &Params) -> Self {
        if dataset.is_empty() {
            return HDBSCAN::new();
        }

        let core_distance = CoreDistance::new(dataset, params.min_samples(), params.metric());
        let mutual_reachability_distance = |u: usize, v: usize| -> F {
            let point_u = &dataset[u];
            let point_v = &dataset[v];
            let distance = params.metric().measure(point_u, point_v);
            distance.max(
                core_distance
                    .distance_at(u)
                    .max(core_distance.distance_at(v)),
            )
        };
        let hierarchical_clustering =
            HierarchicalClustering::fit(dataset, mutual_reachability_distance);
        let hierarchy = hierarchical_clustering.nodes();
        HDBSCAN::condense_tree(hierarchy, params.min_cluster_size());
        todo!()
    }
}

#[derive(Debug, PartialEq)]
struct CondensedNode<F: Float> {
    pub parent_id: usize,
    pub child_id: usize,
    pub lambda: F,
    pub size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
}

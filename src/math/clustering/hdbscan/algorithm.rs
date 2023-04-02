use crate::math::clustering::algorithm::ClusteringAlgorithm;
use crate::math::clustering::cluster::Cluster;
use crate::math::clustering::hdbscan::core_distance::CoreDistance;
use crate::math::clustering::hdbscan::union_find::UnionFind;
use crate::math::clustering::hierarchical::algorithm::HierarchicalClustering;
use crate::math::clustering::hierarchical::node::HierarchicalNode;
use crate::math::clustering::model::Model;
use crate::math::distance::Distance;
use crate::math::number::Float;
use crate::math::point::Point;
use std::collections::{HashMap, HashSet};

/// Struct representing HDBSCAN clustering algorithm.
///
/// # References
/// * [How HDBSCAN Works](https://hdbscan.readthedocs.io/en/latest/how_hdbscan_works.html)
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq)]
pub struct HDBSCAN {
    min_samples: usize,
    min_cluster_size: usize,
    distance: Distance,
}

impl HDBSCAN {
    /// Creates a new `HDBSCAN` instance.
    ///
    /// # Arguments
    /// * `min_samples` - The minimum number of points.
    /// * `min_cluster_size` - The minimum number of points required to form a cluster.
    /// * `distance` - The distance metric to measure core distances.
    ///
    /// # Returns
    /// A new `HDBSCAN` instance.
    #[must_use]
    pub fn new(min_samples: usize, min_cluster_size: usize, distance: Distance) -> Self {
        Self {
            min_samples,
            min_cluster_size,
            distance,
        }
    }

    fn condense_tree<F>(&self, hierarchy: &[HierarchicalNode<F>]) -> Vec<CondensedNode<F>>
    where
        F: Float,
    {
        let n_data = hierarchy.len() + 1;
        let root_id = hierarchy.len() * 2;

        let mut relabel = vec![0; root_id + 1];
        relabel[root_id] = n_data;

        let mut next_label = n_data + 1;
        let mut ignored = vec![false; root_id + 1];
        let mut condensed = vec![];

        let node_ids = Self::bfs_hierarchy(hierarchy, root_id);
        for node_id in node_ids {
            if ignored[node_id] || node_id < n_data {
                continue;
            }

            let node = &hierarchy[node_id - n_data];
            let lambda = if node.weight > F::zero() {
                node.weight.recip()
            } else {
                F::max_value()
            };

            let left_id = node.left;
            let left_size = if left_id < n_data {
                1
            } else {
                hierarchy[left_id - n_data].size
            };

            let right_id = node.right;
            let right_size = if right_id < n_data {
                1
            } else {
                hierarchy[right_id - n_data].size
            };

            match (
                left_size >= self.min_cluster_size,
                right_size >= self.min_cluster_size,
            ) {
                (true, true) => {
                    relabel[left_id] = next_label;
                    condensed.push(CondensedNode {
                        parent_id: relabel[node_id],
                        child_id: relabel[left_id],
                        lambda,
                        size: left_size,
                    });
                    next_label += 1;

                    relabel[right_id] = next_label;
                    condensed.push(CondensedNode {
                        parent_id: relabel[node_id],
                        child_id: relabel[right_id],
                        lambda,
                        size: right_size,
                    });
                    next_label += 1;
                }
                (true, false) => {
                    relabel[left_id] = relabel[node_id];
                    for child_id in Self::bfs_hierarchy(hierarchy, right_id) {
                        if child_id < n_data {
                            condensed.push(CondensedNode {
                                parent_id: relabel[node_id],
                                child_id,
                                lambda,
                                size: 1,
                            });
                        }
                        ignored[child_id] = true;
                    }
                }
                (false, true) => {
                    relabel[right_id] = relabel[node_id];
                    for child_id in Self::bfs_hierarchy(hierarchy, left_id) {
                        if child_id < n_data {
                            condensed.push(CondensedNode {
                                parent_id: relabel[node_id],
                                child_id,
                                lambda,
                                size: 1,
                            });
                        }
                        ignored[child_id] = true;
                    }
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
                                lambda,
                                size: 1,
                            });
                        }
                        ignored[child_id] = true;
                    }
                }
            }
        }
        condensed
    }

    fn extract_clusters<F, P>(
        dataset: &[P],
        condensed: &[CondensedNode<F>],
    ) -> (Vec<Cluster<F, P>>, HashSet<usize>)
    where
        F: Float,
        P: Point<F>,
    {
        let mut stability = Self::compute_stability(condensed);
        let mut cluster_ids: HashSet<usize> = stability.keys().copied().collect();
        let mut node_ids: Vec<usize> = stability.keys().copied().collect();
        node_ids.sort_unstable();
        node_ids.reverse();
        node_ids.remove(node_ids.len() - 1);

        let tree: Vec<&CondensedNode<F>> = condensed.iter().filter(|node| node.size > 1).collect();
        for node_id in node_ids {
            let child_stability = tree.iter().fold(F::zero(), |total, node| {
                if node.parent_id == node_id {
                    total
                        + *stability
                            .get(&node.child_id)
                            .expect("Could not get stability of node")
                } else {
                    total
                }
            });

            stability.entry(node_id).and_modify(|value| {
                if *value < child_stability {
                    cluster_ids.remove(&node_id);
                    *value = child_stability;
                    return;
                }

                for child_id in Self::bfs_condensed_tree(condensed, node_id) {
                    if child_id == node_id {
                        continue;
                    }
                    cluster_ids.remove(&child_id);
                }
            });
        }

        let min_parent = condensed
            .iter()
            .min_by_key(|node| node.parent_id)
            .expect("Could not find minimum parent ID")
            .parent_id;
        let max_parent = condensed
            .iter()
            .max_by_key(|node| node.parent_id)
            .expect("Could not find maximum parent ID")
            .parent_id;
        let mut union_find = UnionFind::new(max_parent + 1);
        condensed.iter().for_each(|node| {
            if cluster_ids.contains(&node.child_id) {
                return;
            }
            union_find.union(node.parent_id, node.child_id);
        });

        let mut cluster_map = HashMap::new();
        let mut outlier_set = HashSet::new();
        #[allow(clippy::needless_range_loop)]
        for node_id in 0..min_parent {
            let cluster_id = union_find.find(node_id);
            if cluster_id > min_parent {
                let cluster = cluster_map
                    .entry(cluster_id)
                    .or_insert_with(Cluster::default);
                cluster.insert(node_id, &dataset[node_id]);
            } else {
                outlier_set.insert(node_id);
            }
        }

        let clusters = cluster_map
            .into_iter()
            .filter_map(|(_, mut cluster)| {
                if cluster.is_empty() {
                    None
                } else {
                    cluster.centroid.div_assign(F::from_usize(cluster.size()));
                    Some(cluster)
                }
            })
            .collect();
        (clusters, outlier_set)
    }

    fn compute_stability<F>(condensed: &[CondensedNode<F>]) -> HashMap<usize, F>
    where
        F: Float,
    {
        let mut births = condensed.iter().fold(HashMap::new(), |mut births, node| {
            let entry = births.entry(node.child_id).or_insert(node.lambda);
            *entry = node.lambda.min(*entry);
            births
        });

        let min_cluster_id = condensed
            .iter()
            .min_by_key(|node| node.parent_id)
            .expect("Could not find minimum cluster ID")
            .parent_id;

        let entry = births.entry(min_cluster_id).or_insert_with(F::zero);
        *entry = F::zero();

        condensed
            .iter()
            .fold(HashMap::new(), |mut stability, node| {
                let birth = births
                    .get(&node.parent_id)
                    .expect("Could not get birth of node");
                let entry = stability.entry(node.parent_id).or_insert(F::zero());
                *entry += (node.lambda - *birth) * F::from_usize(node.size);
                stability
            })
    }

    fn bfs_hierarchy<F>(hierarchy: &[HierarchicalNode<F>], root_id: usize) -> Vec<usize>
    where
        F: Float,
    {
        let n_data = hierarchy.len() + 1;
        let mut to_process = vec![root_id];
        let mut node_ids = vec![];
        while !to_process.is_empty() {
            node_ids.extend_from_slice(to_process.as_slice());
            to_process = to_process
                .into_iter()
                .filter_map(|node_id| {
                    if node_id >= n_data {
                        let node = &hierarchy[node_id - n_data];
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

    fn bfs_condensed_tree<F>(condensed: &[CondensedNode<F>], root_node_id: usize) -> Vec<usize>
    where
        F: Float,
    {
        let mut to_process = vec![root_node_id];
        let mut node_ids = vec![];
        while !to_process.is_empty() {
            node_ids.extend_from_slice(to_process.as_slice());
            to_process = condensed
                .iter()
                .filter_map(|node| {
                    if to_process.contains(&node.parent_id) {
                        Some(node.child_id)
                    } else {
                        None
                    }
                })
                .collect();
        }
        node_ids
    }
}

impl<F, P> ClusteringAlgorithm<F, P> for HDBSCAN
where
    F: Float,
    P: Point<F>,
{
    #[must_use]
    fn train(&self, dataset: &[P]) -> Model<F, P> {
        if dataset.is_empty() {
            return Model::default();
        }

        let core_distance = CoreDistance::new(dataset, self.min_samples, self.distance);
        let mutual_reachability_distance = |u: usize, v: usize| -> F {
            let point_u = &dataset[u];
            let point_v = &dataset[v];
            let distance = self.distance.measure(point_u, point_v);
            distance.max(
                core_distance
                    .distance_at(u)
                    .max(core_distance.distance_at(v)),
            )
        };
        let hierarchical_clustering =
            HierarchicalClustering::fit(dataset, mutual_reachability_distance);
        let hierarchy = hierarchical_clustering.nodes();
        let condensed = self.condense_tree(hierarchy);
        let (clusters, outliers) = HDBSCAN::extract_clusters(dataset, &condensed);
        Model::new(clusters, outliers)
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
    use crate::math::point::Point2;

    #[test]
    fn fit_should_cluster_dataset() {
        let dataset = vec![
            Point2::new(0.0, 0.0), // 0
            Point2::new(0.0, 1.0), // 1
            Point2::new(0.0, 7.0), // 2
            Point2::new(0.0, 8.0), // 3
            Point2::new(1.0, 0.0), // 4
            Point2::new(1.0, 1.0), // 5
            Point2::new(1.0, 2.0), // 6
            Point2::new(1.0, 7.0), // 7
            Point2::new(1.0, 8.0), // 8
            Point2::new(2.0, 1.0), // 9
            Point2::new(2.0, 2.0), // 10
            Point2::new(4.0, 3.0), // 11
            Point2::new(4.0, 4.0), // 12
            Point2::new(4.0, 5.0), // 13
            Point2::new(5.0, 3.0), // 14
            Point2::new(5.0, 4.0), // 15
            Point2::new(8.0, 3.0), // 16
            Point2::new(8.0, 4.0), // 17
            Point2::new(8.0, 5.0), // 18
            Point2::new(8.0, 7.0), // 19
            Point2::new(8.0, 8.0), // 20
        ];

        let hdbscan = HDBSCAN::new(3, 4, Distance::SquaredEuclidean);
        let model = hdbscan.train(&dataset);
        println!("{:?}", model);
    }
}

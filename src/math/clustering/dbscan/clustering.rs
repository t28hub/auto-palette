use crate::math::clustering::cluster::Cluster;
use crate::math::clustering::clustering::Clustering;
use crate::math::clustering::dbscan::label::Label;
use crate::math::clustering::model::Model;
use crate::math::distance::Distance;
use crate::math::neighbors::kdtree::KDTree;
use crate::math::neighbors::nns::{Neighbor, NeighborSearch};
use crate::math::number::Float;
use crate::math::point::Point;
use std::collections::{HashMap, HashSet, VecDeque};

/// Struct representing DBSCAN clustering algorithm.
///
/// # Type Parameters
/// * `F` - The float type used for calculations (e.g., f32 or f64).
/// * `P` - The type of points used in the clustering algorithm.
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq)]
pub struct DBSCAN<F>
where
    F: Float,
{
    min_samples: usize,
    epsilon: F,
    distance: Distance,
}

impl<F> DBSCAN<F>
where
    F: Float,
{
    /// Creates a new `DBSCAN` instance.
    ///
    /// # Arguments
    /// * `min_samples` - The minimum number of points.
    /// * `epsilon` - The maximum distance between two points.
    /// * `distance` - The distance metric.
    ///
    /// # Returns
    /// A new `DBSCAN` instance.
    #[must_use]
    pub fn new(min_samples: usize, epsilon: F, distance: Distance) -> Self {
        Self {
            min_samples,
            epsilon,
            distance,
        }
    }

    fn expand_cluster<P, N>(
        &self,
        cluster_id: usize,
        dataset: &[P],
        ns: &N,
        neighbors: &[Neighbor<F>],
        labels: &mut [Label],
    ) where
        P: Point<F>,
        N: NeighborSearch<F, P>,
    {
        let mut queue = VecDeque::new();
        queue.extend(neighbors.iter().map(|n| n.index));
        while let Some(current_index) = queue.pop_front() {
            if labels[current_index].is_assigned() {
                continue;
            }

            if labels[current_index].is_outlier() {
                labels[current_index] = Label::Assigned(cluster_id);
                continue;
            }

            labels[current_index] = Label::Assigned(cluster_id);

            let point = dataset[current_index];
            let secondary_neighbors = ns.search_radius(&point, self.epsilon);
            if secondary_neighbors.len() < self.min_samples {
                continue;
            }

            for secondary_neighbor in secondary_neighbors.into_iter() {
                let secondary_index = secondary_neighbor.index;
                match labels[secondary_index] {
                    Label::Undefined => {
                        labels[secondary_index] = Label::Marked;
                        queue.push_back(secondary_index);
                    }
                    Label::Outlier => {
                        queue.push_back(secondary_index);
                    }
                    _ => {}
                }
            }
        }
    }
}

impl<F, P> Clustering<F, P> for DBSCAN<F>
where
    F: Float,
    P: Point<F>,
{
    #[must_use]
    fn train(&self, dataset: &[P]) -> Model<F, P> {
        if dataset.is_empty() {
            return Model::default();
        }

        let dataset_vec = dataset.to_vec();
        let nns = KDTree::new(&dataset_vec, self.distance);
        let mut labels = vec![Label::Undefined; dataset.len()];
        let mut cluster_id: usize = 0;
        for (index, point) in dataset.iter().enumerate() {
            if !labels[index].is_undefined() {
                continue;
            }

            let neighbors = nns.search_radius(point, self.epsilon);
            if neighbors.len() < self.min_samples {
                labels[index] = Label::Outlier;
                continue;
            }

            neighbors.iter().for_each(|neighbor| {
                labels[neighbor.index] = Label::Marked;
            });
            self.expand_cluster(cluster_id, dataset, &nns, &neighbors, &mut labels);
            cluster_id += 1;
        }

        let mut cluster_map: HashMap<usize, Cluster<F, P>> = HashMap::new();
        let mut outlier_set: HashSet<usize> = HashSet::new();
        for (index, label) in labels.into_iter().enumerate() {
            match label {
                Label::Assigned(cluster_id) => {
                    let cluster = cluster_map
                        .entry(cluster_id)
                        .or_insert_with(|| Cluster::new(cluster_id));
                    cluster.insert(index, &dataset[index]);
                }
                Label::Outlier => {
                    outlier_set.insert(index);
                }
                _ => unreachable!(
                    "All points in the dataset are assigned to any cluster or labeled as outlier"
                ),
            }
        }

        let clusters: Vec<Cluster<F, P>> = cluster_map
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
        Model::new(clusters, outlier_set)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::distance::Distance;
    use crate::math::point::Point2;

    const DATASET: [Point2<f64>; 16] = [
        Point2(0.0, 0.0), // 0
        Point2(0.0, 1.0), // 0
        Point2(0.0, 7.0), // 1
        Point2(0.0, 8.0), // 1
        Point2(1.0, 0.0), // 0
        Point2(1.0, 1.0), // 0
        Point2(1.0, 2.0), // 0
        Point2(1.0, 7.0), // 2
        Point2(1.0, 8.0), // 2
        Point2(2.0, 1.0), // 0
        Point2(2.0, 2.0), // 0
        Point2(4.0, 3.0), // 2
        Point2(4.0, 4.0), // 2
        Point2(4.0, 5.0), // 2
        Point2(5.0, 3.0), // 2
        Point2(5.0, 4.0), // 2
    ];

    #[test]
    fn fit_should_fit_dataset() {
        let dataset = Vec::from(DATASET);
        let dbscan = DBSCAN::new(4, 2.0_f64.sqrt(), Distance::Euclidean);
        let model = dbscan.train(&dataset);

        let mut centroids: Vec<_> = model
            .clusters()
            .iter()
            .map(|cluster| cluster.centroid())
            .cloned()
            .collect();
        centroids.sort_by(|point1, point2| point1.0.total_cmp(&point2.0));
        assert_eq!(
            centroids,
            Vec::from([Point2(0.5, 7.5), Point2(1.0, 1.0), Point2(4.4, 3.8)])
        );
        assert_eq!(model.outliers(), &HashSet::new());
    }
}

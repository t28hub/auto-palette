use crate::math::clustering::algorithm::ClusteringAlgorithm;
use crate::math::clustering::cluster::Cluster;
use crate::math::clustering::dbscan::label::Label;
use crate::math::distance::DistanceMetric;
use crate::math::neighbors::kdtree::search::KDTreeSearch;
use crate::math::neighbors::neighbor::Neighbor;
use crate::math::neighbors::search::NeighborSearch;
use crate::math::number::Float;
use crate::math::point::Point;
use std::collections::{HashMap, HashSet, VecDeque};

/// Struct representing DBSCAN clustering algorithm.
///
/// # Type Parameters
/// * `F` - The float type used for calculations.
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq)]
pub struct DBSCAN<'a, F>
where
    F: Float,
{
    min_samples: usize,
    epsilon: F,
    metric: &'a DistanceMetric,
}

impl<'a, F> DBSCAN<'a, F>
where
    F: Float,
{
    /// Creates a new `DBSCAN` instance.
    ///
    /// # Arguments
    /// * `min_samples` - The minimum number of points.
    /// * `epsilon` - The maximum distance between two points.
    /// * `metric` - The distance metric.
    ///
    /// # Returns
    /// A new `DBSCAN` instance.
    #[must_use]
    pub fn new(min_samples: usize, epsilon: F, metric: &'a DistanceMetric) -> Self {
        Self {
            min_samples,
            epsilon,
            metric,
        }
    }

    fn expand_cluster<P, N>(
        &self,
        cluster_id: usize,
        points: &[P],
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

            let point = points[current_index];
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

impl<'a, F, P> ClusteringAlgorithm<F, P> for DBSCAN<'a, F>
where
    F: Float,
    P: Point<F>,
{
    type Output = (Vec<Cluster<F, P>>, HashSet<usize>);

    #[must_use]
    fn fit(&self, points: &[P]) -> Self::Output {
        if points.is_empty() {
            return (Vec::new(), HashSet::new());
        }

        let neighbor_search = KDTreeSearch::new(points, self.metric);
        let mut labels = vec![Label::Undefined; points.len()];
        let mut cluster_id: usize = 0;
        for (index, point) in points.iter().enumerate() {
            if !labels[index].is_undefined() {
                continue;
            }

            let neighbors = neighbor_search.search_radius(point, self.epsilon);
            if neighbors.len() < self.min_samples {
                labels[index] = Label::Outlier;
                continue;
            }

            neighbors.iter().for_each(|neighbor| {
                labels[neighbor.index] = Label::Marked;
            });
            self.expand_cluster(
                cluster_id,
                points,
                &neighbor_search,
                &neighbors,
                &mut labels,
            );
            cluster_id += 1;
        }

        let mut cluster_map: HashMap<usize, Cluster<F, P>> = HashMap::new();
        let mut outlier_set: HashSet<usize> = HashSet::new();
        for (index, label) in labels.into_iter().enumerate() {
            match label {
                Label::Assigned(cluster_id) => {
                    let cluster = cluster_map.entry(cluster_id).or_default();
                    cluster.insert(index, &points[index]);
                }
                Label::Outlier => {
                    outlier_set.insert(index);
                }
                _ => {}
            }
        }

        let clusters: Vec<Cluster<F, P>> = cluster_map
            .into_iter()
            .filter_map(|(_, cluster)| {
                if cluster.is_empty() {
                    None
                } else {
                    Some(cluster)
                }
            })
            .collect();
        (clusters, outlier_set)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::distance::DistanceMetric;
    use crate::math::point::Point2;

    #[must_use]
    fn sample_points() -> Vec<Point2<f64>> {
        vec![
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
        ]
    }

    #[test]
    fn test_dbscan() {
        let actual = DBSCAN::new(4, 2.0_f64.sqrt(), &DistanceMetric::Euclidean);
        assert_eq!(actual.min_samples, 4);
        assert_eq!(actual.epsilon, 2.0_f64.sqrt());
        assert_eq!(actual.metric, &DistanceMetric::Euclidean);
    }

    #[test]
    fn test_fit() {
        let points = sample_points();
        let dbscan = DBSCAN::new(4, 2.0_f64.sqrt(), &DistanceMetric::Euclidean);
        let (mut clusters, outliers) = dbscan.fit(&points);

        clusters.sort_by(|cluster1, cluster2| {
            cluster1
                .membership()
                .len()
                .cmp(&cluster2.membership().len())
        });

        assert_eq!(clusters.len(), 3);
        assert_eq!(clusters[0].centroid(), &Point2(0.5, 7.5));
        assert_eq!(clusters[0].membership(), &[2, 3, 7, 8]);
        assert_eq!(clusters[1].centroid(), &Point2(4.4, 3.8));
        assert_eq!(clusters[1].membership(), &[11, 12, 13, 14, 15]);
        assert_eq!(clusters[2].centroid(), &Point2(1.0, 1.0));
        assert_eq!(clusters[2].membership(), &[0, 1, 4, 5, 6, 9, 10]);
        assert_eq!(outliers.len(), 0);
    }
}

use crate::math::clustering::{Cluster, ClusteringAlgorithm};
use crate::math::neighbors::kdtree::KDTreeSearch;
use crate::math::neighbors::neighbor::Neighbor;
use crate::math::neighbors::search::NeighborSearch;
use crate::math::{DistanceMetric, FloatNumber, Point};
use std::collections::{HashMap, VecDeque};

const OUTLIER: i32 = -1;
const MARKED: i32 = -2;
const UNCLASSIFIED: i32 = -3;

/// DBSCAN++ clustering algorithm.
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug)]
pub struct DBSCANpp<T>
where
    T: FloatNumber,
{
    probability: T,
    min_points: usize,
    epsilon: T,
    metric: DistanceMetric,
}

impl<T> DBSCANpp<T>
where
    T: FloatNumber,
{
    /// Creates a new `DBSCANpp` instance.
    ///
    /// # Arguments
    /// * `probability` - The probability of a point being a core point.
    /// * `min_points` - The minimum number of points to form a cluster.
    /// * `epsilon` - The epsilon radius for determining neighbors.
    /// * `metric` - The distance metric to use.
    ///
    /// # Returns
    /// A new `DBSCANpp` instance.
    pub fn new(
        probability: T,
        min_points: usize,
        epsilon: T,
        metric: DistanceMetric,
    ) -> Result<Self, &'static str> {
        if !(T::zero()..=T::one()).contains(&probability) {
            return Err("The probability must be in the range [0, 1].");
        }
        if min_points == 0 {
            return Err("The minimum number of points must be greater than zero.");
        }
        if epsilon <= T::zero() {
            return Err("The epsilon must be greater than zero.");
        }
        Ok(Self {
            probability,
            min_points,
            epsilon,
            metric,
        })
    }

    #[must_use]
    fn find_core_points<const N: usize, NS>(
        &self,
        points: &[Point<T, N>],
        points_search: &NS,
    ) -> Vec<Point<T, N>>
    where
        NS: NeighborSearch<T, N>,
    {
        let step = (T::one() / self.probability).round().to_usize_unsafe();
        points
            .iter()
            .step_by(step)
            .filter_map(|point| {
                let neighbors = points_search.search_radius(point, self.epsilon);
                if neighbors.len() >= self.min_points {
                    Some(*point)
                } else {
                    None
                }
            })
            .collect()
    }

    #[must_use]
    fn label_core_points<const N: usize>(
        &self,
        core_points: &[Point<T, N>],
        core_points_search: &KDTreeSearch<T, N>,
    ) -> Vec<i32> {
        let mut label = 0;
        let mut labels = vec![UNCLASSIFIED; core_points.len()];
        for (index, core_point) in core_points.iter().enumerate() {
            if labels[index] != UNCLASSIFIED {
                continue;
            }

            let neighbors = core_points_search.search_radius(core_point, self.epsilon);
            if neighbors.len() < self.min_points {
                labels[index] = OUTLIER;
                continue;
            }

            for neighbor in &neighbors {
                if labels[neighbor.index] != UNCLASSIFIED {
                    continue;
                }
                labels[neighbor.index] = MARKED;
            }

            self.expand_cluster(
                label,
                &mut labels,
                core_points,
                neighbors,
                core_points_search,
            );
            label += 1;
        }
        labels
    }

    #[inline]
    fn expand_cluster<const N: usize, NS>(
        &self,
        label: i32,
        labels: &mut [i32],
        points: &[Point<T, N>],
        neighbors: Vec<Neighbor<T>>,
        neighbor_search: &NS,
    ) where
        NS: NeighborSearch<T, N>,
    {
        let mut queue = VecDeque::from(neighbors);
        while let Some(neighbor) = queue.pop_front() {
            let index = neighbor.index;
            // Skip if the point is already assigned to a cluster.
            if labels[index] >= 0 {
                continue;
            }

            let point = &points[index];
            if labels[index] == OUTLIER {
                labels[index] = label;
                continue;
            }

            labels[index] = label;

            let secondary_neighbors = neighbor_search.search_radius(point, self.epsilon);
            if secondary_neighbors.len() < self.min_points {
                continue;
            }

            for secondary_neighbor in secondary_neighbors {
                let secondary_index = secondary_neighbor.index;
                if labels[secondary_index] == UNCLASSIFIED {
                    labels[secondary_index] = MARKED;
                    queue.push_back(secondary_neighbor);
                } else if labels[secondary_index] == OUTLIER {
                    queue.push_back(secondary_neighbor);
                }
            }
        }
    }

    #[must_use]
    fn assign_clusters<const N: usize, NS>(
        &self,
        points: &[Point<T, N>],
        core_labels: &[i32],
        core_points_search: &NS,
    ) -> Vec<Cluster<T, N>>
    where
        NS: NeighborSearch<T, N>,
    {
        let mut clusters = HashMap::new();
        for (index, point) in points.iter().enumerate() {
            let Some(nearest) = core_points_search.search_nearest(point) else {
                continue;
            };

            if nearest.distance > self.epsilon {
                continue;
            }

            let core_label = core_labels[nearest.index];
            if core_label < 0 {
                continue;
            }

            let cluster = clusters.entry(core_label).or_insert_with(Cluster::new);
            cluster.add_member(index, point);
        }
        clusters.into_values().collect()
    }
}

impl<T, const N: usize> ClusteringAlgorithm<T, N> for DBSCANpp<T>
where
    T: FloatNumber,
{
    #[must_use]
    fn fit(&self, points: &[Point<T, N>]) -> Vec<Cluster<T, N>> {
        if points.is_empty() {
            return Vec::new();
        }

        let points_search = KDTreeSearch::build(points, self.metric.clone(), 16);
        let core_points = self.find_core_points(points, &points_search);
        let core_points_search = KDTreeSearch::build(&core_points, self.metric.clone(), 16);
        let core_labels = self.label_core_points(&core_points, &core_points_search);
        self.assign_clusters(points, &core_labels, &core_points_search)
    }
}

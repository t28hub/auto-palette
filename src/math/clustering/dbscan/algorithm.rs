use std::collections::VecDeque;

use crate::math::clustering::{Cluster, ClusteringAlgorithm};
use crate::math::neighbors::linear::LinearSearch;
use crate::math::neighbors::neighbor::Neighbor;
use crate::math::neighbors::search::NeighborSearch;
use crate::math::{DistanceMetric, Point};

const OUTLIER: i32 = -1;
const MARKED: i32 = -2;
const UNCLASSIFIED: i32 = -3;

/// A density-based spatial clustering of applications with noise (DBSCAN) algorithm.
#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub struct DBSCAN {
    min_points: usize,
    epsilon: f32,
    metric: DistanceMetric,
}

impl DBSCAN {
    /// Creates a new `DBSCAN` instance.
    ///
    /// # Arguments
    /// * `min_points` - The minimum number of points to form a cluster.
    /// * `epsilon` - The epsilon radius for determining neighbors.
    /// * `metric` - The distance metric to use.
    ///
    /// # Returns
    /// A new `DBSCAN` instance.
    pub fn new(
        min_points: usize,
        epsilon: f32,
        metric: DistanceMetric,
    ) -> Result<Self, &'static str> {
        if min_points == 0 {
            return Err("The minimum number of points must be greater than zero.");
        }
        if epsilon <= 0.0 {
            return Err("The epsilon must be greater than zero.");
        }
        Ok(Self {
            min_points,
            epsilon,
            metric,
        })
    }

    #[inline]
    #[must_use]
    fn expand_cluster<NS, const N: usize>(
        &self,
        label: i32,
        labels: &mut [i32],
        points: &[Point<N>],
        neighbors: Vec<Neighbor>,
        neighbor_search: &NS,
    ) -> Cluster<N>
    where
        NS: NeighborSearch<N>,
    {
        let mut cluster = Cluster::new();
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
                cluster.add_member(index, point);
                continue;
            }

            labels[index] = label;
            cluster.add_member(index, point);

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
        cluster
    }
}

impl ClusteringAlgorithm for DBSCAN {
    #[must_use]
    fn fit<const N: usize>(&self, points: &[Point<N>]) -> Vec<Cluster<N>> {
        if points.is_empty() {
            return Vec::new();
        }

        let mut label = 0;
        let mut labels = vec![UNCLASSIFIED; points.len()];
        let mut clusters = Vec::new();
        let neighbor_search = LinearSearch::build(points, self.metric.clone());
        for (index, point) in points.iter().enumerate() {
            if labels[index] != UNCLASSIFIED {
                continue;
            }

            let neighbors = neighbor_search.search_radius(point, self.epsilon);
            if neighbors.len() < self.min_points {
                labels[index] = OUTLIER;
                continue;
            }

            // Mark the point as a candidate for clustering.
            for neighbor in &neighbors {
                if labels[neighbor.index] != UNCLASSIFIED {
                    continue;
                }
                labels[neighbor.index] = MARKED;
            }

            let cluster =
                self.expand_cluster(label, &mut labels, points, neighbors, &neighbor_search);
            if cluster.len() >= self.min_points {
                clusters.push(cluster);
            }
            label += 1;
        }
        clusters
    }
}

#[cfg(test)]
mod tests {
    use crate::math::DistanceMetric;

    use super::*;

    #[must_use]
    fn sample_points() -> Vec<Point<2>> {
        vec![
            [0.0, 0.0], // 0
            [0.0, 1.0], // 0
            [0.0, 7.0], // 1
            [0.0, 8.0], // 1
            [1.0, 0.0], // 0
            [1.0, 1.0], // 0
            [1.0, 2.0], // 0
            [1.0, 7.0], // 1
            [1.0, 8.0], // 1
            [2.0, 1.0], // 0
            [2.0, 2.0], // 0
            [4.0, 3.0], // 2
            [4.0, 4.0], // 2
            [4.0, 5.0], // 2
            [5.0, 3.0], // 2
            [5.0, 4.0], // 2
            [9.0, 8.0], // Outlier
        ]
    }

    #[test]
    fn test_new_dbscan() {
        // Act
        let dbscan = DBSCAN::new(5, 1e-3, DistanceMetric::Euclidean).unwrap();

        // Assert
        assert_eq!(dbscan.min_points, 5);
        assert_eq!(dbscan.epsilon, 1e-3);
        assert_eq!(dbscan.metric, DistanceMetric::Euclidean);
    }

    #[test]
    fn test_new_dbscan_min_points_zero() {
        // Act
        let result = DBSCAN::new(0, 1e-3, DistanceMetric::Euclidean);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_new_dbscan_epsilon_zero() {
        // Act
        let result = DBSCAN::new(5, 0.0, DistanceMetric::Euclidean);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_fit() {
        // Act
        let points = sample_points();
        let dbscan = DBSCAN::new(4, 2.0, DistanceMetric::Euclidean).unwrap();
        let mut clusters = dbscan.fit(&points);
        clusters.sort_by(|cluster1, cluster2| cluster2.len().cmp(&cluster1.len()));

        // Assert
        assert_eq!(clusters.len(), 3);
        assert_eq!(clusters[0].len(), 7);
        assert_eq!(clusters[0].centroid(), &[1.0, 1.0]);
        assert_eq!(clusters[1].len(), 5);
        assert_eq!(clusters[1].centroid(), &[4.4, 3.8]);
        assert_eq!(clusters[2].len(), 4);
        assert_eq!(clusters[2].centroid(), &[0.5, 7.5]);
    }

    #[test]
    fn test_fit_empty() {
        // Act
        let points: Vec<Point<2>> = Vec::new();
        let dbscan = DBSCAN::new(4, 2.0, DistanceMetric::Euclidean).unwrap();
        let clusters = dbscan.fit(&points);

        // Assert
        assert!(clusters.is_empty());
    }
}

use std::{collections::VecDeque, fmt::Display};

use thiserror::Error;

use crate::math::{
    clustering::{Cluster, ClusteringAlgorithm},
    neighbors::{kdtree::KDTreeSearch, Neighbor, NeighborSearch},
    DistanceMetric,
    FloatNumber,
    Point,
};

#[derive(Debug, PartialEq, Error)]
pub enum DBSCANError<T>
where
    T: FloatNumber + Display,
{
    /// Error when the minimum number of points is invalid.
    #[error("Invalid minimum points: The minimum number of points must be greater than zero: {0}")]
    InvalidMinPoints(usize),

    /// Error when the epsilon is invalid.
    #[error("Invalid epsilon: The epsilon must be greater than zero: {0}")]
    InvalidEpsilon(T),
}

const OUTLIER: i32 = -1;
const MARKED: i32 = -2;
const UNCLASSIFIED: i32 = -3;

/// DBSCAN clustering algorithm.
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
pub struct DBSCAN<T>
where
    T: FloatNumber,
{
    min_points: usize,
    epsilon: T,
    metric: DistanceMetric,
}

impl<T> DBSCAN<T>
where
    T: FloatNumber,
{
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
        epsilon: T,
        metric: DistanceMetric,
    ) -> Result<Self, DBSCANError<T>> {
        if min_points == 0 {
            return Err(DBSCANError::InvalidMinPoints(min_points));
        }
        if epsilon <= T::zero() {
            return Err(DBSCANError::InvalidEpsilon(epsilon));
        }
        Ok(Self {
            min_points,
            epsilon,
            metric,
        })
    }

    #[inline]
    #[must_use]
    fn expand_cluster<const N: usize, NS>(
        &self,
        label: i32,
        labels: &mut [i32],
        points: &[Point<T, N>],
        neighbors: Vec<Neighbor<T>>,
        neighbor_search: &NS,
    ) -> Cluster<T, N>
    where
        NS: NeighborSearch<T, N>,
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

impl<T, const N: usize> ClusteringAlgorithm<T, N> for DBSCAN<T>
where
    T: FloatNumber,
{
    type Err = DBSCANError<T>;

    fn fit(&self, points: &[Point<T, N>]) -> Result<Vec<Cluster<T, N>>, Self::Err> {
        let mut labels = vec![UNCLASSIFIED; points.len()];
        let mut clusters = Vec::new();
        let mut current_label = 0;
        let neighbor_search = KDTreeSearch::build(points, self.metric, 16);
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

            let cluster = self.expand_cluster(
                current_label,
                &mut labels,
                points,
                neighbors,
                &neighbor_search,
            );
            if cluster.len() >= self.min_points {
                clusters.push(cluster);
            }
            current_label += 1;
        }
        Ok(clusters)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::math::DistanceMetric;

    #[must_use]
    fn sample_points() -> Vec<Point<f32, 2>> {
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

    #[must_use]
    fn empty_points() -> Vec<Point<f32, 2>> {
        Vec::new()
    }

    #[test]
    fn test_new() {
        // Act
        let actual = DBSCAN::new(5, 1e-3, DistanceMetric::Euclidean).unwrap();

        // Assert
        assert_eq!(actual.min_points, 5);
        assert_eq!(actual.epsilon, 1e-3);
        assert_eq!(actual.metric, DistanceMetric::Euclidean);
    }

    #[rstest]
    #[case::invalid_min_points(
        0,
        1e-3,
        DistanceMetric::Euclidean,
        DBSCANError::InvalidMinPoints(0)
    )]
    #[case::invalid_epsilon(5, 0.0, DistanceMetric::Euclidean, DBSCANError::InvalidEpsilon(0.0))]
    fn test_new_error(
        #[case] min_points: usize,
        #[case] epsilon: f32,
        #[case] metric: DistanceMetric,
        #[case] expected: DBSCANError<f32>,
    ) {
        // Act
        let actual = DBSCAN::new(min_points, epsilon, metric);

        // Assert
        assert!(actual.is_err());
        assert_eq!(actual, Err(expected));
    }

    #[test]
    fn test_fit() {
        // Act
        let points = sample_points();
        let dbscan = DBSCAN::new(4, 2.0, DistanceMetric::Euclidean).unwrap();

        let mut actual = dbscan.fit(&points).unwrap();
        actual.sort_by(|cluster1, cluster2| cluster2.len().cmp(&cluster1.len()));

        // Assert
        assert_eq!(actual.len(), 3);
        assert_eq!(actual[0].len(), 7);
        assert_eq!(actual[0].centroid(), &[1.0, 1.0]);
        assert_eq!(actual[1].len(), 5);
        assert_eq!(actual[1].centroid(), &[4.4, 3.8]);
        assert_eq!(actual[2].len(), 4);
        assert_eq!(actual[2].centroid(), &[0.5, 7.5]);
    }

    #[test]
    fn test_fit_empty() {
        // Act
        let points = empty_points();
        let dbscan = DBSCAN::new(4, 2.0, DistanceMetric::Euclidean).unwrap();
        let actual = dbscan.fit(&points);

        // Assert
        assert!(actual.is_ok());

        let clusters = actual.unwrap();
        assert_eq!(clusters.len(), 0);
    }
}

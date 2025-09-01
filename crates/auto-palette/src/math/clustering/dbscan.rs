use std::{collections::VecDeque, fmt::Display};

use thiserror::Error;

use crate::math::{
    clustering::ClusteringAlgorithm,
    neighbors::{kdtree::KdTreeSearch, Neighbor, NeighborSearch},
    DistanceMetric,
    FloatNumber,
    Point,
};

/// Error type for the DBSCAN algorithm.
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug, PartialEq, Error)]
pub enum DBSCANError<T>
where
    T: FloatNumber + Display,
{
    /// Error when the minimum number of points is invalid.
    #[error("Minimum number of points must be greater than zero, got: {0}")]
    InvalidMinPoints(usize),

    /// Error when the epsilon is invalid.
    #[error("Epsilon must be greater than zero, got: {0}")]
    InvalidEpsilon(T),
}

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
    /// The initial label for clusters.
    pub const LABEL_INITIAL: usize = 0;

    /// The label used for noise points.
    pub const LABEL_NOISE: usize = usize::MAX - 1;

    /// The label used for unclassified points.
    pub const LABEL_UNCLASSIFIED: usize = usize::MAX;

    const LEAF_SIZE: usize = 16;

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
    fn expand_cluster<const N: usize, NS>(
        &self,
        label: usize,
        labels: &mut [usize],
        points: &[Point<T, N>],
        neighbors: Vec<Neighbor<T>>,
        neighbor_search: &NS,
    ) where
        NS: NeighborSearch<T, N>,
    {
        let mut queue: VecDeque<_> = neighbors.into();
        while let Some(neighbor) = queue.pop_front() {
            let index = neighbor.index;
            let point = &points[index];
            if labels[index] == Self::LABEL_NOISE {
                labels[index] = label;
                continue;
            }

            // Skip if the point is already assigned to a cluster.
            if labels[index] != Self::LABEL_UNCLASSIFIED {
                continue;
            }

            labels[index] = label;

            let secondary_neighbors = neighbor_search.search_radius(point, self.epsilon);
            if secondary_neighbors.len() >= self.min_points {
                queue.extend(secondary_neighbors);
            }
        }
    }
}

impl<T, const N: usize> ClusteringAlgorithm<T, N> for DBSCAN<T>
where
    T: FloatNumber,
{
    type Output = Vec<usize>;

    type Error = DBSCANError<T>;

    fn run(&self, points: &[Point<T, N>]) -> Result<Self::Output, Self::Error> {
        let mut labels = vec![Self::LABEL_UNCLASSIFIED; points.len()];
        let mut current_label = Self::LABEL_INITIAL;
        let neighbor_search = KdTreeSearch::with_leaf_size(points, self.metric, Self::LEAF_SIZE);
        for (index, point) in points.iter().enumerate() {
            if labels[index] != Self::LABEL_UNCLASSIFIED {
                continue;
            }

            let neighbors = neighbor_search.search_radius(point, self.epsilon);
            if neighbors.len() < self.min_points {
                labels[index] = Self::LABEL_NOISE;
                continue;
            }

            self.expand_cluster(
                current_label,
                &mut labels,
                points,
                neighbors,
                &neighbor_search,
            );
            current_label += 1;
        }
        Ok(labels)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::math::DistanceMetric;

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
    fn test_run() {
        // Arrange
        let points = vec![
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
            [9.0, 8.0], // Noise
        ];

        // Act
        let dbscan = DBSCAN::new(4, 2.0, DistanceMetric::Euclidean).unwrap();
        let actual = dbscan.run(&points);

        // Assert
        assert!(actual.is_ok());

        let labels = actual.unwrap();
        assert_eq!(labels.len(), points.len());
        assert_eq!(
            labels,
            vec![
                0,
                0,
                1,
                1,
                0,
                0,
                0,
                1,
                1,
                0,
                0,
                2,
                2,
                2,
                2,
                2,
                DBSCAN::<f64>::LABEL_NOISE
            ]
        );
    }

    #[test]
    fn test_run_with_boundary_points() {
        // Arrange
        let points = vec![
            [1.1, 0.0],  // Noise
            [0.0, 1.1],  // Noise
            [-1.1, 0.0], // Noise
            [0.0, -1.1], // Noise
            [0.0, 0.0],  // Core point
            [0.0, 1.0],  // 0
            [1.0, 0.0],  // 0
            [0.0, -1.0], // 0
            [-1.0, 0.0], // 0
        ];

        // Act
        let dbscan = DBSCAN::new(5, 1.0, DistanceMetric::Euclidean).unwrap();
        let actual = dbscan.run(&points);

        // Assert
        assert!(actual.is_ok());

        let labels = actual.unwrap();
        assert_eq!(labels.len(), points.len());
        assert_eq!(
            labels,
            vec![
                DBSCAN::<f64>::LABEL_NOISE, // Noise
                DBSCAN::<f64>::LABEL_NOISE, // Noise
                DBSCAN::<f64>::LABEL_NOISE, // Noise
                DBSCAN::<f64>::LABEL_NOISE, // Noise
                0,                          // Core point
                0,                          // 0
                0,                          // 0
                0,                          // 0
                0,                          // 0
            ]
        );
    }

    #[test]
    fn test_run_same_points() {
        // Arrange
        let points = vec![
            [0.0, 0.0], // 0
            [0.0, 0.0], // 0
            [0.0, 0.0], // 0
            [0.0, 0.0], // 0
        ];

        // Act
        let dbscan = DBSCAN::new(4, 0.1, DistanceMetric::Euclidean).unwrap();
        let actual = dbscan.run(&points);

        // Assert
        assert!(actual.is_ok());

        let labels = actual.unwrap();
        assert_eq!(labels.len(), points.len());
        assert_eq!(labels, vec![0, 0, 0, 0]); // All points belong to the same cluster
    }

    #[test]
    fn test_run_all_noise() {
        // Arrange
        let points = vec![
            [1.0, 1.0], // Noise
            [2.0, 2.0], // Noise
            [3.0, 3.0], // Noise
            [4.0, 4.0], // Noise
        ];

        // Act
        let dbscan = DBSCAN::new(2, 1.0, DistanceMetric::Euclidean).unwrap();
        let actual = dbscan.run(&points);

        // Assert
        assert!(actual.is_ok());

        let labels = actual.unwrap();
        assert_eq!(labels.len(), points.len());
        assert_eq!(labels, vec![DBSCAN::<f64>::LABEL_NOISE; points.len()]); // All points are noise
    }

    #[test]
    fn test_run_empty() {
        // Arrange
        let points: Vec<Point<f64, 2>> = vec![];

        // Act
        let dbscan = DBSCAN::new(4, 2.0, DistanceMetric::Euclidean).unwrap();
        let actual = dbscan.run(&points);

        // Assert
        assert!(actual.is_ok());

        let labels = actual.unwrap();
        assert_eq!(labels.len(), 0);
    }
}

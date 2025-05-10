use std::collections::{HashMap, VecDeque};

use thiserror::Error;

use crate::math::{
    clustering::{Cluster, ClusteringAlgorithm},
    neighbors::{kdtree::KDTreeSearch, Neighbor, NeighborSearch},
    DistanceMetric,
    FloatNumber,
    Point,
};

/// DBSCAN++ clustering algorithm error type.
#[derive(Debug, PartialEq, Error)]
pub enum DBSCANPlusPlusError<T>
where
    T: FloatNumber,
{
    /// Error when the probability is invalid.
    #[error("Invalid probability: The probability must be in the range (0, 1]: {0}")]
    InvalidProbability(T),

    /// Error when the minimum number of points is invalid.
    #[error("Invalid minimum points: The minimum number of points must be greater than zero: {0}")]
    InvalidMinPoints(usize),

    /// Error when the epsilon is invalid.
    #[error("Invalid epsilon: The epsilon must be greater than zero: {0}")]
    InvalidEpsilon(T),

    /// Error when the points are empty.
    #[error("Empty points: The points must be non-empty.")]
    EmptyPoints,
}

/// Labels for the points in the clustering process.
const INITIAL_LABEL: i32 = 0;
const OUTLIER: i32 = -1;
const MARKED: i32 = -2;
const UNCLASSIFIED: i32 = -3;

/// Default number of leaves for the KDTree.
const DEFAULT_KDTREE_LEAVES: usize = 16;

/// DBSCAN++ clustering algorithm.
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug, PartialEq)]
pub struct DBSCANPlusPlus<T>
where
    T: FloatNumber,
{
    probability: T,
    min_points: usize,
    epsilon: T,
    metric: DistanceMetric,
}

impl<T> DBSCANPlusPlus<T>
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
    /// A new `DBSCANPlusPlus` instance.
    pub fn new(
        probability: T,
        min_points: usize,
        epsilon: T,
        metric: DistanceMetric,
    ) -> Result<Self, DBSCANPlusPlusError<T>> {
        if (probability <= T::zero()) || (probability > T::one()) {
            return Err(DBSCANPlusPlusError::InvalidProbability(probability));
        }
        if min_points == 0 {
            return Err(DBSCANPlusPlusError::InvalidMinPoints(min_points));
        }
        if epsilon <= T::zero() {
            return Err(DBSCANPlusPlusError::InvalidEpsilon(epsilon));
        }
        Ok(Self {
            probability,
            min_points,
            epsilon,
            metric,
        })
    }

    #[must_use]
    fn select_core_points<const N: usize, NS>(
        &self,
        points: &[Point<T, N>],
        points_search: &NS,
    ) -> Vec<Point<T, N>>
    where
        NS: NeighborSearch<T, N>,
    {
        let step = (T::one() / self.probability)
            .round()
            .trunc_to_usize()
            .max(1);
        points
            .iter()
            .step_by(step)
            .filter(|point| {
                let neighbors = points_search.search_radius(point, self.epsilon);
                neighbors.len() >= self.min_points
            })
            .copied()
            .collect()
    }

    #[must_use]
    fn label_core_points<const N: usize>(
        &self,
        core_points: &[Point<T, N>],
        core_points_search: &KDTreeSearch<T, N>,
    ) -> Vec<i32> {
        let mut labels = vec![UNCLASSIFIED; core_points.len()];
        let mut current_label = INITIAL_LABEL;
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
                current_label,
                &mut labels,
                core_points,
                neighbors,
                core_points_search,
            );
            current_label += 1;
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
            if labels[index] >= INITIAL_LABEL {
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
    fn build_clusters<const N: usize, NS>(
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
            let nearest = match core_points_search.search_nearest(point) {
                Some(nearest) => nearest,
                None => continue,
            };

            if nearest.distance > self.epsilon {
                continue;
            }

            let core_label = core_labels[nearest.index];
            if core_label >= INITIAL_LABEL {
                clusters
                    .entry(core_label)
                    .or_insert_with(Cluster::new)
                    .add_member(index, point);
            }
        }
        clusters.into_values().collect()
    }
}

impl<T, const N: usize> ClusteringAlgorithm<T, N> for DBSCANPlusPlus<T>
where
    T: FloatNumber,
{
    type Err = DBSCANPlusPlusError<T>;

    fn fit(&self, points: &[Point<T, N>]) -> Result<Vec<Cluster<T, N>>, DBSCANPlusPlusError<T>> {
        if points.is_empty() {
            return Err(DBSCANPlusPlusError::EmptyPoints);
        }

        let points_search = KDTreeSearch::build(points, self.metric, DEFAULT_KDTREE_LEAVES);
        let core_points = self.select_core_points(points, &points_search);
        if core_points.is_empty() {
            return Ok(Vec::new());
        }

        let core_points_search =
            KDTreeSearch::build(&core_points, self.metric, DEFAULT_KDTREE_LEAVES);
        let core_labels = self.label_core_points(&core_points, &core_points_search);
        let clusters = self.build_clusters(points, &core_labels, &core_points_search);
        Ok(clusters)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[must_use]
    fn sample_points() -> Vec<Point<f32, 3>> {
        vec![
            [0.0, 0.0, 0.0], // 0
            [1.0, 0.0, 0.0], // 0
            [0.0, 1.0, 0.0], // 0
            [0.0, 0.0, 1.0], // 0
            [1.0, 1.0, 0.0], // 0
            [0.0, 1.0, 1.0], // 0
            [1.0, 0.0, 1.0], // 0
            [1.0, 1.0, 1.0], // 0
            [3.0, 4.0, 5.0], // 1
            [3.0, 3.0, 3.0], // 1
            [5.0, 4.0, 3.0], // 1
            [4.0, 3.0, 4.0], // 1
            [5.0, 4.0, 5.0], // 1
            [4.0, 5.0, 3.0], // 1
            [2.0, 9.0, 9.0], // 2
            [0.0, 9.0, 9.0], // 2
            [1.0, 9.0, 8.0], // 2
            [0.0, 8.0, 8.0], // 2
            [0.0, 9.0, 8.0], // 2
            [9.0, 9.0, 9.0], // Outlier
            [9.0, 0.0, 0.0], // Outlier
        ]
    }

    #[must_use]
    fn empty_points() -> Vec<Point<f32, 3>> {
        Vec::new()
    }

    #[test]
    fn test_new() {
        // Act
        let actual = DBSCANPlusPlus::new(0.5, 5, 0.1, DistanceMetric::Euclidean).unwrap();

        // Assert
        assert_eq!(actual.probability, 0.5);
        assert_eq!(actual.min_points, 5);
        assert_eq!(actual.epsilon, 0.1);
        assert_eq!(actual.metric, DistanceMetric::Euclidean);
    }

    #[rstest]
    #[case::invalid_probalitily(
        0.0,
        5,
        0.1,
        DistanceMetric::Euclidean,
        DBSCANPlusPlusError::InvalidProbability(0.0)
    )]
    #[case::invalid_min_points(
        0.5,
        0,
        0.1,
        DistanceMetric::Euclidean,
        DBSCANPlusPlusError::InvalidMinPoints(0)
    )]
    #[case::invalid_epsilon(
        0.5,
        5,
        0.0,
        DistanceMetric::Euclidean,
        DBSCANPlusPlusError::InvalidEpsilon(0.0)
    )]
    fn test_new_error(
        #[case] probability: f64,
        #[case] min_points: usize,
        #[case] epsilon: f64,
        #[case] metric: DistanceMetric,
        #[case] expected: DBSCANPlusPlusError<f64>,
    ) {
        // Act
        let actual = DBSCANPlusPlus::new(probability, min_points, epsilon, metric);

        // Assert
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), expected);
    }

    #[test]
    fn test_fit() {
        // Arrange
        let dbscanpp = DBSCANPlusPlus::new(0.5, 3, 2.0, DistanceMetric::Euclidean).unwrap();
        let points = sample_points();

        let mut actual = dbscanpp.fit(&points).unwrap();
        actual.sort_by(|cluster1, cluster2| cluster2.len().cmp(&cluster1.len()));

        // Assert
        assert_eq!(actual.len(), 3);
    }

    #[test]
    fn test_fit_core_points_empty() {
        // Arrange
        let dbscanpp = DBSCANPlusPlus::new(0.5, 8, 2.0, DistanceMetric::Euclidean).unwrap();
        let points = sample_points();

        // Act
        let actual = dbscanpp.fit(&points).unwrap();

        // Assert
        assert_eq!(actual.len(), 0);
    }

    #[test]
    fn test_fit_empty() {
        // Arrange
        let dbscanpp = DBSCANPlusPlus::new(0.5, 4, 2.0, DistanceMetric::Euclidean).unwrap();
        let points = empty_points();
        let actual = dbscanpp.fit(&points);

        // Assert
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), DBSCANPlusPlusError::EmptyPoints);
    }
}

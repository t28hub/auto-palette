use std::collections::{HashMap, VecDeque};

use crate::math::{
    clustering::{Cluster, ClusteringAlgorithm},
    neighbors::{kdtree::KDTreeSearch, neighbor::Neighbor, search::NeighborSearch},
    DistanceMetric,
    FloatNumber,
    Point,
};

const OUTLIER: i32 = -1;
const MARKED: i32 = -2;
const UNCLASSIFIED: i32 = -3;

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
    ) -> Result<Self, &'static str> {
        if (probability <= T::zero()) || (probability > T::one()) {
            return Err("The probability must be in the range (0, 1].");
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
            let nearest = core_points_search
                .search_nearest(point)
                .expect("No nearest core point found.");

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

impl<T, const N: usize> ClusteringAlgorithm<T, N> for DBSCANPlusPlus<T>
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
        "The probability must be in the range (0, 1]."
    )]
    #[case::invalid_min_points(
        0.5,
        0,
        0.1,
        DistanceMetric::Euclidean,
        "The minimum number of points must be greater than zero."
    )]
    #[case::invalid_epsilon(
        0.5,
        5,
        0.0,
        DistanceMetric::Euclidean,
        "The epsilon must be greater than zero."
    )]
    fn test_new_error(
        #[case] probability: f64,
        #[case] min_points: usize,
        #[case] epsilon: f64,
        #[case] metric: DistanceMetric,
        #[case] expected: &'static str,
    ) {
        // Act
        let actual = DBSCANPlusPlus::new(probability, min_points, epsilon, metric);

        // Assert
        assert!(actual.is_err());
        assert_eq!(actual, Err(expected));
    }

    #[test]
    fn test_fit() {
        // Arrange
        let dbscanpp = DBSCANPlusPlus::new(0.5, 3, 2.0, DistanceMetric::Euclidean).unwrap();
        let points = sample_points();

        let mut actual = dbscanpp.fit(&points);
        actual.sort_by(|cluster1, cluster2| cluster2.len().cmp(&cluster1.len()));

        // Assert
        assert_eq!(actual.len(), 3);
    }

    #[test]
    fn test_fit_empty() {
        // Arrange
        let dbscanpp = DBSCANPlusPlus::new(0.5, 4, 2.0, DistanceMetric::Euclidean).unwrap();
        let points = empty_points();
        let actual = dbscanpp.fit(&points);

        // Assert
        assert!(actual.is_empty());
    }
}

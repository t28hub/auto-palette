use std::{cmp::Ordering, collections::HashSet, fmt::Display};

use rand::{distr::Distribution, Rng};
use rand_distr::weighted::{AliasableWeight, Error as WeightedAliasIndexError, WeightedAliasIndex};
use thiserror::Error;

use crate::math::{
    clustering::{Cluster, ClusteringAlgorithm},
    metrics::DistanceMetric,
    neighbors::{linear::LinearSearch, search::NeighborSearch},
    point::Point,
    FloatNumber,
};

/// K-means clustering algorithm error type.
#[derive(Debug, PartialEq, Error)]
pub enum KmeansError<T>
where
    T: FloatNumber + Display,
{
    /// Error when the number of clusters is invalid.
    #[error("Invalid Cluster Count: The number of clusters must be > 0: {0}")]
    InvalidClusterCount(usize),

    /// Error when the number of iterations is invalid.
    #[error("Invalid Iterations: The number of iterations must be > 0: {0}")]
    InvalidIterations(usize),

    /// Error when the tolerance is invalid.
    #[error("Invalid Tolerance: The tolerance must be > 0: {0}")]
    InvalidTolerance(T),

    /// Error when the distance metric is invalid.
    #[error("Weighted Alias Index Error: {0}")]
    WeightedAliasIndexError(#[from] WeightedAliasIndexError),

    /// Error when the points is empty.
    #[error("Empty Points: The points must be non-empty.")]
    EmptyPoints,
}

/// K-means clustering algorithm.
///
/// # Type Parameters
/// * `T` - The floating point type.
/// * `R` - The random number generator.
#[derive(Debug)]
pub struct KMeans<T, R>
where
    T: FloatNumber,
    R: Rng + Clone,
{
    k: usize,
    max_iter: usize,
    tolerance: T,
    metric: DistanceMetric,
    rng: R,
}

impl<T, R> KMeans<T, R>
where
    T: FloatNumber,
    R: Rng + Clone,
{
    /// Creates a new `Kmeans` instance.
    ///
    /// # Arguments
    /// * `k` - The number of clusters.
    /// * `max_iter` - The maximum number of iterations.
    /// * `tolerance` - The tolerance for convergence conditions.
    /// * `metric` - The distance metric to use.
    /// * `strategy` - The initialization strategy to use.
    ///
    /// # Returns
    /// A new `Kmeans` instance.
    pub fn new(
        k: usize,
        max_iter: usize,
        tolerance: T,
        metric: DistanceMetric,
        rng: R,
    ) -> Result<Self, KmeansError<T>> {
        if k == 0 {
            return Err(KmeansError::InvalidClusterCount(k));
        }
        if max_iter == 0 {
            return Err(KmeansError::InvalidIterations(max_iter));
        }
        if tolerance <= T::zero() {
            return Err(KmeansError::InvalidTolerance(tolerance));
        }
        Ok(Self {
            k,
            max_iter,
            tolerance,
            metric,
            rng,
        })
    }

    fn initialize<const N: usize>(
        &self,
        points: &[Point<T, N>],
        k: usize,
    ) -> Result<Vec<Point<T, N>>, KmeansError<T>>
    where
        T: FloatNumber + AliasableWeight,
        R: Rng,
    {
        let mut selected = HashSet::with_capacity(k);
        let mut centroids = Vec::with_capacity(k);
        let mut rng = self.rng.clone();

        // 1. Randomly select the first centroid from the data points.
        let index = rng.random_range(0..points.len());
        selected.insert(index);
        centroids.push(points[index]);

        // 2. For each remaining centroid, select the point with the maximum distance from the existing centroids.
        while centroids.len() < k {
            let distances: Vec<T> = points
                .iter()
                .enumerate()
                .map(|(index, point)| {
                    if selected.contains(&index) {
                        T::zero()
                    } else {
                        centroids
                            .iter()
                            .map(|centroid| self.metric.measure(centroid, point))
                            .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
                            .unwrap_or(T::epsilon()) // Use an epsilon value to avoid zero distance
                    }
                })
                .collect();

            let weighted_index =
                WeightedAliasIndex::new(distances).map_err(KmeansError::WeightedAliasIndexError)?;
            let index = weighted_index.sample(&mut rng);
            selected.insert(index);
            centroids.push(points[index]);
        }
        Ok(centroids)
    }

    #[must_use]
    fn iterate<const N: usize>(
        &self,
        points: &[Point<T, N>],
        centroids: &mut [Point<T, N>],
        clusters: &mut [Cluster<T, N>],
    ) -> bool {
        clusters.iter_mut().for_each(Cluster::clear);

        let centroid_search = LinearSearch::build(centroids, self.metric);
        for (index, point) in points.iter().enumerate() {
            if let Some(nearest) = centroid_search.search_nearest(point) {
                clusters[nearest.index].add_member(index, point);
            }
        }

        let new_centroids: Vec<Point<T, N>> =
            clusters.iter().map(|cluster| *cluster.centroid()).collect();

        let converged = centroids
            .iter()
            .zip(&new_centroids)
            .all(|(old, new)| self.metric.measure(old, new) <= self.tolerance);

        centroids.copy_from_slice(&new_centroids);
        converged
    }
}

impl<T, const N: usize, R> ClusteringAlgorithm<T, N> for KMeans<T, R>
where
    T: FloatNumber + AliasableWeight,
    R: Rng + Clone,
{
    type Err = KmeansError<T>;

    fn fit(&self, points: &[Point<T, N>]) -> Result<Vec<Cluster<T, N>>, Self::Err> {
        if points.is_empty() {
            return Err(KmeansError::EmptyPoints);
        }

        if self.k >= points.len() {
            let clusters = points
                .iter()
                .enumerate()
                .map(|(index, point)| {
                    let mut cluster = Cluster::new();
                    cluster.add_member(index, point);
                    cluster
                })
                .collect();
            return Ok(clusters);
        }

        let mut centroids = self.initialize(points, self.k)?;
        let mut clusters = vec![Cluster::new(); self.k];
        for _ in 0..self.max_iter {
            let converged = self.iterate(points, &mut centroids, &mut clusters);
            if converged {
                break;
            }
        }
        Ok(clusters)
    }
}

#[cfg(test)]
mod tests {
    use rand::{rng, rngs::ThreadRng};
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_new() {
        // Act
        let metric = DistanceMetric::Euclidean;
        let actual: KMeans<f32, ThreadRng> = KMeans::new(3, 10, 1e-3, metric, rng()).unwrap();

        // Assert
        assert_eq!(actual.k, 3);
        assert_eq!(actual.max_iter, 10);
        assert_eq!(actual.tolerance, 1e-3);
        assert_eq!(actual.metric, DistanceMetric::Euclidean);
    }

    #[rstest]
    #[case::invalid_clusters(
        0,
        10,
        1e-3,
        DistanceMetric::Euclidean,
        KmeansError::InvalidClusterCount(0)
    )]
    #[case::invalid_iterations(
        3,
        0,
        1e-3,
        DistanceMetric::Euclidean,
        KmeansError::InvalidIterations(0)
    )]
    #[case::invalid_tolerance(
        3,
        10,
        0.0,
        DistanceMetric::Euclidean,
        KmeansError::InvalidTolerance(0.0)
    )]
    fn test_new_error(
        #[case] k: usize,
        #[case] max_iter: usize,
        #[case] tolerance: f32,
        #[case] metric: DistanceMetric,
        #[case] expected: KmeansError<f32>,
    ) {
        // Act
        let actual = KMeans::new(k, max_iter, tolerance, metric, rng());

        // Assert
        assert!(actual.is_err());
        assert_eq!(actual.err().unwrap(), expected);
    }

    #[test]
    fn test_fit() {
        // Arrange
        let metric = DistanceMetric::Euclidean;
        let kmeans: KMeans<f32, ThreadRng> = KMeans::new(3, 10, 1e-3, metric, rng()).unwrap();

        // Act
        let points = [
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            [2.0, 2.0, 2.0],
            [2.0, 1.0, 2.0],
            [4.0, 4.0, 4.0],
            [4.0, 4.0, 5.0],
            [3.0, 4.0, 5.0],
        ];
        let actual = kmeans.fit(&points);

        // Assert
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap().len(), 3);
    }

    #[test]
    fn test_fit_single_cluster() {
        // Arrange
        let metric = DistanceMetric::Euclidean;
        let kmeans = KMeans::new(3, 10, 1e-3, metric, rng()).unwrap();

        // Act
        let points = [[0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]];
        let actual = kmeans.fit(&points);

        // Assert
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap().len(), 3);
    }

    #[test]
    fn test_fit_empty() {
        // Arrange
        let metric = DistanceMetric::Euclidean;
        let kmeans: KMeans<f32, ThreadRng> = KMeans::new(3, 10, 1e-3, metric, rng()).unwrap();

        // Act
        let points: Vec<Point<f32, 2>> = Vec::new();
        let actual = kmeans.fit(&points);

        // Assert
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), KmeansError::EmptyPoints);
    }
}

use std::collections::HashSet;

use rand::{distr::Distribution, Rng};
use rand_distr::weighted::{AliasableWeight, WeightedAliasIndex};

use crate::math::{
    clustering::{Cluster, ClusteringAlgorithm},
    metrics::DistanceMetric,
    neighbors::{linear::LinearSearch, search::NeighborSearch},
    point::Point,
    FloatNumber,
};

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
    ///
    /// # Errors
    /// Returns an error if the number of clusters is zero, the maximum number of iterations is zero,
    /// or the tolerance is less than or equal to zero.
    pub fn new(
        k: usize,
        max_iter: usize,
        tolerance: T,
        metric: DistanceMetric,
        rng: R,
    ) -> Result<Self, &'static str> {
        if k == 0 {
            return Err("The number of clusters must be greater than zero.");
        }
        if max_iter == 0 {
            return Err("The maximum number of iterations must be greater than zero.");
        }
        if tolerance <= T::zero() {
            return Err("The tolerance must be greater than zero.");
        }
        Ok(Self {
            k,
            max_iter,
            tolerance,
            metric,
            rng,
        })
    }

    fn initialize<const N: usize>(&self, points: &[Point<T, N>], k: usize) -> Vec<Point<T, N>>
    where
        T: FloatNumber + AliasableWeight,
        R: Rng,
    {
        let mut selected = HashSet::with_capacity(k);
        let mut centroids = Vec::with_capacity(k);

        let mut rng = self.rng.clone();
        let index = rng.random_range(0..points.len());
        selected.insert(index);
        centroids.push(points[index]);

        while centroids.len() < k {
            let mut distances = Vec::with_capacity(points.len());
            for (i, point) in points.iter().enumerate() {
                let distance = if selected.contains(&i) {
                    T::zero()
                } else {
                    centroids
                        .iter()
                        .map(|centroid| self.metric.measure(centroid, point))
                        .min_by(|a, b| a.partial_cmp(b).unwrap())
                        .unwrap()
                };
                distances.push(distance);
            }

            let weighted_index = WeightedAliasIndex::new(distances).unwrap();
            let index = weighted_index.sample(&mut rng);
            selected.insert(index);
            centroids.push(points[index]);
        }
        centroids
    }

    #[must_use]
    fn iterate<const N: usize>(
        &self,
        points: &[Point<T, N>],
        centroids: &mut [Point<T, N>],
        clusters: &mut [Cluster<T, N>],
    ) -> bool {
        for cluster in clusters.iter_mut() {
            cluster.clear();
        }

        let centroid_search = LinearSearch::build(centroids, self.metric.clone());
        for (index, point) in points.iter().enumerate() {
            let nearest = centroid_search
                .search_nearest(point)
                .expect("No nearest centroid found.");
            clusters[nearest.index].add_member(index, point);
        }

        let mut converged = true;
        let new_centroids: Vec<Point<T, N>> =
            clusters.iter().map(|cluster| *cluster.centroid()).collect();
        for (old, new) in centroids.iter().zip(&new_centroids) {
            let distance = self.metric.measure(old, new);
            if distance > self.tolerance {
                converged = false;
                break;
            }
        }
        centroids.copy_from_slice(&new_centroids);
        converged
    }
}

impl<T, const N: usize, R> ClusteringAlgorithm<T, N> for KMeans<T, R>
where
    T: FloatNumber + AliasableWeight,
    R: Rng + Clone,
{
    fn fit(&self, points: &[Point<T, N>]) -> Vec<Cluster<T, N>> {
        if points.is_empty() {
            return Vec::new();
        }

        if self.k >= points.len() {
            return points
                .iter()
                .enumerate()
                .map(|(index, point)| {
                    let mut cluster = Cluster::new();
                    cluster.add_member(index, point);
                    cluster
                })
                .collect();
        }

        let mut centroids = self.initialize(points, self.k);
        let mut clusters = vec![Cluster::new(); self.k];
        for _ in 0..self.max_iter {
            let converged = self.iterate(points, &mut centroids, &mut clusters);
            if converged {
                break;
            }
        }
        clusters
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
        "The number of clusters must be greater than zero."
    )]
    #[case::invalid_iterations(
        3,
        0,
        1e-3,
        DistanceMetric::Euclidean,
        "The maximum number of iterations must be greater than zero."
    )]
    #[case::invalid_tolerance(
        3,
        10,
        0.0,
        DistanceMetric::Euclidean,
        "The tolerance must be greater than zero."
    )]
    fn test_new_error(
        #[case] k: usize,
        #[case] max_iter: usize,
        #[case] tolerance: f32,
        #[case] metric: DistanceMetric,
        #[case] expected: &'static str,
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
        assert_eq!(actual.len(), 3);
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
        assert_eq!(actual.len(), 0);
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
        assert_eq!(actual.len(), 3);
    }
}

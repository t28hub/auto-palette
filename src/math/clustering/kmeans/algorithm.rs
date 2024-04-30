use rand::Rng;

use crate::math::{
    clustering::{kmeans::strategy::InitializationStrategy, Cluster, ClusteringAlgorithm},
    metrics::DistanceMetric,
    neighbors::{linear::LinearSearch, search::NeighborSearch},
    point::Point,
    FloatNumber,
};

/// A k-means clustering algorithm.
///
/// # Type Parameters
/// * `T` - The floating point type.
/// * `N` - The number of dimensions.
/// * `R` - The random number generator.
#[derive(Debug)]
pub struct KMeans<T, const N: usize, R>
where
    T: FloatNumber,
    R: Rng + Clone,
{
    k: usize,
    max_iter: usize,
    tolerance: T,
    metric: DistanceMetric,
    strategy: InitializationStrategy<R>,
}

impl<T, const N: usize, R> KMeans<T, N, R>
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
        strategy: InitializationStrategy<R>,
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
            strategy,
        })
    }

    #[must_use]
    fn iterate(
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
            let Some(nearest) = centroid_search.search_nearest(point) else {
                continue;
            };
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

impl<T, const N: usize, R> ClusteringAlgorithm<T, N> for KMeans<T, N, R>
where
    T: FloatNumber,
    R: Rng + Clone,
{
    #[must_use]
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

        let mut centroids = self.strategy.initialize(points, self.k).unwrap();
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
    use rand::rngs::ThreadRng;

    use super::*;

    #[test]
    fn test_new_kmeans() {
        // Act
        let metric = DistanceMetric::Euclidean;
        let strategy = InitializationStrategy::Random(rand::thread_rng());
        let kmeans: KMeans<f32, 3, ThreadRng> = KMeans::new(3, 10, 1e-3, metric, strategy).unwrap();

        // Assert
        assert_eq!(kmeans.k, 3);
        assert_eq!(kmeans.max_iter, 10);
        assert_eq!(kmeans.tolerance, 1e-3);
        assert_eq!(kmeans.metric, DistanceMetric::Euclidean);
    }

    #[test]
    fn test_fit() {
        // Arrange
        let metric = DistanceMetric::Euclidean;
        let strategy = InitializationStrategy::Random(rand::thread_rng());
        let kmeans: KMeans<f32, 3, ThreadRng> = KMeans::new(3, 10, 1e-3, metric, strategy).unwrap();

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
        let clusters = kmeans.fit(&points);

        // Assert
        assert_eq!(clusters.len(), 3);
    }

    #[test]
    fn test_fit_empty() {
        // Arrange
        let metric = DistanceMetric::Euclidean;
        let strategy = InitializationStrategy::Random(rand::thread_rng());
        let kmeans: KMeans<f32, 3, ThreadRng> = KMeans::new(3, 10, 1e-3, metric, strategy).unwrap();

        // Act
        let clusters = kmeans.fit(&[]);

        // Assert
        assert_eq!(clusters.len(), 0);
    }

    #[test]
    fn test_fit_single_cluster() {
        // Arrange
        let metric = DistanceMetric::Euclidean;
        let strategy = InitializationStrategy::Random(rand::thread_rng());
        let kmeans = KMeans::new(3, 10, 1e-3, metric, strategy).unwrap();

        // Act
        let points = [[0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]];
        let clusters = kmeans.fit(&points);

        // Assert
        assert_eq!(clusters.len(), 3);
    }
}

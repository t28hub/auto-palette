use crate::math::clustering::kmeans::InitializationStrategy;
use crate::math::clustering::Cluster;
use crate::math::clustering::ClusteringAlgorithm;
use crate::math::metrics::DistanceMetric;
use crate::math::neighbors::linear::LinearSearch;
use crate::math::neighbors::search::NeighborSearch;
use crate::math::point::Point;
use rand::Rng;

/// A k-means clustering algorithm.
///
/// # Type Parameters
/// * `R` - The random number generator.
#[derive(Debug)]
pub struct KMeans<R: Rng + Clone> {
    k: usize,
    max_iter: usize,
    tolerance: f32,
    metric: DistanceMetric,
    strategy: InitializationStrategy<R>,
}

impl<R: Rng + Clone> KMeans<R> {
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
        tolerance: f32,
        metric: DistanceMetric,
        strategy: InitializationStrategy<R>,
    ) -> Result<Self, &'static str> {
        if k == 0 {
            return Err("The number of clusters must be greater than zero.");
        }
        if max_iter == 0 {
            return Err("The maximum number of iterations must be greater than zero.");
        }
        if tolerance <= 0.0 {
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
    fn iterate<const N: usize>(
        &self,
        points: &[Point<N>],
        centroids: &mut [Point<N>],
        clusters: &mut [Cluster<N>],
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
        let new_centroids: Vec<Point<N>> =
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

impl<R> ClusteringAlgorithm for KMeans<R>
where
    R: Rng + Clone,
{
    #[must_use]
    fn fit<const N: usize>(&self, points: &[Point<N>]) -> Vec<Cluster<N>> {
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
    use super::*;

    #[test]
    fn test_new_kmeans() {
        // Act
        let metric = DistanceMetric::Euclidean;
        let strategy = InitializationStrategy::Random(rand::thread_rng());
        let kmeans = KMeans::new(3, 10, 1e-3, metric, strategy).unwrap();

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
        let kmeans = KMeans::new(3, 10, 1e-3, metric, strategy).unwrap();

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
        let kmeans = KMeans::new(3, 10, 1e-3, metric, strategy).unwrap();

        // Act
        let clusters = kmeans.fit::<3>(&[]);

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

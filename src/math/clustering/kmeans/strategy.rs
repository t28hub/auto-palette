use crate::math::metrics::DistanceMetric;
use crate::math::point::Point;
use rand::distributions::Distribution;
use rand::prelude::ThreadRng;
use rand::Rng;
use rand_distr::WeightedAliasIndex;
use std::collections::HashSet;

/// The initialization strategy for the KMeans algorithm.
#[derive(Debug, Clone, PartialEq)]
pub enum InitializationStrategy<R: Rng + Clone> {
    /// The random initialization strategy.
    Random(R),
    /// The KMeans++ initialization strategy.
    KmeansPlusPlus(R, DistanceMetric),
}

impl Default for InitializationStrategy<ThreadRng> {
    #[must_use]
    fn default() -> Self {
        InitializationStrategy::Random(rand::thread_rng())
    }
}

impl<R: Rng + Clone> InitializationStrategy<R> {
    /// Initializes the centroids for the KMeans algorithm.
    ///
    /// # Type Parameters
    /// * `N` - The number of dimensions.
    ///
    /// # Arguments
    /// * `points` - The points to cluster.
    /// * `k` - The number of centroids.
    ///
    /// # Returns
    /// The initialized centroids.
    pub fn initialize<const N: usize>(
        &self,
        points: &[Point<N>],
        k: usize,
    ) -> Result<Vec<Point<N>>, &'static str> {
        if k == 0 {
            return Err("The number of centroids must be greater than zero.");
        }
        if points.len() < k {
            return Err(
                "The number of centroids must be less than or equal to the number of points.",
            );
        }

        let centroids = match self {
            InitializationStrategy::Random(rng) => random(rng.clone(), points, k),
            InitializationStrategy::KmeansPlusPlus(rng, metric) => {
                kmeans_plus_plus(rng.clone(), metric, points, k)
            }
        };
        Ok(centroids)
    }
}

/// The initialization strategy trait for the KMeans algorithm.
#[must_use]
fn random<const N: usize, R: Rng>(mut rng: R, points: &[Point<N>], k: usize) -> Vec<Point<N>> {
    let mut selected = HashSet::with_capacity(k);
    let mut centroids = Vec::with_capacity(k);
    while centroids.len() < k {
        let index = rng.gen_range(0..points.len());
        if !selected.insert(index) {
            continue;
        }
        centroids.push(points[index]);
    }
    centroids
}

/// The KMeans++ initialization strategy.
#[must_use]
fn kmeans_plus_plus<const N: usize, R: Rng>(
    mut rng: R,
    metric: &DistanceMetric,
    points: &[Point<N>],
    k: usize,
) -> Vec<Point<N>> {
    let mut selected = HashSet::with_capacity(k);
    let mut centroids = Vec::with_capacity(k);

    let index = rng.gen_range(0..points.len());
    selected.insert(index);
    centroids.push(points[index]);

    while centroids.len() < k {
        let mut distances = Vec::with_capacity(points.len());
        for (i, point) in points.iter().enumerate() {
            let distance = if selected.contains(&i) {
                0.0
            } else {
                centroids
                    .iter()
                    .map(|centroid| metric.measure(centroid, point))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[must_use]
    fn points() -> Vec<Point<2>> {
        vec![
            [0.0, 0.0],
            [1.0, 1.0],
            [2.0, 2.0],
            [3.0, 3.0],
            [4.0, 4.0],
            [5.0, 5.0],
        ]
    }

    #[test]
    fn test_initialize_random() {
        // Arrange
        let rng = rand::thread_rng();
        let strategy = InitializationStrategy::Random(rng);

        // Act
        let points = points();
        let centroids = strategy.initialize(&points, 2).unwrap();

        // Assert
        assert_eq!(centroids.len(), 2);
        assert_eq!(centroids.iter().all(|c| points.contains(c)), true);
    }

    #[test]
    fn test_initialize_kmeans_plus_plus() {
        // Arrange
        let rng = rand::thread_rng();
        let strategy =
            InitializationStrategy::KmeansPlusPlus(rng, DistanceMetric::SquaredEuclidean);

        // Act
        let points = points();
        let centroids = strategy.initialize(&points, 3).unwrap();

        // Assert
        assert_eq!(centroids.len(), 3);
        assert_eq!(centroids.iter().all(|c| points.contains(c)), true);
    }

    #[test]
    fn test_initialize_zero_centroids() {
        // Arrange
        let rng = rand::thread_rng();
        let strategy = InitializationStrategy::Random(rng);

        // Act
        let points = points();
        let result = strategy.initialize(&points, 0);

        // Assert
        assert_eq!(
            result,
            Err("The number of centroids must be greater than zero.")
        );
    }

    #[test]
    fn test_initialize_too_many_centroids() {
        // Arrange
        let rng = rand::thread_rng();
        let strategy = InitializationStrategy::Random(rng);

        // Act
        let points = points();
        let result = strategy.initialize(&points, 7);

        // Assert
        assert_eq!(
            result,
            Err("The number of centroids must be less than or equal to the number of points.")
        );
    }
}

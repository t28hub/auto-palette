use std::collections::HashSet;

use rand::{prelude::ThreadRng, Rng};
use rand_distr::{Distribution, WeightedAliasIndex};

use crate::math::{metrics::DistanceMetric, point::Point, FloatNumber};

/// The initialization strategy for the KMeans algorithm.
///
/// # Type Parameters
/// * `R` - The random number generator.
#[derive(Debug, Clone, PartialEq)]
pub enum InitializationStrategy<R>
where
    R: Rng + Clone,
{
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

impl<R> InitializationStrategy<R>
where
    R: Rng + Clone,
{
    /// Initializes the centroids for the KMeans algorithm.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type.
    /// * `N` - The number of dimensions.
    ///
    /// # Arguments
    /// * `points` - The points to cluster.
    /// * `k` - The number of centroids.
    ///
    /// # Returns
    /// The initialized centroids.
    pub fn initialize<T, const N: usize>(
        &self,
        points: &[Point<T, N>],
        k: usize,
    ) -> Result<Vec<Point<T, N>>, &'static str>
    where
        T: FloatNumber,
    {
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
fn random<T, const N: usize, R>(mut rng: R, points: &[Point<T, N>], k: usize) -> Vec<Point<T, N>>
where
    T: FloatNumber,
    R: Rng,
{
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
fn kmeans_plus_plus<T, const N: usize, R>(
    mut rng: R,
    metric: &DistanceMetric,
    points: &[Point<T, N>],
    k: usize,
) -> Vec<Point<T, N>>
where
    T: FloatNumber,
    R: Rng,
{
    let mut selected = HashSet::with_capacity(k);
    let mut centroids = Vec::with_capacity(k);

    let index = rng.gen_range(0..points.len());
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
    fn points() -> Vec<Point<f32, 2>> {
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

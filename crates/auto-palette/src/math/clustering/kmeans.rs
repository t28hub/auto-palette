use std::fmt::Display;

use thiserror::Error;

use crate::math::{
    clustering::{Cluster, ClusteringAlgorithm, Initializer},
    matrix::MatrixView,
    metrics::DistanceMetric,
    neighbors::{linear::LinearSearch, search::NeighborSearch},
    point::Point,
    FloatNumber,
};

/// K-means clustering algorithm error types.
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug, PartialEq, Error)]
pub enum KmeansError<T>
where
    T: FloatNumber + Display,
{
    /// Error when the shape of the points is invalid.
    #[error("Invalid Shape: The shape of the points must be > 0: {0}x{1}")]
    InvalidShape(usize, usize),

    /// Error when the number of clusters is invalid.
    #[error("Invalid Cluster Count: The number of clusters must be > 0: {0}")]
    InvalidClusterCount(usize),

    /// Error when the number of iterations is invalid.
    #[error("Invalid Iterations: The number of iterations must be > 0: {0}")]
    InvalidIterations(usize),

    /// Error when the tolerance is invalid.
    #[error("Invalid Tolerance: The tolerance must be > 0: {0}")]
    InvalidTolerance(T),

    /// Error when the points is empty.
    #[error("Empty Points: The points must be non-empty.")]
    EmptyPoints,

    /// Error when the points are not in the same shape.
    #[error("Invalid Points: The points must be in the same shape: {0}x{1}")]
    InvalidPoints(usize, usize),
}

/// K-means clustering algorithm.
///
/// This implementation leverages an evenly-spaced grid to pick the initial centroids.
/// For image pixel data this tends to stabilise the result and speeds-up convergence.
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug, PartialEq)]
pub struct KMeans<T>
where
    T: FloatNumber,
{
    shape: (usize, usize),
    k: usize,
    max_iter: usize,
    tolerance: T,
    metric: DistanceMetric,
}

impl<T> KMeans<T>
where
    T: FloatNumber,
{
    /// Creates a new `Kmeans` instance.
    ///
    /// # Arguments
    /// * `shape` - The shape of the points.
    /// * `k` - The number of clusters.
    /// * `max_iter` - The maximum number of iterations.
    /// * `tolerance` - The tolerance for convergence conditions.
    /// * `metric` - The distance metric to use.
    ///
    /// # Returns
    /// A new `Kmeans` instance.
    pub fn new(
        shape: (usize, usize),
        k: usize,
        max_iter: usize,
        tolerance: T,
        metric: DistanceMetric,
    ) -> Result<Self, KmeansError<T>> {
        if shape.0 == 0 || shape.1 == 0 {
            return Err(KmeansError::InvalidShape(shape.0, shape.1));
        }
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
            shape,
            k,
            max_iter,
            tolerance,
            metric,
        })
    }

    /// Initializes the centroids for clustering.
    ///
    /// # Type Parameters
    /// * `N` - The number of dimensions.
    ///
    /// # Arguments
    /// * `points` - The points to cluster.
    ///
    /// # Returns
    /// A vector of centroids for clustering.
    fn initialize<const N: usize>(
        &self,
        points: &[Point<T, N>],
    ) -> Result<Vec<Point<T, N>>, KmeansError<T>> {
        let (cols, rows) = self.shape;
        let matrix = MatrixView::new(cols, rows, points)
            .map_err(|_| KmeansError::InvalidPoints(cols, rows))?;

        let centroids = Initializer::Grid
            .initialize(&matrix, self.k)
            .into_iter()
            .map(|index| points[index])
            .collect();
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

        centroids
            .iter_mut()
            .zip(clusters)
            .fold(true, |converged, (old_centroid, cluster)| {
                let new_centroid = cluster.centroid();
                let distance = self.metric.measure(old_centroid, new_centroid);
                *old_centroid = *new_centroid;
                converged && distance <= self.tolerance
            })
    }
}

impl<T, const N: usize> ClusteringAlgorithm<T, N> for KMeans<T>
where
    T: FloatNumber,
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

        let mut centroids = self.initialize(points)?;
        let mut clusters = vec![Cluster::new(); centroids.len()];
        for _ in 0..self.max_iter {
            if self.iterate(points, &mut centroids, &mut clusters) {
                break;
            }
        }
        Ok(clusters)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_new() {
        // Act
        let metric = DistanceMetric::Euclidean;
        let actual: KMeans<f64> = KMeans::new((192, 128), 3, 10, 1e-3, metric).unwrap();

        // Assert
        assert_eq!(
            actual,
            KMeans {
                shape: (192, 128),
                k: 3,
                max_iter: 10,
                tolerance: 1e-3,
                metric: DistanceMetric::Euclidean,
            }
        );
    }

    #[rstest]
    #[case::invalid_shape_cols(
    (0, 128),
        3,
        10,
        1e-3,
        DistanceMetric::Euclidean,
        KmeansError::InvalidShape(0, 128)
    )]
    #[case::invalid_shape_rows(
    (192, 0),
        3,
        10,
        1e-3,
        DistanceMetric::Euclidean,
        KmeansError::InvalidShape(192, 0)
    )]
    #[case::invalid_clusters(
    (192, 128),
        0,
        10,
        1e-3,
        DistanceMetric::Euclidean,
        KmeansError::InvalidClusterCount(0)
    )]
    #[case::invalid_iterations(
    (192, 128),
        3,
        0,
        1e-3,
        DistanceMetric::Euclidean,
        KmeansError::InvalidIterations(0)
    )]
    #[case::invalid_tolerance(
    (192, 128),
        3,
        10,
        0.0,
        DistanceMetric::Euclidean,
        KmeansError::InvalidTolerance(0.0)
    )]
    fn test_new_error(
        #[case] shape: (usize, usize),
        #[case] k: usize,
        #[case] max_iter: usize,
        #[case] tolerance: f32,
        #[case] metric: DistanceMetric,
        #[case] expected: KmeansError<f32>,
    ) {
        // Act
        let actual = KMeans::new(shape, k, max_iter, tolerance, metric);

        // Assert
        assert!(actual.is_err());
        assert_eq!(actual.err().unwrap(), expected);
    }

    #[test]
    fn test_fit() {
        // Arrange
        let cols = 16;
        let rows = 9;
        let metric = DistanceMetric::Euclidean;
        let kmeans: KMeans<f64> = KMeans::new((cols, rows), 6, 10, 1e-3, metric).unwrap();

        let mut points = vec![[0.0; 3]; cols * rows];
        for i in 0..cols {
            for j in 0..rows {
                points[i * rows + j] = [i as f64, j as f64, 0.0];
            }
        }

        // Act
        let actual = kmeans.fit(&points);

        // Assert
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap().len(), 6);
    }

    #[test]
    fn test_fit_single_cluster() {
        // Arrange
        let metric = DistanceMetric::Euclidean;
        let kmeans = KMeans::new((3, 1), 3, 10, 1e-3, metric).unwrap();

        // Act
        let points = [[0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]];
        let actual = kmeans.fit(&points);

        // Assert
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap().len(), 3);
    }

    #[test]
    fn test_fit_empty_points() {
        // Arrange
        let metric = DistanceMetric::Euclidean;
        let kmeans: KMeans<f64> = KMeans::new((16, 9), 3, 10, 1e-3, metric).unwrap();

        // Act
        let points: Vec<Point<f64, 2>> = Vec::new();
        let actual = kmeans.fit(&points);

        // Assert
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), KmeansError::EmptyPoints);
    }

    #[test]
    fn test_fit_invalid_points() {
        // Arrange
        let metric = DistanceMetric::Euclidean;
        let kmeans: KMeans<f64> = KMeans::new((16, 9), 3, 10, 1e-3, metric).unwrap();

        // Act
        let points: Vec<Point<f64, 2>> = vec![[0.0; 2]; 16 * 8];
        let actual = kmeans.fit(&points);

        // Assert
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), KmeansError::InvalidPoints(16, 9));
    }
}

use crate::math::{
    clustering::{
        kmeans::{error::KmeansError, CentroidInit},
        Cluster,
        ClusteringAlgorithm,
    },
    metrics::DistanceMetric,
    neighbors::{linear::LinearSearch, NeighborSearch},
    point::Point,
    FloatNumber,
};

/// K-means clustering algorithm.
///
/// This implementation leverages an evenly-spaced grid to pick the initial centroids.
/// For image pixel data this tends to stabilise the result and speeds-up convergence.
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug, PartialEq)]
pub struct Kmeans<T>
where
    T: FloatNumber,
{
    k: usize,
    max_iter: usize,
    tolerance: T,
    metric: DistanceMetric,
    init: CentroidInit,
}

impl<T> Kmeans<T>
where
    T: FloatNumber,
{
    /// Creates a new `Kmeans` instance.
    ///
    /// # Arguments
    /// * `k` - The number of clusters.
    /// * `max_iter` - The maximum number of iterations.
    /// * `tolerance` - The tolerance for convergence conditions.
    /// * `initializer` - The centroid initializer to use.
    /// * `metric` - The distance metric to use.
    ///
    /// # Returns
    /// A new `Kmeans` instance.
    pub fn new(
        k: usize,
        max_iter: usize,
        tolerance: T,
        metric: DistanceMetric,
        init: CentroidInit,
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
            init,
        })
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

impl<T, const N: usize> ClusteringAlgorithm<T, N> for Kmeans<T>
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

        let mut centroids = self.init.initialize(points, self.k);
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
        let actual = Kmeans::new(3, 10, 1e-3, metric, CentroidInit::RegularInterval).unwrap();

        // Assert
        assert_eq!(
            actual,
            Kmeans {
                k: 3,
                max_iter: 10,
                tolerance: 1e-3,
                init: CentroidInit::RegularInterval,
                metric: DistanceMetric::Euclidean,
            }
        );
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
        let actual = Kmeans::new(
            k,
            max_iter,
            tolerance,
            metric,
            CentroidInit::RegularInterval,
        );

        // Assert
        assert!(actual.is_err());
        assert_eq!(actual.err().unwrap(), expected);
    }

    #[test]
    fn test_fit() {
        // Arrange
        let kmeans = Kmeans::new(
            6,
            10,
            1e-3,
            DistanceMetric::Euclidean,
            CentroidInit::RegularInterval,
        )
        .unwrap();

        let cols = 16;
        let rows = 9;
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
        let kmeans = Kmeans::new(
            3,
            10,
            1e-3,
            DistanceMetric::Euclidean,
            CentroidInit::RegularInterval,
        )
        .unwrap();

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
        let kmeans = Kmeans::new(
            3,
            10,
            1e-3,
            DistanceMetric::Euclidean,
            CentroidInit::RegularInterval,
        )
        .unwrap();

        // Act
        let points: Vec<Point<f64, 2>> = Vec::new();
        let actual = kmeans.fit(&points);

        // Assert
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), KmeansError::EmptyPoints);
    }
}

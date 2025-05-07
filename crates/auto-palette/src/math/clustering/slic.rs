use std::fmt::Display;

use thiserror::Error;

use crate::{
    math::{
        clustering::{helper::gradient, Cluster, ClusteringAlgorithm, Initializer},
        matrix::{MatrixError, MatrixView},
        neighbors::{kdtree::KDTreeSearch, search::NeighborSearch},
        DistanceMetric,
        Point,
    },
    FloatNumber,
};

/// SLIC algorithm error type.
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug, PartialEq, Error)]
pub enum SLICError<T>
where
    T: FloatNumber + Display,
{
    /// Error when the shape is invalid.
    #[error("Invalid Shape: The shape must be > 0: {0}x{1}")]
    InvalidShape(usize, usize),

    /// Error when the number of segments is invalid.
    #[error("Invalid Segments: The number of segments must be > 0: {0}")]
    InvalidSegments(usize),

    /// Error when the compactness is invalid.
    #[error("Invalid Compactness: The compactness must be > 0: {0}")]
    InvalidCompactness(T),

    /// Error when the maximum number of iterations is invalid.
    #[error("Invalid Iterations: The maximum number of iterations must be > 0: {0}")]
    InvalidIterations(usize),

    /// Error when the tolerance is invalid.
    #[error("Invalid Tolerance: The tolerance must be > 0: {0}")]
    InvalidTolerance(T),

    /// Error when the points are empty.
    #[error("Empty Points: The points must be non-empty.")]
    EmptyPoints,

    /// Error when the points are not equal to the dimensions.
    #[error("Invalid Points: The points slice is not in the expected shape: {0}")]
    InvalidPoints(#[from] MatrixError),
}

/// SLIC (Simple Linear Iterative Clustering) algorithm.
/// The algorithm is based on the following paper:
/// [SLIC Superpixels Compared to State-of-the-art Superpixel Methods](https://core.ac.uk/download/pdf/147983593.pdf)
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
pub struct SLIC<T>
where
    T: FloatNumber,
{
    shape: (usize, usize),
    segments: usize,
    compactness: T,
    max_iter: usize,
    tolerance: T,
    metric: DistanceMetric,
}

impl<T> SLIC<T>
where
    T: FloatNumber,
{
    /// The size of the leaf in the KDTree.
    const LEAF_SIZE: usize = 16;

    /// Creates a new `SLIC` instance.
    ///
    /// # Arguments
    /// * `shape` - The shape of the array to cluster.
    /// * `segments` - The number of segments to create.
    /// * `compactness` - The compactness of the segments.
    /// * `max_iter` - The maximum number of iterations.
    /// * `tolerance` - The tolerance for convergence conditions.
    ///
    /// # Returns
    /// A new `SLIC` instance.
    pub fn new(
        shape: (usize, usize),
        segments: usize,
        compactness: T,
        max_iter: usize,
        tolerance: T,
        metric: DistanceMetric,
    ) -> Result<Self, SLICError<T>> {
        if shape.0 == 0 || shape.1 == 0 {
            return Err(SLICError::InvalidShape(shape.0, shape.1));
        }
        if segments == 0 {
            return Err(SLICError::InvalidSegments(segments));
        }
        if compactness <= T::zero() {
            return Err(SLICError::InvalidCompactness(compactness));
        }
        if max_iter == 0 {
            return Err(SLICError::InvalidIterations(max_iter));
        }
        if tolerance <= T::zero() {
            return Err(SLICError::InvalidTolerance(tolerance));
        }
        Ok(Self {
            shape,
            segments,
            compactness,
            max_iter,
            tolerance,
            metric,
        })
    }

    /// Finds the lowest gradient point in the neighborhood of the given index.
    ///
    /// # Type Parameters
    /// * `N` - The number of dimensions.
    ///
    /// # Arguments
    /// * `matrix` - The matrix of points.
    /// * `index` - The index of the point to check.
    ///
    /// # Returns
    /// The lowest gradient point in the neighborhood if it exists.
    #[inline]
    #[must_use]
    fn find_lowest_gradient_point<const N: usize>(
        &self,
        matrix: &MatrixView<'_, T, N>,
        index: usize,
    ) -> Option<Point<T, N>> {
        let col = index % matrix.cols;
        let row = index / matrix.cols;

        let (_, lowest_point) = matrix.neighbors(col, row).fold(
            (T::max_value(), None),
            |(lowest_score, lowest_point), (neighbor_index, neighbor_point)| {
                let neighbor_col = neighbor_index % matrix.cols;
                let neighbor_row = neighbor_index / matrix.cols;
                let score = gradient(matrix, neighbor_col, neighbor_row, self.metric);
                if score < lowest_score {
                    (score, Some(neighbor_point))
                } else {
                    (lowest_score, lowest_point)
                }
            },
        );
        lowest_point.copied()
    }

    /// Iterates over the points and updates the centroids and clusters.
    ///
    /// # Type Parameters
    /// * `N` - The number of dimensions.
    ///
    /// # Arguments
    /// * `points` - The points to cluster.
    /// * `centroids` - The centroids of the clusters.
    /// * `clusters` - The clusters to update.
    ///
    /// # Returns
    /// `true` if the centroids have converged, `false` otherwise.
    #[inline]
    fn iterate<const N: usize>(
        &self,
        points: &[Point<T, N>],
        centroids: &mut [Point<T, N>],
        clusters: &mut [Cluster<T, N>],
    ) -> bool {
        clusters.iter_mut().for_each(Cluster::clear);

        // According to the paper, a neighborhood search should be performed in the range of 2Sx2S for each point.
        // In order to reduce the time-complexity in this implementation, a neighborhood search using KDTree is applied.
        // Although this is different from the original implementation, similar results are obtained.
        let centroid_search = KDTreeSearch::build(centroids, self.metric, Self::LEAF_SIZE);
        for (index, point) in points.iter().enumerate() {
            // Assign the point to the nearest centroid
            let neighbors = centroid_search.search_radius(point, self.compactness);
            if let Some(nearest) = neighbors.iter().min() {
                clusters[nearest.index].add_member(index, point);
            }
        }

        centroids
            .iter_mut()
            .zip(clusters)
            .fold(true, |converged, (old_centroid, cluster)| {
                let new_centroid = cluster.centroid();
                let difference = self.metric.measure(old_centroid, new_centroid);
                *old_centroid = *new_centroid;
                converged && difference <= self.tolerance
            })
    }
}

impl<T, const N: usize> ClusteringAlgorithm<T, N> for SLIC<T>
where
    T: FloatNumber,
{
    type Err = SLICError<T>;

    fn fit(&self, points: &[Point<T, N>]) -> Result<Vec<Cluster<T, N>>, Self::Err> {
        if points.is_empty() {
            return Err(SLICError::EmptyPoints);
        }

        let (cols, rows) = self.shape;
        let matrix = MatrixView::new(cols, rows, points).map_err(SLICError::InvalidPoints)?;

        let mut centroids = Initializer::Grid
            .initialize(&matrix, self.segments)
            .into_iter()
            .map(|seed_index| {
                let found = self.find_lowest_gradient_point(&matrix, seed_index);
                found.unwrap_or(points[seed_index])
            })
            .collect::<Vec<_>>();

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

    #[must_use]
    fn sample_points<T>(cols: usize, rows: usize) -> Vec<Point<T, 5>>
    where
        T: FloatNumber,
    {
        let half_cols = cols / 2;
        let half_rows = rows / 2;

        let mut points = vec![[T::zero(); 5]; cols * rows];
        for col in 0..cols {
            for row in 0..rows {
                let index = col + row * cols;
                let x = T::from_usize(col + 1) / T::from_usize(cols);
                let y = T::from_usize(row + 1) / T::from_usize(rows);
                points[index] = if col < half_cols && row < half_rows {
                    [T::one(), T::zero(), T::zero(), x, y]
                } else if col >= half_cols && row < half_rows {
                    [T::zero(), T::one(), T::zero(), x, y]
                } else if col < half_cols && row >= half_rows {
                    [T::zero(), T::zero(), T::one(), x, y]
                } else {
                    [T::zero(), T::zero(), T::zero(), x, y]
                };
            }
        }
        points
    }

    #[must_use]
    fn empty_points<T>() -> Vec<Point<T, 5>>
    where
        T: FloatNumber,
    {
        Vec::new()
    }

    #[test]
    fn test_new() {
        // Act
        let actual = SLIC::new((64, 48), 64, 1.0, 10, 1e-3, DistanceMetric::Euclidean);

        // Assert
        assert!(actual.is_ok());
        assert_eq!(
            actual.unwrap(),
            SLIC {
                shape: (64, 48),
                segments: 64,
                compactness: 1.0,
                max_iter: 10,
                tolerance: 1e-3,
                metric: DistanceMetric::Euclidean,
            }
        );
    }

    #[rstest]
    #[case::invalid_shape_rows(
        (0, 48),
        64,
        1.0,
        10,
        1e-3,
        DistanceMetric::Euclidean,
        SLICError::InvalidShape(0, 48)
    )]
    #[case::invalid_shape_columns(
        (64, 0),
        64,
        1.0,
        10,
        1e-3,
        DistanceMetric::Euclidean,
        SLICError::InvalidShape(64, 0)
    )]
    #[case::invalid_segments(
        (64, 48),
        0,
        1.0,
        10,
        1e-3,
        DistanceMetric::Euclidean,
        SLICError::InvalidSegments(0)
    )]
    #[case::invalid_compactness(
        (64, 48),
        64,
        0.0,
        10,
        1e-3,
        DistanceMetric::Euclidean,
        SLICError::InvalidCompactness(0.0)
    )]
    #[case::invalid_iterations(
        (64, 48),
        64,
        1.0,
        0,
        1e-3,
        DistanceMetric::Euclidean,
        SLICError::InvalidIterations(0)
    )]
    #[case::invalid_tolerance(
        (64, 48),
        64,
        1.0,
        10,
        0.0,
        DistanceMetric::Euclidean,
        SLICError::InvalidTolerance(0.0)
    )]
    fn test_new_error(
        #[case] shape: (usize, usize),
        #[case] segments: usize,
        #[case] compactness: f64,
        #[case] max_iter: usize,
        #[case] tolerance: f64,
        #[case] metric: DistanceMetric,
        #[case] expected: SLICError<f64>,
    ) {
        // Act
        let actual = SLIC::new(shape, segments, compactness, max_iter, tolerance, metric);

        // Assert
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), expected);
    }

    #[test]
    fn test_fit() {
        // Arrange
        let column = 48;
        let row = 27;
        let slic = SLIC::new((column, row), 32, 1.0, 10, 1e-3, DistanceMetric::Euclidean).unwrap();

        // Act
        let points = sample_points::<f64>(column, row);
        let actual = slic.fit(&points);

        // Assert
        assert!(actual.is_ok());
        let clusters = actual.unwrap();
        assert_eq!(clusters.len(), 32);
    }

    #[test]
    fn test_fit_empty_points() {
        // Arrange
        let slic = SLIC::new((64, 48), 64, 1.0, 10, 1e-3, DistanceMetric::Euclidean).unwrap();

        // Act
        let points = empty_points::<f64>();
        let actual = slic.fit(&points);

        // Assert
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), SLICError::EmptyPoints);
    }

    #[test]
    fn test_fit_invalid_points() {
        // Arrange
        let slic = SLIC::new((64, 48), 64, 1.0, 10, 1e-3, DistanceMetric::Euclidean).unwrap();

        // Act
        let points = sample_points::<f64>(64, 47);
        let actual = slic.fit(&points);

        // Assert
        assert!(actual.is_err());
        assert_eq!(
            actual.unwrap_err(),
            SLICError::InvalidPoints(MatrixError::InvalidPoints(64, 48))
        );
    }
}

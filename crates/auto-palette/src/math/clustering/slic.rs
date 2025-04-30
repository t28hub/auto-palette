use std::{collections::HashSet, fmt::Display};

use thiserror::Error;

use crate::{
    math::{
        clustering::{Cluster, ClusteringAlgorithm},
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
    #[error("Invalid Points Length: The number of points are not equal to the dimensions: {0}")]
    InvalidPointsLength(usize),
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

    /// Initializes the SLIC algorithm by selecting seed points.
    ///
    /// # Type Parameters
    /// * `N` - The number of dimensions.
    ///
    /// # Arguments
    /// * `matrix` - The matrix of points to cluster.
    ///
    /// # Returns
    /// A set of indices representing the seed points.
    #[must_use]
    fn initialize<const N: usize>(&self, matrix: &MatrixView<'_, T, N>) -> HashSet<usize> {
        let step = (T::from_usize(matrix.size()) / T::from_usize(self.segments))
            .sqrt()
            .round()
            .trunc_to_usize()
            .max(1); // Ensure step is at least 1
        let offset = step / 2;

        let (cols, rows) = matrix.shape();
        let mut seeds = HashSet::with_capacity(self.segments);
        'outer: for i in (offset..cols).step_by(step) {
            'inner: for j in (offset..rows).step_by(step) {
                let col = i.min(cols - 1);
                let row = j.min(rows - 1);
                let index = match matrix.index(col, row) {
                    Some(index) => index,
                    None => continue 'inner,
                };

                seeds.insert(index);
                if seeds.len() >= self.segments {
                    break 'outer;
                }
            }
        }
        seeds
    }

    /// Finds the index of the point with the lowest gradient in the 3x3 neighborhood.
    ///
    /// # Type Parameters
    /// * `N` - The number of dimensions.
    ///
    /// # Arguments
    /// * `points` - The points to cluster.
    /// * `index` - The index of the point to check.
    ///
    /// # Returns
    /// The index of the point with the lowest gradient in the neighborhood.
    #[inline]
    #[must_use]
    fn find_lowest_gradient_index<const N: usize>(
        &self,
        matrix: &MatrixView<'_, T, N>,
        index: usize,
    ) -> usize {
        let (cols, rows) = matrix.shape();
        let col = index % cols;
        let row = index / cols;

        let col_range = col.saturating_sub(1)..=(col + 1).min(cols - 1);
        let row_range = row.saturating_sub(1)..=(row + 1).min(rows - 1);

        let mut lowest_score = self.gradient(matrix, col, row);
        let mut lowest_index = index;
        for i in col_range {
            for j in row_range.clone() {
                let score = self.gradient(matrix, i, j);
                if score >= lowest_score {
                    continue;
                }

                if let Some(index) = matrix.index(i, j) {
                    lowest_index = index;
                    lowest_score = score;
                } else {
                    continue;
                }
            }
        }
        lowest_index
    }

    /// Measures the gradient at a given point.
    ///
    /// # Type Parameters
    /// * `N` - The number of dimensions.
    ///
    /// # Arguments
    /// * `matrix` - The matrix of points.
    /// * `col` - The column of the point.
    /// * `row` - The row of the point.
    ///
    /// # Returns
    /// The gradient at the given point. If the point is out of bounds, returns `T::max_value()`.
    #[inline]
    #[must_use]
    fn gradient<const N: usize>(&self, matrix: &MatrixView<'_, T, N>, col: usize, row: usize) -> T {
        if col == 0 || col == matrix.cols - 1 {
            return T::max_value();
        }
        if row == 0 || row == matrix.rows - 1 {
            return T::max_value();
        }

        let dx = self.axis_gradient(matrix, col - 1, row, col + 1, row);
        let dy = self.axis_gradient(matrix, col, row - 1, col, row + 1);
        dx + dy
    }

    /// Measures the gradient of the axis between two points.
    ///
    /// # Type Parameters
    /// * `N` - The number of dimensions.
    ///
    /// # Arguments
    /// * `matrix` - The matrix of points.
    /// * `col1` - The column of the first point.
    /// * `row1` - The row of the first point.
    /// * `col2` - The column of the second point.
    /// * `row2` - The row of the second point.
    ///
    /// # Returns
    /// The gradient of the axis between the two points. If either point is out of bounds, returns `T::max_value()`.
    #[inline]
    #[must_use]
    fn axis_gradient<const N: usize>(
        &self,
        matrix: &MatrixView<'_, T, N>,
        col1: usize,
        row1: usize,
        col2: usize,
        row2: usize,
    ) -> T {
        match (matrix.get(col1, row1), matrix.get(col2, row2)) {
            (Some(point1), Some(point2)) => self.metric.measure(point1, point2),
            _ => T::max_value(),
        }
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

        let matrix = MatrixView::new(self.shape.0, self.shape.1, points)?;
        let mut centroids: Vec<Point<T, N>> = self
            .initialize(&matrix)
            .into_iter()
            .map(|index| {
                let best_index = self.find_lowest_gradient_index(&matrix, index);
                points[best_index]
            })
            .collect();

        let mut clusters = vec![Cluster::new(); centroids.len()];
        for _ in 0..self.max_iter {
            if self.iterate(points, &mut centroids, &mut clusters) {
                break;
            }
        }
        Ok(clusters)
    }
}

/// Lightweight, read-only view over a matrix of points.
///
/// `MatrixView` does NOT own the points, it only keeps a slice reference to them, making it cheap to copy.
///
/// # Type Parameters
/// * `T` - The floating point type.
/// * `N` - The number of dimensions.
#[derive(Debug, PartialEq)]
struct MatrixView<'a, T, const N: usize>
where
    T: FloatNumber,
{
    /// The number of columns in the matrix.
    cols: usize,

    /// The number of rows in the matrix.
    rows: usize,

    /// The points in the matrix.
    points: &'a [Point<T, N>],
}

impl<'a, T, const N: usize> MatrixView<'a, T, N>
where
    T: FloatNumber,
{
    /// Creates a new `MatrixView` instance.
    ///
    /// # Arguments
    /// * `cols` - The number of columns in the matrix.
    /// * `rows` - The number of rows in the matrix.
    /// * `points` - The points to view.
    ///
    /// # Returns
    /// A new `MatrixView` instance.
    #[inline]
    fn new(cols: usize, rows: usize, points: &'a [Point<T, N>]) -> Result<Self, SLICError<T>> {
        let expected_len = cols * rows;
        if expected_len != points.len() {
            return Err(SLICError::InvalidPointsLength(expected_len));
        }
        Ok(Self { cols, rows, points })
    }

    /// Returns the size of the matrix.
    ///
    /// # Returns
    /// The size of the matrix.
    #[inline]
    #[must_use]
    fn size(&self) -> usize {
        self.points.len()
    }

    /// Returns the shape of the matrix.
    ///
    /// # Returns
    /// The shape of the matrix.
    #[inline]
    #[must_use]
    fn shape(&self) -> (usize, usize) {
        (self.cols, self.rows)
    }

    /// Returns the flattened index of the given column and row.
    ///
    /// # Arguments
    /// * `col` - The column of the point.
    /// * `row` - The row of the point.
    ///
    /// # Returns
    /// The flattened index of the given column and row or `None` if the index is out of bounds.
    #[inline(always)]
    #[must_use]
    fn index(&self, col: usize, row: usize) -> Option<usize> {
        if col < self.cols && row < self.rows {
            Some(col + row * self.cols)
        } else {
            None
        }
    }

    /// Returns the point at the given column and row.
    ///
    /// # Arguments
    /// * `col` - The column of the point.
    /// * `row` - The row of the point.
    ///
    /// # Returns
    /// The point at the given column and row or `None` if the index is out of bounds.
    #[inline(always)]
    #[must_use]
    fn get(&self, col: usize, row: usize) -> Option<&Point<T, N>> {
        self.index(col, row).map(|index| &self.points[index])
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::assert_approx_eq;

    #[must_use]
    fn sample_points<T>(cols: usize, rows: usize) -> Vec<Point<T, 5>>
    where
        T: FloatNumber,
    {
        vec![[T::zero(); 5]; cols * rows]
    }

    #[must_use]
    fn empty_points<T>() -> Vec<Point<T, 5>>
    where
        T: FloatNumber,
    {
        Vec::new()
    }

    fn fill_points<T>(points: &mut [Point<T, 5>], cols: usize, rows: usize)
    where
        T: FloatNumber,
    {
        let half_cols = cols / 2;
        let half_rows = rows / 2;
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
        #[case] n_segments: usize,
        #[case] compactness: f64,
        #[case] max_iter: usize,
        #[case] tolerance: f64,
        #[case] metric: DistanceMetric,
        #[case] expected: SLICError<f64>,
    ) {
        // Act
        let actual = SLIC::new(shape, n_segments, compactness, max_iter, tolerance, metric);

        // Assert
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), expected);
    }

    #[test]
    fn test_fit() {
        // Arrange
        let column = 48;
        let row = 27;

        let mut points = sample_points::<f64>(column, row);
        fill_points(&mut points, column, row);

        let slic = SLIC::new((column, row), 32, 1.0, 10, 1e-3, DistanceMetric::Euclidean).unwrap();

        // Act
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
        let points = empty_points::<f64>();

        // Act
        let actual = slic.fit(&points);

        // Assert
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), SLICError::EmptyPoints);
    }

    #[test]
    fn test_fit_invalid_points_length() {
        // Arrange
        let slic = SLIC::new((64, 48), 64, 1.0, 10, 1e-3, DistanceMetric::Euclidean).unwrap();
        let points = sample_points::<f64>(64, 47);

        // Act
        let actual = slic.fit(&points);

        // Assert
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), SLICError::InvalidPointsLength(64 * 48));
    }

    #[test]
    fn test_initialize() {
        // Arrange
        let cols = 48;
        let rows = 27;
        let mut points = sample_points::<f64>(cols, rows);
        fill_points(&mut points, cols, rows);

        let matrix = MatrixView::new(cols, rows, &points).unwrap();
        let slic = SLIC::new((cols, rows), 32, 1.0, 10, 1e-3, DistanceMetric::Euclidean).unwrap();

        // Act
        let actual = slic.initialize(&matrix);

        // Assert
        assert_eq!(actual.len(), 32);
    }

    #[rstest]
    #[case::left_top(0, 0, f64::MAX)]
    #[case::right_top(47, 0, f64::MAX)]
    #[case::left_bottom(0, 26, f64::MAX)]
    #[case::right_bottom(47, 26, f64::MAX)]
    #[case::edge_x(24, 18, 1.008680)]
    #[case::edge_y(32, 12, 1.008680)]
    #[case::center(24, 12, 2.008680)]
    #[case::normal(12, 6, 0.008680)]
    fn test_gradient(#[case] col: usize, #[case] row: usize, #[case] expected: f64) {
        // Arrange
        let cols = 48;
        let rows = 24;
        let mut points = sample_points::<f64>(cols, rows);
        fill_points(&mut points, cols, rows);

        let matrix = MatrixView::new(cols, rows, &points).unwrap();
        let slic = SLIC::new(
            (cols, rows),
            32,
            1.0,
            10,
            1e-3,
            DistanceMetric::SquaredEuclidean,
        )
        .unwrap();

        // Act
        let actual = slic.gradient(&matrix, col, row);

        // Assert
        assert_approx_eq!(actual, expected);
    }

    #[rstest]
    #[case::x_axis(0, 0, 1, 0, 0.000434)]
    #[case::y_axis(47, 0, 47, 1, 0.001736)]
    #[case::x_axis_out_of_bounds(47, 0, 48, 0, f64::MAX)]
    #[case::y_axis_out_of_bounds(0, 26, 0, 27, f64::MAX)]
    fn test_axis_gradient(
        #[case] col1: usize,
        #[case] row1: usize,
        #[case] col2: usize,
        #[case] row2: usize,
        #[case] expected: f64,
    ) {
        // Arrange
        let cols = 48;
        let rows = 24;
        let mut points = sample_points::<f64>(cols, rows);
        fill_points(&mut points, cols, rows);

        let matrix = MatrixView::new(cols, rows, &points).unwrap();
        let slic = SLIC::new(
            (cols, rows),
            32,
            1.0,
            10,
            1e-3,
            DistanceMetric::SquaredEuclidean,
        )
        .unwrap();

        // Act
        let actual = slic.axis_gradient(&matrix, col1, row1, col2, row2);

        // Assert
        assert_approx_eq!(actual, expected);
    }

    #[test]
    fn test_matrix_view_new() {
        // Arrange
        let cols = 48;
        let rows = 27;
        let points = sample_points::<f64>(cols, rows);

        // Act
        let actual = MatrixView::new(cols, rows, &points);

        // Assert
        assert!(actual.is_ok());
        assert_eq!(
            actual.unwrap(),
            MatrixView {
                cols,
                rows,
                points: &points,
            }
        );
    }

    #[test]
    fn test_matrix_view_new_invalid_length() {
        // Arrange
        let cols = 48;
        let rows = 27;
        let points = sample_points::<f64>(cols, rows - 1);

        // Act
        let matrix = MatrixView::new(cols, rows, &points);

        // Assert
        assert!(matrix.is_err());
        assert_eq!(
            matrix.unwrap_err(),
            SLICError::InvalidPointsLength(cols * rows)
        );
    }

    #[rstest]
    #[case(0, 0, 0)]
    #[case(1, 0, 0)]
    #[case(0, 1, 0)]
    #[case(1, 1, 1)]
    #[case(4, 9, 36)]
    #[case(9, 4, 36)]
    #[case(27, 48, 1296)]
    #[case(48, 27, 1296)]
    fn test_matrix_view_size(#[case] cols: usize, #[case] rows: usize, #[case] expected: usize) {
        // Arrange
        let points = sample_points::<f64>(cols, rows);
        let matrix = MatrixView::new(cols, rows, &points).unwrap();

        // Act
        let actual = matrix.size();

        // Assert
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case(0, 0, (0, 0))]
    #[case(1, 0, (1, 0))]
    #[case(0, 1, (0, 1))]
    #[case(1, 1, (1, 1))]
    #[case(4, 9, (4, 9))]
    #[case(9, 4, (9, 4))]
    fn test_matrix_view_shape(
        #[case] cols: usize,
        #[case] rows: usize,
        #[case] expected: (usize, usize),
    ) {
        // Arrange
        let points = sample_points::<f64>(cols, rows);
        let matrix = MatrixView::new(cols, rows, &points).unwrap();

        // Act
        let actual = matrix.shape();

        // Assert
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case(0, 0, Some(0))]
    #[case(1, 0, Some(1))]
    #[case(0, 1, Some(48))]
    #[case(1, 1, Some(49))]
    #[case(4, 9, Some(4 + 9 * 48))]
    #[case(9, 4, Some(9 + 4 * 48))]
    #[case(0, 26, Some(0 + 26 * 48))]
    #[case(47, 0, Some(47))]
    #[case(47, 26, Some(47 + 26 * 48))]
    #[case(0, 27, None)]
    #[case(48, 0, None)]
    fn test_matrix_view_index(
        #[case] col: usize,
        #[case] row: usize,
        #[case] expected: Option<usize>,
    ) {
        // Arrange
        let cols = 48;
        let rows = 27;
        let points: Vec<Point<f64, 5>> = vec![[0.0; 5]; cols * rows];
        let matrix = MatrixView::new(cols, rows, &points).unwrap();

        // Act
        let actual = matrix.index(col, row);

        // Assert
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case(0, 0, 0)]
    #[case(1, 0, 1)]
    #[case(0, 1, 48)]
    #[case(1, 1, 49)]
    #[case(47, 0, 47)]
    #[case(0, 26, 0 + 26 * 48)]
    #[case(47, 26, 47 + 26 * 48)]
    fn test_matrix_view_get(#[case] col: usize, #[case] row: usize, #[case] index: usize) {
        // Arrange
        let cols = 48;
        let rows = 27;
        let mut points = sample_points::<f64>(cols, rows);
        fill_points(&mut points, cols, rows);
        let matrix = MatrixView::new(cols, rows, &points).unwrap();

        // Act
        let actual = matrix.get(col, row);
        assert!(actual.is_some());
        assert_eq!(actual.unwrap(), &points[index]);
    }

    #[rstest]
    #[case(0, 27)]
    #[case(48, 0)]
    #[case(47, 27)]
    #[case(48, 26)]
    #[case(48, 27)]
    fn test_matrix_view_get_out_of_bounds(#[case] col: usize, #[case] row: usize) {
        // Arrange
        let cols = 48;
        let rows = 27;
        let points = sample_points::<f64>(cols, rows);
        let matrix = MatrixView::new(cols, rows, &points).unwrap();

        // Act
        let actual = matrix.get(col, row);

        // Assert
        assert!(actual.is_none());
    }
}

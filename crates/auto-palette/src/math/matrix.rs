use std::collections::HashSet;

use thiserror::Error;

use crate::{math::Point, FloatNumber};

/// Error type for the `MatrixView` struct.
#[derive(Debug, PartialEq, Error)]
pub enum MatrixError {
    /// Error when the shape of the matrix is invalid.
    #[error("Invalid Shape: The shape must be > 0: {0}x{1}")]
    InvalidShape(usize, usize),

    /// Error when the points slice is not in the expected shape.
    #[error("Invalid Points: The points slice is not in the expected shape: {0}x{1}.")]
    InvalidPoints(usize, usize),
}

/// Lightweight, read-only view over a matrix of points.
///
/// `MatrixView` does NOT own the points, it only keeps a slice reference to them, making it cheap to copy.
///
/// # Type Parameters
/// * `T` - The floating point type.
/// * `N` - The number of dimensions.
#[derive(Debug, PartialEq)]
pub struct MatrixView<'a, T, const N: usize>
where
    T: FloatNumber,
{
    /// The number of columns in the matrix.
    pub(super) cols: usize,

    /// The number of rows in the matrix.
    pub(super) rows: usize,

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
    pub fn new(cols: usize, rows: usize, points: &'a [Point<T, N>]) -> Result<Self, MatrixError> {
        if cols == 0 || rows == 0 {
            return Err(MatrixError::InvalidShape(cols, rows));
        }

        if cols * rows != points.len() {
            return Err(MatrixError::InvalidPoints(cols, rows));
        }
        Ok(Self { cols, rows, points })
    }

    /// Returns the size of the matrix.
    ///
    /// # Returns
    /// The size of the matrix.
    #[inline(always)]
    #[must_use]
    pub fn size(&self) -> usize {
        self.points.len()
    }

    /// Returns the shape of the matrix.
    ///
    /// # Returns
    /// The shape of the matrix.
    #[inline]
    #[must_use]
    pub fn shape(&self) -> (usize, usize) {
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
    pub fn index(&self, col: usize, row: usize) -> Option<usize> {
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
    pub fn get(&self, col: usize, row: usize) -> Option<&Point<T, N>> {
        self.index(col, row).map(|index| &self.points[index])
    }

    /// Applies the `action` function to each neighbor of the point at the given column and row.
    ///
    /// # Arguments
    /// * `col` - The column of the point.
    /// * `row` - The row of the point.
    ///
    /// # Returns
    /// A set of indices of the neighboring points.
    #[inline]
    #[must_use]
    pub fn neighbor_indices(&self, col: usize, row: usize) -> HashSet<usize> {
        if col >= self.cols || row >= self.rows {
            return HashSet::new();
        }

        let mut neighbors = HashSet::with_capacity(8);
        for i in col.saturating_sub(1)..=(col + 1).min(self.cols - 1) {
            for j in row.saturating_sub(1)..=(row + 1).min(self.rows - 1) {
                // Skip the target point itself
                if i == col && j == row {
                    continue;
                }

                if let Some(index) = self.index(i, j) {
                    neighbors.insert(index);
                }
            }
        }
        neighbors
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_new() {
        // Arrange
        let cols = 16;
        let rows = 9;
        let points = vec![[0.0; 3]; cols * rows];

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

    #[rstest]
    #[case(0, 0)]
    #[case(0, 9)]
    #[case(16, 0)]
    fn test_new_invalid_shape(#[case] cols: usize, #[case] rows: usize) {
        // Arrange
        let points = vec![[0.0; 3]; cols * rows];

        // Act
        let matrix = MatrixView::new(cols, rows, &points);

        // Assert
        assert!(matrix.is_err());
        assert_eq!(matrix.unwrap_err(), MatrixError::InvalidShape(cols, rows));
    }

    #[test]
    fn test_new_invalid_points() {
        // Arrange
        let cols = 16;
        let rows = 9;
        let points = vec![[0.0; 3]; cols * rows - 1];

        // Act
        let matrix = MatrixView::new(cols, rows, &points);

        // Assert
        assert!(matrix.is_err());
        assert_eq!(matrix.unwrap_err(), MatrixError::InvalidPoints(cols, rows));
    }

    #[rstest]
    #[case(1, 1, 1)]
    #[case(2, 3, 6)]
    #[case(16, 9, 144)]
    fn test_size(#[case] cols: usize, #[case] rows: usize, #[case] expected: usize) {
        // Arrange
        let points = vec![[0.0; 3]; cols * rows];
        let matrix = MatrixView::new(cols, rows, &points).unwrap();

        // Act
        let actual = matrix.size();

        // Assert
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case(1, 1, (1, 1))]
    #[case(4, 9, (4, 9))]
    #[case(9, 4, (9, 4))]
    #[case(1, 1024, (1, 1024))]
    #[case(1024, 1, (1024, 1))]
    fn test_shape(#[case] cols: usize, #[case] rows: usize, #[case] expected: (usize, usize)) {
        // Arrange
        let points = vec![[0.0; 3]; cols * rows];
        let matrix = MatrixView::new(cols, rows, &points).unwrap();

        // Act
        let actual = matrix.shape();

        // Assert
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case(0, 0, Some(0))]
    #[case(1, 0, Some(1))]
    #[case(15, 0, Some(15))]
    #[case(0, 1, Some(16))]
    #[case(1, 1, Some(17))]
    #[case(0, 8, Some(128))]
    #[case(15, 8, Some(143))]
    #[case(16, 0, None)]
    #[case(0, 9, None)]
    #[case(16, 9, None)]
    fn test_index(#[case] col: usize, #[case] row: usize, #[case] expected: Option<usize>) {
        // Arrange
        let cols = 16;
        let rows = 9;
        let points = vec![[0.0; 3]; cols * rows];
        let matrix = MatrixView::new(cols, rows, &points).unwrap();

        // Act
        let actual = matrix.index(col, row);

        // Assert
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case::center(8, 4, 72)]
    #[case::left_top(0, 0, 0)]
    #[case::left_bottom(0, 8, 128)]
    #[case::right_top(15, 0, 15)]
    #[case::right_bottom(15, 8, 143)]
    fn test_get(#[case] col: usize, #[case] row: usize, #[case] index: usize) {
        // Arrange
        let cols = 16;
        let rows = 9;
        let mut points = vec![[0.0; 3]; cols * rows];
        for i in 0..points.len() {
            points[i] = [i as f64, i as f64, i as f64];
        }
        let matrix = MatrixView::new(cols, rows, &points).unwrap();

        // Act
        let actual = matrix.get(col, row);

        // Assert
        assert!(actual.is_some());
        assert_eq!(actual.unwrap(), &points[index]);
    }

    #[rstest]
    #[case::left_bottom(16, 0)]
    #[case::right_top(0, 9)]
    #[case::right_bottom(16, 9)]
    fn test_get_out_of_bounds(#[case] col: usize, #[case] row: usize) {
        // Arrange
        let cols = 16;
        let rows = 9;
        let points = vec![[0.0; 3]; cols * rows];
        let matrix = MatrixView::new(cols, rows, &points).unwrap();

        // Act
        let actual = matrix.get(col, row);

        // Assert
        assert!(actual.is_none());
    }

    #[rstest]
    #[case((0, 0), vec![1, 16, 17])]
    #[case((0, 1), vec![0, 1, 17, 32, 33])]
    #[case((1, 0), vec![0, 2, 16, 17, 18])]
    #[case((1, 1), vec![0, 1, 2, 16, 18, 32, 33, 34])]
    #[case((0, 8), vec![112, 113, 129])]
    #[case((1, 8), vec![112, 113, 114, 128, 130])]
    #[case((15, 0), vec![14, 30, 31])]
    #[case((15, 7), vec![110, 111, 126, 142, 143])]
    #[case((15, 8), vec![126, 127, 142])]
    fn test_neighbor_indices(#[case] (col, row): (usize, usize), #[case] expected: Vec<usize>) {
        // Arrange
        let cols = 16;
        let rows = 9;
        let points = vec![[0.0; 3]; cols * rows];
        let matrix = MatrixView::new(cols, rows, &points).unwrap();

        // Act
        let actual = matrix.neighbor_indices(col, row);

        // Assert
        assert_eq!(actual.len(), expected.len());
        assert_eq!(actual, HashSet::from_iter(expected));
    }

    #[rstest]
    #[case::right_top(0, 9)]
    #[case::left_bottom(16, 0)]
    #[case::right_bottom(16, 9)]
    fn test_neighbor_indices_empty(#[case] col: usize, #[case] row: usize) {
        // Arrange
        let cols = 16;
        let rows = 9;
        let points = vec![[0.0; 3]; cols * rows];
        let matrix = MatrixView::new(cols, rows, &points).unwrap();

        // Act
        let actual = matrix.neighbor_indices(col, row);

        // Assert
        assert!(actual.is_empty());
    }
}

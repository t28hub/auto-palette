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

    /// Returns an iterator over the 8 neighboring points of the point at the given column and row.
    ///
    /// # Arguments
    /// * `col` - The column of the point.
    /// * `row` - The row of the point.
    ///
    /// # Returns
    /// An iterator over the neighboring points.
    #[inline]
    #[must_use]
    pub fn neighbors(&self, col: usize, row: usize) -> NeighborIterator<T, N> {
        NeighborIterator::new(self, col, row)
    }
}

#[derive(Debug, PartialEq)]
pub struct NeighborIterator<'a, T, const N: usize>
where
    T: FloatNumber,
{
    matrix: &'a MatrixView<'a, T, N>,
    col: usize,
    row: usize,
    dx: i8,
    dy: i8,
}

impl<'a, T, const N: usize> NeighborIterator<'a, T, N>
where
    T: FloatNumber,
{
    /// Initial delta x value.
    const INITIAL_DX: i8 = -1;

    /// Initial delta y value.
    const INITIAL_DY: i8 = -1;

    /// Creates a new `NeighborIterator` instance.
    ///
    /// # Arguments
    /// * `matrix` - The matrix to iterate over.
    /// * `col` - The column of the point.
    /// * `row` - The row of the point.
    ///
    /// # Returns
    /// A new `NeighborIterator` instance.
    #[inline]
    #[must_use]
    pub fn new(matrix: &'a MatrixView<'a, T, N>, col: usize, row: usize) -> Self {
        Self {
            matrix,
            col,
            row,
            dx: Self::INITIAL_DX,
            dy: Self::INITIAL_DY,
        }
    }
}

impl<'a, T, const N: usize> Iterator for NeighborIterator<'a, T, N>
where
    T: FloatNumber,
{
    type Item = (usize, &'a Point<T, N>);

    fn next(&mut self) -> Option<Self::Item> {
        let (max_cols, max_rows) = self.matrix.shape();

        // Check if the current indices are out of bounds
        if self.col >= max_cols || self.row >= max_rows {
            return None;
        }

        while self.dy <= 1 {
            let dy = self.dy;
            while self.dx <= 1 {
                let dx = self.dx;
                self.dx += 1;

                // Skip the target point itself
                if dx == 0 && dy == 0 {
                    continue;
                }

                let col = self.col.checked_add_signed(dx.into());
                let row = self.row.checked_add_signed(dy.into());
                match (col, row) {
                    (Some(col), Some(row)) => {
                        // Check if the indices are within bounds
                        if col >= max_cols || row >= max_rows {
                            continue;
                        }

                        let index = col + row * max_cols;
                        let point = &self.matrix.points[index];
                        return Some((index, point));
                    }
                    _ => continue,
                }
            }
            self.dx = Self::INITIAL_DX;
            self.dy += 1;
        }
        None
    }
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
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
    fn test_neighbors(#[case] (col, row): (usize, usize), #[case] expected: Vec<usize>) {
        // Arrange
        let cols = 16;
        let rows = 9;
        let points = vec![[0.0; 3]; cols * rows];
        let matrix = MatrixView::new(cols, rows, &points).unwrap();

        // Act
        let actual =
            matrix
                .neighbors(col, row)
                .fold(Vec::with_capacity(8), |mut acc, (index, _)| {
                    acc.push(index);
                    acc
                });

        // Assert
        assert_eq!(actual.len(), expected.len());
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case::right_top(0, 9)]
    #[case::left_bottom(16, 0)]
    #[case::right_bottom(16, 9)]
    fn test_neighbors_empty(#[case] col: usize, #[case] row: usize) {
        // Arrange
        let cols = 16;
        let rows = 9;
        let points = vec![[0.0; 3]; cols * rows];
        let matrix = MatrixView::new(cols, rows, &points).unwrap();

        // Act
        let actual =
            matrix
                .neighbors(col, row)
                .fold(Vec::with_capacity(8), |mut acc, (index, _)| {
                    acc.push(index);
                    acc
                });

        // Assert
        assert!(actual.is_empty());
    }

    #[test]
    fn test_neighbor_iterator_new() {
        // Arrange
        let cols = 16;
        let rows = 9;
        let points = vec![[0.0; 3]; cols * rows];
        let matrix = MatrixView::new(cols, rows, &points).unwrap();

        // Act
        let actual = NeighborIterator::new(&matrix, 8, 4);

        // Assert
        assert_eq!(
            actual,
            NeighborIterator {
                matrix: &matrix,
                col: 8,
                row: 4,
                dx: -1,
                dy: -1,
            }
        );
    }

    #[test]
    fn test_neighbor_iterator_next() {
        // Arrange
        let cols = 3;
        let rows = 2;
        let mut points = vec![[0.0; 2]; cols * rows];
        for i in 0..points.len() {
            points[i] = [i as f64, i as f64];
        }
        let matrix = MatrixView::new(cols, rows, &points).unwrap();

        let mut iterator = NeighborIterator::new(&matrix, 1, 1);

        // Act & Assert
        assert_eq!(iterator.next(), Some((0, &points[0])));
        assert_eq!(iterator.next(), Some((1, &points[1])));
        assert_eq!(iterator.next(), Some((2, &points[2])));
        assert_eq!(iterator.next(), Some((3, &points[3])));
        assert_eq!(iterator.next(), Some((5, &points[5])));
        assert_eq!(iterator.next(), None);
    }

    #[rstest]
    #[case::cols(3, 1)]
    #[case::rows(2, 2)]
    #[case::cols_rows(3, 2)]
    fn test_neighbor_iterator_next_out_of_bounds(#[case] col: usize, #[case] row: usize) {
        // Arrange
        let cols = 3;
        let rows = 2;
        let points = vec![[0.0; 2]; cols * rows];
        let matrix = MatrixView::new(cols, rows, &points).unwrap();

        let mut iterator = NeighborIterator::new(&matrix, col, row);

        // Act & Assert
        assert_eq!(iterator.next(), None);
    }
}

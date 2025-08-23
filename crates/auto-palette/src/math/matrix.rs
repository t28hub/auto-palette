use thiserror::Error;

use crate::{math::Point, FloatNumber};

/// Errors that can occur when creating or using a `MatrixView`.
#[derive(Debug, PartialEq, Error)]
pub enum MatrixError {
    /// The provided dimensions do not match the points slice length.
    ///
    /// This occurs when `cols * rows != points.len()`.
    #[error("dimension mismatch: expected {expected} points ({cols}x{rows}), but got {actual}")]
    DimensionMismatch {
        cols: usize,
        rows: usize,
        expected: usize,
        actual: usize,
    },
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
    pub(crate) cols: usize,

    /// The number of rows in the matrix.
    pub(crate) rows: usize,

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
        let expected = cols * rows;
        let actual = points.len();

        if expected != actual {
            return Err(MatrixError::DimensionMismatch {
                cols,
                rows,
                expected,
                actual,
            });
        }

        Ok(Self { cols, rows, points })
    }

    /// Returns the total number of points in the matrix.
    #[inline(always)]
    #[must_use]
    pub fn size(&self) -> usize {
        self.points.len()
    }

    /// Returns the shape of the matrix as (columns, rows).
    #[inline]
    #[must_use]
    pub fn shape(&self) -> (usize, usize) {
        (self.cols, self.rows)
    }

    /// Returns the flattened index of the given column and row.
    ///
    /// Converts 2D coordinates (col, row) to a 1D index in row-major order.
    ///
    /// # Arguments
    /// * `col` - The column of the point.
    /// * `row` - The row of the point.
    ///
    /// # Returns
    /// The flattened index of the given column and row or `None` if the index is out of bounds.
    #[inline(always)]
    #[must_use]
    pub fn flatten_index(&self, col: usize, row: usize) -> Option<usize> {
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
        self.flatten_index(col, row)
            .map(|index| &self.points[index])
    }

    /// Returns an iterator over the neighbors of the given point.
    ///
    /// The iterator yields all adjacent points within a radius of 1, excluding the center point itself.
    ///
    /// # Arguments
    /// * `col` - The column of the point.
    /// * `row` - The row of the point.
    ///
    /// # Returns
    /// An iterator over the neighbors of the given point.
    #[inline]
    #[must_use]
    pub fn neighbors(&self, col: usize, row: usize) -> NeighborIterator<'_, T, N> {
        NeighborIterator::new(self, col, row, 1)
    }

    /// Returns an iterator over the neighbors within a given `radius` around the given `(col, row)`.
    ///
    /// The iterator yields all points within the square neighborhood defined by the radius,
    /// excluding the center point itself. For example, with radius=1, it yields up to 8 points
    /// (the 3x3 grid minus the center).
    ///
    /// # Arguments
    /// * `col` - The column of the point.
    /// * `row` - The row of the point.
    /// * `radius` - The size of the neighborhood.
    ///
    /// # Returns
    /// An iterator over the neighbors of the given point within the specified radius.
    #[inline]
    #[must_use]
    pub fn neighbors_within(
        &self,
        col: usize,
        row: usize,
        radius: usize,
    ) -> NeighborIterator<'_, T, N> {
        NeighborIterator::new(self, col, row, radius)
    }
}

/// An iterator over the neighbors of a point in a matrix.
///
/// This iterator yields all points within a square neighborhood of a given radius,
/// excluding the center point itself. The iteration proceeds in row-major order,
/// from top-left to bottom-right.
///
/// # Type Parameters
/// * `T` - The floating point type.
/// * `N` - The number of dimensions.
#[derive(Debug, PartialEq)]
pub struct NeighborIterator<'a, T, const N: usize>
where
    T: FloatNumber,
{
    matrix: &'a MatrixView<'a, T, N>,
    col: usize,
    row: usize,
    radius: isize,
    dx: isize,
    dy: isize,
}

impl<'a, T, const N: usize> NeighborIterator<'a, T, N>
where
    T: FloatNumber,
{
    /// Creates a new `NeighborIterator` instance.
    ///
    /// # Arguments
    /// * `matrix` - The matrix to iterate over.
    /// * `col` - The column of the point.
    /// * `row` - The row of the point.
    /// * `radius` - The size of the neighborhood.
    ///
    /// # Returns
    /// A new `NeighborIterator` instance.
    #[inline]
    #[must_use]
    pub fn new(matrix: &'a MatrixView<'a, T, N>, col: usize, row: usize, radius: usize) -> Self {
        let radius = radius as isize;
        Self {
            matrix,
            col,
            row,
            radius,
            dx: -radius,
            dy: -radius,
        }
    }
}

impl<'a, T, const N: usize> Iterator for NeighborIterator<'a, T, N>
where
    T: FloatNumber,
{
    type Item = (usize, &'a Point<T, N>);

    fn next(&mut self) -> Option<Self::Item> {
        let (cols, rows) = self.matrix.shape();

        // Check if the current indices are out of bounds
        if self.col >= cols || self.row >= rows {
            return None;
        }

        while self.dy <= self.radius {
            let dy = self.dy;

            while self.dx <= self.radius {
                let dx = self.dx;
                self.dx += 1;

                // Skip the target point itself
                if dx == 0 && dy == 0 {
                    continue;
                }

                let col = self.col.checked_add_signed(dx);
                let row = self.row.checked_add_signed(dy);
                match (col, row) {
                    (Some(col), Some(row)) => {
                        // Check if the indices are within bounds
                        if col >= cols || row >= rows {
                            continue;
                        }

                        let index = col + row * cols;
                        let point = &self.matrix.points[index];
                        return Some((index, point));
                    }
                    _ => continue,
                }
            }

            // Reset the X offset and move to the next Y offset
            self.dx = -self.radius;
            self.dy += 1;
        }
        None
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

    #[test]
    fn test_new_empty() {
        // Act
        let points = Vec::<[f64; 3]>::new();
        let matrix = MatrixView::new(0, 0, &points);

        // Assert
        assert!(matrix.is_ok());

        let matrix = matrix.unwrap();
        assert_eq!(matrix.cols, 0);
        assert_eq!(matrix.rows, 0);
        assert_eq!(matrix.points.len(), 0);
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
        assert_eq!(
            matrix.unwrap_err(),
            MatrixError::DimensionMismatch {
                cols,
                rows,
                expected: cols * rows,
                actual: cols * rows - 1,
            }
        );
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
    fn test_flatten_index(#[case] col: usize, #[case] row: usize, #[case] expected: Option<usize>) {
        // Arrange
        let cols = 16;
        let rows = 9;
        let points = vec![[0.0; 3]; cols * rows];
        let matrix = MatrixView::new(cols, rows, &points).unwrap();

        // Act
        let actual = matrix.flatten_index(col, row);

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
    fn test_neighbors_out_of_bounds(#[case] col: usize, #[case] row: usize) {
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

    #[rstest]
    #[case(0, (0, 0), vec![])]
    #[case(1, (0, 0), vec![1, 16, 17])]
    #[case(2, (0, 0), vec![1, 2, 16, 17, 18, 32, 33, 34])]
    #[case(0, (8, 4), vec![])]
    #[case(1, (8,4), vec![55, 56, 57, 71, 73, 87, 88, 89])]
    #[case(2, (8,4), vec![38, 39, 40, 41, 42, 54, 55, 56, 57, 58, 70, 71, 73, 74, 86, 87, 88, 89, 90, 102, 103, 104, 105, 106])]
    #[case(0, (15, 8), vec![])]
    #[case(1, (15, 8), vec![126, 127, 142])]
    #[case(2, (15, 8), vec![109, 110, 111, 125, 126, 127, 141, 142])]
    fn test_neighbors_within(
        #[case] radius: usize,
        #[case] (col, row): (usize, usize),
        #[case] expected: Vec<usize>,
    ) {
        // Arrange
        let cols = 16;
        let rows = 9;
        let points = vec![[0.0; 3]; cols * rows];
        let matrix = MatrixView::new(cols, rows, &points).unwrap();

        // Act
        let actual =
            matrix
                .neighbors_within(col, row, radius)
                .fold(Vec::new(), |mut acc, (index, _)| {
                    acc.push(index);
                    acc
                });

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_neighbor_iterator_new() {
        // Arrange
        let cols = 16;
        let rows = 9;
        let points = vec![[0.0; 3]; cols * rows];
        let matrix = MatrixView::new(cols, rows, &points).unwrap();

        // Act
        let actual = NeighborIterator::new(&matrix, 8, 4, 1);

        // Assert
        assert_eq!(
            actual,
            NeighborIterator {
                matrix: &matrix,
                col: 8,
                row: 4,
                radius: 1,
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

        let mut iterator = NeighborIterator::new(&matrix, 1, 1, 1);

        // Act & Assert
        assert_eq!(iterator.next(), Some((0, &points[0])));
        assert_eq!(iterator.next(), Some((1, &points[1])));
        assert_eq!(iterator.next(), Some((2, &points[2])));
        assert_eq!(iterator.next(), Some((3, &points[3])));
        assert_eq!(iterator.next(), Some((5, &points[5])));
        assert_eq!(iterator.next(), None);
    }

    #[rstest]
    #[case::col_out_of_bounds(3, 1)]
    #[case::row_out_of_bounds(2, 2)]
    #[case::both_out_of_bounds(3, 2)]
    fn test_neighbor_iterator_next_out_of_bounds(#[case] col: usize, #[case] row: usize) {
        // Arrange
        let cols = 3;
        let rows = 2;
        let points = vec![[0.0; 2]; cols * rows];
        let matrix = MatrixView::new(cols, rows, &points).unwrap();

        let mut iterator = NeighborIterator::new(&matrix, col, row, 1);

        // Act & Assert
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn test_neighbors_single_cell_matrix() {
        // Arrange: 1x1 matrix
        let points = vec![[1.0; 3]];
        let matrix = MatrixView::new(1, 1, &points).unwrap();

        // Act
        let neighbors: Vec<_> = matrix.neighbors(0, 0).collect();

        // Assert: No neighbors for a 1x1 matrix
        assert!(neighbors.is_empty());
    }

    #[test]
    fn test_neighbors_with_large_radius() {
        // Arrange: 5x5 matrix with radius=10 (much larger than matrix)
        let cols = 5;
        let rows = 5;
        let points = vec![[0.0; 3]; cols * rows];
        let matrix = MatrixView::new(cols, rows, &points).unwrap();

        // Act: Center point with huge radius
        let neighbors: Vec<_> = matrix
            .neighbors_within(2, 2, 10)
            .map(|(idx, _)| idx)
            .collect();

        // Assert: Should return all points except center (index 12)
        let expected: Vec<_> = (0..25).filter(|&i| i != 12).collect();
        assert_eq!(neighbors, expected);
    }

    #[test]
    fn test_neighbor_iterator_collect() {
        // Arrange
        let cols = 3;
        let rows = 3;
        let points = vec![[0.0; 2]; cols * rows];
        let matrix = MatrixView::new(cols, rows, &points).unwrap();

        // Act: Use collect() instead of fold
        let neighbors: Vec<usize> = matrix.neighbors(1, 1).map(|(idx, _)| idx).collect();

        // Assert
        assert_eq!(neighbors, vec![0, 1, 2, 3, 5, 6, 7, 8]);
    }

    #[test]
    fn test_neighbor_iterator_with_take() {
        // Arrange
        let cols = 4;
        let rows = 4;
        let points = vec![[0.0; 2]; cols * rows];
        let matrix = MatrixView::new(cols, rows, &points).unwrap();

        // Act: Take only first 3 neighbors
        let first_three: Vec<usize> = matrix.neighbors(2, 2).take(3).map(|(idx, _)| idx).collect();

        // Assert
        assert_eq!(first_three.len(), 3);
        assert_eq!(first_three, vec![5, 6, 7]);
    }
}

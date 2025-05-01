use crate::{
    math::{matrix::MatrixView, DistanceMetric},
    FloatNumber,
};

/// Measures the gradient at a given point.
///
/// # Type Parameters
/// * `N` - The number of dimensions.
///
/// # Arguments
/// * `matrix` - The matrix of points.
/// * `col` - The column of the point.
/// * `row` - The row of the point.
/// * `metric` - The distance metric to use.
///
/// # Returns
/// The gradient at the given point. If the point is out of bounds, returns `T::max_value()`.
#[inline]
#[must_use]
pub fn gradient<T, const N: usize>(
    matrix: &MatrixView<'_, T, N>,
    col: usize,
    row: usize,
    metric: DistanceMetric,
) -> T
where
    T: FloatNumber,
{
    if col == 0 || col == matrix.cols - 1 {
        return T::max_value();
    }
    if row == 0 || row == matrix.rows - 1 {
        return T::max_value();
    }

    let dx = axis_gradient(matrix, col - 1, row, col + 1, row, metric);
    let dy = axis_gradient(matrix, col, row - 1, col, row + 1, metric);
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
/// * `metric` - The distance metric to use.
///
/// # Returns
/// The gradient of the axis between the two points. If either point is out of bounds, returns `T::max_value()`.
#[inline(always)]
#[must_use]
fn axis_gradient<T, const N: usize>(
    matrix: &MatrixView<'_, T, N>,
    col1: usize,
    row1: usize,
    col2: usize,
    row2: usize,
    metric: DistanceMetric,
) -> T
where
    T: FloatNumber,
{
    match (matrix.get(col1, row1), matrix.get(col2, row2)) {
        (Some(point1), Some(point2)) => metric.measure(point1, point2),
        _ => T::max_value(),
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::{
        assert_approx_eq,
        math::{matrix::MatrixView, Point},
    };

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
                    [T::one(), T::one(), T::one(), x, y]
                };
            }
        }
        points
    }

    #[rstest]
    #[case::left_top(0, 0, f64::MAX)]
    #[case::right_top(23, 0, f64::MAX)]
    #[case::left_bottom(0, 11, f64::MAX)]
    #[case::right_bottom(23, 11, f64::MAX)]
    #[case::edge_x(12, 3, 2.034722)]
    #[case::edge_y(18, 6, 2.034722)]
    #[case::center(12, 6, 4.034722)]
    #[case::normal(6, 3, 0.034722)]
    fn test_gradient(#[case] col: usize, #[case] row: usize, #[case] expected: f64) {
        // Arrange
        let cols = 24;
        let rows = 12;
        let points = sample_points::<f64>(cols, rows);
        let matrix = MatrixView::new(cols, rows, &points).unwrap();

        // Act
        let actual = gradient(&matrix, col, row, DistanceMetric::SquaredEuclidean);

        // Assert
        assert_approx_eq!(actual, expected);
    }

    #[rstest]
    #[case::x_axis((0, 0), (1, 0), 0.001736)]
    #[case::y_axis((23, 0), (23, 1), 0.003906)]
    #[case::x_axis_out_of_bounds((23, 0), (24,0), f64::MAX)]
    #[case::y_axis_out_of_bounds((0, 16), (0, 17), f64::MAX)]
    fn test_axis_gradient(
        #[case] (col1, row1): (usize, usize),
        #[case] (col2, row2): (usize, usize),
        #[case] expected: f64,
    ) {
        // Arrange
        let cols = 24;
        let rows = 16;
        let points = sample_points::<f64>(cols, rows);
        let matrix = MatrixView::new(cols, rows, &points).unwrap();

        // Act
        let actual = axis_gradient(
            &matrix,
            col1,
            row1,
            col2,
            row2,
            DistanceMetric::SquaredEuclidean,
        );

        // Assert
        assert_approx_eq!(actual, expected);
    }
}

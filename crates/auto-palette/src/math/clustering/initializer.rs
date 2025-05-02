use std::collections::HashSet;

use crate::{math::matrix::MatrixView, FloatNumber};

/// Seed initializer for clustering algorithms.
#[derive(Debug)]
pub enum Initializer {
    /// Seed initializer using a grid pattern.
    Grid,
}

impl Initializer {
    /// Initializes the seeds for clustering.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type.
    /// * `N` - The number of dimensions.
    ///
    /// # Arguments
    /// * `matrix` - The matrix of points.
    /// * `k` - The number of seeds to initialize.
    ///
    /// # Returns
    /// A set of indices representing the seeds for clustering.
    #[must_use]
    pub fn initialize<T, const N: usize>(
        &self,
        matrix: &MatrixView<'_, T, N>,
        k: usize,
    ) -> HashSet<usize>
    where
        T: FloatNumber,
    {
        if k == 0 {
            return HashSet::new();
        }

        if k > matrix.size() {
            return HashSet::from_iter(0..matrix.size());
        }

        match self {
            Self::Grid => self.initialize_grid(matrix, k),
        }
    }

    /// Initializes the seeds using a grid pattern.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type.
    /// * `N` - The number of dimensions.
    ///
    /// # Arguments
    /// * `matrix` - The matrix of points.
    /// * `k` - The number of seeds to initialize.
    ///
    /// # Returns
    /// A set of indices representing the seeds for clustering.
    #[must_use]
    fn initialize_grid<T, const N: usize>(
        &self,
        matrix: &MatrixView<'_, T, N>,
        k: usize,
    ) -> HashSet<usize>
    where
        T: FloatNumber,
    {
        let step = (T::from_usize(matrix.size()) / T::from_usize(k))
            .sqrt()
            .round()
            .trunc_to_usize()
            .max(1); // Ensure step is at least 1
        let offset = step / 2;

        let (cols, rows) = matrix.shape();
        let mut seeds = HashSet::with_capacity(k);
        'outer: for i in (offset..cols).step_by(step) {
            'inner: for j in (offset..rows).step_by(step) {
                let col = i.min(cols - 1);
                let row = j.min(rows - 1);
                let index = match matrix.index(col, row) {
                    Some(index) => index,
                    None => continue 'inner,
                };

                seeds.insert(index);
                if seeds.len() >= k {
                    break 'outer;
                }
            }
        }
        seeds
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::math::{matrix::MatrixView, Point};

    #[must_use]
    fn sample_points<T>(cols: usize, rows: usize) -> Vec<Point<T, 2>>
    where
        T: FloatNumber,
    {
        vec![[T::zero(); 2]; cols * rows]
    }

    #[rstest]
    #[case(1, vec![65])] // (5, 5)
    #[case(2, vec![39, 46])] // (3, 3), (10, 3)
    #[case(4, vec![26, 31, 86, 91])] // (2, 2), (7, 2), (2, 7), (7, 7)
    #[case(6, vec![26, 30, 34, 74, 78, 82])] // (2, 2), (6, 2), (10, 2), (2, 6), (6, 6), (10, 6)
    fn test_initialize_grid(#[case] k: usize, #[case] expected: Vec<usize>) {
        // Arrange
        let cols = 12;
        let rows = 9;
        let points = sample_points::<f64>(cols, rows);
        let matrix = MatrixView::new(cols, rows, &points).unwrap();

        // Act
        let actual = Initializer::Grid.initialize(&matrix, k);

        // Assert
        assert_eq!(actual.len(), expected.len());
        assert_eq!(actual, HashSet::from_iter(expected));
    }

    #[test]
    fn test_initialize_zero_seeds() {
        // Arrange
        let cols = 4;
        let rows = 9;
        let points = sample_points::<f64>(cols, rows);
        let matrix = MatrixView::new(cols, rows, &points).unwrap();

        // Act
        let actual = Initializer::Grid.initialize(&matrix, 0);

        // Assert
        assert_eq!(actual.len(), 0);
    }

    #[test]
    fn test_initialize_too_many_seeds() {
        // Arrange
        let cols = 4;
        let rows = 9;
        let points = sample_points::<f64>(cols, rows);
        let matrix = MatrixView::new(cols, rows, &points).unwrap();

        // Act
        let actual = Initializer::Grid.initialize(&matrix, cols * rows + 1);

        // Assert
        assert_eq!(actual.len(), 36);
    }
}

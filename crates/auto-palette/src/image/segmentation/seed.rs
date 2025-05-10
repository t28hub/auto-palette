use std::collections::HashSet;

use crate::{math::Point, FloatNumber};

/// SeedGenerator is an enum representing different methods for generating seed points for clustering.
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub enum SeedGenerator {
    /// Generates seeds using a regular grid pattern.
    #[default]
    RegularGrid,
}

impl SeedGenerator {
    /// Generates a set of seed indices for clustering.
    ///
    /// # Arguments
    /// * `width` - The width of the image.
    /// * `height` - The height of the image.
    /// * `pixels` - The pixels of the image.
    /// * `k` - The number of seeds to generate.
    ///
    /// # Returns
    /// A set of indices representing the seeds for clustering.
    #[must_use]
    pub fn generate<T, const N: usize>(
        &self,
        width: usize,
        height: usize,
        pixels: &[Point<T, N>],
        k: usize,
    ) -> HashSet<usize>
    where
        T: FloatNumber,
    {
        if k == 0 {
            return HashSet::new();
        }

        if k > pixels.len() {
            return HashSet::from_iter(0..pixels.len());
        }

        match self {
            Self::RegularGrid => regular_grid(width, height, pixels, k),
        }
    }
}

#[inline]
#[must_use]
fn regular_grid<T, const N: usize>(
    width: usize,
    height: usize,
    pixels: &[Point<T, N>],
    k: usize,
) -> HashSet<usize>
where
    T: FloatNumber,
{
    let step = (T::from_usize(pixels.len()) / T::from_usize(k))
        .sqrt()
        .round()
        .trunc_to_usize()
        .max(1); // Ensure step is at least 1
    let half = step / 2;
    let mut seeds = HashSet::with_capacity(k);
    'outer: for y in (half..height).step_by(step) {
        for x in (half..width).step_by(step) {
            let index = x + y * width;
            if index < pixels.len() {
                seeds.insert(index);
            }

            if seeds.len() == k {
                break 'outer;
            }
        }
    }
    seeds
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[must_use]
    fn sample_points<T>(cols: usize, rows: usize) -> Vec<Point<T, 2>>
    where
        T: FloatNumber,
    {
        vec![[T::zero(); 2]; cols * rows]
    }

    #[test]
    fn test_default() {
        // Act
        let generator = SeedGenerator::default();

        // Assert
        assert_eq!(generator, SeedGenerator::RegularGrid);
    }

    #[rstest]
    #[case(0, vec![])]
    #[case(1, vec![65])] // (5, 5)
    #[case(2, vec![39, 46])] // (3, 3), (10, 3)
    #[case(4, vec![26, 31, 86, 91])] // (2, 2), (7, 2), (2, 7), (7, 7)
    #[case(6, vec![26, 30, 34, 74, 78, 82])] // (2, 2), (6, 2), (10, 2), (2, 6), (6, 6), (10, 6)
    fn test_regular_grid_generate(#[case] k: usize, #[case] expected: Vec<usize>) {
        // Arrange
        let width = 12;
        let height = 9;
        let points = sample_points::<f64>(width, height);

        // Act
        let generator = SeedGenerator::RegularGrid;
        let actual = generator.generate(width, height, &points, k);

        // Assert
        assert_eq!(actual.len(), expected.len());
        assert_eq!(actual, HashSet::from_iter(expected));
    }

    #[test]
    fn test_generate_zero_seeds() {
        // Arrange
        let width = 4;
        let height = 3;
        let points = sample_points::<f64>(width, height);

        // Act
        let generator = SeedGenerator::default();
        let actual = generator.generate(width, height, &points, 0);

        // Assert
        assert_eq!(actual.len(), 0);
    }

    #[test]
    fn test_generate_too_many_seeds() {
        // Arrange
        let width = 4;
        let height = 3;
        let points = sample_points::<f64>(width, height);

        // Act
        let generator = SeedGenerator::default();
        let actual = generator.generate(width, height, &points, 13);

        // Assert
        assert_eq!(actual.len(), 12);
    }
}

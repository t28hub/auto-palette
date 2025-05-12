use crate::{math::Point, FloatNumber};

/// CentroidInit represents the initialization method for centroids.
#[derive(Debug, PartialEq)]
pub enum CentroidInit {
    /// Regular interval initialization.
    RegularInterval,
}

impl CentroidInit {
    /// Initializes the `k` centroids from the given points.
    ///
    /// # Arguments
    /// * `points` - The points to initialize the centroids from.
    /// * `k` - The number of centroids to initialize.
    ///
    /// # Returns
    /// A vector of initialized centroids.
    #[must_use]
    pub fn initialize<T, const N: usize>(
        &self,
        points: &[Point<T, N>],
        k: usize,
    ) -> Vec<Point<T, N>>
    where
        T: FloatNumber,
    {
        if k == 0 {
            return Vec::new();
        }

        if points.len() < k {
            return points.to_vec();
        }

        match self {
            Self::RegularInterval => regular_interval(points, k),
        }
    }
}

#[must_use]
fn regular_interval<const N: usize, T>(points: &[Point<T, N>], k: usize) -> Vec<Point<T, N>>
where
    T: FloatNumber,
{
    let step = (points.len() / k).max(1);
    let half = step / 2;
    points
        .iter()
        .skip(half)
        .step_by(step)
        .take(k)
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(0, vec![])]
    #[case(1, vec![[2.0, 2.0]])]
    #[case(2, vec![[1.0, 1.0], [3.0, 3.0]])]
    #[case(3, vec![[0.0, 0.0], [1.0, 1.0], [2.0, 2.0]])]
    #[case(4, vec![[0.0, 0.0], [1.0, 1.0], [2.0, 2.0], [3.0, 3.0]])]
    #[case(5, vec![[0.0, 0.0], [1.0, 1.0], [2.0, 2.0], [3.0, 3.0], [4.0, 4.0]])]
    #[case(6, vec![[0.0, 0.0], [1.0, 1.0], [2.0, 2.0], [3.0, 3.0], [4.0, 4.0]])]
    fn test_regular_interval(#[case] k: usize, #[case] expected: Vec<[f64; 2]>) {
        // Arrange
        let points = vec![[0.0, 0.0], [1.0, 1.0], [2.0, 2.0], [3.0, 3.0], [4.0, 4.0]];

        // Act
        let init = CentroidInit::RegularInterval;
        let actual = init.initialize(&points, k);

        // Assert
        assert_eq!(actual.len(), expected.len());
        assert_eq!(actual, expected);
    }
}

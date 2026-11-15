use super::Vector;

/// A metric for the distance between vectors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DistanceMetric {
    /// The Euclidean distance.
    #[cfg_attr(not(test), expect(dead_code, reason = "not used yet"))]
    Euclidean,
    /// The squared Euclidean distance.
    SquaredEuclidean,
}

impl DistanceMetric {
    /// Measures the distance between two vectors.
    ///
    /// # Arguments
    /// * `a` - One vector.
    /// * `b` - The other vector.
    ///
    /// # Returns
    /// The distance between `a` and `b`.
    #[inline]
    pub(crate) fn measure<const N: usize>(self, a: Vector<N>, b: Vector<N>) -> f64 {
        let mut sum = 0.0;
        for i in 0..N {
            let difference = a.0[i] - b.0[i];
            sum += difference * difference;
        }
        match self {
            Self::Euclidean => sum.sqrt(),
            Self::SquaredEuclidean => sum,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::assert_approx_eq;

    #[rstest]
    #[case::euclidean(DistanceMetric::Euclidean, 0.5)]
    #[case::squared_euclidean(DistanceMetric::SquaredEuclidean, 0.25)]
    fn test_measure(#[case] metric: DistanceMetric, #[case] expected: f64) {
        // Act
        let actual = metric.measure(Vector([0.0, 0.0]), Vector([0.3, 0.4]));

        // Assert
        assert_approx_eq!(actual, expected);
    }
}

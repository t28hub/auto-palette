use crate::math::distance::Distance;
use crate::math::number::Float;

/// Struct representing G-means clustering algorithm.
///
/// # Type Parameters
/// * `F` - The float type used for calculations.
#[derive(Debug, PartialEq)]
pub struct Gmeans<F>
where
    F: Float,
{
    max_k: usize,
    max_iter: usize,
    tolerance: F,
    distance: Distance,
}

impl<F> Gmeans<F>
where
    F: Float,
{
    /// Creates a new `Gmeans` instance.
    ///
    /// # Arguments
    /// * `max_k` - The maximum number of clusters.
    /// * `max_iter` - The maximum number of iterations.
    /// * `tolerance` - The minimum change in cluster centroids required to continue iterating.
    /// * `distance` - The distance metric to use for calculating distances between points.
    ///
    /// # Returns
    /// A new `Gmeans` instance.
    #[must_use]
    pub fn new(max_k: usize, max_iter: usize, tolerance: F, distance: Distance) -> Self {
        assert!(
            max_k >= 2,
            "The maximum number of clusters must be at least 2."
        );
        Self {
            max_k,
            max_iter,
            tolerance,
            distance,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let gmeans = Gmeans::new(2, 10, 0.01_f64, Distance::Euclidean);
        assert_eq!(gmeans.max_k, 2);
        assert_eq!(gmeans.max_iter, 10);
        assert_eq!(gmeans.tolerance, 0.01_f64);
        assert_eq!(gmeans.distance, Distance::Euclidean);
    }

    #[test]
    #[should_panic]
    fn test_new_panic() {
        let _ = Gmeans::new(1, 10, 0.01_f64, Distance::Euclidean);
    }
}

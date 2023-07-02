use crate::math::distance::Distance;
use crate::math::neighbors::kdtree::search::KDTreeSearch;
use crate::math::neighbors::search::NeighborSearch;
use crate::math::number::Float;
use crate::math::point::Point;

/// Struct for calculating the core distance of points in a dataset using HDBSCAN algorithm.
///
/// # Type Parameters
/// * `F` - The float type used for calculations (e.g., f32 or f64).
///
/// # References
/// [How HDBSCAN Works](https://hdbscan.readthedocs.io/en/latest/how_hdbscan_works.html#transform-the-space)
#[derive(Debug)]
pub struct CoreDistance<F: Float> {
    distances: Vec<F>,
}

impl<F> CoreDistance<F>
where
    F: Float,
{
    /// Creates a new `CoreDistance` instance.
    ///
    /// # Arguments
    /// * `dataset` - The dataset to be clustered.
    /// * `min_samples` - The number of samples required to form a dense region.
    /// * `distance` - The distance metric to use for calculating core distances.
    ///
    /// # Returns
    /// A new `CoreDistance` instance.
    #[must_use]
    pub fn new<P: Point<F>>(points: &[P], min_samples: usize, metric: &Distance) -> Self {
        if points.is_empty() {
            return Self::default();
        }

        let k = points.len().min(min_samples);
        let neighbor_search = KDTreeSearch::new(points, metric);
        let mut distances = Vec::with_capacity(points.len());
        for (index, point) in points.iter().enumerate() {
            let neighbors = neighbor_search.search(point, k);
            if let Some(core_neighbor) = neighbors.last() {
                distances.insert(index, core_neighbor.distance);
            } else {
                distances.insert(index, F::max_value());
            }
        }
        Self { distances }
    }

    /// Returns the core distance at the given index.
    ///
    /// # Arguments
    /// * `index` - The index of the point in the dataset.
    ///
    /// # Returns
    /// The core distance of the point at the given index.
    ///
    /// # Panics
    /// Panics if the given index is out of bounds.
    #[inline]
    #[must_use]
    pub fn distance_at(&self, index: usize) -> F {
        assert!(index < self.distances.len());
        self.distances[index]
    }
}

impl<F> Default for CoreDistance<F>
where
    F: Float,
{
    #[must_use]
    fn default() -> Self {
        Self {
            distances: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::point::Point2;
    use statrs::assert_almost_eq;

    #[test]
    fn test_core_distance() {
        let points = Vec::from([
            Point2(0.0, 0.0),
            Point2(1.1, 2.1),
            Point2(2.0, 3.0),
            Point2(1.0, 2.0),
            Point2(0.9, 1.9),
            Point2(2.5, 3.5),
        ]);

        let actual = CoreDistance::new(&points, 3, &Distance::SquaredEuclidean);
        assert_eq!(actual.distances.len(), 6);
        assert_almost_eq!(actual.distance_at(0), 5.00, 1e-5);
        assert_almost_eq!(actual.distance_at(1), 0.08, 1e-5);
        assert_almost_eq!(actual.distance_at(2), 1.62, 1e-5);
        assert_almost_eq!(actual.distance_at(3), 0.02, 1e-5);
        assert_almost_eq!(actual.distance_at(4), 0.08, 1e-5);
        assert_almost_eq!(actual.distance_at(5), 3.92, 1e-5);
    }

    #[test]
    fn test_default() {
        let actual = CoreDistance::<f64>::default();
        assert_eq!(actual.distances.len(), 0);
    }
}

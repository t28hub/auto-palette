use crate::math::distance::metric::DistanceMetric;
use crate::math::neighbors::kdtree::KDTree;
use crate::math::neighbors::nns::NeighborSearch;
use crate::math::number::Float;
use crate::math::point::Point;

/// Core distance struct.
#[derive(Debug, Clone)]
pub(crate) struct CoreDistance<F: Float> {
    distances: Vec<F>,
}

impl<F> CoreDistance<F>
where
    F: Float,
{
    /// Create a core distance for the given dataset.
    pub fn new<P: Point<F>>(dataset: &[P], min_samples: usize, metric: &DistanceMetric) -> Self {
        if dataset.is_empty() {
            return Self {
                distances: Vec::new(),
            };
        }

        let k = dataset.len().min(min_samples);
        let dataset_vec = dataset.to_vec();
        let neighbor_search = KDTree::new(&dataset_vec, metric);
        let mut distances = Vec::with_capacity(dataset.len());
        for (index, point) in dataset.iter().enumerate() {
            let neighbors = neighbor_search.search(point, k);
            if let Some(core_neighbor) = neighbors.last() {
                distances.insert(index, core_neighbor.distance);
            } else {
                distances.insert(index, F::max_value());
            }
        }
        Self { distances }
    }

    /// Returns the distance corresponding to the index.
    pub fn distance_at(&self, index: usize) -> F {
        assert!(index < self.distances.len());
        self.distances[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::point::Point2;

    #[test]
    fn new_should_create_core_distance() {
        let dataset = Vec::from([
            Point2::new(0.0, 0.0),
            Point2::new(1.1, 2.1),
            Point2::new(2.0, 3.0),
            Point2::new(1.0, 2.0),
            Point2::new(0.9, 1.9),
            Point2::new(2.5, 3.5),
        ]);
        let core_distance = CoreDistance::new(&dataset, 3, &DistanceMetric::SquaredEuclidean);
        assert_eq!(core_distance.distances.len(), 6);
        assert_eq!(core_distance.distance_at(0), 5.62);
        assert_eq!(core_distance.distance_at(1), 1.6199999999999997);
        assert_eq!(core_distance.distance_at(2), 2.0);
        assert_eq!(core_distance.distance_at(3), 2.0);
        assert_eq!(core_distance.distance_at(4), 2.4200000000000004);
        assert_eq!(core_distance.distance_at(5), 4.5);
    }
}

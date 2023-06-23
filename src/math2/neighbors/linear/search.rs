use crate::math2::distance::DistanceMetric;
use crate::math2::neighbors::neighbor::Neighbor;
use crate::math2::neighbors::search::NeighborSearch;
use crate::number::Float;
use ndarray::{ArrayView1, CowArray, Ix2};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Struct representing linear search algorithm for neighbor search.
///
/// # Type Parameters
/// * `F` - The float type used for calculations.
pub struct LinearSearch<'a, F: Float> {
    points: CowArray<'a, F, Ix2>,
    metric: &'a DistanceMetric,
}

impl<'a, F> LinearSearch<'a, F>
where
    F: Float,
{
    /// Creates a new `LinearSearch` instance.
    ///
    /// # Arguments
    /// * `points` - The reference of a dataset of points.
    /// * `metric` - The distance metric to use.
    ///
    /// # Returns
    /// A new `LinearSearch` instance.
    #[must_use]
    #[allow(unused)]
    pub fn new(points: CowArray<'a, F, Ix2>, metric: &'a DistanceMetric) -> Self {
        Self { points, metric }
    }
}

impl<'a, F> NeighborSearch<F> for LinearSearch<'a, F>
where
    F: Float,
{
    #[inline]
    #[must_use]
    fn search(&self, query: &ArrayView1<F>, k: usize) -> Vec<Neighbor<F>> {
        if k == 0 {
            return vec![];
        }

        let mut heap = BinaryHeap::with_capacity(self.points.len());
        for (index, point) in self.points.outer_iter().enumerate() {
            let distance = self.metric.measure(&point, query);
            let neighbor = Neighbor::new(index, distance);
            heap.push(Reverse(neighbor));
        }

        let mut neighbors = Vec::with_capacity(k);
        while let Some(Reverse(neighbor)) = heap.pop() {
            neighbors.push(neighbor);
            if neighbors.len() == k {
                break;
            }
        }
        neighbors
    }

    #[inline]
    #[must_use]
    fn search_nearest(&self, query: &ArrayView1<F>) -> Option<Neighbor<F>> {
        self.search(query, 1).pop()
    }

    #[inline]
    #[must_use]
    fn search_radius(&self, query: &ArrayView1<F>, radius: F) -> Vec<Neighbor<F>> {
        if radius.is_zero() {
            return vec![];
        }

        self.points
            .outer_iter()
            .enumerate()
            .filter_map(|(index, point)| {
                let distance = self.metric.measure(&point, query);
                if distance <= radius {
                    let neighbor = Neighbor::new(index, distance);
                    Some(neighbor)
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::{array, aview1, Array2};

    #[must_use]
    fn sample_points() -> Array2<f64> {
        array![[1.0, 2.0], [3.0, 1.0], [4.0, 5.0], [5.0, 5.0], [2.0, 4.0],]
    }

    #[test]
    fn test_linear_search() {
        let points = sample_points();
        let search = LinearSearch::new(points.into(), &DistanceMetric::Euclidean);
        assert_eq!(search.points.nrows(), 5);
        assert_eq!(search.metric, &DistanceMetric::Euclidean)
    }

    #[test]
    fn test_search() {
        let points = sample_points();
        let search = LinearSearch::new(points.into(), &DistanceMetric::Euclidean);

        let actual = search.search(&aview1(&[3.0, 2.0]), 0);
        assert_eq!(actual.len(), 0);

        let actual = search.search(&aview1(&[3.0, 2.0]), 1);
        assert_eq!(actual.len(), 1);
        assert_eq!(actual[0], Neighbor::new(1, 1.0));

        let actual = search.search(&aview1(&[3.0, 2.0]), 2);
        assert_eq!(actual.len(), 2);
        assert_eq!(actual[0], Neighbor::new(1, 1.0));
        assert_eq!(actual[1], Neighbor::new(0, 2.0));

        let actual = search.search(&aview1(&[3.0, 2.0]), 16);
        assert_eq!(actual.len(), 5);
    }
}

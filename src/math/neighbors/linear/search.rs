use crate::math::distance::Distance;
use crate::math::neighbors::neighbor::Neighbor;
use crate::math::neighbors::search::NeighborSearch;
use crate::math::number::Float;
use crate::math::point::Point;
use std::cmp::Ordering;
use std::marker::PhantomData;

/// Struct representing linear search algorithm for neighbor search.
///
/// # Type Parameters
/// * `F` - The float type used for calculations.
/// * `P` - The type of points used in the neighbor search algorithm.
#[derive(Debug)]
pub struct LinearSearch<'a, F, P>
where
    F: Float,
    P: Point<F>,
{
    dataset: &'a Vec<P>,
    distance: Distance,
    _marker: PhantomData<F>,
}

impl<'a, F, P> LinearSearch<'a, F, P>
where
    F: Float,
    P: Point<F>,
{
    /// Creates a new `LinearSearch` instance.
    ///
    /// # Arguments
    /// * `dataset` - The reference of a dataset of points.
    /// * `distance` - The distance metric to use.
    ///
    /// # Returns
    /// A new `LinearSearch` instance.
    #[must_use]
    pub fn new(dataset: &'a Vec<P>, distance: Distance) -> Self {
        Self {
            dataset,
            distance,
            _marker: PhantomData::default(),
        }
    }
}

impl<'a, F, P> NeighborSearch<F, P> for LinearSearch<'a, F, P>
where
    F: Float,
    P: Point<F>,
{
    #[must_use]
    fn search(&self, query: &P, k: usize) -> Vec<Neighbor<F>> {
        if k == 0 {
            return Vec::new();
        }

        let mut neighbors: Vec<Neighbor<F>> = self
            .dataset
            .iter()
            .enumerate()
            .map(|(index, point)| {
                let distance = self.distance.measure(point, query);
                Neighbor::new(index, distance)
            })
            .collect();

        neighbors.sort_unstable_by(|neighbor1, neighbor2| {
            neighbor1
                .distance
                .partial_cmp(&neighbor2.distance)
                .unwrap_or(Ordering::Equal)
        });
        neighbors.truncate(k);
        neighbors
    }

    #[must_use]
    fn search_nearest(&self, query: &P) -> Option<Neighbor<F>> {
        self.search(query, 1).pop()
    }

    #[must_use]
    fn search_radius(&self, query: &P, radius: F) -> Vec<Neighbor<F>> {
        if radius < F::zero() {
            return Vec::new();
        }

        self.dataset
            .iter()
            .enumerate()
            .filter_map(|(index, point)| {
                let distance = self.distance.measure(point, query);
                if distance <= radius {
                    Some(Neighbor::new(index, distance))
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
    use crate::math::point::Point2;

    const DATASET: [Point2<f64>; 5] = [
        Point2(1.0, 2.0),
        Point2(3.0, 1.0),
        Point2(4.0, 5.0),
        Point2(5.0, 5.0),
        Point2(2.0, 4.0),
    ];

    #[test]
    fn search_should_return_knearest_neighbors() {
        let dataset = Vec::new();
        let linear_search = LinearSearch::new(&dataset, Distance::SquaredEuclidean);
        assert_eq!(linear_search.search(&Point2(3.0, 3.0), 4), vec![]);

        let dataset = Vec::from(DATASET);
        let linear_search = LinearSearch::new(&dataset, Distance::SquaredEuclidean);
        assert_eq!(linear_search.search(&Point2(3.0, 3.0), 0), vec![]);
        assert_eq!(
            linear_search.search(&Point2(3.0, 3.0), 3),
            vec![
                Neighbor::new(4, 2.0),
                Neighbor::new(1, 4.0),
                Neighbor::new(0, 5.0),
            ]
        );
        assert_eq!(
            linear_search.search(&Point2(3.0, 3.0), 5),
            vec![
                Neighbor::new(4, 2.0),
                Neighbor::new(1, 4.0),
                Neighbor::new(0, 5.0),
                Neighbor::new(2, 5.0),
                Neighbor::new(3, 8.0),
            ]
        );
        assert_eq!(
            linear_search.search(&Point2(3.0, 3.0), 6),
            vec![
                Neighbor::new(4, 2.0),
                Neighbor::new(1, 4.0),
                Neighbor::new(0, 5.0),
                Neighbor::new(2, 5.0),
                Neighbor::new(3, 8.0),
            ]
        );
    }

    #[test]
    fn search_nearest_should_return_nearest_neighbor() {
        let dataset = Vec::new();
        let linear_search = LinearSearch::new(&dataset, Distance::SquaredEuclidean);
        assert_eq!(linear_search.search_nearest(&Point2(0.0, 1.0)), None);

        let dataset = Vec::from(DATASET);
        let linear_search = LinearSearch::new(&dataset, Distance::SquaredEuclidean);
        assert_eq!(
            linear_search.search_nearest(&Point2(2.5, 3.0)),
            Some(Neighbor::new(4, 1.25))
        );
    }

    #[test]
    fn search_radius_should_return_neighbors_within_radius() {
        let dataset = Vec::from(DATASET);
        let linear_search = LinearSearch::new(&dataset, Distance::SquaredEuclidean);
        assert_eq!(linear_search.search_radius(&Point2(2.0, 3.0), -1.0), vec![]);
        assert_eq!(linear_search.search_radius(&Point2(2.0, 3.0), 0.0), vec![]);
        assert_eq!(
            linear_search.search_radius(&Point2(2.0, 3.0), 1.0),
            vec![Neighbor::new(4, 1.0)]
        );
        assert_eq!(
            linear_search.search_radius(&Point2(2.0, 3.0), 1.5),
            vec![Neighbor::new(4, 1.0)]
        );
        assert_eq!(
            linear_search.search_radius(&Point2(2.0, 3.0), 10.0),
            vec![
                Neighbor::new(0, 2.0),
                Neighbor::new(1, 5.0),
                Neighbor::new(2, 8.0),
                Neighbor::new(4, 1.0),
            ]
        );
        assert_eq!(
            linear_search.search_radius(&Point2(2.0, 3.0), 15.0),
            vec![
                Neighbor::new(0, 2.0),
                Neighbor::new(1, 5.0),
                Neighbor::new(2, 8.0),
                Neighbor::new(3, 13.0),
                Neighbor::new(4, 1.0),
            ]
        );
    }
}

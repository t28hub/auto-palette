use crate::math::distance::DistanceMetric;
use crate::math::neighbors::neighbor::Neighbor;
use crate::math::neighbors::search::NeighborSearch;
use crate::math::number::Float;
use crate::math::point::Point;
use std::borrow::Cow;
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
    points: Cow<'a, [P]>,
    metric: &'a DistanceMetric,
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
    /// * `points` - The reference of a dataset of points.
    /// * `metric` - The distance metric to use.
    ///
    /// # Returns
    /// A new `LinearSearch` instance.
    #[allow(unused)]
    #[must_use]
    pub fn new(points: &'a Vec<P>, metric: &'a DistanceMetric) -> Self {
        Self {
            points: Cow::Borrowed(points),
            metric,
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
            .points
            .iter()
            .enumerate()
            .map(|(index, point)| {
                let distance = self.metric.measure(point, query);
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

        let mut neighbors: Vec<_> = self
            .points
            .iter()
            .enumerate()
            .filter_map(|(index, point)| {
                let distance = self.metric.measure(point, query);
                if distance <= radius {
                    Some(Neighbor::new(index, distance))
                } else {
                    None
                }
            })
            .collect();

        neighbors.sort_unstable_by(|neighbor1, neighbor2| {
            neighbor1
                .distance
                .partial_cmp(&neighbor2.distance)
                .unwrap_or(Ordering::Equal)
        });
        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::point::Point2;

    #[must_use]
    fn sample_points() -> Vec<Point2<f64>> {
        vec![
            Point2(1.0, 2.0),
            Point2(3.0, 1.0),
            Point2(4.0, 5.0),
            Point2(5.0, 5.0),
            Point2(2.0, 4.0),
            Point2(0.0, 5.0),
            Point2(2.0, 1.0),
            Point2(5.0, 2.0),
        ]
    }

    #[test]
    fn test_linear_search() {
        let points = sample_points();
        let linear = LinearSearch::new(&points, &DistanceMetric::SquaredEuclidean);
        assert_eq!(linear.points.as_ref(), &points);
        assert_eq!(linear.metric, &DistanceMetric::SquaredEuclidean);
        assert_eq!(linear._marker, PhantomData::default());
    }

    #[test]
    fn test_search() {
        let points = sample_points();
        let linear = LinearSearch::new(&points, &DistanceMetric::SquaredEuclidean);
        assert_eq!(linear.search(&Point2(3.0, 3.0), 0), vec![]);
        assert_eq!(
            linear.search(&Point2(3.0, 3.0), 3),
            vec![
                Neighbor::new(4, 2.0),
                Neighbor::new(1, 4.0),
                Neighbor::new(0, 5.0),
            ]
        );
        assert_eq!(
            linear.search(&Point2(3.0, 3.0), 5),
            vec![
                Neighbor::new(4, 2.0),
                Neighbor::new(1, 4.0),
                Neighbor::new(0, 5.0),
                Neighbor::new(2, 5.0),
                Neighbor::new(6, 5.0),
            ]
        );
        assert_eq!(linear.search(&Point2(3.0, 3.0), 10).len(), 8,);
    }

    #[test]
    fn test_search_empty() {
        let points: Vec<Point2<f64>> = vec![];
        let linear = LinearSearch::new(&points, &DistanceMetric::SquaredEuclidean);
        assert_eq!(linear.search(&Point2(3.0, 3.0), 0), vec![]);
        assert_eq!(linear.search(&Point2(3.0, 3.0), 3), vec![]);
        assert_eq!(linear.search(&Point2(3.0, 3.0), 5), vec![]);
        assert_eq!(linear.search(&Point2(3.0, 3.0), 6), vec![]);
    }

    #[test]
    fn test_search_nearest() {
        let points = sample_points();
        let linear = LinearSearch::new(&points, &DistanceMetric::SquaredEuclidean);
        assert_eq!(
            linear.search_nearest(&Point2(2.5, 3.0)),
            Some(Neighbor::new(4, 1.25))
        );
    }

    #[test]
    fn test_search_nearest_empty() {
        let points: Vec<Point2<f64>> = vec![];
        let linear = LinearSearch::new(&points, &DistanceMetric::SquaredEuclidean);
        assert_eq!(linear.search_nearest(&Point2(3.0, 3.0)), None);
    }

    #[test]
    fn test_search_radius() {
        let points = sample_points();
        let linear = LinearSearch::new(&points, &DistanceMetric::SquaredEuclidean);
        assert_eq!(linear.search_radius(&Point2(2.0, 3.0), -1.0), vec![]);
        assert_eq!(linear.search_radius(&Point2(2.0, 3.0), 0.0), vec![]);
        assert_eq!(
            linear.search_radius(&Point2(2.0, 3.0), 1.0),
            vec![Neighbor::new(4, 1.0)]
        );
        assert_eq!(
            linear.search_radius(&Point2(2.0, 3.0), 2.0),
            vec![Neighbor::new(4, 1.0), Neighbor::new(0, 2.0)]
        );
        assert_eq!(
            linear.search_radius(&Point2(2.0, 3.0), 10.0),
            vec![
                Neighbor::new(4, 1.0),
                Neighbor::new(0, 2.0),
                Neighbor::new(6, 4.0),
                Neighbor::new(1, 5.0),
                Neighbor::new(2, 8.0),
                Neighbor::new(5, 8.0),
                Neighbor::new(7, 10.0),
            ]
        );
        assert_eq!(linear.search_radius(&Point2(2.0, 3.0), 64.0).len(), 8);
    }

    #[test]
    fn test_search_radius_empty() {
        let points: Vec<Point2<f64>> = vec![];
        let linear = LinearSearch::new(&points, &DistanceMetric::SquaredEuclidean);
        assert_eq!(linear.search_radius(&Point2(2.0, 3.0), -1.0), vec![]);
        assert_eq!(linear.search_radius(&Point2(2.0, 3.0), 0.0), vec![]);
        assert_eq!(linear.search_radius(&Point2(2.0, 3.0), 1.0), vec![]);
        assert_eq!(linear.search_radius(&Point2(2.0, 3.0), 1.5), vec![]);
        assert_eq!(linear.search_radius(&Point2(2.0, 3.0), 10.0), vec![]);
        assert_eq!(linear.search_radius(&Point2(2.0, 3.0), 15.0), vec![]);
    }
}

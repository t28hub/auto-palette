use std::collections::BinaryHeap;

use crate::math::{
    metrics::DistanceMetric,
    neighbors::{neighbor::Neighbor, search::NeighborSearch},
    point::Point,
    FloatNumber,
};

/// Linear search algorithm for finding nearest neighbors.
///
/// # Type Parameters
/// * `T` - The floating point type.
/// * `N` - The dimension of the points.
#[derive(Debug)]
pub struct LinearSearch<'a, T, const N: usize>
where
    T: FloatNumber,
{
    points: &'a [Point<T, N>],
    metric: DistanceMetric,
}

impl<'a, T, const N: usize> LinearSearch<'a, T, N>
where
    T: FloatNumber,
{
    /// Builds a new linear search algorithm.
    ///
    /// # Arguments
    /// * `points` - The points to search.
    /// * `metric` - The distance metric to use.
    ///
    /// # Returns
    /// A new linear search algorithm.
    pub fn build(points: &'a [Point<T, N>], metric: DistanceMetric) -> Self {
        Self { points, metric }
    }
}

impl<T, const N: usize> NeighborSearch<T, N> for LinearSearch<'_, T, N>
where
    T: FloatNumber,
{
    #[must_use]
    fn search(&self, query: &Point<T, N>, k: usize) -> Vec<Neighbor<T>> {
        let mut neighbors = BinaryHeap::with_capacity(k);
        for (index, point) in self.points.iter().enumerate() {
            let distance = self.metric.measure(query, point);
            let neighbor = Neighbor::new(index, distance);
            neighbors.push(neighbor);
            if neighbors.len() > k {
                neighbors.pop();
            }
        }
        neighbors.into_sorted_vec()
    }

    #[must_use]
    fn search_nearest(&self, query: &Point<T, N>) -> Option<Neighbor<T>> {
        let mut nearest = Neighbor::new(0, T::infinity());
        for (index, other) in self.points.iter().enumerate() {
            let distance = self.metric.measure(query, other);
            if distance < nearest.distance {
                nearest.index = index;
                nearest.distance = distance;
            }
        }
        Some(nearest)
    }

    #[must_use]
    fn search_radius(&self, query: &Point<T, N>, radius: T) -> Vec<Neighbor<T>> {
        let mut neighbors = Vec::new();
        for (index, point) in self.points.iter().enumerate() {
            let distance = self.metric.measure(query, point);
            if distance <= radius {
                let neighbor = Neighbor::new(index, distance);
                neighbors.push(neighbor);
            }
        }
        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[must_use]
    fn sample_points() -> Vec<[f32; 3]> {
        vec![
            [1.0, 2.0, 3.0], // 0
            [5.0, 1.0, 2.0], // 1
            [9.0, 3.0, 4.0], // 2
            [3.0, 9.0, 1.0], // 3
            [4.0, 8.0, 3.0], // 4
            [9.0, 1.0, 1.0], // 5
            [5.0, 0.0, 0.0], // 6
            [1.0, 1.0, 1.0], // 7
            [7.0, 2.0, 2.0], // 8
            [5.0, 9.0, 1.0], // 9
            [1.0, 1.0, 9.0], // 10
            [9.0, 8.0, 7.0], // 11
            [2.0, 3.0, 4.0], // 12
            [4.0, 5.0, 4.0], // 13
        ]
    }

    #[test]
    fn test_build() {
        // Act
        let points = sample_points();
        let search = LinearSearch::build(&points, DistanceMetric::Euclidean);

        // Assert
        assert_eq!(search.points.len(), 14);
        assert_eq!(search.metric, DistanceMetric::Euclidean);
    }

    #[test]
    fn test_search() {
        // Arrange
        let points = sample_points();
        let search = LinearSearch::build(&points, DistanceMetric::Euclidean);

        // Act
        let query = [2.0, 5.0, 6.0];
        let neighbors = search.search(&query, 3);

        // Assert
        assert_eq!(neighbors.len(), 3);
        assert_eq!(neighbors[0].index, 12);
        assert_eq!(neighbors[0].distance, 8.0_f32.sqrt());
        assert_eq!(neighbors[1].index, 13);
        assert_eq!(neighbors[1].distance, 8.0_f32.sqrt());
        assert_eq!(neighbors[2].index, 0);
        assert_eq!(neighbors[2].distance, 19.0_f32.sqrt());
    }

    #[test]
    fn test_search_nearest() {
        // Arrange
        let points = sample_points();
        let search = LinearSearch::build(&points, DistanceMetric::Euclidean);

        // Act
        let query = [2.0, 5.0, 6.0];
        let nearest = search.search_nearest(&query).unwrap();

        // Assert
        assert_eq!(nearest.index, 12);
        assert_eq!(nearest.distance, 8.0_f32.sqrt());
    }

    #[test]
    fn test_search_radius() {
        // Arrange
        let points = sample_points();
        let search = LinearSearch::build(&points, DistanceMetric::Euclidean);

        // Act
        let query = [2.0, 5.0, 6.0];
        let neighbors = search.search_radius(&query, 4.0);

        // Assert
        assert_eq!(neighbors.len(), 2);
        assert_eq!(neighbors[0].index, 12);
        assert_eq!(neighbors[0].distance, 8.0_f32.sqrt());
        assert_eq!(neighbors[1].index, 13);
        assert_eq!(neighbors[1].distance, 8.0_f32.sqrt());
    }
}

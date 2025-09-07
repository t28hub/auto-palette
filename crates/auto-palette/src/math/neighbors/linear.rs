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
/// * `T` - The floating point type used for distances (e.g., `f32`, `f64`).
/// * `N` - The dimension of the points.
#[derive(Debug)]
pub struct LinearSearch<'a, T, const N: usize>
where
    T: FloatNumber,
{
    /// Reference to the points to search.
    points: &'a [Point<T, N>],

    /// The distance metric used for measuring distances.
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
    #[allow(dead_code)]
    pub fn build(points: &'a [Point<T, N>], metric: DistanceMetric) -> Self {
        Self { points, metric }
    }
}

impl<T, const N: usize> NeighborSearch<T, N> for LinearSearch<'_, T, N>
where
    T: FloatNumber,
{
    fn search(&self, query: &Point<T, N>, k: usize) -> Vec<Neighbor<T>> {
        if k == 0 || self.points.is_empty() {
            return Vec::new();
        }

        self.points
            .iter()
            .enumerate()
            .fold(
                BinaryHeap::with_capacity(k),
                |mut neighbors, (index, point)| {
                    let distance = self.metric.measure(query, point);
                    let neighbor = Neighbor::new(index, distance);
                    neighbors.push(neighbor);
                    if neighbors.len() > k {
                        neighbors.pop();
                    }
                    neighbors
                },
            )
            .into_sorted_vec()
    }

    fn search_nearest(&self, query: &Point<T, N>) -> Option<Neighbor<T>> {
        if self.points.is_empty() {
            return None;
        }

        self.points
            .iter()
            .enumerate()
            .fold(None, |nearest, (index, point)| {
                let distance = self.metric.measure(query, point);
                if let Some(best) = nearest {
                    if distance < best.distance() {
                        Some(Neighbor::new(index, distance))
                    } else {
                        Some(best)
                    }
                } else {
                    Some(Neighbor::new(index, distance))
                }
            })
    }

    fn search_within_radius(&self, query: &Point<T, N>, radius: T) -> Vec<Neighbor<T>> {
        if radius < T::zero() || self.points.is_empty() {
            return Vec::new();
        }

        self.points.iter().enumerate().fold(
            Vec::with_capacity(self.points.len()),
            |mut neighbors, (index, point)| {
                let distance = self.metric.measure(query, point);
                if distance <= radius {
                    let neighbor = Neighbor::new(index, distance);
                    neighbors.push(neighbor);
                }
                neighbors
            },
        )
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

    #[must_use]
    fn empty_points() -> Vec<Point<f32, 3>> {
        Vec::new()
    }

    #[test]
    fn test_build() {
        // Act
        let points = sample_points();
        let actual = LinearSearch::build(&points, DistanceMetric::Euclidean);

        // Assert
        assert_eq!(actual.points.len(), 14);
        assert_eq!(actual.metric, DistanceMetric::Euclidean);
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
        assert_eq!(neighbors[0], Neighbor::new(12, 8.0_f32.sqrt()));
        assert_eq!(neighbors[1], Neighbor::new(13, 8.0_f32.sqrt()));
        assert_eq!(neighbors[2], Neighbor::new(0, 19.0_f32.sqrt()));
    }

    #[test]
    fn test_search_zero() {
        // Arrange
        let points = sample_points();
        let search = LinearSearch::build(&points, DistanceMetric::Euclidean);

        // Act
        let query = [2.0, 5.0, 6.0];
        let actual = search.search(&query, 0);

        // Assert
        assert!(actual.is_empty());
    }

    #[test]
    fn test_search_k_greater_than_points() {
        // Arrange
        let points = sample_points();
        let search = LinearSearch::build(&points, DistanceMetric::Euclidean);

        // Act
        let query = [2.0, 5.0, 6.0];
        let actual = search.search(&query, 20);

        // Assert
        assert_eq!(actual.len(), points.len());
    }

    #[test]
    fn test_search_empty() {
        // Arrange
        let points = empty_points();
        let search = LinearSearch::build(&points, DistanceMetric::Euclidean);

        // Act
        let query = [2.0, 5.0, 6.0];
        let actual = search.search(&query, 3);

        // Assert
        assert!(actual.is_empty());
    }

    #[test]
    fn test_search_nearest() {
        // Arrange
        let points = sample_points();
        let search = LinearSearch::build(&points, DistanceMetric::Euclidean);

        // Act
        let query = [2.0, 5.0, 6.0];
        let actual = search.search_nearest(&query);

        // Assert
        assert!(actual.is_some());
        assert_eq!(actual.unwrap(), Neighbor::new(12, 8.0_f32.sqrt()));
    }

    #[test]
    fn test_search_nearest_empty() {
        // Arrange
        let points = empty_points();
        let search = LinearSearch::build(&points, DistanceMetric::Euclidean);

        // Act
        let query = [2.0, 5.0, 6.0];
        let actual = search.search_nearest(&query);

        // Assert
        assert!(actual.is_none());
    }

    #[test]
    fn test_search_within_radius() {
        // Arrange
        let points = sample_points();
        let search = LinearSearch::build(&points, DistanceMetric::Euclidean);

        // Act
        let query = [2.0, 5.0, 6.0];
        let actual = search.search_within_radius(&query, 4.0);

        // Assert
        assert_eq!(actual.len(), 2);
        assert_eq!(actual[0], Neighbor::new(12, 8.0_f32.sqrt()));
        assert_eq!(actual[1], Neighbor::new(13, 8.0_f32.sqrt()));
    }

    #[test]
    fn test_search_within_radius_zero() {
        // Arrange
        let points = sample_points();
        let search = LinearSearch::build(&points, DistanceMetric::Euclidean);

        // Act
        let query = [2.0, 5.0, 6.0];
        let actual = search.search_within_radius(&query, 0.0);

        // Assert
        assert!(actual.is_empty());
    }

    #[test]
    fn test_search_within_radius_empty() {
        // Arrange
        let points = empty_points();
        let search = LinearSearch::build(&points, DistanceMetric::Euclidean);

        // Act
        let query = [2.0, 5.0, 6.0];
        let actual = search.search_within_radius(&query, 4.0);

        // Assert
        assert!(actual.is_empty());
    }
}

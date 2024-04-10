use crate::math::sampling::strategy::SamplingStrategy;
use crate::math::{DistanceMetric, Point};
use std::collections::HashSet;

/// Farthest point sampling algorithm.
///
/// # Type Parameters
/// * `N` - The number of dimensions.
#[derive(Debug, PartialEq)]
pub struct FarthestPointSampling<const N: usize> {
    metric: DistanceMetric,
}

impl<const N: usize> FarthestPointSampling<N> {
    /// Creates a new `FarthestPointSampling` instance.
    ///
    /// # Arguments
    /// * `metric` - The distance metric to use.
    ///
    /// # Returns
    /// A new `FarthestPointSampling` instance.
    #[must_use]
    pub fn new(metric: DistanceMetric) -> Self {
        Self { metric }
    }

    #[inline]
    fn update_distances(
        &self,
        points: &[Point<N>],
        distances: &mut [f32],
        selected: &HashSet<usize>,
        farthest_point: &Point<N>,
    ) {
        for (index, point) in points.iter().enumerate() {
            if selected.contains(&index) {
                distances[index] = 0.0;
                continue;
            }

            let distance = self.metric.measure(point, farthest_point);
            distances[index] = distances[index].min(distance);
        }
    }
}

impl<const N: usize> SamplingStrategy<N> for FarthestPointSampling<N> {
    #[must_use]
    fn sample(&self, points: &[Point<N>], n: usize) -> HashSet<usize> {
        if n == 0 || points.is_empty() {
            return HashSet::new();
        }

        if points.len() <= n {
            return (0..points.len()).collect();
        }

        let mut selected = HashSet::with_capacity(n);
        let initial_index = 0;
        selected.insert(initial_index);

        let mut distances = vec![f32::INFINITY; points.len()];
        let initial_point = &points[initial_index];
        self.update_distances(points, &mut distances, &selected, initial_point);

        while selected.len() < n {
            let farthest_index = find_farthest_index(&distances, &selected);
            selected.insert(farthest_index);

            let farthest_point = &points[farthest_index];
            self.update_distances(points, &mut distances, &selected, farthest_point);
        }
        selected
    }
}

#[inline]
#[must_use]
fn find_farthest_index(distances: &[f32], selected: &HashSet<usize>) -> usize {
    let mut farthest_index = 0;
    let mut farthest_distance = 0.0;
    for (index, &distance) in distances.iter().enumerate() {
        if selected.contains(&index) {
            continue;
        }

        if distance > farthest_distance {
            farthest_index = index;
            farthest_distance = distance;
        }
    }
    farthest_index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[must_use]
    fn sample_points() -> Vec<Point<3>> {
        vec![
            [0.5, 0.5, 0.5],
            [0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 2.0, 0.0],
            [2.0, 2.0, 0.0],
        ]
    }

    #[must_use]
    fn empty_points() -> Vec<Point<3>> {
        vec![]
    }

    #[test]
    fn test_new() {
        // Act
        let sampling: FarthestPointSampling<3> =
            FarthestPointSampling::new(DistanceMetric::Euclidean);

        // Assert
        assert_eq!(
            sampling,
            FarthestPointSampling {
                metric: DistanceMetric::Euclidean
            }
        );
    }

    #[test]
    fn test_sample() {
        // Arrange
        let points = sample_points();
        let sampling = FarthestPointSampling::new(DistanceMetric::Euclidean);

        // Act & Assert
        let selected = sampling.sample(&points, 0);
        assert!(selected.is_empty());

        let selected = sampling.sample(&points, 1);
        assert_eq!(selected.len(), 1);
        assert!(selected.contains(&0));

        let selected = sampling.sample(&points, 2);
        assert_eq!(selected.len(), 2);
        assert!(selected.contains(&0));
        assert!(selected.contains(&4));

        let selected = sampling.sample(&points, 3);
        assert_eq!(selected.len(), 3);
        assert!(selected.contains(&0));
        assert!(selected.contains(&3));
        assert!(selected.contains(&4));

        let selected = sampling.sample(&points, 5);
        assert_eq!(selected.len(), 5);
        assert!(selected.contains(&0));
        assert!(selected.contains(&1));
        assert!(selected.contains(&2));
        assert!(selected.contains(&3));
        assert!(selected.contains(&4));
    }

    #[test]
    fn test_sample_empty() {
        // Arrange
        let points = empty_points();
        let sampling = FarthestPointSampling::new(DistanceMetric::Euclidean);

        // Act & Assert
        let selected = sampling.sample(&points, 0);
        assert!(selected.is_empty());

        let selected = sampling.sample(&points, 3);
        assert!(selected.is_empty());
    }
}

use crate::math::{DistanceMetric, Point};
use std::collections::HashSet;

/// Strategy for sampling points from a set of points.
#[derive(Debug)]
pub enum SamplingStrategy {
    /// Farthest point sampling strategy.
    /// The distance between two points is measured using the given distance metric.
    ///
    /// # Arguments
    /// * `DistanceMetric` - The distance metric used to measure the distance between points.
    #[allow(dead_code)]
    FarthestPointSampling(DistanceMetric),
    /// Weighted farthest point sampling strategy.
    /// The distance between two points is multiplied by the weight of the first point.
    ///
    /// # Arguments
    /// * `DistanceMetric` - The distance metric used to measure the distance between points.
    /// * `Vec<f32>` - The weights of the points.
    WeightedFarthestPointSampling(DistanceMetric, Vec<f32>),
}

impl SamplingStrategy {
    /// Samples points from the given set of points.
    ///
    /// # Type Parameters
    /// * `N` - The number of dimensions of the points.
    ///
    /// # Arguments
    /// * `points` - The set of points to sample from.
    /// * `n` - The number of points to sample.
    ///
    /// # Returns
    /// The indices of the sampled points.
    pub fn sample<const N: usize>(&self, points: &[Point<N>], n: usize) -> HashSet<usize> {
        match self {
            SamplingStrategy::FarthestPointSampling(metric) => {
                sample_with_distance_fn(points, n, |_, point1, point2| {
                    metric.measure(point1, point2)
                })
            }
            SamplingStrategy::WeightedFarthestPointSampling(metric, weights) => {
                debug_assert_eq!(
                    points.len(),
                    weights.len(),
                    "The number of points and weights must be equal."
                );
                sample_with_distance_fn(points, n, |index, point1, point2| {
                    metric.measure(point1, point2) * weights[index]
                })
            }
        }
    }
}

#[must_use]
fn sample_with_distance_fn<F, const N: usize>(
    points: &[Point<N>],
    n: usize,
    distance_fn: F,
) -> HashSet<usize>
where
    F: Fn(usize, &Point<N>, &Point<N>) -> f32,
{
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
    update_distances(
        points,
        &mut distances,
        &selected,
        initial_point,
        &distance_fn,
    );

    while selected.len() < n {
        let farthest_index = find_farthest_index(&distances, &selected);
        if !selected.insert(farthest_index) {
            break;
        }

        let farthest_point = &points[farthest_index];
        update_distances(
            points,
            &mut distances,
            &selected,
            farthest_point,
            &distance_fn,
        );
    }
    selected
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

#[inline]
fn update_distances<const N: usize, F>(
    points: &[Point<N>],
    distances: &mut [f32],
    selected: &HashSet<usize>,
    farthest_point: &Point<N>,
    distance_fn: &F,
) where
    F: Fn(usize, &Point<N>, &Point<N>) -> f32,
{
    for (index, point) in points.iter().enumerate() {
        if selected.contains(&index) {
            distances[index] = 0.0;
            continue;
        }

        let distance = distance_fn(index, point, farthest_point);
        distances[index] = distances[index].min(distance);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[must_use]
    fn sample_points() -> Vec<Point<2>> {
        vec![
            [0.0, 0.0], // 0
            [0.1, 0.1], // 1
            [0.1, 0.2], // 2
            [0.2, 0.2], // 3
            [0.2, 0.4], // 4
            [0.3, 0.5], // 5
            [0.1, 0.0], // 6
            [0.0, 0.1], // 7
            [0.0, 0.2], //8
        ]
    }

    #[must_use]
    fn empty_points() -> Vec<Point<2>> {
        vec![]
    }

    #[rstest]
    #[case(0, vec![])]
    #[case(1, vec![0])]
    #[case(3, vec![0, 3, 5])]
    #[case(5, vec![0, 1, 3, 5, 8])]
    #[case(9, vec![0, 1, 2, 3, 4, 5, 6, 7, 8])]
    #[case(10, vec![0, 1, 2, 3, 4, 5, 6, 7, 8])]
    fn test_sample_farthest_point_sampling(#[case] n: usize, #[case] expected: Vec<usize>) {
        // Act
        let sampling = SamplingStrategy::FarthestPointSampling(DistanceMetric::Euclidean);
        let points = sample_points();
        let sampled = sampling.sample(&points, n);

        // Assert
        assert_eq!(sampled, expected.into_iter().collect());
    }

    #[test]
    fn test_sample_farthest_point_sampling_empty() {
        // Act
        let sampling = SamplingStrategy::FarthestPointSampling(DistanceMetric::Euclidean);
        let points = empty_points();
        let sampled = sampling.sample(&points, 2);

        // Assert
        assert!(sampled.is_empty());
    }

    #[rstest]
    #[case(0, vec![])]
    #[case(1, vec![0])]
    #[case(3, vec![0, 5, 8])]
    #[case(5, vec![0, 5, 6, 7,8])]
    #[case(9, vec![0, 1, 2, 3, 4, 5, 6, 7, 8])]
    #[case(10, vec![0, 1, 2, 3, 4, 5, 6, 7, 8])]
    fn test_sample_weighted_farthest_point_sampling(
        #[case] n: usize,
        #[case] expected: Vec<usize>,
    ) {
        // Act
        let weights = vec![1.0, 1.0, 2.0, 3.0, 5.0, 8.0, 13.0, 21.0, 34.0];
        let sampling =
            SamplingStrategy::WeightedFarthestPointSampling(DistanceMetric::Euclidean, weights);
        let points = sample_points();
        let sampled = sampling.sample(&points, n);

        // Assert
        assert_eq!(sampled, expected.into_iter().collect());
    }
}

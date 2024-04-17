use crate::math::{DistanceMetric, FloatNumber, Point};
use std::collections::HashSet;

/// Strategy for sampling points from a set of points.
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug, PartialEq)]
pub enum SamplingStrategy<T>
where
    T: FloatNumber,
{
    /// Farthest point sampling strategy.
    /// The distance between two points is measured using the given distance metric.
    FarthestPointSampling,
    /// Weighted farthest point sampling strategy.
    /// The distance between two points is multiplied by the weight of the first point.
    ///
    /// # Arguments
    /// * `Vec<f32>` - The weights of the points.
    WeightedFarthestPointSampling(Vec<T>),
}

impl<T> SamplingStrategy<T>
where
    T: FloatNumber,
{
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
    pub fn sample<const N: usize>(&self, points: &[Point<T, N>], n: usize) -> HashSet<usize> {
        let metric = DistanceMetric::SquaredEuclidean;
        match self {
            SamplingStrategy::FarthestPointSampling => {
                sample_with_distance_fn(points, n, 0, |_, point1, point2| {
                    metric.measure(point1, point2)
                })
            }
            SamplingStrategy::WeightedFarthestPointSampling(weights) => {
                debug_assert_eq!(
                    points.len(),
                    weights.len(),
                    "The number of points and weights must be equal."
                );
                let (initial_index, _) = weights
                    .iter()
                    .enumerate()
                    .max_by(|(_, weight1), (_, weight2)| weight1.partial_cmp(weight2).unwrap())
                    .unwrap();
                sample_with_distance_fn(points, n, initial_index, |index, point1, point2| {
                    metric.measure(point1, point2) * weights[index]
                })
            }
        }
    }
}

#[must_use]
fn sample_with_distance_fn<T, const N: usize, F>(
    points: &[Point<T, N>],
    n: usize,
    initial_index: usize,
    distance_fn: F,
) -> HashSet<usize>
where
    T: FloatNumber,
    F: Fn(usize, &Point<T, N>, &Point<T, N>) -> T,
{
    if n == 0 || points.is_empty() {
        return HashSet::new();
    }

    if points.len() <= n {
        return (0..points.len()).collect();
    }

    let mut selected = HashSet::with_capacity(n);
    selected.insert(initial_index);

    let mut distances = vec![T::infinity(); points.len()];
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
fn find_farthest_index<T>(distances: &[T], selected: &HashSet<usize>) -> usize
where
    T: FloatNumber,
{
    let mut farthest_index = 0;
    let mut farthest_distance = T::zero();
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
fn update_distances<T, const N: usize, F>(
    points: &[Point<T, N>],
    distances: &mut [T],
    selected: &HashSet<usize>,
    farthest_point: &Point<T, N>,
    distance_fn: &F,
) where
    T: FloatNumber,
    F: Fn(usize, &Point<T, N>, &Point<T, N>) -> T,
{
    for (index, point) in points.iter().enumerate() {
        if selected.contains(&index) {
            distances[index] = T::zero();
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
    fn sample_points() -> Vec<Point<f32, 2>> {
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
    fn empty_points() -> Vec<Point<f32, 2>> {
        vec![]
    }

    #[rstest]
    #[case(0, vec ! [])]
    #[case(1, vec ! [0])]
    #[case(3, vec ! [0, 3, 5])]
    #[case(5, vec ! [0, 1, 3, 5, 8])]
    #[case(9, vec ! [0, 1, 2, 3, 4, 5, 6, 7, 8])]
    #[case(10, vec ! [0, 1, 2, 3, 4, 5, 6, 7, 8])]
    fn test_sample_farthest_point_sampling(#[case] n: usize, #[case] expected: Vec<usize>) {
        // Act
        let points = sample_points();
        let sampling = SamplingStrategy::FarthestPointSampling;
        let sampled = sampling.sample(&points, n);

        // Assert
        assert_eq!(sampled, expected.into_iter().collect());
    }

    #[test]
    fn test_sample_farthest_point_sampling_empty() {
        // Act
        let points = empty_points();
        let sampling = SamplingStrategy::FarthestPointSampling;
        let sampled = sampling.sample(&points, 2);

        // Assert
        assert!(sampled.is_empty());
    }

    #[rstest]
    #[case(0, vec ! [])]
    #[case(1, vec ! [8])]
    #[case(3, vec ! [5,6, 8])]
    #[case(5, vec ! [3, 5, 6, 7, 8])]
    #[case(9, vec ! [0, 1, 2, 3, 4, 5, 6, 7, 8])]
    #[case(10, vec ! [0, 1, 2, 3, 4, 5, 6, 7, 8])]
    fn test_sample_weighted_farthest_point_sampling(
        #[case] n: usize,
        #[case] expected: Vec<usize>,
    ) {
        // Act
        let weights = vec![1.0, 1.0, 2.0, 3.0, 5.0, 8.0, 13.0, 21.0, 34.0];
        let points = sample_points();
        let sampling = SamplingStrategy::WeightedFarthestPointSampling(weights);
        let sampled = sampling.sample(&points, n);

        // Assert
        assert_eq!(sampled, expected.into_iter().collect());
    }
}

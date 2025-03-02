use std::collections::HashSet;

use crate::math::{DistanceMetric, FloatNumber, Point};

/// Strategy for sampling points from a set of points.
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug, Default, PartialEq)]
pub enum SamplingStrategy<T>
where
    T: FloatNumber,
{
    /// Farthest point sampling strategy.
    /// The distance between two points is measured using the given distance metric.
    #[default]
    Farthest,
    /// Weighted farthest point sampling strategy.
    /// The distance between two points is multiplied by the weight of the first point.
    ///
    /// # Arguments
    /// * `Vec<T>` - The weights of the points.
    #[allow(dead_code)]
    WeightedFarthest(Vec<T>),
    /// Diversity sampling strategy.
    /// The diversity is calculated using the scores of the points.
    /// The algorithm is based on the following paper:
    /// [Improving Recommendation Lists Through Topic Diversification](https://dl.acm.org/doi/10.1145/1060745.1060754)
    ///
    /// # Arguments
    /// * `T` - The weight of the diversity.
    /// * `Vec<T>` - The scores of the points.
    Diversity(T, Vec<T>),
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
        if n == 0 || points.is_empty() {
            return HashSet::new();
        }

        if points.len() <= n {
            return (0..points.len()).collect();
        }

        match self {
            SamplingStrategy::Farthest => {
                sample_with_distance_fn(points, n, 0, |_, point1, point2| {
                    DistanceMetric::SquaredEuclidean.measure(point1, point2)
                })
            }
            SamplingStrategy::WeightedFarthest(weights) => {
                debug_assert_eq!(
                    points.len(),
                    weights.len(),
                    "The number of points and weights must be equal."
                );
                let (initial_index, _) = weights
                    .iter()
                    .enumerate()
                    .max_by(|(_, weight1), (_, weight2)| weight1.partial_cmp(weight2).unwrap())
                    .expect("No weights provided.");
                sample_with_distance_fn(points, n, initial_index, |index, point1, point2| {
                    DistanceMetric::SquaredEuclidean.measure(point1, point2) * weights[index]
                })
            }
            SamplingStrategy::Diversity(weight, scores) => {
                debug_assert_eq!(
                    points.len(),
                    scores.len(),
                    "The number of points and scores must be equal."
                );
                sample_diversity(points, scores, *weight, n)
            }
        }
    }
}

/// Samples points using the diversity sampling strategy.
///
/// # Type Parameters
/// * `T` - The floating point type.
/// * `N` - The number of dimensions of the points.
///
/// # Arguments
/// * `points` - The set of points to sample from.
/// * `scores` - The scores of the points.
/// * `weight` - The weight of the diversity.
/// * `n` - The number of points to sample.
#[must_use]
fn sample_diversity<T, const N: usize>(
    points: &[Point<T, N>],
    scores: &[T],
    weight: T,
    n: usize,
) -> HashSet<usize>
where
    T: FloatNumber,
{
    // Sort scores in descending order
    let mut indices: Vec<usize> = (0..scores.len()).collect();
    indices.sort_by(|&index1, &index2| scores[index2].partial_cmp(&scores[index1]).unwrap());
    let mut score_rankings = vec![0; scores.len()];
    for (rank, &index) in indices.iter().enumerate() {
        score_rankings[index] = rank;
    }

    // Initialize the set of selected points with the highest score point
    let best_index = indices.first().copied().expect("No scores provided.");
    let mut selected = HashSet::with_capacity(n);
    selected.insert(best_index);

    let mut best_point = &points[best_index];
    let mut similarities = vec![T::max_value(); points.len()];

    while selected.len() < n {
        // Update similarities with the best point
        for (index, point) in points.iter().enumerate() {
            if selected.contains(&index) {
                similarities[index] = T::zero();
                continue;
            }

            let similarity = DistanceMetric::SquaredEuclidean.measure(point, best_point);
            similarities[index] = similarities[index].min(similarity);
        }

        // Sort similarities in descending order to get dissimilarities
        let mut indices: Vec<usize> = (0..scores.len()).collect();
        indices.sort_by(|&index1, &index2| {
            similarities[index2]
                .partial_cmp(&similarities[index1])
                .unwrap()
        });
        let mut dissimilarities_rankings = vec![0; scores.len()];
        for (rank, &index) in indices.iter().enumerate() {
            dissimilarities_rankings[index] = rank;
        }

        // Find the best point based on the combined score that considers both the score and dissimilarity
        let mut best_index = None;
        let mut best_score = T::max_value();
        for (index, _) in points.iter().enumerate() {
            if selected.contains(&index) {
                continue;
            }

            let score_rank = 1 + score_rankings
                .get(index)
                .copied()
                .expect("Invalid score ranking.");
            let dissimilarity_rank = 1 + dissimilarities_rankings
                .get(index)
                .copied()
                .expect("Invalid dissimilarity ranking.");
            let combined_score = T::from_usize(score_rank) * (T::one() - weight)
                + T::from_usize(dissimilarity_rank) * weight;
            if combined_score < best_score {
                best_score = combined_score;
                best_index = Some(index);
            }
        }

        if let Some(index) = best_index {
            selected.insert(index);
            best_point = &points[index];
        } else {
            break;
        }
    }
    selected
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
    use rstest::rstest;

    use super::*;

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
            [0.0, 0.2], // 8
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
        let sampling = SamplingStrategy::Farthest;
        let sampled = sampling.sample(&points, n);

        // Assert
        assert_eq!(sampled, expected.into_iter().collect());
    }

    #[test]
    fn test_sample_farthest_point_sampling_empty() {
        // Act
        let points = empty_points();
        let sampling = SamplingStrategy::Farthest;
        let sampled = sampling.sample(&points, 2);

        // Assert
        assert!(sampled.is_empty());
    }

    #[rstest]
    #[case(0, vec ! [])]
    #[case(1, vec ! [8])]
    #[case(3, vec ! [5, 6, 8])]
    #[case(5, vec ! [3, 5, 6, 7, 8])]
    #[case(9, vec ! [0, 1, 2, 3, 4, 5, 6, 7, 8])]
    #[case(10, vec ! [0, 1, 2, 3, 4, 5, 6, 7, 8])]
    fn test_sample_weighted_farthest_point_sampling(
        #[case] n: usize,
        #[case] expected: Vec<usize>,
    ) {
        // Arrange
        let weights = vec![1.0, 1.0, 2.0, 3.0, 5.0, 8.0, 13.0, 21.0, 34.0];
        let sampling = SamplingStrategy::WeightedFarthest(weights);

        // Act
        let points = sample_points();
        let actual = sampling.sample(&points, n);

        // Assert
        assert_eq!(actual, expected.into_iter().collect());
    }

    #[test]
    fn test_sample_weighted_farthest_point_sampling_empty() {
        // Arrange
        let weights = vec![];
        let sampling = SamplingStrategy::WeightedFarthest(weights);

        // Act
        let points = empty_points();
        let actual = sampling.sample(&points, 2);

        // Assert
        assert!(actual.is_empty());
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn test_sample_weighted_farthest_point_sampling_invalid() {
        // Arrange
        let weights = vec![1.0, 2.0];
        let sampling = SamplingStrategy::WeightedFarthest(weights);

        // Act
        let points = sample_points();
        let _ = sampling.sample(&points, 2);
    }

    #[rstest]
    #[case(0, vec ! [])]
    #[case(1, vec ! [8])]
    #[case(3, vec ! [5, 6, 8])]
    #[case(5, vec ! [3, 4, 5, 6, 8])]
    #[case(9, vec ! [0, 1, 2, 3, 4, 5, 6, 7, 8])]
    #[case(10, vec ! [0, 1, 2, 3, 4, 5, 6, 7, 8])]
    fn test_sample_diversity_sampling(#[case] n: usize, #[case] expected: Vec<usize>) {
        // Arrange
        let scores = vec![1.0, 1.0, 2.0, 3.0, 5.0, 8.0, 13.0, 21.0, 34.0];
        let weight = 0.5;
        let sampling = SamplingStrategy::Diversity(weight, scores);

        // Act
        let points = sample_points();
        let actual = sampling.sample(&points, n);

        // Assert
        assert_eq!(actual, expected.into_iter().collect());
    }

    #[test]
    fn test_sample_diversity_sampling_empty() {
        // Arrange
        let points = empty_points();
        let scores: Vec<f32> = vec![];
        let weight = 0.5;

        // Act
        let sampling = SamplingStrategy::Diversity(weight, scores);
        let actual = sampling.sample(&points, 3);

        // Assert
        assert!(actual.is_empty());
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn test_sample_diversity_sampling_invalid() {
        // Arrange
        let weight = 0.5;
        let scores = vec![1.0, 2.0, 3.0];
        let sampling = SamplingStrategy::Diversity(weight, scores);

        // Act
        let points = sample_points();
        let _ = sampling.sample(&points, 2);
    }
}

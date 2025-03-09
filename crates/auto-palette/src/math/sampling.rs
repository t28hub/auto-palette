use std::{cmp::Ordering, collections::HashSet};

use num_traits::clamp;

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
    /// The distance between two points is measured using the squared Euclidean distance.
    /// This strategy selects points that are maximally distant from each other.
    #[default]
    Farthest,

    /// Weighted farthest point sampling strategy.
    /// The distance between two points is multiplied by the weight of the point.
    /// Points with higher weights are more likely to be selected.
    ///
    /// # Arguments
    /// * `Vec<T>` - The weights of the points. Must have the same length as the point set.
    WeightedFarthest(Vec<T>),

    /// Diversity sampling strategy.
    /// The diversity is calculated using the scores of the points.
    /// The algorithm balances between selecting high-scored points and maintaining diversity in the selected set.
    ///
    /// The algorithm is based on the following paper:
    /// [Improving Recommendation Lists Through Topic Diversification](https://dl.acm.org/doi/10.1145/1060745.1060754)
    ///
    /// # Arguments
    /// * `T` - The weight of the diversity (0.0 to 1.0). Higher values prioritize diversity.
    /// * `Vec<T>` - The scores of the points. Must have the same length as the point set.
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
            SamplingStrategy::Farthest => sample_farthest_point(points, n),
            SamplingStrategy::WeightedFarthest(weights) => {
                debug_assert_eq!(
                    points.len(),
                    weights.len(),
                    "The number of points and weights must be equal."
                );
                sample_weighted_farthest_point(points, weights, n)
            }
            SamplingStrategy::Diversity(weight, scores) => {
                debug_assert_eq!(
                    points.len(),
                    scores.len(),
                    "The number of points and scores must be equal."
                );
                let normalized_weight = clamp(*weight, T::zero(), T::one());
                sample_diversity(points, scores, normalized_weight, n)
            }
        }
    }
}

/// Samples points using the farthest point sampling strategy.
///
/// This strategy selects points that are maximally distant from each other.
/// The first point is chosen as the origin (index 0).
///
/// # Type Parameters
/// * `T` - The floating point type.
/// * `N` - The number of dimensions of the points.
///
/// # Arguments
/// * `points` - The set of points to sample from.
/// * `n` - The number of points to sample.
///
/// # Returns
/// The indices of the sampled points.
#[must_use]
fn sample_farthest_point<T, const N: usize>(points: &[Point<T, N>], n: usize) -> HashSet<usize>
where
    T: FloatNumber,
{
    sample_with_distance_fn(points, n, 0, |_, point1, point2| {
        DistanceMetric::SquaredEuclidean.measure(point1, point2)
    })
}

/// Samples points using the weighted farthest point sampling strategy.
///
/// This strategy selects points that are maximally distant from each other, with distances weighted by the provided weights.
///
/// # Type Parameters
/// * `T` - The floating point type.
/// * `N` - The number of dimensions of the points.
///
/// # Arguments
/// * `points` - The set of points to sample from.
/// * `weights` - The weights of the points.
/// * `n` - The number of points to sample.
///
/// # Returns
/// The indices of the sampled points.
#[must_use]
fn sample_weighted_farthest_point<T, const N: usize>(
    points: &[Point<T, N>],
    weights: &[T],
    n: usize,
) -> HashSet<usize>
where
    T: FloatNumber,
{
    let (initial_index, _) = weights
        .iter()
        .enumerate()
        .max_by(|(_, weight1), (_, weight2)| {
            weight1.partial_cmp(weight2).unwrap_or(Ordering::Equal)
        })
        .unwrap_or((0, &T::zero()));

    sample_with_distance_fn(points, n, initial_index, |index, point1, point2| {
        DistanceMetric::SquaredEuclidean.measure(point1, point2) * weights[index]
    })
}

/// Sample points using a generic distance function.
///
/// # Type Parameters
/// * `T` - The floating point type.
/// * `N` - The number of dimensions of the points.
///
/// # Arguments
/// * `points` - The set of points to sample from.
/// * `n` - The number of points to sample.
/// * `initial_index` - The index of the initial point to include in the selection.
/// * `distance_fn` - A function that calculates the distance between two points.
///
/// # Returns
/// The indices of the sampled points.
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

/// Finds the index of the point with the maximum distance.
///
/// # Type Parameters
/// * `T` - The floating point type.
///
/// # Arguments
/// * `distances` - The distances from each point to the set of selected points.
/// * `selected` - The set of indices of the points that have already been selected.
///
/// # Returns
/// The index of the point with the maximum distance.
#[inline]
#[must_use]
fn find_farthest_index<T>(distances: &[T], selected: &HashSet<usize>) -> usize
where
    T: FloatNumber,
{
    distances
        .iter()
        .enumerate()
        .filter(|(index, _)| !selected.contains(index))
        .max_by(|(_, distance1), (_, distance2)| {
            distance1.partial_cmp(distance2).unwrap_or(Ordering::Equal)
        })
        .map(|(index, _)| index)
        .unwrap_or(0)
}

/// Updates the minimum distances from each point to the set of selected points.
///
/// # Type Parameters
/// * `T` - The floating point type.
/// * `N` - The number of dimensions of the points.
/// * `F` - The distance function type.
///
/// # Arguments
/// * `points` - The set of points.
/// * `distances` - The current minimum distances from each point to the set of selected points.
/// * `selected` - The set of indices of the points that have already been selected.
/// * `new_point` - The most recently selected point.
/// * `distance_fn` - A function that calculates the distance between two points.
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
///
/// # Returns
/// The indices of the sampled points.
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
    let mut selected = HashSet::with_capacity(n);

    let score_rankings = sort_scores_descending(scores);
    let best_index = score_rankings.indices[0];
    let mut best_point = &points[best_index];
    selected.insert(best_index);

    let mut similarities = vec![T::max_value(); points.len()];
    while selected.len() < n {
        // Update similarities with the best point
        for (index, point) in points.iter().enumerate() {
            // Skip already selected points
            if selected.contains(&index) {
                similarities[index] = T::zero();
                continue;
            }

            let similarity = DistanceMetric::SquaredEuclidean.measure(point, best_point);
            similarities[index] = similarities[index].min(similarity);
        }

        let dissimilarity_rankings = sort_scores_descending(&similarities);
        let best_index = find_best_index(
            &score_rankings.rankings,
            &dissimilarity_rankings.rankings,
            &selected,
            weight,
        );
        match best_index {
            Some(index) => {
                selected.insert(index);
                best_point = &points[index];
            }
            None => break,
        }
    }
    selected
}

/// Ranked scores used in diversity sampling.
struct RankedScores {
    /// The rank of each item (lower is better)
    rankings: Vec<usize>,
    /// The indices of items sorted by their score (best first)
    indices: Vec<usize>,
}

/// Sorts scores in descending order and returns the ranks and indices.
///
/// # Type Parameters
/// * `T` - The type of the scores.
///
/// # Arguments
/// * `scores` - The scores to sort and rank.
///
/// # Returns
/// A tuple containing the ranks and indices of the sorted scores.
#[must_use]
fn sort_scores_descending<T>(scores: &[T]) -> RankedScores
where
    T: PartialOrd,
{
    let mut indices: Vec<usize> = (0..scores.len()).collect();

    // Sort indices based on scores in descending order
    indices.sort_by(|&index1, &index2| {
        scores[index2]
            .partial_cmp(&scores[index1])
            .unwrap_or(Ordering::Equal)
    });

    let rankings =
        indices
            .iter()
            .enumerate()
            .fold(vec![0; scores.len()], |mut acc, (rank, &index)| {
                acc[index] = rank;
                acc
            });

    RankedScores { rankings, indices }
}

/// Finds the best index to select next based on score rank and dissimilarity rank.
///
/// # Type Parameters
/// * `T` - The floating point type.
///
/// # Arguments
/// * `score_rankings` - The rank of each point based on its score.
/// * `dissimilarity_rankings` - The rank of each point based on its dissimilarity to selected points.
/// * `selected` - The set of indices of the points that have already been selected.
/// * `weight` - The weight to give to dissimilarity vs. score (0.0 to 1.0).
///
/// # Returns
/// The index of the best point to select next, or None if no suitable point is found.
#[must_use]
fn find_best_index<T>(
    score_rankings: &[usize],
    dissimilarity_rankings: &[usize],
    selected: &HashSet<usize>,
    weight: T,
) -> Option<usize>
where
    T: FloatNumber,
{
    const RANK_OFFSET: usize = 1;
    let (best_index, _) = score_rankings.iter().enumerate().fold(
        (None, T::max_value()),
        |(best_index, best_score), (index, &score_rank)| {
            if selected.contains(&index) {
                return (best_index, best_score);
            }

            let dissimilarity_rank = dissimilarity_rankings[index];
            let combined_score = T::from_usize(score_rank + RANK_OFFSET) * (T::one() - weight)
                + T::from_usize(dissimilarity_rank + RANK_OFFSET) * weight;
            if combined_score < best_score {
                (Some(index), combined_score)
            } else {
                (best_index, best_score)
            }
        },
    );
    best_index
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use rstest::rstest;

    use super::*;

    /// Sample points for testing.
    #[must_use]
    fn sample_points() -> Vec<Point<f32, 2>> {
        vec![
            [0.0, 0.0], // 0: origin point
            [0.1, 0.1], // 1: the closest point to the origin
            [0.1, 0.2], // 2
            [0.2, 0.2], // 3
            [0.2, 0.4], // 4
            [0.3, 0.5], // 5: the farthest point from the origin
            [0.1, 0.0], // 6
            [0.0, 0.1], // 7
            [0.0, 0.2], // 8
        ]
    }

    /// Empty points for testing.
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
        let sampled = SamplingStrategy::Farthest.sample(&points, n);

        // Assert
        assert_eq!(sampled, expected.into_iter().collect());
    }

    #[test]
    fn test_sample_farthest_point_sampling_empty() {
        // Act
        let points = empty_points();
        let sampled = SamplingStrategy::Farthest.sample(&points, 2);

        // Assert
        assert!(
            sampled.is_empty(),
            "Sampling from empty points should return empty set"
        );
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
        assert!(
            actual.is_empty(),
            "Sampling from empty points should return empty set"
        );
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "The number of points and weights must be equal.")]
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

    #[test]
    fn test_sort_scores_descending() {
        // Act
        let scores = vec![3.0, 1.0, 2.0, 4.0, 5.0];
        let ranked = sort_scores_descending(&scores);

        // Assert
        assert_eq!(ranked.rankings, vec![2, 4, 3, 1, 0]);
        assert_eq!(ranked.indices, vec![4, 3, 0, 2, 1]);
    }

    #[test]
    fn test_find_best_index() {
        // Arrange
        let score_rankings = vec![0, 1, 2, 3, 4];
        let dissimilarity_rankings = vec![4, 3, 2, 1, 0];
        let selected = HashSet::from([0, 1]);
        let weight = 0.5;

        // Act
        let best_index =
            find_best_index(&score_rankings, &dissimilarity_rankings, &selected, weight);

        // Assert
        assert_eq!(best_index, Some(2));
    }
}

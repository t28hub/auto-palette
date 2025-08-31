use std::{cmp::Ordering, collections::HashSet};

use crate::{
    math::{
        sampling::{algorithm::SamplingAlgorithm, error::SamplingError},
        DistanceMetric,
        Point,
    },
    FloatNumber,
};

/// Diversity-based point sampling algorithm.
///
/// This algorithm balances between selecting high-scoring points and maintaining diversity
/// in the selected set. It uses a trade-off parameter to control the balance between
/// maximizing individual point scores and maximizing the diversity of the selected set.
///
/// # Performance
/// Time complexity: O(n² × k) where n is the number of points and k is the number of samples.
/// Space complexity: O(n) for storing distances and rankings.
///
/// # References
/// Based on: [Improving Recommendation Lists Through Topic Diversification](https://dl.acm.org/doi/10.1145/1060745.1060754)
///
/// # Type Parameters
/// * `T` - The floating point type used for calculations.
#[derive(Debug, PartialEq)]
pub struct DiversitySampling<T>
where
    T: FloatNumber,
{
    diversity_factor: T,
    ranked: RankedScores<T>,
    metric: DistanceMetric,
}

impl<T> DiversitySampling<T>
where
    T: FloatNumber,
{
    /// Creates a new `DiversitySampling` instance with the specified parameters.
    ///
    /// # Arguments
    /// * `diversity_factor` - The diversity factor λ (0 ≤ λ ≤ 1).
    ///   - λ = 0: Pure score-based selection (no diversity)
    ///   - λ = 1: Pure diversity-based selection (ignore scores)
    ///   - 0 < λ < 1: Balance between score and diversity
    /// * `weights` - The weights/scores of the points. Higher weights indicate higher preference.
    /// * `metric` - The distance metric to use for measuring diversity.
    ///
    /// # Returns
    /// A new `DiversitySampling` instance.
    ///
    /// # Errors
    /// * [`SamplingError::DiversityOutOfRange`] - If diversity_factor is not in range [0, 1] or is not finite
    /// * [`SamplingError::EmptyWeights`] - If weights vector is empty
    pub fn new(
        diversity_factor: T,
        weights: Vec<T>,
        metric: DistanceMetric,
    ) -> Result<Self, SamplingError> {
        if !diversity_factor.is_finite() {
            return Err(SamplingError::DiversityOutOfRange);
        }
        if diversity_factor < T::zero() || diversity_factor > T::one() {
            return Err(SamplingError::DiversityOutOfRange);
        }
        if weights.is_empty() {
            return Err(SamplingError::EmptyWeights);
        }

        Ok(Self {
            diversity_factor,
            ranked: RankedScores::new(weights),
            metric,
        })
    }
}

impl<T> SamplingAlgorithm<T> for DiversitySampling<T>
where
    T: FloatNumber,
{
    /// Selects the point with the highest initial score/weight.
    ///
    /// This implementation returns the index of the point with the maximum weight,
    /// which is the first element in the pre-sorted `ranked.indices` vector.
    fn select_initial_index<const N: usize>(
        &self,
        points: &[Point<T, N>],
    ) -> Result<usize, SamplingError> {
        if points.is_empty() {
            return Err(SamplingError::EmptyPoints);
        }

        if self.ranked.len() != points.len() {
            return Err(SamplingError::LengthMismatch {
                points_len: points.len(),
                weights_len: self.ranked.len(),
            });
        }

        // Select the point with the highest score (first in ranked indices)
        Ok(self.ranked.sorted_indices[0])
    }

    fn sample<const N: usize>(
        &self,
        points: &[Point<T, N>],
        num_samples: usize,
    ) -> Result<HashSet<usize>, SamplingError> {
        let best_index = self.select_initial_index(points)?;
        let mut best_point = points[best_index];

        if num_samples == 0 {
            return Ok(HashSet::new());
        }
        if points.len() <= num_samples {
            return Ok((0..points.len()).collect());
        }

        let mut selected = HashSet::with_capacity(num_samples);
        selected.insert(best_index);

        let mut similarities = vec![T::max_value(); points.len()];
        while selected.len() < num_samples {
            // Update similarities with the best point
            for (index, point) in points.iter().enumerate() {
                // Skip already selected points
                if selected.contains(&index) {
                    similarities[index] = T::zero();
                } else {
                    let similarity = self.metric.measure(&best_point, point);
                    similarities[index] = similarities[index].min(similarity);
                }
            }

            let dissimilarity_rankings = RankedScores::new(similarities.clone());
            let best_index = find_best_index(
                &self.ranked.ranks,
                &dissimilarity_rankings.ranks,
                &selected,
                self.diversity_factor,
            );
            match best_index {
                Some(index) => {
                    selected.insert(index);
                    best_point = points[index];
                }
                None => break,
            }
        }
        Ok(selected)
    }
}

/// Container for scores with their rankings and sorted indices.
///
/// This struct maintains three synchronized views of the same data:
/// - Original scores in their input order
/// - Rankings for each item (0 = best, n-1 = worst)
/// - Indices sorted by score (descending order)
///
/// # Type Parameters
/// * `T` - The floating point type used for scores.
#[derive(Debug, PartialEq)]
struct RankedScores<T>
where
    T: FloatNumber,
{
    /// The original scores of the items
    scores: Vec<T>,
    /// The rankings of the items (0 = highest score)
    ranks: Vec<usize>,
    /// The indices of the items sorted by score (highest score first)
    sorted_indices: Vec<usize>,
}

impl<T> RankedScores<T>
where
    T: FloatNumber,
{
    /// Creates a new `RankedScores` instance from a vector of scores.
    ///
    /// Scores are ranked in descending order (highest score gets rank 0).
    ///
    /// # Arguments
    /// * `scores` - The scores to rank.
    ///
    /// # Returns
    /// A new `RankedScores` instance with computed rankings.
    #[must_use]
    fn new(scores: Vec<T>) -> Self {
        let mut sorted_indices: Vec<usize> = (0..scores.len()).collect();
        sorted_indices.sort_by(|&index1, &index2| {
            scores[index2]
                .partial_cmp(&scores[index1])
                .unwrap_or(Ordering::Equal)
        });
        let ranks = sorted_indices.iter().enumerate().fold(
            vec![0; scores.len()],
            |mut acc, (rank, &index)| {
                acc[index] = rank;
                acc
            },
        );
        RankedScores {
            scores,
            ranks,
            sorted_indices,
        }
    }

    /// Returns the number of scores.
    ///
    /// # Returns
    /// The number of scores in this container.
    pub fn len(&self) -> usize {
        self.scores.len()
    }
}

/// Finds the best unselected index using a weighted combination of score and dissimilarity rankings.
///
/// The selection criterion is:
/// ```text
/// combined_score = (1 - weight) × (score_rank + 1) + weight × (dissimilarity_rank + 1)
/// ```
/// The +1 offset ensures non-zero values for multiplication.
///
/// # Type Parameters
/// * `T` - The floating point type.
///
/// # Arguments
/// * `score_rankings` - Rankings based on original scores (0 = highest score).
/// * `dissimilarity_rankings` - Rankings based on distance to selected set (0 = farthest).
/// * `selected` - Set of already selected indices to exclude.
/// * `weight` - The diversity weight λ for balancing score vs dissimilarity.
///
/// # Returns
/// The index with the lowest combined score, or `None` if all indices are selected.
#[inline]
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
mod tests {
    use rstest::rstest;

    use super::*;

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

    #[must_use]
    fn empty_points() -> Vec<Point<f32, 2>> {
        vec![]
    }

    #[test]
    fn test_new() {
        // Arrange
        let weights = vec![1.0, 2.0, 3.0];
        let actual =
            DiversitySampling::new(0.5, weights.clone(), DistanceMetric::SquaredEuclidean).unwrap();

        // Assert
        assert_eq!(
            actual,
            DiversitySampling {
                diversity_factor: 0.5,
                ranked: RankedScores::new(weights),
                metric: DistanceMetric::SquaredEuclidean,
            }
        );
    }

    #[rstest]
    #[case::diversity_lt_0(-0.1)]
    #[case::diversity_gt_1(1.1)]
    #[case::diversity_nan(f32::NAN)]
    #[case::diversity_infinite(f32::INFINITY)]
    fn test_new_invalid_diversity(#[case] diversity: f32) {
        // Act
        let weights = vec![1.0, 2.0, 3.0];
        let actual = DiversitySampling::new(diversity, weights, DistanceMetric::SquaredEuclidean);

        // Assert
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), SamplingError::DiversityOutOfRange);
    }

    #[test]
    fn test_new_empty_weights() {
        // Act
        let weights = vec![];
        let actual = DiversitySampling::new(0.5, weights, DistanceMetric::SquaredEuclidean);

        // Assert
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), SamplingError::EmptyWeights);
    }

    #[rstest]
    #[case(0, vec ! [])]
    #[case(1, vec ! [8])]
    #[case(3, vec ! [5, 6, 8])]
    #[case(5, vec ! [3, 4, 5, 6, 8])]
    #[case(9, vec ! [0, 1, 2, 3, 4, 5, 6, 7, 8])]
    #[case(10, vec ! [0, 1, 2, 3, 4, 5, 6, 7, 8])]
    fn test_sample(#[case] num_samples: usize, #[case] expected: Vec<usize>) {
        // Arrange
        let weights = vec![1.0, 1.0, 2.0, 3.0, 5.0, 8.0, 13.0, 21.0, 34.0];
        let algorithm =
            DiversitySampling::new(0.8, weights, DistanceMetric::SquaredEuclidean).unwrap();

        // Act
        let points = sample_points();
        let actual = algorithm.sample(&points, num_samples).unwrap();

        // Assert
        assert_eq!(actual.len(), expected.len());
        assert_eq!(actual, expected.into_iter().collect());
    }

    #[test]
    fn test_sample_empty_points() {
        // Arrange
        let weights = vec![1.0, 1.0, 2.0, 3.0, 5.0, 8.0, 13.0, 21.0, 34.0];
        let algorithm =
            DiversitySampling::new(0.8, weights, DistanceMetric::SquaredEuclidean).unwrap();

        // Act
        let points = empty_points();
        let actual = algorithm.sample(&points, 3);

        // Assert
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), SamplingError::EmptyPoints);
    }

    #[test]
    fn test_sample_weights_length_mismatch() {
        // Arrange
        let weights = vec![1.0, 2.0, 3.0];
        let algorithm =
            DiversitySampling::new(0.8, weights.clone(), DistanceMetric::SquaredEuclidean).unwrap();

        // Act
        let points = sample_points();
        let actual = algorithm.sample(&points, 3);

        // Assert
        assert!(actual.is_err());
        assert_eq!(
            actual.unwrap_err(),
            SamplingError::LengthMismatch {
                points_len: points.len(),
                weights_len: weights.len(),
            }
        );
    }

    #[rstest]
    #[case::pure_score(0.0)]
    #[case::mostly_score(0.25)]
    #[case::balanced(0.5)]
    #[case::mostly_diverse(0.75)]
    #[case::pure_diverse(1.0)]
    fn test_select_initial_index(#[case] diversity: f32) {
        // Arrange
        let weights = vec![1.0, 1.0, 2.0, 3.0, 5.0, 8.0, 13.0, 21.0, 34.0]; // 34.0 at index 8 is max
        let algorithm =
            DiversitySampling::new(diversity, weights, DistanceMetric::Euclidean).unwrap();
        let points = sample_points();

        // Act
        let actual = algorithm.select_initial_index(&points);

        // Assert
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), 8);
    }

    #[test]
    fn test_select_initial_index_single_point() {
        // Arrange
        let weights = vec![42.0];
        let algorithm = DiversitySampling::new(0.5, weights, DistanceMetric::Euclidean).unwrap();
        let points = vec![[1.0, 2.0]];

        // Act
        let actual = algorithm.select_initial_index(&points);

        // Assert
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), 0);
    }

    #[test]
    fn test_select_initial_index_empty_points() {
        // Arrange
        let weights = vec![1.0, 2.0, 3.0];
        let algorithm = DiversitySampling::new(0.5, weights, DistanceMetric::Euclidean).unwrap();
        let points = empty_points();

        // Act
        let actual = algorithm.select_initial_index(&points);

        // Assert
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), SamplingError::EmptyPoints);
    }

    #[test]
    fn test_select_initial_index_weights_length_mismatch() {
        // Arrange
        let weights = vec![1.0, 2.0, 3.0];
        let algorithm = DiversitySampling::new(0.5, weights, DistanceMetric::Euclidean).unwrap();
        let points = vec![[0.0, 0.0], [1.0, 0.0]]; // 2 points but 3 weights

        // Act
        let actual = algorithm.select_initial_index(&points);

        // Assert
        assert!(actual.is_err());
        assert_eq!(
            actual.unwrap_err(),
            SamplingError::LengthMismatch {
                points_len: 2,
                weights_len: 3,
            }
        );
    }

    #[rstest]
    #[case::mixed_values(
        vec![5.0, 2.0, 8.0, 1.0, 3.0],
        vec![1, 3, 0, 4, 2],
        vec![2, 0, 4, 1, 3]
    )]
    #[case::single_element(
        vec![42.0],
        vec![0],
        vec![0]
    )]
    #[case::equal_values(
        vec![3.0, 3.0, 3.0],
        vec![0, 1, 2],
        vec![0, 1, 2]
    )]
    #[case::with_zeros(
        vec![0.0, 5.0, 0.0, 3.0],
        vec![2, 0, 3, 1],
        vec![1, 3, 0, 2]
    )]
    #[case::negative_values(
        vec![-1.0, 2.0, -3.0, 0.0],
        vec![2, 0, 3, 1],
        vec![1, 3, 0, 2]
    )]
    fn test_ranked_scores(
        #[case] scores: Vec<f32>,
        #[case] expected_ranks: Vec<usize>,
        #[case] expected_indices: Vec<usize>,
    ) {
        // Act
        let actual = RankedScores::new(scores.clone());

        // Assert
        assert_eq!(
            actual,
            RankedScores {
                scores: scores.clone(),
                sorted_indices: expected_indices.clone(),
                ranks: expected_ranks.clone(),
            }
        );
    }

    #[rstest]
    #[case::empty(vec![], 0)]
    #[case::non_empty(vec![1.0, 2.0, 3.0, 4.0, 5.0], 5)]
    fn test_ranked_scores_len(#[case] scores: Vec<f32>, #[case] expected: usize) {
        // Act
        let ranked = RankedScores::new(scores);
        let actual = ranked.len();

        // Assert
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case::pure_score(0.0, 1)]
    #[case::balanced(0.5, 0)]
    #[case::pure_diverse(1.0, 2)]
    fn test_find_best_index_basic(#[case] weight: f32, #[case] expected: usize) {
        // Arrange
        let score_rankings = vec![2, 0, 3, 1];
        let dissimilarity_rankings = vec![1, 3, 0, 2];
        let selected = HashSet::new();

        // Act
        let actual = find_best_index(&score_rankings, &dissimilarity_rankings, &selected, weight);

        // Assert
        assert_eq!(actual, Some(expected));
    }

    #[test]
    fn test_find_best_index_with_selected() {
        // Arrange
        let score_rankings = vec![2, 0, 3, 1];
        let dissimilarity_rankings = vec![1, 3, 0, 2];
        let mut selected = HashSet::new();
        selected.insert(1); // Best score is already selected

        // Act
        let result = find_best_index(&score_rankings, &dissimilarity_rankings, &selected, 0.0);

        // Assert
        assert_eq!(result, Some(3)); // Next best score (rank 1)
    }

    #[test]
    fn test_find_best_index_all_selected() {
        // Arrange
        let score_rankings = vec![0, 1];
        let dissimilarity_rankings = vec![1, 0];
        let mut selected = HashSet::new();
        selected.insert(0);
        selected.insert(1);

        // Act
        let result = find_best_index(&score_rankings, &dissimilarity_rankings, &selected, 0.5);

        // Assert
        assert_eq!(result, None); // All points are selected
    }
}

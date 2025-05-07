use std::{cmp::Ordering, collections::HashSet};

use crate::{
    math::{
        sampling::{algorithm::SamplingAlgorithm, error::SamplingError},
        DistanceMetric,
        Point,
    },
    FloatNumber,
};

/// Diversity point sampling algorithm.
///
/// This algorithm selects points that are diverse from each other in the given dataset.
/// It uses a scoring system to determine the diversity of points.
/// The algorithm is based on the following paper:
/// [Improving Recommendation Lists Through Topic Diversification](https://dl.acm.org/doi/10.1145/1060745.1060754)
///
/// # Type Parameters
/// * `T` - The floating point type.
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
    /// Creates a new `DiversitySampling` instance.
    ///
    /// # Arguments
    /// * `diversity_factor` - The diversity factor (between 0 and 1). Higher values mean more diversity.
    /// * `weights` - The weights of the points. Must be the same length as the points.
    /// * `metric` - The distance metric to use for sampling.
    ///
    /// # Returns
    /// A new `DiversitySampling` instance.
    pub fn new(
        diversity_factor: T,
        weights: Vec<T>,
        metric: DistanceMetric,
    ) -> Result<Self, SamplingError> {
        if diversity_factor < T::zero() || diversity_factor > T::one() {
            return Err(SamplingError::InvalidDiversity);
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
    fn sample<const N: usize>(
        &self,
        points: &[Point<T, N>],
        num_samples: usize,
    ) -> Result<HashSet<usize>, SamplingError> {
        if points.is_empty() {
            return Err(SamplingError::EmptyPoints);
        }
        if self.ranked.len() != points.len() {
            return Err(SamplingError::WeightsLengthMismatch {
                points_len: points.len(),
                weights_len: self.ranked.len(),
            });
        }

        if num_samples == 0 {
            return Ok(HashSet::new());
        }
        if points.len() <= num_samples {
            return Ok((0..points.len()).collect());
        }

        let best_index = self.ranked.indices[0];
        let mut best_point = points[best_index];

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
                &self.ranked.rankings,
                &dissimilarity_rankings.rankings,
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

/// Struct to hold the ranked scores and their indices.
///
/// This struct is used to store the scores and their rankings.
/// It is used in the `DiversitySampling` algorithm to find the best index based on the scores.
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug, PartialEq)]
struct RankedScores<T>
where
    T: FloatNumber,
{
    /// The original scores of the items
    scores: Vec<T>,
    /// The rank of each item (lower is better)
    rankings: Vec<usize>,
    /// The indices of items sorted by their score (best first)
    indices: Vec<usize>,
}

impl<T> RankedScores<T>
where
    T: FloatNumber,
{
    /// Creates a new `RankedScores` instance.
    ///
    /// # Arguments
    /// * `scores` - The scores to rank.
    ///
    /// # Returns
    /// A new `RankedScores` instance.
    #[must_use]
    fn new(scores: Vec<T>) -> Self {
        let mut indices: Vec<usize> = (0..scores.len()).collect();
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
        RankedScores {
            scores,
            rankings,
            indices,
        }
    }

    /// Returns the length of the scores.
    ///
    /// # Returns
    /// The length of the scores.
    pub fn len(&self) -> usize {
        self.scores.len()
    }
}

/// Finds the best index based on the score and dissimilarity rankings.
///
/// This function combines the score and dissimilarity rankings to find the best index.
/// The best index is the one with the lowest combined score and dissimilarity.
///
/// # Type Parameters
/// * `T` - The floating point type.
///
/// # Arguments
/// * `score_rankings` - The rankings of the scores.
/// * `dissimilarity_rankings` - The rankings of the dissimilarities.
/// * `selected` - The set of already selected indices.
/// * `weight` - The weight to use for combining the scores and dissimilarities.
///
/// # Returns
/// The index of the best item based on the combined score and dissimilarity.
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
    fn test_new_invalid_diversity(#[case] diversity: f32) {
        // Act
        let weights = vec![1.0, 2.0, 3.0];
        let actual = DiversitySampling::new(diversity, weights, DistanceMetric::SquaredEuclidean);

        // Assert
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), SamplingError::InvalidDiversity,);
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
    fn test_sample_scores_length_mismatch() {
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
            SamplingError::WeightsLengthMismatch {
                points_len: points.len(),
                weights_len: weights.len(),
            }
        );
    }
}

use std::{cmp::Ordering, collections::HashSet};

use crate::{
    math::{
        sampling::{algorithm::SamplingAlgorithm, error::SamplingError},
        DistanceMetric,
        Point,
    },
    FloatNumber,
};

/// Maximal Marginal Relevance (MMR) sampling algorithm with a relevance gate.
///
/// The selection runs in two stages:
///
/// 1. **Relevance gate** — only points whose weight is at least
///    `relevance_gate × max_weight` are eligible. This guarantees that every
///    selected point is relevant (e.g. fits the requested theme), no matter
///    how the diversity stage plays out. If fewer than `num_samples` points
///    pass the gate, only the gated points are returned.
/// 2. **Greedy MMR** — among the eligible points, greedily select the point
///    maximizing a weighted combination of relevance and diversity:
///
///    ```text
///    marginal(i) = (1 - λ) * weight(i) / max_weight + λ * min_dist(i) / max_dist
///    ```
///
/// Both terms are normalized to `[0, 1]`, so the trade-off is controlled only
/// by the diversity factor λ and is independent of the absolute magnitude of
/// the weights. `max_dist` is the maximum pairwise distance over the eligible
/// points, which makes the diversity term reflect the absolute spread: once
/// only near-duplicates remain, the diversity term vanishes and the selection
/// falls back to the highest-scoring points.
///
/// The additive form is chosen deliberately over multiplicative
/// (`weight × distance`) forms: when the weights span several orders of
/// magnitude (e.g. Gaussian theme scores), a multiplicative criterion is
/// dominated by the weight term and degenerates into selecting
/// near-duplicates of the top-scoring point. In the additive form the
/// relevance term is bounded by `1 - λ`, so a distinct point can always
/// overtake a near-duplicate. The gate complements this by keeping
/// completely irrelevant outliers out regardless of how distant they are.
///
/// # Performance
/// Time complexity: O(m²) for the pairwise distance bound plus O(m × k) for
/// the greedy selection, where m is the number of gated points and k is the
/// number of samples. Space complexity: O(m).
///
/// # References
/// Based on: [The Use of MMR, Diversity-Based Reranking for Reordering Documents and Producing Summaries](https://dl.acm.org/doi/10.1145/290941.291025)
///
/// # Type Parameters
/// * `T` - The floating point type used for calculations.
#[derive(Debug, Clone, PartialEq)]
pub struct MmrSampling<T>
where
    T: FloatNumber,
{
    diversity_factor: T,
    relevance_gate: T,
    min_separation: T,
    weights: Vec<T>,
    metric: DistanceMetric,
}

impl<T> MmrSampling<T>
where
    T: FloatNumber,
{
    /// Creates a new `MmrSampling` instance with the specified parameters.
    ///
    /// # Arguments
    /// * `diversity_factor` - The diversity factor λ (0 ≤ λ ≤ 1).
    ///   - λ = 0: Pure score-based selection (no diversity)
    ///   - λ = 1: Pure diversity-based selection (ignore scores)
    ///   - 0 < λ < 1: Balance between score and diversity
    /// * `relevance_gate` - The relative relevance threshold (0 ≤ gate ≤ 1).
    ///   Only points with `weight ≥ relevance_gate × max_weight` are eligible
    ///   for selection. `0` disables the gate.
    /// * `weights` - The weights/scores of the points. Higher weights indicate higher preference.
    ///   Must have the same length as the points to be sampled.
    /// * `metric` - The distance metric to use for measuring diversity.
    ///
    /// # Returns
    /// A new `MmrSampling` instance.
    ///
    /// # Errors
    /// * [`SamplingError::DiversityOutOfRange`] - If `diversity_factor` or `relevance_gate` is not in range [0, 1] or is not finite.
    /// * [`SamplingError::EmptyWeights`] - If the weights vector is empty.
    pub fn new(
        diversity_factor: T,
        relevance_gate: T,
        weights: Vec<T>,
        metric: DistanceMetric,
    ) -> Result<Self, SamplingError> {
        if !diversity_factor.is_finite() {
            return Err(SamplingError::DiversityOutOfRange);
        }
        if diversity_factor < T::zero() || diversity_factor > T::one() {
            return Err(SamplingError::DiversityOutOfRange);
        }
        if !relevance_gate.is_finite() {
            return Err(SamplingError::DiversityOutOfRange);
        }
        if relevance_gate < T::zero() || relevance_gate > T::one() {
            return Err(SamplingError::DiversityOutOfRange);
        }
        if weights.is_empty() {
            return Err(SamplingError::EmptyWeights);
        }
        Ok(Self {
            diversity_factor,
            relevance_gate,
            min_separation: T::zero(),
            weights,
            metric,
        })
    }

    /// Sets the minimum separation between selected points.
    ///
    /// Points closer than `min_separation` to an already selected point are
    /// skipped as long as more distant candidates remain; once the eligible
    /// set offers nothing beyond the separation, the constraint is relaxed
    /// and the remaining slots are filled with the best-scoring candidates.
    /// This prevents near-duplicates from crowding out visually distinct
    /// points without ever returning fewer points than available.
    ///
    /// # Arguments
    /// * `min_separation` - The minimum distance between selected points,
    ///   measured with the configured metric. `0` disables the constraint.
    ///
    /// # Returns
    /// The `MmrSampling` with the minimum separation applied.
    #[must_use]
    pub fn with_min_separation(mut self, min_separation: T) -> Self {
        self.min_separation = min_separation;
        self
    }

    /// Returns the maximum pairwise distance between the eligible points.
    ///
    /// # Arguments
    /// * `points` - All points in the dataset.
    /// * `eligible` - The indices of the eligible points.
    ///
    /// # Returns
    /// The maximum pairwise distance. Returns `0` for fewer than two points.
    fn max_pairwise_distance<const N: usize>(
        &self,
        points: &[Point<T, N>],
        eligible: &[usize],
    ) -> T {
        let mut max_distance = T::zero();
        for (i, &index1) in eligible.iter().enumerate() {
            for &index2 in eligible.iter().skip(i + 1) {
                max_distance =
                    max_distance.max(self.metric.measure(&points[index1], &points[index2]));
            }
        }
        max_distance
    }
}

impl<T> SamplingAlgorithm<T> for MmrSampling<T>
where
    T: FloatNumber,
{
    /// Selects the point with the maximum weight as the initial point.
    ///
    /// This ensures that the most relevant point (highest weight) is always
    /// included in the sample set.
    fn select_initial_index<const N: usize>(
        &self,
        points: &[Point<T, N>],
    ) -> Result<usize, SamplingError> {
        if points.is_empty() {
            return Err(SamplingError::EmptyPoints);
        }
        if points.len() != self.weights.len() {
            return Err(SamplingError::LengthMismatch {
                points_len: points.len(),
                weights_len: self.weights.len(),
            });
        }

        self.weights
            .iter()
            .enumerate()
            .max_by(|(_, weight1), (_, weight2)| {
                weight1.partial_cmp(weight2).unwrap_or(Ordering::Equal)
            })
            .map(|(index, _)| index)
            .ok_or(SamplingError::EmptyWeights)
    }

    fn sample<const N: usize>(
        &self,
        points: &[Point<T, N>],
        num_samples: usize,
    ) -> Result<HashSet<usize>, SamplingError> {
        let initial_index = self.select_initial_index(points)?;
        if num_samples == 0 {
            return Ok(HashSet::new());
        }

        // Restrict the selection to points that pass the relevance gate.
        let max_weight = self.weights[initial_index];
        let threshold = self.relevance_gate * max_weight;
        let eligible: Vec<usize> = (0..points.len())
            .filter(|&index| self.weights[index] >= threshold)
            .collect();
        if eligible.len() <= num_samples {
            return Ok(eligible.into_iter().collect());
        }

        let max_distance = self.max_pairwise_distance(points, &eligible);

        let mut selected = HashSet::with_capacity(num_samples);
        selected.insert(initial_index);

        // The minimum distance from each eligible point to the selected set.
        let mut min_distances: Vec<T> = points
            .iter()
            .map(|point| self.metric.measure(&points[initial_index], point))
            .collect();

        while selected.len() < num_samples {
            let marginal_of = |index: usize| {
                let relevance = if max_weight > T::zero() {
                    self.weights[index] / max_weight
                } else {
                    T::zero()
                };
                let diversity = if max_distance > T::zero() {
                    min_distances[index] / max_distance
                } else {
                    T::zero()
                };
                (T::one() - self.diversity_factor) * relevance + self.diversity_factor * diversity
            };

            // Prefer candidates that keep the minimum separation from the
            // selected set; fall back to the remaining candidates once the
            // eligible set offers nothing sufficiently distinct.
            let best_index = eligible
                .iter()
                .copied()
                .filter(|index| {
                    !selected.contains(index) && min_distances[*index] >= self.min_separation
                })
                .map(|index| (index, marginal_of(index)))
                .max_by(|(_, marginal1), (_, marginal2)| {
                    marginal1.partial_cmp(marginal2).unwrap_or(Ordering::Equal)
                })
                .or_else(|| {
                    eligible
                        .iter()
                        .copied()
                        .filter(|index| !selected.contains(index))
                        .map(|index| (index, marginal_of(index)))
                        .max_by(|(_, marginal1), (_, marginal2)| {
                            marginal1.partial_cmp(marginal2).unwrap_or(Ordering::Equal)
                        })
                })
                .map(|(index, _)| index);

            let Some(best_index) = best_index else {
                break;
            };
            selected.insert(best_index);

            let best_point = &points[best_index];
            for &index in &eligible {
                let distance = self.metric.measure(best_point, &points[index]);
                min_distances[index] = min_distances[index].min(distance);
            }
        }
        Ok(selected)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[must_use]
    fn sample_points() -> Vec<Point<f64, 2>> {
        vec![
            [0.0, 0.0],   // 0: highest score
            [0.05, 0.0],  // 1: near-duplicate of 0, high score
            [0.1, 0.05],  // 2: near-duplicate of 0, high score
            [10.0, 0.0],  // 3: far from 0, low score
            [0.0, 10.0],  // 4: far from 0, low score
            [10.0, 10.0], // 5: farthest from 0, lowest score
        ]
    }

    #[must_use]
    fn sample_weights() -> Vec<f64> {
        vec![1.0, 0.95, 0.9, 0.5, 0.5, 0.4]
    }

    #[test]
    fn test_new() {
        // Act
        let actual = MmrSampling::new(0.5, 0.0, sample_weights(), DistanceMetric::Euclidean);

        // Assert
        assert!(actual.is_ok());
    }

    #[rstest]
    #[case::negative(-0.1)]
    #[case::too_large(1.1)]
    #[case::nan(f64::NAN)]
    #[case::infinite(f64::INFINITY)]
    fn test_new_invalid_diversity_factor(#[case] diversity_factor: f64) {
        // Act
        let actual = MmrSampling::new(
            diversity_factor,
            0.0,
            sample_weights(),
            DistanceMetric::Euclidean,
        );

        // Assert
        assert_eq!(actual.unwrap_err(), SamplingError::DiversityOutOfRange);
    }

    #[test]
    fn test_new_empty_weights() {
        // Act
        let weights: Vec<f64> = Vec::new();
        let actual = MmrSampling::new(0.5, 0.0, weights, DistanceMetric::Euclidean);

        // Assert
        assert_eq!(actual.unwrap_err(), SamplingError::EmptyWeights);
    }

    #[test]
    fn test_sample_balances_score_and_diversity() {
        // Arrange
        let algorithm =
            MmrSampling::new(0.5, 0.0, sample_weights(), DistanceMetric::Euclidean).unwrap();

        // Act
        let actual = algorithm.sample(&sample_points(), 3).unwrap();

        // Assert
        // The highest-scoring point is always selected, and the remaining
        // picks favor spatially distant points over near-duplicates,
        // even though the near-duplicates have much higher scores.
        assert!(actual.contains(&0));
        assert!(!actual.contains(&1));
        assert!(!actual.contains(&2));
        assert_eq!(actual.len(), 3);
    }

    #[test]
    fn test_sample_pure_score() {
        // Arrange
        let algorithm =
            MmrSampling::new(0.0, 0.0, sample_weights(), DistanceMetric::Euclidean).unwrap();

        // Act
        let actual = algorithm.sample(&sample_points(), 3).unwrap();

        // Assert
        // With λ = 0, the selection is purely score-based and picks the
        // near-duplicates despite their proximity.
        assert_eq!(actual, [0, 1, 2].into_iter().collect());
    }

    #[test]
    fn test_sample_pure_diversity() {
        // Arrange
        let algorithm =
            MmrSampling::new(1.0, 0.0, sample_weights(), DistanceMetric::Euclidean).unwrap();

        // Act
        let actual = algorithm.sample(&sample_points(), 3).unwrap();

        // Assert
        // With λ = 1, the selection ignores scores except for the initial
        // point and maximizes the spatial spread.
        assert!(actual.contains(&0));
        assert!(actual.contains(&5));
        assert_eq!(actual.len(), 3);
    }

    #[test]
    fn test_sample_identical_points_falls_back_to_scores() {
        // Arrange
        let points: Vec<Point<f64, 2>> = vec![[1.0, 1.0]; 4];
        let weights = vec![0.4, 0.8, 0.2, 0.6];
        let algorithm = MmrSampling::new(0.5, 0.0, weights, DistanceMetric::Euclidean).unwrap();

        // Act
        let actual = algorithm.sample(&points, 2).unwrap();

        // Assert
        // All points coincide, so the diversity term vanishes and the
        // highest-scoring points are selected.
        assert_eq!(actual, [1, 3].into_iter().collect());
    }

    #[test]
    fn test_sample_zero_samples() {
        // Arrange
        let algorithm =
            MmrSampling::new(0.5, 0.0, sample_weights(), DistanceMetric::Euclidean).unwrap();

        // Act
        let actual = algorithm.sample(&sample_points(), 0).unwrap();

        // Assert
        assert!(actual.is_empty());
    }

    #[test]
    fn test_sample_more_than_points() {
        // Arrange
        let algorithm =
            MmrSampling::new(0.5, 0.0, sample_weights(), DistanceMetric::Euclidean).unwrap();

        // Act
        let actual = algorithm.sample(&sample_points(), 10).unwrap();

        // Assert
        assert_eq!(actual, (0..6).collect());
    }

    #[test]
    fn test_sample_empty_points() {
        // Arrange
        let algorithm =
            MmrSampling::new(0.5, 0.0, sample_weights(), DistanceMetric::Euclidean).unwrap();
        let points: Vec<Point<f64, 2>> = Vec::new();

        // Act
        let actual = algorithm.sample(&points, 2);

        // Assert
        assert_eq!(actual.unwrap_err(), SamplingError::EmptyPoints);
    }

    #[test]
    fn test_sample_weights_length_mismatch() {
        // Arrange
        let weights = vec![1.0, 2.0, 3.0];
        let algorithm = MmrSampling::new(0.5, 0.0, weights, DistanceMetric::Euclidean).unwrap();

        // Act
        let actual = algorithm.sample(&sample_points(), 2);

        // Assert
        assert_eq!(
            actual.unwrap_err(),
            SamplingError::LengthMismatch {
                points_len: 6,
                weights_len: 3,
            }
        );
    }
}

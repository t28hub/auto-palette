use std::{cmp::Ordering, collections::HashSet};

use crate::{
    math::{
        sampling::{algorithm::SamplingAlgorithm, error::SamplingError},
        DistanceMetric,
        Point,
    },
    FloatNumber,
};

/// Weighted farthest point sampling algorithm.
///
/// This algorithm selects points that are farthest apart from each other in the given dataset,
/// taking into account the weights of the points.
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug, Clone, PartialEq)]
pub struct WeightedFarthestSampling<T>
where
    T: FloatNumber,
{
    weights: Vec<T>,
    metric: DistanceMetric,
}

impl<T> WeightedFarthestSampling<T>
where
    T: FloatNumber,
{
    /// Creates a new `WeightedFarthestSampling` instance.
    ///
    /// # Arguments
    /// * `weights` - The weights of the points.
    /// * `metric` - The distance metric to use for sampling.
    ///
    /// # Returns
    /// A new `WeightedFarthestSampling` instance.
    pub fn new(weights: Vec<T>, metric: DistanceMetric) -> Result<Self, SamplingError> {
        if weights.is_empty() {
            return Err(SamplingError::EmptyWeights);
        }
        Ok(Self { weights, metric })
    }

    /// Updates the minimum distances to the selected points.
    ///
    ///
    /// # Arguments
    /// * `distances` - The vector of minimum distances to the selected points.
    /// * `points` - The points to consider for distance calculation.
    /// * `selected_indices` - The indices of the selected points.
    /// * `selected_point` - The point to update distances against.
    #[inline]
    fn update_min_distances<const N: usize>(
        &self,
        distances: &mut [T],
        points: &[Point<T, N>],
        selected_indices: &HashSet<usize>,
        selected_point: &Point<T, N>,
    ) {
        for (i, point) in points.iter().enumerate() {
            if selected_indices.contains(&i) {
                distances[i] = T::zero();
            } else {
                let weighted_distance =
                    self.metric.measure(selected_point, point) * self.weights[i];
                distances[i] = distances[i].min(weighted_distance);
            }
        }
    }
}

const DEFAULT_INITIAL_INDEX: usize = 0;

impl<T> SamplingAlgorithm<T> for WeightedFarthestSampling<T>
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
        if points.len() != self.weights.len() {
            return Err(SamplingError::WeightsLengthMismatch {
                points_len: points.len(),
                weights_len: self.weights.len(),
            });
        }

        if num_samples == 0 {
            return Ok(HashSet::new());
        }
        if points.len() <= num_samples {
            return Ok((0..points.len()).collect());
        }

        let initial_index = self
            .weights
            .iter()
            .enumerate()
            .max_by(|(_, weight1), (_, weight2)| {
                weight1.partial_cmp(weight2).unwrap_or(Ordering::Equal)
            })
            .map(|(index, _)| index)
            .unwrap_or(DEFAULT_INITIAL_INDEX);
        let mut selected = HashSet::with_capacity(num_samples);
        selected.insert(initial_index);

        let mut min_distances = vec![T::infinity(); points.len()];
        self.update_min_distances(
            &mut min_distances,
            points,
            &selected,
            &points[initial_index],
        );

        while selected.len() < num_samples {
            let farthest_index = min_distances
                .iter()
                .enumerate()
                .filter(|(i, _)| !selected.contains(i))
                .max_by(|(_, distance1), (_, distance2)| {
                    distance1.partial_cmp(distance2).unwrap_or(Ordering::Equal)
                })
                .map(|(index, _)| index)
                .unwrap_or(DEFAULT_INITIAL_INDEX);
            // If the farthest point is already selected, break the loop
            if !selected.insert(farthest_index) {
                break;
            }

            let farthest_point = &points[farthest_index];
            self.update_min_distances(&mut min_distances, points, &selected, farthest_point);
        }
        Ok(selected)
    }
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
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

    #[must_use]
    fn sample_weights() -> Vec<f32> {
        vec![1.0, 1.0, 2.0, 3.0, 5.0, 8.0, 13.0, 21.0, 34.0]
    }

    #[test]
    fn test_new() {
        // Act
        let weights = vec![1.0, 1.0, 2.0, 3.0, 5.0, 8.0, 13.0, 21.0, 34.0];
        let metric = DistanceMetric::Euclidean;
        let actual = WeightedFarthestSampling::new(weights.clone(), metric);

        // Assert
        assert!(actual.is_ok());
        assert_eq!(
            actual.unwrap(),
            WeightedFarthestSampling {
                weights,
                metric: DistanceMetric::Euclidean,
            }
        );
    }

    #[test]
    fn test_new_empty_weights() {
        // Act
        let weights: Vec<f64> = Vec::new();
        let metric = DistanceMetric::Euclidean;
        let actual = WeightedFarthestSampling::new(weights, metric);

        // Assert
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), SamplingError::EmptyWeights);
    }

    #[rstest]
    #[case(0, vec ! [])]
    #[case(1, vec ! [8])]
    #[case(3, vec ! [5, 6, 8])]
    #[case(5, vec ! [3, 5, 6, 7, 8])]
    #[case(9, vec ! [0, 1, 2, 3, 4, 5, 6, 7, 8])]
    #[case(10, vec ! [0, 1, 2, 3, 4, 5, 6, 7, 8])]
    fn test_sample(#[case] num_samples: usize, #[case] expected: Vec<usize>) {
        // Arrange
        let weights = sample_weights();
        let algorithm =
            WeightedFarthestSampling::new(weights.clone(), DistanceMetric::SquaredEuclidean)
                .unwrap();

        // Act
        let points = sample_points();
        let actual = algorithm.sample(&points, num_samples).unwrap();
        println!("actual: {:?}", actual);

        // Assert
        assert_eq!(actual.len(), expected.len());
        assert!(actual.is_subset(&expected.into_iter().collect()));
    }

    #[test]
    fn test_sample_empty_points() {
        // Arrange
        let weights = vec![1.0, 1.0, 2.0, 3.0, 5.0, 8.0, 13.0, 21.0, 34.0];
        let algorithm = WeightedFarthestSampling::new(weights, DistanceMetric::Euclidean).unwrap();

        // Act
        let actual = algorithm.sample(&empty_points(), 2);

        // Assert
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), SamplingError::EmptyPoints);
    }

    #[test]
    fn test_sample_weights_length_mismatch() {
        // Arrange
        let weights = vec![1.0, 2.0, 3.0];
        let algorithm =
            WeightedFarthestSampling::new(weights.clone(), DistanceMetric::Euclidean).unwrap();

        // Act
        let points = sample_points();
        let actual = algorithm.sample(&points, 2);

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

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
/// This algorithm extends the farthest point sampling by incorporating weights into
/// the selection process. Points with higher weights are more likely to be selected,
/// while still maintaining good spatial distribution through distance-based selection.
///
/// # Performance
/// Time complexity: O(n * k) where n is the number of points and k is the number of samples.
/// Space complexity: O(n) for storing minimum distances.
///
/// # Type Parameters
/// * `T` - The floating point type used for calculations.
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
    /// Creates a new `WeightedFarthestSampling` instance with the specified parameters.
    ///
    /// # Arguments
    /// * `weights` - The weights of the points. Higher weights increase selection probability.
    ///   Must have the same length as the points to be sampled.
    /// * `metric` - The distance metric to use for measuring spatial distribution.
    ///
    /// # Returns
    /// A new `WeightedFarthestSampling` instance.
    ///
    /// # Errors
    /// * [`SamplingError::EmptyWeights`] - If the weights vector is empty.
    pub fn new(weights: Vec<T>, metric: DistanceMetric) -> Result<Self, SamplingError> {
        if weights.is_empty() {
            return Err(SamplingError::EmptyWeights);
        }
        Ok(Self { weights, metric })
    }

    /// Updates the minimum weighted distances from unselected points to the selected set.
    ///
    /// This method maintains the invariant that `distances[i]` contains the minimum
    /// weighted distance from point `i` to any selected point. The weighted distance
    /// is calculated as `distance × weight[i]`, giving preference to points with
    /// higher weights.
    ///
    /// # Arguments
    /// * `distances` - The vector of minimum weighted distances to update.
    /// * `points` - All points in the dataset.
    /// * `selected_indices` - The set of already selected point indices.
    /// * `selected_point` - The newly selected point to update distances against.
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

impl<T> SamplingAlgorithm<T> for WeightedFarthestSampling<T>
where
    T: FloatNumber,
{
    /// Selects the point with the maximum weight as the initial point.
    ///
    /// This ensures that the most important point (highest weight) is always
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

        // Find and return the index of the point with the maximum weight
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
        if points.len() <= num_samples {
            return Ok((0..points.len()).collect());
        }

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
                .unwrap_or(0);
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
            SamplingError::LengthMismatch {
                points_len: points.len(),
                weights_len: weights.len(),
            }
        );
    }

    #[rstest]
    #[case::multiple_points(
        vec![1.0, 2.0, 8.0, 3.0, 5.0], // 8.0 at index 2 is the max
        vec![[0.0, 0.0], [1.0, 0.0], [2.0, 0.0], [3.0, 0.0], [4.0, 0.0]],
    2
    )]
    #[case::single_point(
        vec![42.0],
        vec![[1.0, 2.0]],
    0
    )]
    #[case::equal_weights(
        vec![5.0, 5.0, 5.0],
        vec![[0.0, 0.0], [1.0, 0.0], [2.0, 0.0]],
    2
    )]
    fn test_select_initial_index(
        #[case] weights: Vec<f32>,
        #[case] points: Vec<Point<f32, 2>>,
        #[case] expected_index: usize,
    ) {
        // Arrange
        let algorithm = WeightedFarthestSampling::new(weights, DistanceMetric::Euclidean).unwrap();

        // Act
        let actual = algorithm.select_initial_index(&points);

        // Assert
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected_index);
    }

    #[test]
    fn test_select_initial_index_empty_points() {
        // Arrange
        let weights = vec![1.0, 2.0, 3.0];
        let algorithm = WeightedFarthestSampling::new(weights, DistanceMetric::Euclidean).unwrap();
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
        let algorithm = WeightedFarthestSampling::new(weights, DistanceMetric::Euclidean).unwrap();
        let points = vec![[0.0, 0.0], [1.0, 0.0]];

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
}

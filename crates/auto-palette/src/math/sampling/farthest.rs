use std::{cmp::Ordering, collections::HashSet, marker::PhantomData};

use crate::{
    math::{
        sampling::{algorithm::SamplingAlgorithm, error::SamplingError},
        DistanceMetric,
        Point,
    },
    FloatNumber,
};

/// Farthest point sampling algorithm.
///
/// This algorithm iteratively selects points that maximize the minimum distance
/// to already selected points. It provides a good spatial distribution of samples
/// across the dataset.
///
/// # Performance
/// Time complexity: O(n * k) where n is the number of points and k is the number of samples.
/// Space complexity: O(n) for storing minimum distances.
///
/// # Type Parameters
/// * `T` - The floating point type used for distance calculations.
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct FarthestSampling<T>
where
    T: FloatNumber,
{
    metric: DistanceMetric,
    _marker: PhantomData<T>,
}

impl<T> FarthestSampling<T>
where
    T: FloatNumber,
{
    /// Creates a new `FarthestSampling` instance with the specified distance metric.
    ///
    /// # Arguments
    /// * `metric` - The distance metric to use for sampling.
    ///
    /// # Returns
    /// A new `FarthestSampling` instance.
    #[must_use]
    #[allow(dead_code)]
    pub fn new(metric: DistanceMetric) -> Self {
        Self {
            metric,
            _marker: PhantomData,
        }
    }

    /// Updates the minimum distances from unselected points to the nearest selected point.
    ///
    /// This method maintains the invariant that `distances[i]` contains the minimum
    /// distance from point `i` to any selected point. Points that are already selected
    /// have their distance set to zero.
    ///
    /// # Arguments
    /// * `distances` - The vector of minimum distances to update.
    /// * `points` - All points in the dataset.
    /// * `selected_indices` - The set of already selected point indices.
    /// * `selected_point` - The newly selected point to update distances against.
    #[inline]
    #[allow(dead_code)]
    fn update_min_distances<const N: usize>(
        &self,
        distances: &mut [T],
        points: &[Point<T, N>],
        selected_indices: &HashSet<usize>,
        selected_point: &Point<T, N>,
    ) {
        for (index, point) in points.iter().enumerate() {
            if selected_indices.contains(&index) {
                distances[index] = T::zero();
            } else {
                let distance = self.metric.measure(selected_point, point);
                distances[index] = distances[index].min(distance);
            }
        }
    }
}

impl<T> Default for FarthestSampling<T>
where
    T: FloatNumber,
{
    /// Creates a default `FarthestSampling` instance using Euclidean distance.
    fn default() -> Self {
        Self::new(DistanceMetric::default())
    }
}

impl<T> SamplingAlgorithm<T> for FarthestSampling<T>
where
    T: FloatNumber,
{
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
        let initial_point = &points[initial_index];
        self.update_min_distances(&mut min_distances, points, &selected, initial_point);

        while selected.len() < num_samples {
            let farthest_index = find_farthest_index(&min_distances, &selected);
            if !selected.insert(farthest_index) {
                break;
            }

            let farthest_point = &points[farthest_index];
            self.update_min_distances(&mut min_distances, points, &selected, farthest_point);
        }
        Ok(selected)
    }
}

/// Finds the index of the unselected point that is farthest from all selected points.
///
/// # Type Parameters
/// * `T` - The floating point type.
///
/// # Arguments
/// * `distances` - The minimum distances from each point to the selected set.
/// * `selected` - The set of already selected indices.
///
/// # Returns
/// The index of the farthest unselected point, or 0 if all points are selected.
#[inline]
#[must_use]
#[allow(dead_code)]
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
        Vec::new()
    }

    #[test]
    fn test_new() {
        // Act
        let actual = FarthestSampling::<f32>::new(DistanceMetric::SquaredEuclidean);

        // Assert
        assert_eq!(
            actual,
            FarthestSampling {
                metric: DistanceMetric::SquaredEuclidean,
                _marker: PhantomData,
            }
        );
    }

    #[test]
    fn test_default() {
        // Act
        let actual = FarthestSampling::<f32>::default();

        // Assert
        assert_eq!(
            actual,
            FarthestSampling {
                metric: DistanceMetric::Euclidean,
                _marker: PhantomData,
            }
        );
    }

    #[rstest]
    #[case(0, vec ! [])]
    #[case(1, vec ! [0])]
    #[case(3, vec ! [0, 3, 5])]
    #[case(5, vec ! [0, 1, 3, 5, 8])]
    #[case(9, vec ! [0, 1, 2, 3, 4, 5, 6, 7, 8])]
    #[case(10, vec ! [0, 1, 2, 3, 4, 5, 6, 7, 8])]
    fn test_sample_farthest_point_sampling(
        #[case] num_samples: usize,
        #[case] expected: Vec<usize>,
    ) {
        // Act
        let algorithm = FarthestSampling::default();
        let points = sample_points();
        let actual = algorithm.sample(&points, num_samples);

        // Assert
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected.into_iter().collect());
    }

    #[test]
    fn test_sample_empty_points() {
        // Act
        let points = empty_points();
        let algorithm = FarthestSampling::default();
        let actual = algorithm.sample(&points, 3);

        // Assert
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), SamplingError::EmptyPoints);
    }

    #[test]
    fn test_select_initial_index() {
        // Arrange
        let algorithm = FarthestSampling::default();
        let points = sample_points();

        // Act
        let actual = algorithm.select_initial_index(&points);

        // Assert
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), 0); // Should return the first index
    }

    #[test]
    fn test_select_initial_index_single_point() {
        // Arrange
        let algorithm = FarthestSampling::default();
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
        let algorithm = FarthestSampling::default();
        let points: Vec<Point<f32, 2>> = vec![];

        // Act
        let actual = algorithm.select_initial_index(&points);

        // Assert
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), SamplingError::EmptyPoints);
    }
}

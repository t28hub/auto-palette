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
/// This algorithm selects points that are farthest apart from each other in the given dataset.
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug, Clone, PartialEq)]
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
    /// Creates a new `FarthestSampling` instance.
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

    /// Updates the minimum distances to the selected points.
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
    fn default() -> Self {
        Self::new(DistanceMetric::default())
    }
}

const INITIAL_INDEX: usize = 0;

impl<T> SamplingAlgorithm<T> for FarthestSampling<T>
where
    T: FloatNumber,
{
    fn sample<const N: usize>(
        &self,
        points: &[Point<T, N>],
        num_samples: usize,
    ) -> Result<HashSet<usize>, SamplingError<T>> {
        if points.is_empty() {
            return Err(SamplingError::EmptyPoints);
        }

        if num_samples == 0 {
            return Ok(HashSet::new());
        }

        if points.len() <= num_samples {
            return Ok((0..points.len()).collect());
        }

        let mut selected = HashSet::with_capacity(num_samples);
        selected.insert(INITIAL_INDEX);

        let mut min_distances = vec![T::infinity(); points.len()];
        let initial_point = &points[INITIAL_INDEX];
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
        .unwrap_or(INITIAL_INDEX)
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
    fn test_sample_empty() {
        // Act
        let points = empty_points();
        let algorithm = FarthestSampling::default();
        let actual = algorithm.sample(&points, 3);

        // Assert
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), SamplingError::EmptyPoints);
    }
}

use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
    marker::PhantomData,
};

use thiserror::Error;

use crate::{
    math::{
        clustering::{helper::gradient, Cluster, ClusteringAlgorithm, Initializer},
        matrix::{MatrixError, MatrixView},
        DistanceMetric,
        Point,
    },
    FloatNumber,
};

/// SNIC algorithm error type.
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug, PartialEq, Error)]
pub enum SNICError {
    /// Error when the shape is invalid.
    #[error("Invalid Shape: The shape must be > 0: {0}x{1}")]
    InvalidShape(usize, usize),

    /// Error when the number of segments is invalid.
    #[error("Invalid Segments: The number of segments must be > 0: {0}")]
    InvalidSegments(usize),

    /// Error when the points are empty.
    #[error("Empty Points: The points must be non-empty.")]
    EmptyPoints,

    /// Error when the points are not in the expected shape.
    #[error("Invalid Points: The points slice is not in the expected shape: {0}")]
    InvalidPoints(#[from] MatrixError),
}

/// SNIC (Simple Non-Iterative Clustering) algorithm.
///
/// This implementation is based on the following paper:
/// [Superpixels and Polygons using Simple Non-Iterative Clustering](https://openaccess.thecvf.com/content_cvpr_2017/papers/Achanta_Superpixels_and_Polygons_CVPR_2017_paper.pdf)
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
pub struct SNIC<T>
where
    T: FloatNumber,
{
    shape: (usize, usize),
    segments: usize,
    metric: DistanceMetric,
    _marker: PhantomData<T>,
}

impl<T> SNIC<T>
where
    T: FloatNumber,
{
    /// The label for unclassified points.
    const LABEL_UNCLASSIFIED: usize = usize::MAX;

    /// Creates a new `SNIC` instance.
    ///
    /// # Arguments
    /// * `shape` - The shape of the points to cluster as a tuple of (cols, rows).
    /// * `segments` - The number of segments to create.
    /// * `metric` - The distance metric to use.
    ///
    /// # Returns
    /// A new `SNIC` instance.
    pub fn new(
        shape: (usize, usize),
        segments: usize,
        metric: DistanceMetric,
    ) -> Result<Self, SNICError> {
        if shape.0 == 0 || shape.1 == 0 {
            return Err(SNICError::InvalidShape(shape.0, shape.1));
        }
        if segments == 0 {
            return Err(SNICError::InvalidSegments(segments));
        }
        Ok(Self {
            shape,
            segments,
            metric,
            _marker: PhantomData,
        })
    }

    /// Finds the index of the lowest gradient point in the matrix.
    ///
    /// # Type Parameters
    /// * `N` - The number of dimensions.
    ///
    /// # Arguments
    /// * `matrix` - The matrix of points.
    /// * `index` - The index of the point to check.
    ///
    /// # Returns
    /// The index of the point with the lowest gradient in the neighborhood if it exists.
    #[must_use]
    fn find_lowest_gradient_index<const N: usize>(
        &self,
        matrix: &MatrixView<T, N>,
        index: usize,
    ) -> Option<usize> {
        let col = index % matrix.cols;
        let row = index / matrix.cols;

        let mut lowest_score = T::max_value();
        let mut lowest_index = None;
        matrix.neighbors(col, row).for_each(|(neighbor_index, _)| {
            let neighbor_col = neighbor_index % matrix.cols;
            let neighbor_row = neighbor_index / matrix.cols;
            let score = gradient(matrix, neighbor_col, neighbor_row, self.metric);
            if score < lowest_score {
                lowest_score = score;
                lowest_index = Some(neighbor_index);
            }
        });
        lowest_index
    }
}

impl<T, const N: usize> ClusteringAlgorithm<T, N> for SNIC<T>
where
    T: FloatNumber,
{
    type Err = SNICError;

    fn fit(&self, points: &[Point<T, N>]) -> Result<Vec<Cluster<T, N>>, Self::Err> {
        if points.is_empty() {
            return Err(SNICError::EmptyPoints);
        }

        let (cols, rows) = self.shape;
        let matrix = MatrixView::new(cols, rows, points)?;

        // 1. Initialize the seeds using a grid pattern.
        let seeds = Initializer::Grid
            .initialize(&matrix, self.segments)
            .into_iter()
            .map(|seed_index| {
                let found = self.find_lowest_gradient_index(&matrix, seed_index);
                found.unwrap_or(seed_index)
            })
            .collect::<Vec<_>>();

        // 2. Initialize the priority queue with the seeds.
        let mut clusters = vec![Cluster::new(); seeds.len()];
        let mut queue = seeds.into_iter().enumerate().fold(
            BinaryHeap::with_capacity(matrix.size()),
            |mut heap, (cluster_label, point_index)| {
                let element = Element {
                    col: point_index % cols,
                    row: point_index / cols,
                    distance: T::zero(),
                    cluster_label,
                };
                heap.push(Reverse(element));
                heap
            },
        );

        // 3. Main clustering loop
        let mut labels = vec![Self::LABEL_UNCLASSIFIED; matrix.size()];
        while let Some(Reverse(element)) = queue.pop() {
            let point_index = element.col + element.row * cols;
            // Skip if the point is already assigned to a cluster.
            if labels[point_index] != Self::LABEL_UNCLASSIFIED {
                continue;
            }

            // Assign the nearest cluster label to the point.
            let cluster_label = element.cluster_label;
            labels[point_index] = cluster_label;

            let cluster = &mut clusters[cluster_label];
            cluster.add_member(point_index, &points[point_index]);

            // Traverse the neighbors of the point and add them as candidates for clustering.
            let centroid = cluster.centroid();
            matrix
                .neighbors(element.col, element.row)
                .filter(|(neighbor_index, _)| labels[*neighbor_index] == Self::LABEL_UNCLASSIFIED)
                .for_each(|(neighbor_index, neighbor_point)| {
                    let distance = self.metric.measure(centroid, neighbor_point);
                    let element = Element {
                        cluster_label,
                        col: neighbor_index % cols,
                        row: neighbor_index / cols,
                        distance,
                    };
                    queue.push(Reverse(element));
                });
        }
        Ok(clusters)
    }
}

/// An element representing a candidate for clustering.
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug)]
struct Element<T>
where
    T: FloatNumber,
{
    /// The column index of the element.
    col: usize,

    /// The row index of the element.
    row: usize,

    /// The distance of the element from the cluster centroid.
    distance: T,

    /// The cluster label of the element.
    cluster_label: usize,
}

impl<T> PartialEq for Element<T>
where
    T: FloatNumber,
{
    fn eq(&self, other: &Self) -> bool {
        self.cluster_label == other.cluster_label
            && self.distance == other.distance
            && self.col == other.col
            && self.row == other.row
    }
}

impl<T> Eq for Element<T> where T: FloatNumber {}

impl<T> PartialOrd for Element<T>
where
    T: FloatNumber,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Element<T>
where
    T: FloatNumber,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance
            .partial_cmp(&other.distance)
            .unwrap_or(Ordering::Less)
    }
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod tests {
    use rstest::rstest;

    use super::*;

    #[must_use]
    fn sample_points<T>(cols: usize, rows: usize) -> Vec<Point<T, 5>>
    where
        T: FloatNumber,
    {
        let half_cols = cols / 2;
        let half_rows = rows / 2;

        let mut points = vec![[T::zero(); 5]; cols * rows];
        for col in 0..cols {
            for row in 0..rows {
                let index = col + row * cols;
                let x = T::from_usize(col + 1) / T::from_usize(cols);
                let y = T::from_usize(row + 1) / T::from_usize(rows);
                points[index] = if col < half_cols && row < half_rows {
                    [T::one(), T::zero(), T::zero(), x, y]
                } else if col >= half_cols && row < half_rows {
                    [T::zero(), T::one(), T::zero(), x, y]
                } else if col < half_cols && row >= half_rows {
                    [T::zero(), T::zero(), T::one(), x, y]
                } else {
                    [T::zero(), T::zero(), T::zero(), x, y]
                };
            }
        }
        points
    }

    #[must_use]
    fn empty_points<T>() -> Vec<Point<T, 5>>
    where
        T: FloatNumber,
    {
        Vec::new()
    }

    #[test]
    fn test_new() {
        // Arrange
        let shape = (32, 18);
        let segments = 12;
        let metric = DistanceMetric::Euclidean;

        // Act
        let actual = SNIC::<f64>::new(shape, segments, metric);

        // Assert
        assert!(actual.is_ok());
        assert_eq!(
            actual.unwrap(),
            SNIC {
                shape,
                segments,
                metric,
                _marker: PhantomData,
            }
        );
    }

    #[rstest]
    #[case::invalid_shape_cols((0, 18), 12, DistanceMetric::Euclidean, SNICError::InvalidShape(0, 18))]
    #[case::invalid_shape_rows((32, 0), 12, DistanceMetric::Euclidean, SNICError::InvalidShape(32, 0))]
    #[case::invalid_segments((32, 18), 0, DistanceMetric::Euclidean, SNICError::InvalidSegments(0))]
    fn test_new_error(
        #[case] shape: (usize, usize),
        #[case] segments: usize,
        #[case] metric: DistanceMetric,
        #[case] expected: SNICError,
    ) {
        // Act
        let actual = SNIC::<f64>::new(shape, segments, metric);

        // Assert
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), expected);
    }

    #[test]
    fn test_fit() {
        // Arrange
        let cols = 32;
        let rows = 18;
        let segments = 12;
        let snic =
            SNIC::<f64>::new((cols, rows), segments, DistanceMetric::SquaredEuclidean).unwrap();

        // Act
        let points = sample_points::<f64>(cols, rows);
        let actual = snic.fit(&points);

        // Assert
        assert!(actual.is_ok());
        let clusters = actual.unwrap();
        assert_eq!(clusters.len(), segments);
    }

    #[test]
    fn test_fit_empty_points() {
        // Arrange
        let snic = SNIC::<f64>::new((32, 18), 12, DistanceMetric::SquaredEuclidean).unwrap();

        // Act
        let points = empty_points::<f64>();
        let actual = snic.fit(&points);

        // Assert
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), SNICError::EmptyPoints);
    }

    #[test]
    fn test_fit_invalid_points() {
        // Arrange
        let cols = 32;
        let rows = 18;
        let snic = SNIC::<f64>::new((cols, rows), 12, DistanceMetric::SquaredEuclidean).unwrap();

        // Act
        let points = sample_points(cols - 1, rows);
        let actual = snic.fit(&points);

        // Assert
        assert!(actual.is_err());
        assert_eq!(
            actual.unwrap_err(),
            SNICError::InvalidPoints(MatrixError::InvalidPoints(cols, rows))
        );
    }

    #[test]
    fn test_element_eq() {
        // Arrange
        let element1 = Element {
            col: 1,
            row: 2,
            distance: 3.0,
            cluster_label: 4,
        };
        let element2 = Element {
            col: 1,
            row: 2,
            distance: 3.0,
            cluster_label: 4,
        };

        // Act & Assert
        assert_eq!(element1, element2);
    }

    #[test]
    fn test_element_cmp() {
        // Arrange
        let element1 = Element {
            col: 1,
            row: 2,
            distance: 3.0,
            cluster_label: 4,
        };
        let element2 = Element {
            col: 1,
            row: 2,
            distance: 4.0,
            cluster_label: 4,
        };

        // Act & Assert
        assert_eq!(element1.cmp(&element1), Ordering::Equal);
        assert_eq!(element2.cmp(&element2), Ordering::Equal);
        assert_eq!(element1.cmp(&element2), Ordering::Less);
        assert_eq!(element2.cmp(&element1), Ordering::Greater);
    }

    #[test]
    fn test_element_partial_cmp() {
        // Arrange
        let element1 = Element {
            col: 1,
            row: 2,
            distance: 3.0,
            cluster_label: 4,
        };
        let element2 = Element {
            col: 1,
            row: 2,
            distance: 4.0,
            cluster_label: 4,
        };

        // Act & Assert
        assert_eq!(element1.partial_cmp(&element1), Some(Ordering::Equal));
        assert_eq!(element2.partial_cmp(&element2), Some(Ordering::Equal));
        assert_eq!(element1.partial_cmp(&element2), Some(Ordering::Less));
        assert_eq!(element2.partial_cmp(&element1), Some(Ordering::Greater));
    }

    #[test]
    fn test_cmp_nan() {
        // Arrange
        let element1 = Element {
            col: 1,
            row: 2,
            distance: 3.0,
            cluster_label: 4,
        };
        let element2 = Element {
            col: 1,
            row: 2,
            distance: f64::NAN,
            cluster_label: 4,
        };

        // Act & Assert
        assert_eq!(element1.cmp(&element2), Ordering::Less);
        assert_eq!(element2.cmp(&element1), Ordering::Less);
    }
}

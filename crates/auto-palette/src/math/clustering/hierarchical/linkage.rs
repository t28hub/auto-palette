use crate::math::distance::DistanceMetric;
use crate::math::point::Point;
use crate::number::Float;

/// Trait representing a linkage.
///
/// # Type Parameters
/// * `F` - The float type used for calculations (e.g., f32 or f64).
pub trait Linkage<F>
where
    F: Float,
{
    /// Returns the distance between the points with the given indices.
    ///
    /// # Arguments
    /// * `i` - The index of the 1st point.
    /// * `j` - The index of the 2nd point.
    ///
    /// # Returns
    /// The distance between the points with the given indices.
    #[must_use]
    fn distance(&self, i: usize, j: usize) -> F;

    /// Merges the points with the given indices.
    ///
    /// # Arguments
    /// * `i` - The index of the 1st point.
    /// * `j` - The index of the 2nd point.
    ///
    /// # Returns
    /// The label of the new merged point.
    #[must_use]
    fn merge(&mut self, i: usize, j: usize) -> usize;
}

/// Struct representing a distance matrix.
///
/// # Type Parameters
/// * `F` - The float type used for calculations (e.g., f32 or f64).
#[derive(Debug)]
struct DistanceMatrix<F>
where
    F: Float,
{
    distances: Vec<F>,
    size: usize,
}

impl<F> DistanceMatrix<F>
where
    F: Float,
{
    /// Creates a new `DistanceMatrix` instance.
    ///
    /// # Arguments
    /// * `points` - The points to use for calculating distances.
    /// * `metric` - The distance metric to use.
    ///
    /// # Returns
    /// A new `DistanceMatrix` instance.
    #[must_use]
    pub fn new<'a, P>(points: &'a [P], metric: &'a DistanceMetric) -> Self
    where
        P: Point<F>,
    {
        let n_points = points.len();
        let size = n_points * 2 - 1;
        let capacity = size * (size + 1) / 2;
        let mut distances = vec![F::max_value(); capacity];
        for i in 0..n_points {
            for j in (i + 1)..n_points {
                let index = capacity - (size + 1 - i) * (size - i) / 2 + j - i;
                let distance = metric.measure(&points[i], &points[j]);
                distances[index] = distance;
            }
        }

        Self { distances, size }
    }

    /// Returns the size of this distance matrix.
    ///
    /// # Returns
    /// The size of this distance matrix.
    #[must_use]
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns the distance between the points with the given indices.
    ///
    /// # Arguments
    /// * `i` - The index of the 1st point.
    /// * `j` - The index of the 2nd point.
    ///
    /// # Returns
    /// The distance between the points with the given indices.
    #[must_use]
    fn get(&self, i: usize, j: usize) -> F {
        let index = self.index(i, j);
        self.distances[index]
    }

    /// Sets the distance between the points with the given indices.
    ///
    /// # Arguments
    /// * `i` - The index of the 1st point.
    /// * `j` - The index of the 2nd point.
    fn set(&mut self, i: usize, j: usize, value: F) {
        let index = self.index(i, j);
        self.distances[index] = value;
    }

    /// Returns the index of the distance between the points with the given indices.
    ///
    /// # Arguments
    /// * `i` - The index of the 1st point.
    /// * `j` - The index of the 2nd point.
    ///
    /// # Returns
    /// The index of the distance between the points with the given indices.
    #[must_use]
    fn index(&self, i: usize, j: usize) -> usize {
        let min_index = i.min(j);
        let max_index = i.max(j);
        self.distances.len() - (self.size - min_index + 1) * (self.size - min_index) / 2 + max_index
            - min_index
    }
}

#[derive(Debug)]
pub struct SingleLinkage<F>
where
    F: Float,
{
    matrix: DistanceMatrix<F>,
    next_index: usize,
}

impl<F> SingleLinkage<F>
where
    F: Float,
{
    #[must_use]
    pub fn new<'a, P>(points: &'a [P], metric: &'a DistanceMetric) -> Self
    where
        P: Point<F>,
    {
        Self {
            matrix: DistanceMatrix::new(points, metric),
            next_index: points.len(),
        }
    }
}

impl<F> Linkage<F> for SingleLinkage<F>
where
    F: Float,
{
    #[must_use]
    fn distance(&self, i: usize, j: usize) -> F {
        self.matrix.get(i, j)
    }

    #[must_use]
    fn merge(&mut self, i: usize, j: usize) -> usize {
        assert!(i < j, "i must be less than j: {} < {}", i, j);

        let label = self.next_index;
        for k in 0..label {
            let distance1 = self.distance(i, k);
            let distance2 = self.distance(j, k);
            self.matrix.set(k, label, distance1.min(distance2));
        }
        self.next_index += 1;
        label
    }
}

/// Struct representing a complete linkage.
///
/// # Type Parameters
/// * `F` - The float type used for calculations (e.g., f32 or f64).
#[derive(Debug)]
pub struct CompleteLinkage<F>
where
    F: Float,
{
    matrix: DistanceMatrix<F>,
    next_index: usize,
}

impl<F> CompleteLinkage<F>
where
    F: Float,
{
    /// Creates a new `CompleteLinkage` instance.
    ///
    /// # Type Parameters
    /// * `P` - The type of points.
    ///
    /// # Arguments
    /// * `points` - The points to use for calculating distances.
    /// * `metric` - The distance metric to use.
    ///
    /// # Returns
    /// A new `CompleteLinkage` instance.
    #[must_use]
    pub fn new<'a, P>(points: &'a [P], metric: &'a DistanceMetric) -> Self
    where
        P: Point<F>,
    {
        Self {
            matrix: DistanceMatrix::new(points, metric),
            next_index: points.len(),
        }
    }
}

impl<F> Linkage<F> for CompleteLinkage<F>
where
    F: Float,
{
    #[must_use]
    fn distance(&self, i: usize, j: usize) -> F {
        self.matrix.get(i, j)
    }

    #[must_use]
    fn merge(&mut self, i: usize, j: usize) -> usize {
        assert!(i < j, "i must be less than j: {} < {}", i, j);

        let label = self.next_index;
        for k in 0..label {
            let distance1 = self.distance(i, k);
            let distance2 = self.distance(j, k);
            if i == k {
                self.matrix.set(k, label, distance2);
            } else if j == k {
                self.matrix.set(k, label, distance1);
            } else {
                self.matrix.set(k, label, distance1.max(distance2));
            }
        }
        self.next_index += 1;
        label
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::point::Point2;

    #[test]
    fn test_single_linkage() {
        let points = vec![
            Point2(0.0, 0.0),
            Point2(1.0, 0.0),
            Point2(0.0, 1.0),
            Point2(1.0, 1.0),
        ];
        let mut linkage = SingleLinkage::new(&points, &DistanceMetric::SquaredEuclidean);
        assert_eq!(linkage.distance(0, 1), 1.0);
        assert_eq!(linkage.distance(0, 2), 1.0);
        assert_eq!(linkage.distance(0, 3), 2.0);
        assert_eq!(linkage.distance(1, 2), 2.0);
        assert_eq!(linkage.distance(1, 3), 1.0);
        assert_eq!(linkage.distance(2, 3), 1.0);
        assert_eq!(linkage.distance(0, 4), f64::MAX);
        assert_eq!(linkage.distance(0, 5), f64::MAX);

        assert_eq!(linkage.merge(0, 1), 4);
        assert_eq!(linkage.distance(0, 4), 1.0);
        assert_eq!(linkage.distance(1, 4), 1.0);
        assert_eq!(linkage.distance(2, 4), 1.0);
        assert_eq!(linkage.distance(3, 4), 1.0);
        assert_eq!(linkage.distance(4, 5), f64::MAX);
    }

    #[test]
    fn test_complete_linkage() {
        let points = vec![
            Point2(0.0, 0.0),
            Point2(1.0, 0.0),
            Point2(0.0, 1.0),
            Point2(1.0, 1.0),
        ];
        let mut linkage = CompleteLinkage::new(&points, &DistanceMetric::SquaredEuclidean);
        assert_eq!(linkage.distance(0, 1), 1.0);
        assert_eq!(linkage.distance(0, 2), 1.0);
        assert_eq!(linkage.distance(0, 3), 2.0);
        assert_eq!(linkage.distance(1, 2), 2.0);
        assert_eq!(linkage.distance(1, 3), 1.0);
        assert_eq!(linkage.distance(2, 3), 1.0);
        assert_eq!(linkage.distance(0, 4), f64::MAX);
        assert_eq!(linkage.distance(0, 5), f64::MAX);

        assert_eq!(linkage.merge(0, 1), 4);
        assert_eq!(linkage.distance(0, 4), 1.0);
        assert_eq!(linkage.distance(1, 4), 1.0);
        assert_eq!(linkage.distance(2, 4), 2.0);
        assert_eq!(linkage.distance(3, 4), 2.0);
        assert_eq!(linkage.distance(4, 5), f64::MAX);
    }
}

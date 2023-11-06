use crate::number::Float;
use std::collections::HashSet;

/// Trait representing a linkage.
///
/// # Type Parameters
/// * `F` - The float type used for calculations (e.g., f32 or f64).
pub trait Linkage<F>
where
    F: Float,
{
    /// Returns the distance between the dataset with the given indices.
    ///
    /// # Arguments
    /// * `i` - The index of the 1st dataset.
    /// * `j` - The index of the 2nd dataset.
    ///
    /// # Returns
    /// The distance between the dataset with the given indices.
    #[must_use]
    fn distance(&self, i: usize, j: usize) -> F;

    /// Merges the dataset with the given indices.
    ///
    /// # Arguments
    /// * `i` - The index of the 1st dataset.
    /// * `j` - The index of the 2nd dataset.
    ///
    /// # Returns
    /// The new label of the merged dataset.
    #[must_use]
    fn merge(&mut self, i: usize, j: usize) -> usize;
}

/// Struct representing a distance matrix.
///
/// # Type Parameters
/// * `F` - The float type used for calculations (e.g., f32 or f64).
#[derive(Debug, PartialEq)]
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
    /// * `dataset` - The dataset to use for calculating distances.
    /// * `distance_fn` - The distance function to use.
    ///
    /// # Returns
    /// A new `DistanceMatrix` instance.
    ///
    /// # Type Parameters
    /// * `T` - The type of the elements in the dataset.
    /// * `DF` - The type of the distance function.
    #[must_use]
    fn new<'a, T, DF>(dataset: &'a [T], distance_fn: &'a DF) -> Self
    where
        DF: Fn(&T, &T) -> F,
    {
        let n_elements = dataset.len();
        let size = n_elements * 2 - 1;
        let capacity = size * (size + 1) / 2;
        let mut distances = vec![F::max_value(); capacity];
        for i in 0..n_elements {
            for j in (i + 1)..n_elements {
                let index = capacity - (size + 1 - i) * (size - i) / 2 + j - i;
                let distance = distance_fn(&dataset[i], &dataset[j]);
                distances[index] = distance;
            }
        }
        Self { distances, size }
    }

    /// Returns the distance between the dataset with the given indices.
    ///
    /// # Arguments
    /// * `i` - The index of the 1st dataset.
    /// * `j` - The index of the 2nd dataset.
    ///
    /// # Returns
    /// The distance between the dataset with the given indices.
    #[inline]
    #[must_use]
    fn get(&self, i: usize, j: usize) -> F {
        let index = self.index(i, j);
        self.distances[index]
    }

    /// Sets the distance between the dataset with the given indices.
    ///
    /// # Arguments
    /// * `i` - The index of the 1st dataset.
    /// * `j` - The index of the 2nd dataset.
    #[inline]
    fn set(&mut self, i: usize, j: usize, value: F) {
        let index = self.index(i, j);
        self.distances[index] = value;
    }

    /// Returns the index of the distance between the dataset with the given indices.
    ///
    /// # Arguments
    /// * `i` - The index of the 1st dataset.
    /// * `j` - The index of the 2nd dataset.
    ///
    /// # Returns
    /// The index of the distance between the dataset with the given indices.
    ///
    /// # Panics
    /// Panics if `i` or `j` is out of range.
    #[inline]
    #[must_use]
    fn index(&self, i: usize, j: usize) -> usize {
        assert!(
            i < self.size,
            "i must be in range [0, {}): {}",
            self.size,
            i
        );
        assert!(
            j < self.size,
            "j must be in range [0, {}): {}",
            self.size,
            j
        );

        let min_index = i.min(j);
        let max_index = i.max(j);
        self.distances.len() - (self.size - min_index + 1) * (self.size - min_index) / 2 + max_index
            - min_index
    }
}

/// Struct representing a single linkage.
///
/// # Type Parameters
/// * `F` - The float type used for calculations (e.g., f32 or f64).
#[derive(Debug, PartialEq)]
pub struct SingleLinkage<F>
where
    F: Float,
{
    matrix: DistanceMatrix<F>,
    inactive: HashSet<usize>,
    next_index: usize,
}

impl<F> SingleLinkage<F>
where
    F: Float,
{
    /// Creates a new `SingleLinkage` instance.
    ///
    /// # Type Parameters
    /// * `T` - The type of points.
    #[must_use]
    pub fn new<'a, T, DF>(points: &'a [T], distance_fn: &'a DF) -> Self
    where
        DF: Fn(&T, &T) -> F,
    {
        Self {
            matrix: DistanceMatrix::new(points, distance_fn),
            inactive: HashSet::new(),
            next_index: points.len(),
        }
    }
}

impl<F> Linkage<F> for SingleLinkage<F>
where
    F: Float,
{
    #[inline]
    #[must_use]
    fn distance(&self, i: usize, j: usize) -> F {
        if self.inactive.contains(&i) || self.inactive.contains(&j) {
            return F::max_value();
        }
        self.matrix.get(i, j)
    }

    #[inline]
    #[must_use]
    fn merge(&mut self, i: usize, j: usize) -> usize {
        assert!(i < j, "i must be less than j: {} < {}", i, j);

        let label = self.next_index;
        for k in 0..label {
            let distance1 = self.distance(i, k);
            let distance2 = self.distance(j, k);
            self.matrix.set(k, label, distance1.min(distance2));
        }

        self.inactive.insert(i);
        self.inactive.insert(j);
        self.next_index += 1;
        label
    }
}

/// Struct representing a complete linkage.
///
/// # Type Parameters
/// * `F` - The float type used for calculations (e.g., f32 or f64).
#[derive(Debug, PartialEq)]
pub struct CompleteLinkage<F>
where
    F: Float,
{
    matrix: DistanceMatrix<F>,
    inactive: HashSet<usize>,
    next_index: usize,
}

impl<F> CompleteLinkage<F>
where
    F: Float,
{
    /// Creates a new `CompleteLinkage` instance.
    ///
    /// # Arguments
    /// * `dataset` - The dataset to use for calculating distances.
    /// * `distance_fn` - The distance function to use.
    //
    /// # Returns
    /// A new `CompleteLinkage` instance.
    ///
    /// # Type Parameters
    /// * `T` - The type of the elements in the dataset.
    /// * `DF` - The type of the distance function.
    #[must_use]
    pub fn new<'a, T, DF>(dataset: &'a [T], distance_fn: &'a DF) -> Self
    where
        DF: Fn(&T, &T) -> F,
    {
        Self {
            matrix: DistanceMatrix::new(dataset, distance_fn),
            inactive: HashSet::new(),
            next_index: dataset.len(),
        }
    }
}

impl<F> Linkage<F> for CompleteLinkage<F>
where
    F: Float,
{
    #[inline]
    #[must_use]
    fn distance(&self, i: usize, j: usize) -> F {
        if self.inactive.contains(&i) || self.inactive.contains(&j) {
            return F::max_value();
        }
        self.matrix.get(i, j)
    }

    #[inline]
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

        self.inactive.insert(i);
        self.inactive.insert(j);
        self.next_index += 1;
        label
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_matrix_new() {
        let dataset = vec![1.0, 0.0, 10.0];
        let distance_matrix = DistanceMatrix::new(&dataset, &|x1: &f64, x2: &f64| (x1 - x2).abs());

        assert_eq!(
            distance_matrix,
            DistanceMatrix {
                distances: vec![
                    f64::MAX, // 0, 0
                    1.0,      // 0, 1
                    9.0,      // 0, 2
                    f64::MAX, // 0, 3
                    f64::MAX, // 0, 4
                    f64::MAX, // 1, 1
                    10.0,     // 1, 2
                    f64::MAX, // 1, 3
                    f64::MAX, // 1, 4
                    f64::MAX, // 2, 2
                    f64::MAX, // 2, 3
                    f64::MAX, // 2, 4
                    f64::MAX, // 3, 3
                    f64::MAX, // 3, 4
                    f64::MAX, // 4, 4
                ],
                size: 5,
            }
        );
    }

    #[test]
    fn test_distance_matrix_get() {
        let dataset = vec![1.0, 0.0, 10.0];
        let distance_matrix = DistanceMatrix::new(&dataset, &|x1: &f64, x2: &f64| (x1 - x2).abs());

        assert_eq!(distance_matrix.get(0, 0), f64::MAX);
        assert_eq!(distance_matrix.get(0, 1), 1.0);
        assert_eq!(distance_matrix.get(0, 2), 9.0);
        assert_eq!(distance_matrix.get(1, 2), 10.0);
        assert_eq!(distance_matrix.get(3, 4), f64::MAX);
    }

    #[test]
    fn test_distance_matrix_set() {
        let dataset = vec![1.0, 0.0, 10.0];
        let mut distance_matrix =
            DistanceMatrix::new(&dataset, &|x1: &f64, x2: &f64| (x1 - x2).abs());
        distance_matrix.set(0, 3, 2.0);
        distance_matrix.set(1, 2, 3.0);

        assert_eq!(distance_matrix.get(0, 3), 2.0);
        assert_eq!(distance_matrix.get(1, 2), 3.0);
    }

    #[test]
    fn test_distance_matrix_index() {
        let dataset = vec![1.0, 0.0, 10.0];
        let distance_matrix = DistanceMatrix::new(&dataset, &|x1: &f64, x2: &f64| (x1 - x2).abs());

        assert_eq!(distance_matrix.index(0, 0), 0);
        assert_eq!(distance_matrix.index(0, 1), 1);
        assert_eq!(distance_matrix.index(0, 2), 2);
        assert_eq!(distance_matrix.index(1, 2), 6);
        assert_eq!(distance_matrix.index(3, 4), 13);
    }

    #[test]
    #[should_panic(expected = "i must be in range [0, 5): 5")]
    fn test_distance_matrix_index_panic_i() {
        let dataset = vec![1.0, 0.0, 10.0];
        let distance_matrix = DistanceMatrix::new(&dataset, &|x1: &f64, x2: &f64| (x1 - x2).abs());
        let _ = distance_matrix.index(5, 0);
    }

    #[test]
    #[should_panic(expected = "j must be in range [0, 5): 5")]
    fn test_distance_matrix_index_panic_j() {
        let dataset = vec![1.0, 0.0, 10.0];
        let distance_matrix = DistanceMatrix::new(&dataset, &|x1: &f64, x2: &f64| (x1 - x2).abs());
        let _ = distance_matrix.index(0, 5);
    }

    #[test]
    fn test_single_linkage_new() {
        let dataset = vec![1.0, 0.0, 10.0];
        let distance_fn = |x1: &f64, x2: &f64| (x1 - x2).abs();
        let linkage = SingleLinkage::new(&dataset, &distance_fn);

        assert_eq!(linkage.matrix, DistanceMatrix::new(&dataset, &distance_fn));
        assert_eq!(linkage.inactive, HashSet::new());
        assert_eq!(linkage.next_index, 3);
    }

    #[test]
    fn test_single_linkage_distance() {
        let dataset = vec![1.0, 0.0, 10.0];
        let linkage = SingleLinkage::new(&dataset, &|x1: &f64, x2: &f64| (x1 - x2).abs());

        assert_eq!(linkage.distance(0, 0), f64::MAX);
        assert_eq!(linkage.distance(0, 1), 1.0);
        assert_eq!(linkage.distance(0, 2), 9.0);
        assert_eq!(linkage.distance(1, 1), f64::MAX);
        assert_eq!(linkage.distance(1, 2), 10.0);
        assert_eq!(linkage.distance(3, 4), f64::MAX);
    }

    #[test]
    fn test_single_linkage_merge() {
        let dataset = vec![1.0, 0.0, 10.0];
        let mut linkage = SingleLinkage::new(&dataset, &|x1: &f64, x2: &f64| (x1 - x2).abs());

        assert_eq!(linkage.merge(0, 1), 3);
        assert_eq!(linkage.distance(0, 3), f64::MAX);
        assert_eq!(linkage.distance(1, 3), f64::MAX);
        assert_eq!(linkage.distance(2, 3), 9.0);

        assert_eq!(linkage.merge(2, 3), 4);
        assert_eq!(linkage.distance(0, 4), f64::MAX);
        assert_eq!(linkage.distance(1, 4), f64::MAX);
        assert_eq!(linkage.distance(2, 4), f64::MAX);
        assert_eq!(linkage.distance(3, 4), f64::MAX);
    }

    #[test]
    fn test_complete_linkage_new() {
        let dataset = vec![1.0, 0.0, 10.0];
        let distance_fn = |x1: &f64, x2: &f64| (x1 - x2).abs();
        let linkage = CompleteLinkage::new(&dataset, &distance_fn);

        assert_eq!(linkage.matrix, DistanceMatrix::new(&dataset, &distance_fn));
        assert_eq!(linkage.inactive, HashSet::new());
        assert_eq!(linkage.next_index, 3);
    }

    #[test]
    fn test_complete_linkage_distance() {
        let dataset = vec![1.0, 0.0, 10.0];
        let distance_fn = |x1: &f64, x2: &f64| (x1 - x2).abs();
        let linkage = CompleteLinkage::new(&dataset, &distance_fn);

        assert_eq!(linkage.distance(0, 0), f64::MAX);
        assert_eq!(linkage.distance(0, 1), 1.0);
        assert_eq!(linkage.distance(0, 2), 9.0);
        assert_eq!(linkage.distance(1, 1), f64::MAX);
        assert_eq!(linkage.distance(1, 2), 10.0);
        assert_eq!(linkage.distance(3, 4), f64::MAX);
    }

    #[test]
    fn test_complete_linkage_merge() {
        let dataset = vec![1.0, 0.0, 10.0];
        let mut linkage = CompleteLinkage::new(&dataset, &|x1: &f64, x2: &f64| (x1 - x2).abs());

        assert_eq!(linkage.merge(0, 1), 3);
        assert_eq!(linkage.distance(0, 3), f64::MAX);
        assert_eq!(linkage.distance(1, 3), f64::MAX);
        assert_eq!(linkage.distance(2, 3), 10.0);

        assert_eq!(linkage.merge(2, 3), 4);
        assert_eq!(linkage.distance(0, 4), f64::MAX);
        assert_eq!(linkage.distance(1, 4), f64::MAX);
        assert_eq!(linkage.distance(2, 4), f64::MAX);
        assert_eq!(linkage.distance(3, 4), f64::MAX);
    }
}

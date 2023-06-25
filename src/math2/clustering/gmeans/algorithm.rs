use crate::math::stats::{anderson_darling_test, standardize};
use crate::math2::clustering::algorithm::ClusteringAlgorithm;
use crate::math2::clustering::gmeans::cmp::SizeOrdered;
use crate::math2::distance::DistanceMetric;
use crate::math2::neighbors::linear::search::LinearSearch;
use crate::math2::neighbors::search::NeighborSearch;
use crate::number::Float;
use ndarray::{Array1, Array2, ArrayView1, ArrayView2, CowArray, Ix2, NdFloat};
use std::collections::BinaryHeap;

/// Struct representing the G-Means clustering algorithm.
///
/// # Type parameters
/// * `F` - The float type used for calculations.
pub struct GMeans<'a, F>
where
    F: Float,
{
    max_k: usize,
    max_iter: usize,
    min_cluster_size: usize,
    tolerance: F,
    metric: &'a DistanceMetric,
}

impl<'a, F> GMeans<'a, F>
where
    F: Float + NdFloat,
{
    /// Creates a new `GMeans` instance.
    ///
    /// # Arguments
    /// * `max_k` - The maximum number of clusters.
    /// * `max_iter` - The maximum number of iterations.
    /// * `min_cluster_size` - The minimum number of points required to form a cluster.
    /// * `tolerance` - The minimum change in cluster centroids required to continue iterating.
    /// * `metric` - The distance metric to use for calculating distances between points.
    ///
    /// # Returns
    /// A new `GMeans` instance.
    ///
    /// # Panics
    /// * `max_k` must be greater than 2.
    #[allow(unused)]
    #[must_use]
    pub fn new(
        max_k: usize,
        max_iter: usize,
        min_cluster_size: usize,
        tolerance: F,
        metric: &'a DistanceMetric,
    ) -> Self {
        assert!(max_k > 2, "max_k must be greater than 2");
        Self {
            max_k,
            max_iter,
            min_cluster_size,
            tolerance,
            metric,
        }
    }

    #[inline]
    fn assign(
        &self,
        points: &CowArray<F, Ix2>,
        indices: &[usize],
        clusters: &mut [Cluster<F>],
    ) -> bool {
        let mut centroids = Array2::from_elem((clusters.len(), points.ncols()), F::zero());
        for (index, cluster) in clusters.iter_mut().enumerate() {
            centroids.row_mut(index).assign(&cluster.centroid());
            cluster.clear();
        }

        let nns = LinearSearch::new(centroids.clone().into(), self.metric);
        for &index in indices.iter() {
            let point = points.row(index);
            let Some(nearest) = nns.search_nearest(&point) else {
                continue;
            };
            clusters[nearest.index].insert(index, &point);
        }

        let mut converged = true;
        for (cluster, old_centroid) in clusters.iter_mut().zip(centroids.outer_iter()) {
            if cluster.is_empty() {
                continue;
            }

            let difference = self.metric.measure(&cluster.centroid(), &old_centroid);
            if difference > self.tolerance {
                converged = false;
            }
        }
        converged
    }

    #[must_use]
    fn split(&self, cluster: &Cluster<F>, points: &CowArray<F, Ix2>) -> (Cluster<F>, Cluster<F>) {
        let membership = cluster.membership();
        let mut clusters = Vec::with_capacity(2);
        for index in 0..2 {
            let membership_index = cluster.size() * (1 + index) / 3;
            let centroid_index = membership[membership_index];
            let centroid = points.row(centroid_index);
            clusters.push(Cluster::new(centroid.to_owned(), Vec::new()));
        }

        for _ in 0..self.max_iter {
            if self.assign(points, membership, &mut clusters) {
                break;
            }
        }
        (clusters[0].clone(), clusters[1].clone())
    }
}

impl<'a, F> ClusteringAlgorithm<F> for GMeans<'a, F>
where
    F: Float + NdFloat,
{
    type Output = Vec<Cluster<F>>;

    #[must_use]
    fn fit(&self, points: ArrayView2<'_, F>) -> Self::Output {
        if points.is_empty() {
            return Vec::new();
        }

        let points: CowArray<F, Ix2> = points.into();
        let median = points.nrows() / 2;
        let centroid = points.row(median).to_owned();
        let membership: Vec<_> = (0..points.nrows()).collect();
        let mut clusters = vec![Cluster::new(centroid, Vec::new())];
        self.assign(&points, &membership, &mut clusters);

        let mut heap = BinaryHeap::with_capacity(self.max_k);
        if let Some(cluster) = clusters.pop() {
            heap.push(SizeOrdered::new(cluster.size(), cluster));
        }

        while clusters.len() < self.max_k {
            let Some(largest) = heap.pop() else {
                break;
            };

            if largest.size() < self.min_cluster_size || largest.size() <= 1 {
                break;
            }

            let (cluster1, cluster2) = self.split(largest.data(), &points);
            let centroid1 = cluster1.centroid();
            let centroid2 = cluster2.centroid();

            // Anderson Darling test
            let v = &centroid1 - &centroid2;
            let vp = v.dot(&v);
            let mut x = Vec::with_capacity(largest.size());
            for &index in largest.data().membership().iter() {
                let point = points.row(index);
                x.push(point.dot(&v) / vp);
            }
            standardize(&mut x);
            let Some(score) = anderson_darling_test(&x) else {
                break;
            };
            if score < F::from_f64(1.8692) {
                clusters.push(cluster1);
                clusters.push(cluster2);
            } else {
                heap.push(SizeOrdered::new(cluster1.size(), cluster1));
                heap.push(SizeOrdered::new(cluster2.size(), cluster2));
            }
        }
        clusters
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cluster<F>
where
    F: Float,
{
    centroid: Array1<F>,
    membership: Vec<usize>,
}

impl<F> Cluster<F>
where
    F: Float + NdFloat,
{
    #[must_use]
    pub fn new(centroid: Array1<F>, membership: Vec<usize>) -> Self {
        Self {
            centroid,
            membership,
        }
    }

    #[inline]
    #[must_use]
    pub fn centroid(&self) -> ArrayView1<F> {
        self.centroid.view()
    }

    #[inline]
    #[must_use]
    pub fn membership(&self) -> &[usize] {
        &self.membership
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.membership.is_empty()
    }

    #[inline]
    #[must_use]
    pub fn size(&self) -> usize {
        self.membership.len()
    }

    #[inline]
    pub fn insert(&mut self, index: usize, point: &ArrayView1<F>) {
        let diff = point - &self.centroid;
        let size = F::from_usize(self.membership.len() + 1);
        self.centroid += &(&diff / size);
        self.membership.push(index);
    }

    #[inline]
    pub fn clear(&mut self) {
        self.centroid.fill(F::zero());
        self.membership.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_gmeans() {
        let gmeans = GMeans::new(5, 10, 2, 0.01_f64, &DistanceMetric::Euclidean);
        assert_eq!(gmeans.max_k, 5);
        assert_eq!(gmeans.max_iter, 10);
        assert_eq!(gmeans.min_cluster_size, 2);
        assert_eq!(gmeans.tolerance, 0.01_f64);
        assert_eq!(gmeans.metric, &DistanceMetric::Euclidean);
    }

    #[test]
    fn test_fit() {
        let points = array![
            [1.0, 1.0],
            [3.5, 5.0],
            [0.0, 1.0],
            [0.0, 0.0],
            [5.0, 4.0],
            [5.0, 6.0],
            [1.0, 0.0],
        ];
        let gmeans = GMeans::new(5, 10, 2, 0.01_f64, &DistanceMetric::Euclidean);
        let actual = gmeans.fit(points.view());

        assert_eq!(actual.len(), 2);
        assert_eq!(actual[0], Cluster::new(array![0.5, 0.5], vec![0, 2, 3, 6]));
        assert_eq!(actual[1], Cluster::new(array![4.5, 5.0], vec![1, 4, 5]));
    }

    #[test]
    fn test_fit_empty() {
        let points = Array2::zeros((0, 2));
        let gmeans = GMeans::new(5, 10, 2, 0.01_f64, &DistanceMetric::Euclidean);
        let actual = gmeans.fit(points.view());

        assert_eq!(actual.len(), 0);
    }
}

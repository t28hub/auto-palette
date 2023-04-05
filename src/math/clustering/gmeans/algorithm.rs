use crate::math::clustering::algorithm::ClusteringAlgorithm;
use crate::math::clustering::cluster::Cluster;
use crate::math::clustering::gmeans::cmp::SizeOrdered;
use crate::math::clustering::model::Model;
use crate::math::distance::Distance;
use crate::math::neighbors::linear::search::LinearSearch;
use crate::math::neighbors::search::NeighborSearch;
use crate::math::number::Float;
use crate::math::point::Point;
use statrs::distribution::{ContinuousCDF, Normal};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

/// Struct representing G-means clustering algorithm.
///
/// # Type Parameters
/// * `F` - The float type used for calculations.
///
/// # References
/// * [The Gaussian-means (G-means) algorithm](https://proceedings.neurips.cc/paper_files/paper/2003/file/234833147b97bb6aed53a8f4f1c7a7d8-Paper.pdf)
#[derive(Debug, PartialEq)]
pub struct Gmeans<F>
where
    F: Float,
{
    max_k: usize,
    max_iter: usize,
    min_cluster_size: usize,
    tolerance: F,
    distance: Distance,
}

impl<F> Gmeans<F>
where
    F: Float,
{
    /// Creates a new `Gmeans` instance.
    ///
    /// # Arguments
    /// * `max_k` - The maximum number of clusters.
    /// * `max_iter` - The maximum number of iterations.
    /// * `min_cluster_size` - The minimum number of points required to form a cluster.
    /// * `tolerance` - The minimum change in cluster centroids required to continue iterating.
    /// * `distance` - The distance metric to use for calculating distances between points.
    ///
    /// # Returns
    /// A new `Gmeans` instance.
    #[must_use]
    pub fn new(
        max_k: usize,
        max_iter: usize,
        min_cluster_size: usize,
        tolerance: F,
        distance: Distance,
    ) -> Self {
        assert!(
            max_k >= 2,
            "The maximum number of clusters must be at least 2."
        );
        Self {
            max_k,
            max_iter,
            min_cluster_size,
            tolerance,
            distance,
        }
    }

    #[must_use]
    fn split<P: Point<F>>(
        &self,
        cluster: &Cluster<F, P>,
        dataset: &[P],
    ) -> (Cluster<F, P>, Cluster<F, P>) {
        let membership = cluster.membership();
        let mut clusters = Vec::with_capacity(2);
        for i in 0..2 {
            let index = cluster.size() * (i + 1) / 3;
            let centroid_index = membership[index];
            let centroid = dataset[centroid_index];
            clusters.push(Cluster::new(centroid));
        }

        for _ in 0..self.max_iter {
            let converged = self.assign(&mut clusters, membership, dataset);
            if converged {
                break;
            }
        }
        (clusters[0].clone(), clusters[1].clone())
    }

    #[must_use]
    fn assign<P: Point<F>>(
        &self,
        clusters: &mut [Cluster<F, P>],
        indices: &[usize],
        dataset: &[P],
    ) -> bool {
        let mut centroids = Vec::with_capacity(clusters.len());
        for cluster in clusters.iter_mut() {
            centroids.push(*cluster.centroid());
            cluster.clear();
        }

        // Use the linear search algorithm because the number of centroids is only 2.
        let neighbor_search = LinearSearch::new(&centroids, self.distance);
        for index in indices.iter() {
            let point = dataset[*index];
            let Some(nearest) = neighbor_search.search_nearest(&point) else {
                continue;
            };
            clusters[nearest.index].insert(*index, &point);
        }

        let mut converged = false;
        for (cluster, bold_centroid) in clusters.iter_mut().zip(centroids) {
            if cluster.is_empty() {
                continue;
            }

            let difference = self.distance.measure(&bold_centroid, cluster.centroid());
            if difference < self.tolerance {
                converged = true;
            }
        }
        converged
    }
}

impl<F, P> ClusteringAlgorithm<F, P> for Gmeans<F>
where
    F: Float,
    P: Point<F>,
{
    #[must_use]
    fn train(&self, dataset: &[P]) -> Model<F, P> {
        if dataset.is_empty() {
            return Model::default();
        }

        let cluster = {
            let median = dataset.len() / 2;
            Cluster::new(dataset[median])
            // cluster.membership = (0..dataset.len()).collect();
            // cluster
        };

        let mut clusters = vec![cluster];
        let membership: Vec<usize> = (0..dataset.len()).collect();
        if self.assign(&mut clusters, &membership, dataset) {
            return Model::new(clusters, HashSet::new());
        }

        let mut heap = BinaryHeap::with_capacity(self.max_k);
        if let Some(cluster) = clusters.pop() {
            heap.push(SizeOrdered(cluster));
        }

        let mut clusters = Vec::with_capacity(self.max_k);
        while clusters.len() < self.max_k {
            let Some(largest) = heap.pop() else {
                break;
            };

            if largest.size() < self.min_cluster_size {
                continue;
            }

            let largest_cluster = &largest.0;
            let (cluster1, cluster2) = self.split(largest_cluster, dataset);
            let centroid1 = *cluster1.centroid();
            let centroid2 = *cluster2.centroid();

            // Anderson Darling test
            let v = centroid1.sub(centroid2);
            let vp = dot(&v, &v);
            let mut x = Vec::with_capacity(largest.size());
            for index in largest_cluster.membership().iter() {
                let point = dataset[*index];
                x.push(dot(&point, &v) / vp);
            }
            standardize(&mut x);
            let score = anderson_darling_test(&mut x);
            if score < F::from_f64(1.8692) {
                clusters.push(cluster1);
                clusters.push(cluster2);
            } else {
                heap.push(SizeOrdered(cluster1));
                heap.push(SizeOrdered(cluster2));
            }
        }
        Model::new(clusters, HashSet::new())
    }
}

#[inline]
#[must_use]
fn dot<F: Float, P: Point<F>>(point1: &P, point2: &P) -> F {
    let mut sum = F::zero();
    for i in 0..point1.dimension() {
        sum += point1[i] * point2[i];
    }
    sum
}

#[inline]
fn standardize<F: Float>(x: &mut [F]) {
    let n = F::from_usize(x.len());
    let mean = x.iter().fold(F::zero(), |total, value| total + *value) / n;
    let variance = x
        .iter()
        .map(|value| (*value - mean).powi(2))
        .fold(F::zero(), |total, value| total + value)
        / n;
    let sd = variance.sqrt();
    for value in x.iter_mut() {
        *value = (*value - mean) / sd;
    }
}

/// Tests whether a sample comes from a normal distribution.
///
/// [Anderson–Darling test - Wikipedia](https://en.wikipedia.org/wiki/Anderson%E2%80%93Darling_test)
#[inline]
#[must_use]
fn anderson_darling_test<F: Float>(x: &mut [F]) -> F {
    x.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

    let normal = Normal::new(0.0, 1.0).unwrap();
    for value in x.iter_mut() {
        let p = normal.cdf(value.to_f64().unwrap_or(0.0));
        *value = F::from_f64(p);
    }

    let n = x.len();
    let n_f = F::from_usize(n);
    let mut sum = F::zero();
    for i in 0..n {
        sum += F::from_usize(2 * i + 1) * (x[i].ln() + (F::one() - x[n - 1 - i]).ln());
    }
    let a_squared = sum / -n_f - n_f;
    a_squared * (F::one() + F::from_u32(4) / n_f + F::from_u32(25) / n_f.powi(2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::point::Point2;

    #[test]
    fn test_new() {
        let gmeans = Gmeans::new(5, 10, 16, 0.01_f64, Distance::Euclidean);
        assert_eq!(gmeans.max_k, 5);
        assert_eq!(gmeans.max_iter, 10);
        assert_eq!(gmeans.min_cluster_size, 16);
        assert_eq!(gmeans.tolerance, 0.01_f64);
        assert_eq!(gmeans.distance, Distance::Euclidean);
    }

    #[test]
    #[should_panic(expected = "The maximum number of clusters must be at least 2.")]
    fn test_new_panic() {
        let _ = Gmeans::new(1, 10, 2, 0.01_f64, Distance::Euclidean);
    }

    #[test]
    fn test_train() {
        let gmeans = Gmeans::new(5, 10, 2, 0.01_f64, Distance::Euclidean);
        let dataset = vec![
            Point2::new(1.0, 1.0),
            Point2::new(3.5, 5.0),
            Point2::new(0.0, 1.0),
            Point2::new(0.0, 0.0),
            Point2::new(5.0, 4.0),
            Point2::new(5.0, 6.0),
            Point2::new(1.0, 0.0),
        ];
        let actual = gmeans.train(&dataset);

        assert_eq!(actual.clusters().len(), 2);
        assert_eq!(actual.outliers().len(), 0);
        assert_eq!(actual.clusters()[0], {
            let mut cluster = Cluster::default();
            cluster.insert(0, &dataset[0]);
            cluster.insert(2, &dataset[2]);
            cluster.insert(3, &dataset[3]);
            cluster.insert(6, &dataset[6]);
            cluster
        });
        assert_eq!(actual.clusters()[1], {
            let mut cluster = Cluster::default();
            cluster.insert(1, &dataset[1]);
            cluster.insert(4, &dataset[4]);
            cluster.insert(5, &dataset[5]);
            cluster
        });
    }
}

use crate::math::clustering::algorithm::ClusteringAlgorithm;
use crate::math::clustering::cluster::Cluster;
use crate::math::clustering::gmeans::cmp::SizeOrdered;
use crate::math::distance::DistanceMetric;
use crate::math::neighbors::kdtree::search::KDTreeSearch;
use crate::math::neighbors::search::NeighborSearch;
use crate::math::number::Float;
use crate::math::point::Point;
use crate::math::stats::{anderson_darling_test, standardize};
use std::collections::BinaryHeap;

/// Struct representing G-means clustering algorithm.
///
/// # Type Parameters
/// * `F` - The float type used for calculations.
///
/// # References
/// * [The Gaussian-means (G-means) algorithm](https://proceedings.neurips.cc/paper_files/paper/2003/file/234833147b97bb6aed53a8f4f1c7a7d8-Paper.pdf)
#[derive(Debug, PartialEq)]
pub struct Gmeans<'a, F>
where
    F: Float,
{
    max_k: usize,
    max_iter: usize,
    min_cluster_size: usize,
    tolerance: F,
    metric: &'a DistanceMetric,
}

impl<'a, F> Gmeans<'a, F>
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
    /// * `metric` - The distance metric to use.
    ///
    /// # Returns
    /// A new `Gmeans` instance.
    #[must_use]
    pub fn new(
        max_k: usize,
        max_iter: usize,
        min_cluster_size: usize,
        tolerance: F,
        metric: &'a DistanceMetric,
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
            metric,
        }
    }

    #[must_use]
    fn split<P: Point<F>>(
        &self,
        cluster: &Cluster<F, P>,
        points: &[P],
    ) -> (Cluster<F, P>, Cluster<F, P>) {
        let membership = cluster.membership();
        let mut clusters = Vec::with_capacity(2);
        for i in 0..2 {
            let index = cluster.size() * (i + 1) / 3;
            let centroid_index = membership[index];
            let centroid = points[centroid_index];
            clusters.push(Cluster::new(centroid));
        }

        for _ in 0..self.max_iter {
            let converged = self.assign(&mut clusters, membership, points);
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
        points: &[P],
    ) -> bool {
        let mut centroids = Vec::with_capacity(clusters.len());
        for cluster in clusters.iter_mut() {
            centroids.push(*cluster.centroid());
            cluster.clear();
        }

        let neighbor_search = KDTreeSearch::new(&centroids, self.metric);
        for &index in indices.iter() {
            let point = &points[index];
            let Some(nearest) = neighbor_search.search_nearest(point) else {
                continue;
            };
            clusters[nearest.index].insert(index, point);
        }

        let mut converged = true;
        for (cluster, old_centroid) in clusters.iter_mut().zip(centroids) {
            if cluster.is_empty() {
                continue;
            }

            let difference = self.metric.measure(&old_centroid, cluster.centroid());
            if difference >= self.tolerance {
                converged = false;
            }
        }
        converged
    }
}

impl<'a, F, P> ClusteringAlgorithm<F, P> for Gmeans<'a, F>
where
    F: Float,
    P: Point<F>,
{
    type Output = Vec<Cluster<F, P>>;

    #[must_use]
    fn fit(&self, points: &[P]) -> Self::Output {
        if points.is_empty() {
            return Vec::new();
        }

        let cluster = {
            let median = points.len() / 2;
            Cluster::new(points[median])
        };

        let mut clusters = vec![cluster];
        let membership: Vec<usize> = (0..points.len()).collect();
        if self.assign(&mut clusters, &membership, points) {
            return clusters;
        }

        let mut heap = BinaryHeap::from_iter(clusters.into_iter().map(SizeOrdered));
        let mut clusters = Vec::with_capacity(self.max_k);
        while clusters.len() < self.max_k {
            let Some(largest) = heap.pop() else {
                break;
            };

            if largest.size() < self.min_cluster_size || largest.size() <= 1 {
                break;
            }

            let largest_cluster = &largest.0;
            let (cluster1, cluster2) = self.split(largest_cluster, points);
            let centroid1 = cluster1.centroid();
            let centroid2 = cluster2.centroid();

            // Anderson Darling test
            let v = centroid1.sub(centroid2);
            let vp = v.dot(&v);
            let mut x = Vec::with_capacity(largest.size());
            for &index in largest_cluster.membership().iter() {
                let point = &points[index];
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
                heap.push(SizeOrdered(cluster1));
                heap.push(SizeOrdered(cluster2));
            }
        }
        clusters
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::point::Point2;

    #[test]
    fn test_new() {
        let gmeans = Gmeans::new(5, 10, 16, 0.01_f64, &DistanceMetric::Euclidean);
        assert_eq!(gmeans.max_k, 5);
        assert_eq!(gmeans.max_iter, 10);
        assert_eq!(gmeans.min_cluster_size, 16);
        assert_eq!(gmeans.tolerance, 0.01_f64);
        assert_eq!(gmeans.metric, &DistanceMetric::Euclidean);
    }

    #[test]
    #[should_panic(expected = "The maximum number of clusters must be at least 2.")]
    fn test_new_panic() {
        let _ = Gmeans::new(1, 10, 2, 0.01_f64, &DistanceMetric::Euclidean);
    }

    #[test]
    fn test_train() {
        let gmeans = Gmeans::new(5, 10, 2, 0.01_f64, &DistanceMetric::Euclidean);
        let points = vec![
            Point2(1.0, 1.0),
            Point2(3.5, 5.0),
            Point2(0.0, 1.0),
            Point2(0.0, 0.0),
            Point2(5.0, 4.0),
            Point2(5.0, 6.0),
            Point2(1.0, 0.0),
        ];
        let clusters = gmeans.fit(&points);

        assert_eq!(clusters.len(), 2);
        assert_eq!(clusters[0], {
            let mut cluster = Cluster::default();
            cluster.insert(0, &points[0]);
            cluster.insert(2, &points[2]);
            cluster.insert(3, &points[3]);
            cluster.insert(6, &points[6]);
            cluster
        });
        assert_eq!(clusters[1], {
            let mut cluster = Cluster::default();
            cluster.insert(1, &points[1]);
            cluster.insert(4, &points[4]);
            cluster.insert(5, &points[5]);
            cluster
        });
    }
}

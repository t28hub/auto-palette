use crate::math::clustering::cluster::Cluster;
use crate::math::clustering::clustering::Clustering;
use crate::math::clustering::model::Model;
use crate::math::distance::Distance;
use crate::math::neighbors::linear_search::LinearSearch;
use crate::math::neighbors::neighbor_search::NeighborSearch;
use crate::math::number::Float;
use crate::math::point::Point;
use std::collections::HashSet;

/// Struct representing G-means clustering algorithm.
///
/// # Type Parameters
/// * `F` - The float type used for calculations.
#[derive(Debug, PartialEq)]
pub struct Gmeans<F>
where
    F: Float,
{
    max_k: usize,
    max_iter: usize,
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
    /// * `tolerance` - The minimum change in cluster centroids required to continue iterating.
    /// * `distance` - The distance metric to use for calculating distances between points.
    ///
    /// # Returns
    /// A new `Gmeans` instance.
    #[must_use]
    pub fn new(max_k: usize, max_iter: usize, tolerance: F, distance: Distance) -> Self {
        assert!(
            max_k >= 2,
            "The maximum number of clusters must be at least 2."
        );
        Self {
            max_k,
            max_iter,
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
            let data_index = membership[index];
            let centroid = dataset[data_index];
            clusters.push(Cluster::new(centroid.clone()));
        }

        for _ in 0..self.max_iter {
            let converged = self.reassign(&mut clusters, membership, dataset);
            if converged {
                break;
            }
        }
        (clusters[0].clone(), clusters[1].clone())
    }

    #[must_use]
    fn reassign<P: Point<F>>(
        &self,
        clusters: &mut [Cluster<F, P>],
        indices: &[usize],
        dataset: &[P],
    ) -> bool {
        let mut centroids = Vec::with_capacity(clusters.len());
        for cluster in clusters.iter_mut() {
            centroids.push(cluster.centroid().clone());
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

            cluster.centroid.div_assign(F::from_usize(cluster.size()));
            let difference = self.distance.measure(&bold_centroid, cluster.centroid());
            if difference < self.tolerance {
                converged = true;
            }
        }
        converged
    }
}

impl<F, P> Clustering<F, P> for Gmeans<F>
where
    F: Float,
    P: Point<F>,
{
    fn train(&self, dataset: &[P]) -> Model<F, P> {
        if dataset.is_empty() {
            return Model::default();
        }

        let cluster = {
            let mut cluster = Cluster::default();
            let median = dataset.len() / 2;
            cluster.centroid = dataset[median].clone();
            cluster.membership = (0..dataset.len()).collect();
            cluster
        };
        let mut clusters = Vec::with_capacity(self.max_k);
        clusters.push(cluster);

        let mut k = 1;
        while k < self.max_k {
            let largest = clusters
                .iter()
                .enumerate()
                .max_by(|(_, cluster1), (_, cluster2)| cluster1.size().cmp(&cluster2.size()));
            let Some((largest_index, largest_cluster)) = largest else {
                break;
            };

            let (cluster1, cluster2) = self.split(largest_cluster, dataset);
            clusters.remove(largest_index);
            clusters.push(cluster1);
            clusters.push(cluster2);

            k += 1;
        }
        Model::new(clusters, HashSet::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let gmeans = Gmeans::new(2, 10, 0.01_f64, Distance::Euclidean);
        assert_eq!(gmeans.max_k, 2);
        assert_eq!(gmeans.max_iter, 10);
        assert_eq!(gmeans.tolerance, 0.01_f64);
        assert_eq!(gmeans.distance, Distance::Euclidean);
    }

    #[test]
    #[should_panic(expected = "The maximum number of clusters must be at least 2.")]
    fn test_new_panic() {
        let _ = Gmeans::new(1, 10, 0.01_f64, Distance::Euclidean);
    }
}

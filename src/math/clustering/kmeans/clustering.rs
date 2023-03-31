use crate::math::clustering::cluster::Cluster;
use crate::math::clustering::clustering::Clustering;
use crate::math::clustering::kmeans::init::Initialization;
use crate::math::clustering::model::Model;
use crate::math::distance::Distance;
use crate::math::neighbors::kdtree::kdtree_search::KDTreeSearch;
use crate::math::neighbors::neighbor_search::NeighborSearch;
use crate::math::number::Float;
use crate::math::point::Point;
use rand::Rng;
use std::collections::HashSet;

/// Struct representing K-means clustering algorithm.
///
/// # Type Parameters
/// * `F` - The float type used for calculations (e.g., f32 or f64).
/// * `R` - The type of random number generator used for initializing centroids.
#[derive(Debug, PartialEq)]
pub struct Kmeans<F, P, R>
where
    F: Float,
    P: Point<F>,
    R: Rng + Clone,
{
    k: usize,
    max_iter: usize,
    tolerance: F,
    initialization: Initialization<F, P, R>,
}

impl<F, P, R> Kmeans<F, P, R>
where
    F: Float,
    P: Point<F>,
    R: Rng + Clone,
{
    /// Creates a new `Kmeans` instance.
    ///
    /// # Arguments
    /// * `k` - The number of clusters.
    /// * `max_iter` - The maximum number of iterations.
    /// * `tolerance` - The minimum change in cluster centroids required to continue iterating.
    /// * `initializer` - The cluster centroids initialization method.
    ///
    /// # Returns
    /// A new `Kmeans` instance.
    #[must_use]
    pub fn new(
        k: usize,
        max_iter: usize,
        tolerance: F,
        initialization: Initialization<F, P, R>,
    ) -> Self {
        Self {
            k,
            max_iter,
            tolerance,
            initialization,
        }
    }

    #[must_use]
    fn reassign(&self, dataset: &[P], clusters: &mut [Cluster<F, P>]) -> bool {
        let mut centroids = Vec::with_capacity(clusters.len());
        for cluster in clusters.iter_mut() {
            centroids.push(*cluster.centroid());
            cluster.clear();
        }

        let nns = KDTreeSearch::new(&centroids, Distance::SquaredEuclidean);
        dataset.iter().enumerate().for_each(|(index, data)| {
            let result = nns.search_nearest(data);
            if let Some(nearest) = result {
                let cluster = clusters
                    .get_mut(nearest.index)
                    .expect("No cluster is found");
                cluster.insert(index, data);
            }
        });

        let mut converged = false;
        clusters
            .iter_mut()
            .zip(centroids)
            .for_each(|(cluster, old_centroid)| {
                if cluster.is_empty() {
                    return;
                }

                cluster.centroid.div_assign(F::from_usize(cluster.size()));

                let difference =
                    Distance::SquaredEuclidean.measure(&old_centroid, cluster.centroid());
                if difference < self.tolerance {
                    converged = true;
                }
            });
        converged
    }
}

impl<F, P, R> Clustering<F, P> for Kmeans<F, P, R>
where
    F: Float,
    P: Point<F>,
    R: Rng + Clone,
{
    #[must_use]
    fn train(&self, dataset: &[P]) -> Model<F, P> {
        if self.k == 0 {
            return Model::default();
        }

        if self.k >= dataset.len() {
            let clusters = dataset
                .iter()
                .enumerate()
                .map(|(index, data)| {
                    let mut cluster = Cluster::new(index);
                    cluster.insert(index, data);
                    cluster
                })
                .collect();
            return Model::new(clusters, HashSet::new());
        }

        let mut clusters: Vec<Cluster<F, P>> = self
            .initialization
            .initialize(dataset, self.k)
            .into_iter()
            .enumerate()
            .map(|(cluster_id, centroid)| {
                let mut cluster = Cluster::new(cluster_id);
                cluster.centroid = centroid;
                cluster
            })
            .collect();
        for _ in 0..self.max_iter {
            let converged = self.reassign(dataset, &mut clusters);
            if converged {
                break;
            }
        }
        Model::new(clusters, HashSet::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::clustering::clustering::Clustering;
    use crate::math::clustering::kmeans::init::Initialization;
    use crate::math::point::Point2;
    use rand::thread_rng;

    #[test]
    fn new_should_create_kmeans() {
        let dataset = vec![
            Point2(1.0, 2.0),
            Point2(3.0, 1.0),
            Point2(4.0, 5.0),
            Point2(5.0, 5.0),
            Point2(2.0, 4.0),
        ];
        let initializer = Initialization::KmeansPlusPlus(Distance::SquaredEuclidean, thread_rng());
        let kmeans = Kmeans::new(2, 10, 0.001_f64, initializer);
        let model = kmeans.train(&dataset);
        println!("{:?}", model);
    }
}

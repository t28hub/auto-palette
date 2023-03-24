use crate::math::clustering::cluster::Cluster;
use crate::math::clustering::clustering::Clustering;
use crate::math::clustering::kmeans::params::KmeansParams;
use crate::math::distance::metric::DistanceMetric;
use crate::math::neighbors::kdtree::KDTree;
use crate::math::neighbors::nns::NeighborSearch;
use crate::math::number::Float;
use crate::math::point::Point;
use rand::Rng;
use std::marker::PhantomData;

pub struct Kmeans<F, P, R>
where
    F: Float,
    P: Point<F>,
    R: Rng + Clone,
{
    clusters: Vec<Cluster<F, P>>,
    outliers: Vec<usize>,
    _phantom: PhantomData<(F, R)>,
}

impl<F, P, R> Kmeans<F, P, R>
where
    F: Float,
    P: Point<F>,
    R: Rng + Clone,
{
    fn reassign(
        dataset: &[P],
        clusters: &mut [Cluster<F, P>],
        metric: &DistanceMetric,
        tolerance: F,
    ) -> bool {
        let mut centroids = Vec::with_capacity(clusters.len());
        for cluster in clusters.iter_mut() {
            centroids.push(*cluster.centroid());
            cluster.clear();
        }

        let nns = KDTree::new(&centroids, metric);
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

                let difference = metric.measure(&old_centroid, cluster.centroid());
                if difference < tolerance {
                    converged = true;
                }
            });
        converged
    }
}

impl<F, P, R> Default for Kmeans<F, P, R>
where
    F: Float,
    P: Point<F>,
    R: Rng + Clone,
{
    fn default() -> Self {
        Self {
            clusters: Vec::new(),
            outliers: Vec::new(),
            _phantom: PhantomData::default(),
        }
    }
}

impl<F, P, R> Clustering<F, P> for Kmeans<F, P, R>
where
    F: Float,
    P: Point<F>,
    R: Rng + Clone,
{
    type Params = KmeansParams<F, R>;

    #[must_use]
    fn fit(dataset: &[P], params: &Self::Params) -> Self {
        if params.k() == 0 {
            return Kmeans::default();
        }

        if params.k() >= dataset.len() {
            let clusters = dataset
                .iter()
                .enumerate()
                .map(|(index, data)| {
                    let mut cluster = Cluster::new(index);
                    cluster.insert(index, data);
                    cluster
                })
                .collect();
            return Self {
                clusters,
                outliers: Vec::new(),
                _phantom: PhantomData::default(),
            };
        }

        let mut clusters: Vec<Cluster<F, P>> = params
            .initializer()
            .initialize(dataset, params.k(), params.metric())
            .into_iter()
            .enumerate()
            .map(|(cluster_id, centroid)| {
                let mut cluster = Cluster::new(cluster_id);
                cluster.centroid = centroid;
                cluster
            })
            .collect();
        for _ in 0..params.max_iterations() {
            let converged =
                Self::reassign(dataset, &mut clusters, params.metric(), params.tolerance());
            if converged {
                break;
            }
        }
        Kmeans {
            clusters,
            outliers: Vec::new(),
            _phantom: PhantomData::default(),
        }
    }

    #[must_use]
    fn clusters(&self) -> &[Cluster<F, P>] {
        &self.clusters
    }

    #[must_use]
    fn outliers(&self) -> &[usize] {
        &self.outliers
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::clustering::clustering::Clustering;
    use crate::math::clustering::kmeans::init::Initializer;
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
        let metric = DistanceMetric::SquaredEuclidean;
        let initializer = Initializer::KmeansPlusPlus(thread_rng());
        let mut params = KmeansParams::new(2, metric, initializer);
        let _kmeans = Kmeans::fit(&dataset, &mut params);
    }
}

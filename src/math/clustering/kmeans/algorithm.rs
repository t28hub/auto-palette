use crate::math::clustering::algorithm::Algorithm;
use crate::math::clustering::kmeans::cluster::Cluster;
use crate::math::clustering::kmeans::params::KmeansParams;
use crate::math::distance::metric::DistanceMetric;
use crate::math::neighbors::kdtree::KDTree;
use crate::math::neighbors::nns::NeighborSearch;
use crate::math::number::Float;
use crate::math::point::Point;
use rand::Rng;
use std::marker::PhantomData;

pub struct Kmeans<F, P>
where
    F: Float,
    P: Point<F>,
{
    _t: PhantomData<F>,
    clusters: Vec<Cluster<F, P>>,
    outliers: Vec<usize>,
}

impl<F, P> Kmeans<F, P>
where
    F: Float,
    P: Point<F>,
{
    pub(crate) fn centroids(&self) -> Vec<P> {
        self.clusters
            .iter()
            .map(|cluster| -> P { *cluster.centroid() })
            .collect()
    }

    pub(crate) fn count_at(&self, index: usize) -> usize {
        let cluster = self.clusters.get(index);
        cluster.map_or(0, |c| c.size())
    }

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

                cluster.update_centroid();

                let difference = metric.measure(&old_centroid, cluster.centroid());
                if difference < tolerance {
                    converged = true;
                }
            });
        converged
    }
}

impl<F, P, R> Algorithm<F, P, KmeansParams<F, R>> for Kmeans<F, P>
where
    F: Float,
    P: Point<F>,
    R: Rng + Clone,
{
    #[must_use]
    fn fit(dataset: &[P], params: &KmeansParams<F, R>) -> Self {
        if params.k() == 0 {
            return Self {
                _t: PhantomData::default(),
                clusters: Vec::new(),
                outliers: Vec::new(),
            };
        }

        if params.k() >= dataset.len() {
            let clusters = dataset
                .iter()
                .enumerate()
                .map(|(index, data)| {
                    let mut cluster = Cluster::new(data);
                    cluster.insert(index, data);
                    cluster
                })
                .collect();
            return Self {
                _t: PhantomData::default(),
                clusters,
                outliers: Vec::new(),
            };
        }

        let mut clusters: Vec<Cluster<F, P>> = params
            .initializer()
            .initialize(dataset, params.k(), params.metric())
            .iter()
            .map(|centroid| Cluster::new(centroid))
            .collect();
        for _ in 0..params.max_iterations() {
            let converged =
                Self::reassign(dataset, &mut clusters, params.metric(), params.tolerance());
            if converged {
                break;
            }
        }
        Kmeans {
            _t: PhantomData::default(),
            clusters,
            outliers: Vec::new(),
        }
    }

    #[must_use]
    fn outliers(&self) -> &[usize] {
        &self.outliers
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::clustering::algorithm::Algorithm;
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

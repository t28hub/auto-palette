use crate::math::clustering::hdbscan::core_distance::CoreDistance;
use crate::math::clustering::hdbscan::params::Params;
use crate::math::clustering::hierarchical::algorithm::HierarchicalClustering;
use crate::math::clustering::traits::Fit;
use crate::math::number::Float;
use crate::math::point::Point;

/// HDBSCAN clustering algorithm.
#[derive(Debug, Clone)]
struct HDBSCAN {}

impl HDBSCAN {
    /// Create an HDBSCAN.
    fn new() -> Self {
        Self {}
    }
}

impl<F, P> Fit<F, P, Params> for HDBSCAN
where
    F: Float,
    P: Point<F>,
{
    fn fit(dataset: &[P], params: &Params) -> Self {
        if dataset.is_empty() {
            return HDBSCAN::new();
        }

        let core_distance = CoreDistance::new(dataset, params.min_samples(), params.metric());
        let mutual_reachability_distance = |u: usize, v: usize| -> F {
            let point_u = &dataset[u];
            let point_v = &dataset[v];
            let distance = params.metric().measure(point_u, point_v);
            distance.max(core_distance.distance_at(u).max(core_distance.distance_at(v)))
        };
        let _hierarchical_clustering = HierarchicalClustering::fit(dataset, mutual_reachability_distance);
        todo!()
    }

}

#[cfg(test)]
mod tests {
    use super::*;
}

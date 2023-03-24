use crate::math::distance::metric::DistanceMetric;

/// Parameters of DBSCAN clustering algorithm.
#[derive(Debug, PartialEq)]
pub struct HDBCANParams {
    /// The minimum number of neighboring points required for a point to be considered as a core point.
    min_samples: usize,

    /// The minimum number of points required to form a cluster.
    min_cluster_size: usize,

    /// The distance metric to calculate core distances.
    metric: DistanceMetric,
}

impl HDBCANParams {
    /// Create a params.
    #[must_use]
    pub fn new(min_samples: usize, min_cluster_size: usize, metric: DistanceMetric) -> Self {
        Self {
            min_samples,
            min_cluster_size,
            metric,
        }
    }

    pub fn min_samples(&self) -> usize {
        self.min_samples
    }

    pub fn min_cluster_size(&self) -> usize {
        self.min_cluster_size
    }

    pub fn metric(&self) -> &DistanceMetric {
        &self.metric
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_create_params() {
        let params = HDBCANParams::new(4, 25, DistanceMetric::Euclidean);
        assert_eq!(params.min_samples, 4);
        assert_eq!(params.min_cluster_size, 25);
        assert_eq!(params.metric, DistanceMetric::Euclidean);
    }
}

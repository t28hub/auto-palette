use crate::math::clustering::cluster::Cluster;
use crate::math::number::Float;
use crate::math::point::Point;
use std::collections::HashSet;

/// Struct representing a trained clustering model.
///
/// # Type Parameters
/// * `F` - The float type used for calculations (e.g., f32 or f64).
/// * `P` - The type of points used in the clustering algorithm.
#[derive(Debug, PartialEq)]
pub struct Model<F: Float, P: Point<F>> {
    clusters: Vec<Cluster<F, P>>,
    outliers: HashSet<usize>,
}

impl<F, P> Model<F, P>
where
    F: Float,
    P: Point<F>,
{
    /// Create a new `Model` instance.
    ///
    /// # Arguments
    /// * `clusters` - The clusters found in the dataset.
    /// * `outliers` - The indices of the data points that are outliers.
    ///
    /// # Returns
    /// A new `Model` instance.
    pub fn new(clusters: Vec<Cluster<F, P>>, outliers: HashSet<usize>) -> Self {
        Self { clusters, outliers }
    }

    /// Return the clusters.
    ///
    /// # Returns
    /// A reference to a slice of clusters.
    #[must_use]
    pub fn clusters(&self) -> &[Cluster<F, P>] {
        &self.clusters
    }

    /// Return the indices of outliers.
    ///
    /// # Returns
    /// A slice containing the indices of the outliers.
    #[must_use]
    pub fn outliers(&self) -> &HashSet<usize> {
        &self.outliers
    }
}

impl<F, P> Default for Model<F, P>
where
    F: Float,
    P: Point<F>,
{
    #[must_use]
    fn default() -> Self {
        Self {
            clusters: Vec::new(),
            outliers: HashSet::new(),
        }
    }
}

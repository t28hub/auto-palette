use crate::math::clustering::hierarchical::linkage::{Linkage, SingleLinkage};
use crate::math::distance::DistanceMetric;
use crate::math::point::Point;
use crate::number::Float;

pub struct HierarchicalClustering;

impl<'a> HierarchicalClustering {
    pub fn fit<F, P>(&self, points: &'a [P])
    where
        F: Float,
        P: Point<F>,
    {
        self.fit_with_linkage(
            points,
            &SingleLinkage::new(points, &DistanceMetric::Euclidean),
        )
    }

    pub fn fit_with_linkage<F, P>(&self, points: &'a [P], linkage: &impl Linkage<F>)
    where
        F: Float,
        P: Point<F>,
    {
        todo!()
    }
}

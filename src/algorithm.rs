use crate::math::clustering::clustering::Clustering;
use crate::math::clustering::dbscan::clustering::DBSCAN;
use crate::math::clustering::hdbscan::clustering::HDBSCAN;
use crate::math::clustering::model::Model;
use crate::math::distance::Distance;
use crate::math::number::Float;
use crate::math::point::Point;

pub enum Algorithm {
    DBSCAN,
    HDBSCAN,
}

impl Algorithm {
    pub(crate) fn apply<F, P>(&self, dataset: &[P]) -> Model<F, P>
    where
        F: Float,
        P: Point<F>,
    {
        match self {
            Algorithm::DBSCAN => {
                let dbscan = DBSCAN::new(9, F::from_f64(0.0025), Distance::SquaredEuclidean);
                dbscan.train(dataset)
            }
            Algorithm::HDBSCAN => {
                let hdbscan = HDBSCAN::new(9, 25, Distance::SquaredEuclidean);
                hdbscan.train(dataset)
            }
        }
    }
}

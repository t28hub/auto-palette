use crate::math::clustering::model::Model;
use crate::math::number::Float;
use crate::math::point::Point;

/// Trait representing a clustering algorithm.
pub trait Clustering<F, P>
where
    F: Float,
    P: Point<F>,
{
    /// Train the algorithm using the given dataset.
    ///
    /// # Arguments
    /// * `dataset` - The dataset to train the algorithm with.
    ///
    /// # Returns
    /// The trained model.
    #[must_use]
    fn train(&self, dataset: &[P]) -> Model<F, P>;
}

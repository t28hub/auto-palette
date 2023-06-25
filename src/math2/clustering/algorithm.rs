use crate::number::Float;
use ndarray::ArrayView2;

pub trait ClusteringAlgorithm<F>
where
    F: Float,
{
    type Output;

    #[must_use]
    fn fit(&self, points: ArrayView2<'_, F>) -> Self::Output;
}

use crate::math::clustering::cluster::Cluster;
use crate::math::number::Float;
use crate::math::point::Point;
use std::cmp::Ordering;

pub struct ReversedSize<F: Float, P: Point<F>>(pub Cluster<F, P>);

impl<F, P> ReversedSize<F, P>
where
    F: Float,
    P: Point<F>,
{
    #[must_use]
    pub fn size(&self) -> usize {
        self.0.size()
    }
}

impl<F, P> PartialOrd for ReversedSize<F, P>
where
    F: Float,
    P: Point<F>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.size().cmp(&self.size()))
    }
}

impl<F, P> Ord for ReversedSize<F, P>
where
    F: Float,
    P: Point<F>,
{
    fn cmp(&self, other: &Self) -> Ordering {
        other.size().cmp(&self.size())
    }
}

impl<F, P> PartialEq for ReversedSize<F, P>
where
    F: Float,
    P: Point<F>,
{
    fn eq(&self, other: &Self) -> bool {
        self.size() == other.size()
    }
}

impl<F, P> Eq for ReversedSize<F, P>
where
    F: Float,
    P: Point<F>,
{
}

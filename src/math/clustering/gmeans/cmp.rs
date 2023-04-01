use crate::math::clustering::cluster::Cluster;
use crate::math::number::Float;
use crate::math::point::Point;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct SizeOrder<F: Float, P: Point<F>>(pub Cluster<F, P>);

impl<F, P> SizeOrder<F, P>
where
    F: Float,
    P: Point<F>,
{
    #[must_use]
    pub fn size(&self) -> usize {
        self.0.size()
    }
}

impl<F, P> PartialOrd for SizeOrder<F, P>
where
    F: Float,
    P: Point<F>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.size().cmp(&other.size()))
    }
}

impl<F, P> Ord for SizeOrder<F, P>
where
    F: Float,
    P: Point<F>,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.size().cmp(&other.size())
    }
}

impl<F, P> PartialEq for SizeOrder<F, P>
where
    F: Float,
    P: Point<F>,
{
    fn eq(&self, other: &Self) -> bool {
        self.size() == other.size()
    }
}

impl<F, P> Eq for SizeOrder<F, P>
where
    F: Float,
    P: Point<F>,
{
}

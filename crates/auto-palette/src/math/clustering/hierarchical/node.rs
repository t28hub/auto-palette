use crate::number::Float;

#[derive(Debug, PartialEq)]
pub struct Node<F>
where
    F: Float,
{
    pub label: usize,
    pub node1: Option<usize>,
    pub node2: Option<usize>,
    pub distance: F,
}

impl<F> Node<F>
where
    F: Float,
{
    #[must_use]
    pub fn new(label: usize, node1: Option<usize>, node2: Option<usize>, distance: F) -> Self {
        Self {
            label,
            node1,
            node2,
            distance,
        }
    }
}

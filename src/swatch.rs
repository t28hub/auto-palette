use crate::math::number::Float;
use std::cmp::Ordering;

/// Color swatch.
#[derive(Clone, Debug, PartialEq)]
pub struct Swatch<F: Float> {
    /// The representative color.
    pub color: (u8, u8, u8),

    /// The position of this swatch.
    pub position: (u32, u32),

    /// The percentage of this swatch.
    pub percentage: F,
}

impl<F> Default for Swatch<F>
where
    F: Float,
{
    #[must_use]
    fn default() -> Self {
        Self {
            color: (0, 0, 0),
            position: (0, 0),
            percentage: F::zero(),
        }
    }
}

impl<F> Eq for Swatch<F> where F: Float {}

impl<F> PartialOrd for Swatch<F>
where
    F: Float,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.percentage.partial_cmp(&other.percentage)
    }
}

impl<F> Ord for Swatch<F>
where
    F: Float,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

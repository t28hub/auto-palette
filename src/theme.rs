use crate::color_trait::Color;
use crate::lab::Lab;
use crate::math::number::{Normalize, Number};
use crate::Swatch;
use num_traits::One;

/// Enum representing a color theme.
pub enum Theme {
    Dominant,
    Vivid,
    Muted,
}

impl Theme {
    /// Scores the given swatch.
    ///
    /// # Arguments
    /// * `swatch` - The swatch to score.
    ///
    /// # Returns
    /// The score of the given swatch.
    ///
    /// # Type Parameters
    /// * `C` - The type of color.
    #[inline]
    #[must_use]
    pub(crate) fn score<C>(&self, swatch: &Swatch<C>) -> C::F
    where
        C: Color,
    {
        match self {
            Self::Dominant => {
                let population = swatch.population();
                C::F::from_usize(population)
            }
            Self::Vivid => {
                let chroma: C::F = swatch.color().to_lab().chroma();
                chroma.normalize(
                    Lab::<C::F, C::WP>::min_chroma(),
                    Lab::<C::F, C::WP>::max_chroma(),
                )
            }
            Self::Muted => {
                let chroma: C::F = swatch.color().to_lab().chroma();
                let c_score = chroma.normalize(
                    Lab::<C::F, C::WP>::min_chroma(),
                    Lab::<C::F, C::WP>::max_chroma(),
                );
                C::F::one() - c_score
            }
        }
    }
}

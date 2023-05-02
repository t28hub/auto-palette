use crate::lab::Lab;
use crate::math::number::Float;
use crate::white_point::WhitePoint;

/// Enum representing a color theme.
pub enum Theme {
    Dominant,
    Vivid,
    Muted,
}

impl Theme {
    /// Scores the given color.
    ///
    /// # Arguments
    /// * `lab` - The CIE L*a*b* color to score.
    ///
    /// # Returns
    /// The score of the given color.
    ///
    /// # Type Parameters
    /// * `F` - The floating point type.
    /// * `WP` - The white point type.
    #[inline]
    #[must_use]
    pub(crate) fn score<F, WP>(&self, color: &Lab<F, WP>) -> F
    where
        F: Float,
        WP: WhitePoint<F>,
    {
        match self {
            Self::Dominant => F::one(),
            Self::Vivid => {
                let c_score = color
                    .chroma()
                    .normalize(Lab::<F, WP>::min_chroma(), Lab::<F, WP>::max_chroma());
                c_score.powi(2)
            }
            Self::Muted => {
                let c_score = color
                    .chroma()
                    .normalize(Lab::<F, WP>::min_chroma(), Lab::<F, WP>::max_chroma());
                (F::one() - c_score).powi(2)
            }
        }
    }
}

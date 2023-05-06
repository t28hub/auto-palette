use crate::color_trait::Color;
use crate::lab::Lab;
use crate::math::number::Normalize;
use crate::Swatch;
use num_traits::One;

/// Enum representing a color theme.
pub enum Theme {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::white_point::D65;
    use statrs::assert_almost_eq;

    #[test]
    fn test_vivid_score() {
        let black = Lab::<f64, D65>::new(0.0, 0.0, 0.0);
        let swatch = Swatch::new(black, (0, 0), 128);
        assert_almost_eq!(Theme::Vivid.score(&swatch), 0.0, 1e-4);

        let white = Lab::<f64, D65>::new(100.0, 0.0, 0.0);
        let swatch = Swatch::new(white, (0, 0), 128);
        assert_almost_eq!(Theme::Vivid.score(&swatch), 0.0, 1e-4);

        let magenta = Lab::<f64, D65>::new(60.32, 98.25, -60.84);
        let swatch = Swatch::new(magenta, (0, 0), 128);
        assert_almost_eq!(Theme::Vivid.score(&swatch), 0.9028, 1e-4);

        let marigold = Lab::<f64, D65>::new(71.85, 17.06, 70.08);
        let swatch = Swatch::new(marigold, (0, 0), 128);
        assert_almost_eq!(Theme::Vivid.score(&swatch), 0.5634, 1e-4);
    }

    #[test]
    fn test_muted_score() {
        let black = Lab::<f64, D65>::new(0.0, 0.0, 0.0);
        let swatch = Swatch::new(black, (0, 0), 128);
        assert_almost_eq!(Theme::Muted.score(&swatch), 1.0, 1e-4);

        let white = Lab::<f64, D65>::new(100.0, 0.0, 0.0);
        let swatch = Swatch::new(white, (0, 0), 128);
        assert_almost_eq!(Theme::Muted.score(&swatch), 1.0, 1e-4);

        let magenta = Lab::<f64, D65>::new(60.32, 98.25, -60.84);
        let swatch = Swatch::new(magenta, (0, 0), 128);
        assert_almost_eq!(Theme::Muted.score(&swatch), 0.0971, 1e-4);

        let marigold = Lab::<f64, D65>::new(71.85, 17.06, 70.08);
        let swatch = Swatch::new(marigold, (0, 0), 128);
        assert_almost_eq!(Theme::Muted.score(&swatch), 0.4365, 1e-4);
    }
}

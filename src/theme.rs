use crate::lab::Lab;
use crate::math::number::Float;
use crate::Swatch;

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
    pub(crate) fn score<F>(&self, swatch: &Swatch<F>) -> F
    where
        F: Float,
    {
        match self {
            Self::Vivid => {
                let chroma: F = swatch.color().to_lab().chroma();
                chroma.normalize(Lab::<F>::min_chroma(), Lab::<F>::max_chroma())
            }
            Self::Muted => {
                let chroma: F = swatch.color().to_lab().chroma();
                let c_score = chroma.normalize(Lab::<F>::min_chroma(), Lab::<F>::max_chroma());
                F::one() - c_score
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color_struct::Color;
    use crate::rgb::RGB;
    use statrs::assert_almost_eq;

    #[test]
    fn test_vivid_score() {
        let black = Color::<f64>::from(&RGB { r: 0, g: 0, b: 0 });
        let swatch = Swatch::new(black, (0, 0), 128);
        assert_almost_eq!(Theme::Vivid.score(&swatch), 0.0, 1e-4);

        let white = Color::<f64>::from(&RGB {
            r: 255,
            g: 255,
            b: 255,
        });
        let swatch = Swatch::new(white, (0, 0), 128);
        assert_almost_eq!(Theme::Vivid.score(&swatch), 0.0001, 1e-4);

        let magenta = Color::<f64>::from(&RGB {
            r: 255,
            g: 0,
            b: 255,
        });
        let swatch = Swatch::new(magenta, (0, 0), 128);
        assert_almost_eq!(Theme::Vivid.score(&swatch), 0.9026, 1e-4);

        let marigold = Color::<f64>::from(&RGB {
            r: 234,
            g: 162,
            b: 33,
        });
        let swatch = Swatch::new(marigold, (0, 0), 128);
        assert_almost_eq!(Theme::Vivid.score(&swatch), 0.5634, 1e-4);
    }

    #[test]
    fn test_muted_score() {
        let black = Color::<f64>::from(&RGB { r: 0, g: 0, b: 0 });
        let swatch = Swatch::new(black, (0, 0), 128);
        assert_almost_eq!(Theme::Muted.score(&swatch), 1.0, 1e-4);

        let white = Color::<f64>::from(&RGB {
            r: 255,
            g: 255,
            b: 255,
        });
        let swatch = Swatch::new(white, (0, 0), 128);
        assert_almost_eq!(Theme::Muted.score(&swatch), 0.9998, 1e-4);

        let magenta = Color::<f64>::from(&RGB {
            r: 255,
            g: 0,
            b: 255,
        });
        let swatch = Swatch::new(magenta, (0, 0), 128);
        assert_almost_eq!(Theme::Muted.score(&swatch), 0.0973, 1e-4);

        let marigold = Color::<f64>::from(&RGB {
            r: 234,
            g: 162,
            b: 33,
        });
        let swatch = Swatch::new(marigold, (0, 0), 128);
        assert_almost_eq!(Theme::Muted.score(&swatch), 0.4365, 1e-4);
    }
}

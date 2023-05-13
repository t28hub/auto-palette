use crate::lab::Lab;
use crate::math::number::{Float, Fraction};
use crate::Swatch;

/// Trait representing a theme.
pub trait Theme {
    /// Weights a swatch based on the theme.
    ///
    /// # Arguments
    /// * `swatch` - The swatch to be weighted.
    ///
    /// # Returns
    /// The weight of the swatch.
    ///
    /// # Type Parameters
    /// * `F` - The floating type for the weight.
    #[must_use]
    fn weight<F>(&self, swatch: &Swatch<F>) -> Fraction<F>
    where
        F: Float;
}

/// Struct representing a vivid theme.
pub struct Vivid;

impl Theme for Vivid {
    #[inline]
    #[must_use]
    fn weight<F>(&self, swatch: &Swatch<F>) -> Fraction<F>
    where
        F: Float,
    {
        let chroma: F = swatch.color().chroma();
        let normalized = chroma.normalize(Lab::<F>::min_chroma(), Lab::<F>::max_chroma());
        Fraction::new(normalized)
    }
}

/// Struct representing a muted theme.
pub struct Muted;

impl Theme for Muted {
    #[inline]
    #[must_use]
    fn weight<F>(&self, swatch: &Swatch<F>) -> Fraction<F>
    where
        F: Float,
    {
        let chroma: F = swatch.color().chroma();
        let normalized = chroma.normalize(Lab::<F>::min_chroma(), Lab::<F>::max_chroma());
        Fraction::new(F::one() - normalized)
    }
}

/// Struct representing a light theme.
pub struct Light;

impl Theme for Light {
    #[inline]
    #[must_use]
    fn weight<F>(&self, swatch: &Swatch<F>) -> Fraction<F>
    where
        F: Float,
    {
        let lightness = swatch.color().lightness();
        let normalized = lightness / F::from_f64(100.0);
        Fraction::new(normalized)
    }
}

/// Struct representing a dark theme.
pub struct Dark;

impl Theme for Dark {
    #[inline]
    #[must_use]
    fn weight<F>(&self, swatch: &Swatch<F>) -> Fraction<F>
    where
        F: Float,
    {
        let lightness = swatch.color().lightness();
        let normalized = lightness / F::from_f64(100.0);
        Fraction::new(F::one() - normalized)
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
        let actual = Vivid.weight(&swatch);
        assert_almost_eq!(actual.value(), 0.0, 1e-4);

        let white = Color::<f64>::from(&RGB {
            r: 255,
            g: 255,
            b: 255,
        });
        let swatch = Swatch::new(white, (0, 0), 128);
        let actual = Vivid.weight(&swatch);
        assert_almost_eq!(actual.value(), 0.0001, 1e-4);

        let magenta = Color::<f64>::from(&RGB {
            r: 255,
            g: 0,
            b: 255,
        });
        let swatch = Swatch::new(magenta, (0, 0), 128);
        let actual = Vivid.weight(&swatch);
        assert_almost_eq!(actual.value(), 0.9026, 1e-4);

        let marigold = Color::<f64>::from(&RGB {
            r: 234,
            g: 162,
            b: 33,
        });
        let swatch = Swatch::new(marigold, (0, 0), 128);
        let actual = Vivid.weight(&swatch);
        assert_almost_eq!(actual.value(), 0.5634, 1e-4);
    }

    #[test]
    fn test_muted_score() {
        let black = Color::<f64>::from(&RGB { r: 0, g: 0, b: 0 });
        let swatch = Swatch::new(black, (0, 0), 128);
        let actual = Muted.weight(&swatch);
        assert_almost_eq!(actual.value(), 1.0, 1e-4);

        let white = Color::<f64>::from(&RGB {
            r: 255,
            g: 255,
            b: 255,
        });
        let swatch = Swatch::new(white, (0, 0), 128);
        let actual = Muted.weight(&swatch);
        assert_almost_eq!(actual.value(), 0.9998, 1e-4);

        let magenta = Color::<f64>::from(&RGB {
            r: 255,
            g: 0,
            b: 255,
        });
        let swatch = Swatch::new(magenta, (0, 0), 128);
        let actual = Muted.weight(&swatch);
        assert_almost_eq!(actual.value(), 0.0973, 1e-4);

        let marigold = Color::<f64>::from(&RGB {
            r: 234,
            g: 162,
            b: 33,
        });
        let swatch = Swatch::new(marigold, (0, 0), 128);
        let actual = Muted.weight(&swatch);
        assert_almost_eq!(actual.value(), 0.4365, 1e-4);
    }

    #[test]
    fn test_light_score() {
        let black = Color::<f64>::from(&RGB { r: 0, g: 0, b: 0 });
        let swatch = Swatch::new(black, (0, 0), 128);
        let actual = Light.weight(&swatch);
        assert_almost_eq!(actual.value(), 0.0, 1e-4);

        let white = Color::<f64>::from(&RGB {
            r: 255,
            g: 255,
            b: 255,
        });
        let swatch = Swatch::new(white, (0, 0), 128);
        let actual = Light.weight(&swatch);
        assert_almost_eq!(actual.value(), 1.0, 1e-4);

        let magenta = Color::<f64>::from(&RGB {
            r: 255,
            g: 0,
            b: 255,
        });
        let swatch = Swatch::new(magenta, (0, 0), 128);
        let actual = Light.weight(&swatch);
        assert_almost_eq!(actual.value(), 0.6032, 1e-4);

        let marigold = Color::<f64>::from(&RGB {
            r: 234,
            g: 162,
            b: 33,
        });
        let swatch = Swatch::new(marigold, (0, 0), 128);
        let actual = Light.weight(&swatch);
        assert_almost_eq!(actual.value(), 0.7185, 1e-4);
    }

    #[test]
    fn test_dark_score() {
        let black = Color::<f64>::from(&RGB { r: 0, g: 0, b: 0 });
        let swatch = Swatch::new(black, (0, 0), 128);
        let actual = Dark.weight(&swatch);
        assert_almost_eq!(actual.value(), 1.0, 1e-4);

        let white = Color::<f64>::from(&RGB {
            r: 255,
            g: 255,
            b: 255,
        });
        let swatch = Swatch::new(white, (0, 0), 128);
        let actual = Dark.weight(&swatch);
        assert_almost_eq!(actual.value(), 0.0, 1e-4);

        let magenta = Color::<f64>::from(&RGB {
            r: 255,
            g: 0,
            b: 255,
        });
        let swatch = Swatch::new(magenta, (0, 0), 128);
        let actual = Dark.weight(&swatch);
        assert_almost_eq!(actual.value(), 0.3968, 1e-4);

        let marigold = Color::<f64>::from(&RGB {
            r: 234,
            g: 162,
            b: 33,
        });
        let swatch = Swatch::new(marigold, (0, 0), 128);
        let actual = Dark.weight(&swatch);
        assert_almost_eq!(actual.value(), 0.2815, 1e-4);
    }
}

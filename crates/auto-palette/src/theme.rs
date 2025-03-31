use std::{fmt::Debug, str::FromStr};

use crate::{
    math::{gaussian, FloatNumber},
    Error,
    Swatch,
};

/// The theme representation for scoring the swatches.
/// The definition of the themes is based on the PCCS (Practical Color Co-ordinate System) color theory.
/// @see [Practical Color Coordinate System - Wikipedia](https://en.wikipedia.org/wiki/Practical_Color_Coordinate_System)
///
/// # Examples
/// ```
/// use std::str::FromStr;
///
/// use auto_palette::Theme;
///
/// let theme = Theme::from_str("vivid").unwrap();
/// assert_eq!(theme, Theme::Vivid);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Theme {
    /// The theme selects the swatches based on the moderate chroma and lightness.
    /// The high chroma and lightness swatches are scored higher.
    Colorful,
    /// The theme selects the swatches based on the high chroma and moderate lightness.
    /// The high chroma and lightness swatches are scored higher.
    Vivid,
    /// The theme selects the swatches based on the low chroma and moderate lightness.
    /// The low chroma and lightness swatches are scored higher.
    Muted,
    /// The theme selects the swatches based on the moderate chroma and high lightness.
    /// The moderate chroma and high lightness swatches are scored higher.
    Light,
    /// The theme selects the swatches based on the low chroma and low lightness.
    /// The low chroma and lightness swatches are scored higher.
    Dark,
}

impl Theme {
    /// Gaussian parameters for the themes.
    /// The parameters are based on the PCCS color theory.
    ///
    /// The parameters are:
    /// * `center_c`: The center chroma value.
    /// * `center_l`: The center lightness value.
    /// * `sigma`: The standard deviation of the Gaussian function.
    ///
    /// The parameters are used to score the swatches based on their chroma and lightness values.
    ///
    /// The parameters are:
    /// * `COLORFUL_CHROMA_GAUSSIAN`: The Gaussian parameters for the colorful theme.
    /// * `VIVID_CHROMA_GAUSSIAN`: The Gaussian parameters for the vivid theme.
    /// * `MUTED_CHROMA_GAUSSIAN`: The Gaussian parameters for the muted theme.
    /// * `LIGHT_CHROMA_GAUSSIAN`: The Gaussian parameters for the light theme.
    /// * `DARK_CHROMA_GAUSSIAN`: The Gaussian parameters for the dark theme.
    const COLORFUL_CHROMA_GAUSSIAN: (f64, f64, f64) = (60.0, 50.0, 15.0);
    const VIVID_CHROMA_GAUSSIAN: (f64, f64, f64) = (75.0, 50.0, 15.0);
    const MUTED_CHROMA_GAUSSIAN: (f64, f64, f64) = (20.0, 40.0, 15.0);
    const LIGHT_CHROMA_GAUSSIAN: (f64, f64, f64) = (40.0, 75.0, 15.0);
    const DARK_CHROMA_GAUSSIAN: (f64, f64, f64) = (20.0, 25.0, 15.0);

    /// The maximum score for the swatch.
    const MAX_SCORE: f64 = 1.0;

    /// Scores the swatch based on the theme.
    ///
    /// # Type Parameters
    /// * `T` - The float number type.
    ///
    /// # Arguments
    /// * `swatch` - The swatch to score.
    ///
    /// # Returns
    /// The score of the swatch.
    #[inline]
    #[must_use]
    pub(crate) fn score<T>(&self, swatch: &Swatch<T>) -> T
    where
        T: FloatNumber,
    {
        let (center_c, center_l, sigma) = match self {
            Theme::Colorful => Self::COLORFUL_CHROMA_GAUSSIAN,
            Theme::Vivid => Self::VIVID_CHROMA_GAUSSIAN,
            Theme::Muted => Self::MUTED_CHROMA_GAUSSIAN,
            Theme::Light => Self::LIGHT_CHROMA_GAUSSIAN,
            Theme::Dark => Self::DARK_CHROMA_GAUSSIAN,
        };

        let color = swatch.color();
        let score_c = gaussian(color.chroma(), T::from_f64(center_c), T::from_f64(sigma))
            .unwrap_or(T::from_f64(Self::MAX_SCORE));
        let score_l = gaussian(color.lightness(), T::from_f64(center_l), T::from_f64(sigma))
            .unwrap_or(T::from_f64(Self::MAX_SCORE));
        score_c * score_l
    }
}

impl FromStr for Theme {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "colorful" => Ok(Theme::Colorful),
            "vivid" => Ok(Theme::Vivid),
            "muted" => Ok(Theme::Muted),
            "light" => Ok(Theme::Light),
            "dark" => Ok(Theme::Dark),
            _ => Err(Error::UnsupportedTheme {
                name: s.to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rstest::rstest;

    use super::*;
    use crate::{assert_approx_eq, color::Color};

    #[rstest]
    #[case::black("#000000", 0.000001)]
    #[case::gray("#808080", 0.000326)]
    #[case::white("#ffffff", 0.000001)]
    #[case::red("#ff0000", 0.011879)]
    #[case::green("#00ff00", 0.000015)]
    #[case::blue("#0000ff", 0.000002)]
    #[case::yellow("#ffff00", 0.000347)]
    #[case::cyan("#00ffff", 0.018807)]
    #[case::magenta("#ff00ff", 0.000829)]
    #[case::orange("#ff8000", 0.123426)]
    #[case::purple("#8000ff", 0.000069)]
    #[case::lime("#80ff00", 0.000129)]
    fn test_score_colorful(#[case] hex: &str, #[case] expected: f64) {
        // Act
        let color: Color<f64> = Color::from_str(hex).unwrap();
        let swatch = Swatch::new(color, (32, 64), 256, 0.5);
        let score = Theme::Colorful.score(&swatch);

        // Assert
        assert_approx_eq!(score, expected);
    }

    #[rstest]
    #[case::black("#000000", 0.0)]
    #[case::gray("#808080", 0.000003)]
    #[case::white("#ffffff", 0.0)]
    #[case::red("#ff0000", 0.140403)]
    #[case::green("#00ff00", 0.000490)]
    #[case::blue("#0000ff", 0.000228)]
    #[case::yellow("#ffff00", 0.002468)]
    #[case::cyan("#00ffff", 0.005902)]
    #[case::magenta("#ff00ff", 0.020425)]
    #[case::orange("#ff8000", 0.410010)]
    #[case::purple("#8000ff", 0.003222)]
    #[case::lime("#80ff00", 0.002103)]
    fn test_score_vivid(#[case] hex: &str, #[case] expected: f64) {
        // Act
        let color: Color<f64> = Color::from_str(hex).unwrap();
        let swatch = Swatch::new(color, (32, 64), 256, 0.5);
        let score = Theme::Vivid.score(&swatch);

        // Assert
        assert_approx_eq!(score, expected);
    }

    #[rstest]
    #[case::black("#000000", 0.011743)]
    #[case::gray("#808080", 0.273009)]
    #[case::white("#ffffff", 0.000138)]
    #[case::red("#ff0000", 0.0)]
    #[case::green("#00ff00", 0.0)]
    #[case::blue("#0000ff", 0.0)]
    #[case::yellow("#ffff00", 0.0)]
    #[case::cyan("#00ffff", 0.000400)]
    #[case::magenta("#ff00ff", 0.0)]
    #[case::orange("#ff8000", 0.000014)]
    #[case::purple("#8000ff", 0.0)]
    #[case::lime("#80ff00", 0.0)]
    fn test_score_muted(#[case] hex: &str, #[case] expected: f64) {
        // Act
        let color: Color<f64> = Color::from_str(hex).unwrap();
        let swatch = Swatch::new(color, (32, 64), 256, 0.5);
        let score = Theme::Muted.score(&swatch);

        // Assert
        assert_approx_eq!(score, expected);
    }

    #[rstest]
    #[case::black("#000000", 0.0)]
    #[case::gray("#808080", 0.010325)]
    #[case::white("#ffffff", 0.007137)]
    #[case::red("#ff0000", 0.000033)]
    #[case::green("#00ff00", 0.0)]
    #[case::blue("#0000ff", 0.0)]
    #[case::yellow("#ffff00", 0.000252)]
    #[case::cyan("#00ffff", 0.447280)]
    #[case::magenta("#ff00ff", 0.000001)]
    #[case::orange("#ff8000", 0.008716)]
    #[case::purple("#8000ff", 0.0)]
    #[case::lime("#80ff00", 0.000013)]
    fn test_score_light(#[case] hex: &str, #[case] expected: f64) {
        // Act
        let color: Color<f64> = Color::from_str(hex).unwrap();
        let swatch = Swatch::new(color, (32, 64), 256, 0.5);
        let score = Theme::Light.score(&swatch);

        // Assert
        assert_approx_eq!(score, expected);
    }

    #[rstest]
    #[case::black("#000000", 0.102511)]
    #[case::gray("#808080", 0.066942)]
    #[case::white("#ffffff", 0.000001)]
    #[case::red("#ff0000", 0.0)]
    #[case::green("#00ff00", 0.0)]
    #[case::blue("#0000ff", 0.0)]
    #[case::yellow("#ffff00", 0.0)]
    #[case::cyan("#00ffff", 0.000008)]
    #[case::magenta("#ff00ff", 0.0)]
    #[case::orange("#ff8000", 0.000001)]
    #[case::purple("#8000ff", 0.0)]
    #[case::lime("#80ff00", 0.0)]
    fn test_score_dark(#[case] hex: &str, #[case] expected: f64) {
        // Act
        let color: Color<f64> = Color::from_str(hex).unwrap();
        let swatch = Swatch::new(color, (32, 64), 256, 0.5);
        let score = Theme::Dark.score(&swatch);

        // Assert
        assert_approx_eq!(score, expected);
    }

    #[rstest]
    #[case::colorful("colorful", Theme::Colorful)]
    #[case::vivid("vivid", Theme::Vivid)]
    #[case::muted("muted", Theme::Muted)]
    #[case::light("light", Theme::Light)]
    #[case::dark("dark", Theme::Dark)]
    #[case::colorful_upper("COLORFUL", Theme::Colorful)]
    #[case::vivid_upper("VIVID", Theme::Vivid)]
    #[case::muted_upper("MUTED", Theme::Muted)]
    #[case::light_upper("LIGHT", Theme::Light)]
    #[case::dark_upper("DARK", Theme::Dark)]
    #[case::colorful_capitalized("Colorful", Theme::Colorful)]
    #[case::vivid_capitalized("Vivid", Theme::Vivid)]
    #[case::muted_capitalized("Muted", Theme::Muted)]
    #[case::light_capitalized("Light", Theme::Light)]
    #[case::dark_capitalized("Dark", Theme::Dark)]
    fn test_from_str(#[case] str: &str, #[case] expected: Theme) {
        // Act
        let actual = Theme::from_str(str).unwrap();

        // Assert
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case::empty("")]
    #[case::invalid("unknown")]
    fn test_from_str_error(#[case] str: &str) {
        // Act
        let actual = Theme::from_str(str);

        // Assert
        assert!(actual.is_err());
        assert_eq!(
            actual.unwrap_err().to_string(),
            format!("Unsupported theme specified: '{}'", str),
        );
    }
}

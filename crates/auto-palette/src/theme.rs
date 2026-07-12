use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use crate::{color::Gamut, error::Error, math::normalize, Swatch};

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
/// assert_eq!(theme.to_string(), "vivid");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    /// The maximum lightness value for the theme scoring.
    const MAX_LIGHTNESS: f64 = 100.0;

    /// Scores the swatch based on the theme.
    ///
    /// # Arguments
    /// * `swatch` - The swatch to score.
    ///
    /// # Returns
    /// The score of the swatch, in `0.0..=1.0`.
    #[inline]
    #[must_use]
    pub(crate) fn score(&self, swatch: &Swatch) -> f64 {
        let params = match self {
            Theme::Colorful => ThemeParams {
                mean_chroma: 0.75,
                sigma_chroma: 0.18,
                mean_lightness: 0.60,
                sigma_lightness: 0.20,
            },
            Theme::Vivid => ThemeParams {
                mean_chroma: 0.90,
                sigma_chroma: 0.10,
                mean_lightness: 0.70,
                sigma_lightness: 0.20,
            },
            Theme::Muted => ThemeParams {
                mean_chroma: 0.20,
                sigma_chroma: 0.15,
                mean_lightness: 0.40,
                sigma_lightness: 0.15,
            },
            Theme::Light => ThemeParams {
                mean_chroma: 0.60,
                sigma_chroma: 0.15,
                mean_lightness: 0.85,
                sigma_lightness: 0.15,
            },
            Theme::Dark => ThemeParams {
                mean_chroma: 0.25,
                sigma_chroma: 0.15,
                mean_lightness: 0.15,
                sigma_lightness: 0.15,
            },
        };

        let color = swatch.color();
        let max_chroma = Gamut::default().max_chroma(color.hue());
        let chroma = normalize(color.chroma(), 0.0, max_chroma);
        let lightness = normalize(color.lightness(), 0.0, Self::MAX_LIGHTNESS);

        let dc = (chroma - params.mean_chroma) / params.sigma_chroma;
        let dl = (lightness - params.mean_lightness) / params.sigma_lightness;
        (-0.5 * (dc * dc + dl * dl)).exp()
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
            _ => Err(Error::UnknownTheme(s.to_string())),
        }
    }
}

impl Display for Theme {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Theme::Colorful => write!(f, "colorful"),
            Theme::Vivid => write!(f, "vivid"),
            Theme::Muted => write!(f, "muted"),
            Theme::Light => write!(f, "light"),
            Theme::Dark => write!(f, "dark"),
        }
    }
}

/// The mean and standard deviation parameters for the theme scoring.
#[derive(Debug, PartialEq)]
struct ThemeParams {
    /// The mean chroma value for the theme.
    mean_chroma: f64,

    /// The standard deviation of the chroma values for the theme.
    sigma_chroma: f64,

    /// The mean lightness value for the theme.
    mean_lightness: f64,

    /// The standard deviation of the lightness values for the theme.
    sigma_lightness: f64,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rstest::rstest;

    use super::*;
    use crate::{assert_approx_eq, color::Color};

    #[rstest]
    #[case::black("#000000", 0.000001)]
    #[case::gray("#808080", 0.000162)]
    #[case::white("#ffffff", 0.000023)]
    #[case::red("#ff0000", 0.359991)]
    #[case::green("#00ff00", 0.145718)]
    #[case::blue("#0000ff", 0.146085)]
    #[case::yellow("#ffff00", 0.067976)]
    #[case::cyan("#00ffff", 0.113646)]
    #[case::magenta("#ff00ff", 0.381121)]
    #[case::orange("#ff8000", 0.358194)]
    #[case::purple("#8000ff", 0.241719)]
    #[case::lime("#80ff00", 0.124594)]
    fn test_score_colorful(#[case] hex: &str, #[case] expected: f64) {
        // Act
        let color = Color::from_str(hex).unwrap();
        let swatch = Swatch::new(color, (32, 64), 256, 0.5);
        let actual = Theme::Colorful.score(&swatch);

        // Assert
        assert_approx_eq!(actual, expected);
    }

    #[rstest]
    #[case::black("#000000", 0.0)]
    #[case::gray("#808080", 0.0)]
    #[case::white("#ffffff", 0.0)]
    #[case::red("#ff0000", 0.426884)]
    #[case::green("#00ff00", 0.409349)]
    #[case::blue("#0000ff", 0.102639)]
    #[case::yellow("#ffff00", 0.241562)]
    #[case::cyan("#00ffff", 0.347395)]
    #[case::magenta("#ff00ff", 0.539526)]
    #[case::orange("#ff8000", 0.599979)]
    #[case::purple("#8000ff", 0.210622)]
    #[case::lime("#80ff00", 0.369553)]
    fn test_score_vivid(#[case] hex: &str, #[case] expected: f64) {
        // Act
        let color = Color::from_str(hex).unwrap();
        let swatch = Swatch::new(color, (32, 64), 256, 0.5);
        let actual = Theme::Vivid.score(&swatch);

        // Assert
        assert_approx_eq!(actual, expected);
    }

    #[rstest]
    #[case::black("#000000", 0.011743)]
    #[case::gray("#808080", 0.273211)]
    #[case::white("#ffffff", 0.000138)]
    #[case::red("#ff0000", 0.0)]
    #[case::green("#00ff00", 0.0)]
    #[case::blue("#0000ff", 0.0)]
    #[case::yellow("#ffff00", 0.0)]
    #[case::cyan("#00ffff", 0.0)]
    #[case::magenta("#ff00ff", 0.0)]
    #[case::orange("#ff8000", 0.0)]
    #[case::purple("#8000ff", 0.0)]
    #[case::lime("#80ff00", 0.0)]
    fn test_score_muted(#[case] hex: &str, #[case] expected: f64) {
        // Act
        let color = Color::from_str(hex).unwrap();
        let swatch = Swatch::new(color, (32, 64), 256, 0.5);
        let actual = Theme::Muted.score(&swatch);

        // Assert
        assert_approx_eq!(actual, expected);
    }

    #[rstest]
    #[case::black("#000000", 0.0)]
    #[case::gray("#808080", 0.000037)]
    #[case::white("#ffffff", 0.000204)]
    #[case::red("#ff0000", 0.003035)]
    #[case::green("#00ff00", 0.028094)]
    #[case::blue("#0000ff", 0.000059)]
    #[case::yellow("#ffff00", 0.020589)]
    #[case::cyan("#00ffff", 0.026287)]
    #[case::magenta("#ff00ff", 0.007381)]
    #[case::orange("#ff8000", 0.013962)]
    #[case::purple("#8000ff", 0.000380)]
    #[case::lime("#80ff00", 0.027076)]
    fn test_score_light(#[case] hex: &str, #[case] expected: f64) {
        // Act
        let color = Color::from_str(hex).unwrap();
        let swatch = Swatch::new(color, (32, 64), 256, 0.5);
        let actual = Theme::Light.score(&swatch);

        // Assert
        assert_approx_eq!(actual, expected);
    }

    #[rstest]
    #[case::black("#000000", 0.151239)]
    #[case::gray("#808080", 0.009136)]
    #[case::white("#ffffff", 0.0)]
    #[case::red("#ff0000", 0.0)]
    #[case::green("#00ff00", 0.0)]
    #[case::blue("#0000ff", 0.000001)]
    #[case::yellow("#ffff00", 0.0)]
    #[case::cyan("#00ffff", 0.0)]
    #[case::magenta("#ff00ff", 0.0)]
    #[case::orange("#ff8000", 0.0)]
    #[case::purple("#8000ff", 0.0)]
    #[case::lime("#80ff00", 0.0)]
    fn test_score_dark(#[case] hex: &str, #[case] expected: f64) {
        // Act
        let color = Color::from_str(hex).unwrap();
        let swatch = Swatch::new(color, (32, 64), 256, 0.5);
        let actual = Theme::Dark.score(&swatch);

        // Assert
        assert_approx_eq!(actual, expected);
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
        let actual = Theme::from_str(str);

        // Assert
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
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
            format!("unsupported theme: '{}'", str),
        );
    }

    #[rstest]
    #[case::colorful(Theme::Colorful, "colorful")]
    #[case::vivid(Theme::Vivid, "vivid")]
    #[case::muted(Theme::Muted, "muted")]
    #[case::light(Theme::Light, "light")]
    #[case::dark(Theme::Dark, "dark")]
    fn test_fmt(#[case] theme: Theme, #[case] expected: &str) {
        // Act & Assert
        assert_eq!(theme.to_string(), expected);
    }
}

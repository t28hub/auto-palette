use std::str::FromStr;

use crate::{math::FloatNumber, Error, Swatch};

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
/// let theme = Theme::from_str("basic").unwrap();
/// assert_eq!(theme, Theme::Basic);
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Theme {
    /// The theme selects the swatches based on the ratio of the swatch.
    /// The swatches with the highest ratio are selected.
    #[default]
    Basic,
    /// The theme selects the swatches based on the moderate chroma and lightness.
    /// The high chroma and lightness swatches are scored higher.
    /// Gaussian parameters: chroma = 60, lightness = 50, sigma = 15.
    Colorful,
    /// The theme selects the swatches based on the high chroma and moderate lightness.
    /// The high chroma and lightness swatches are scored higher.
    /// Gaussian parameters: chroma = 75, lightness = 50, sigma = 15.
    Vivid,
    /// The theme selects the swatches based on the low chroma and moderate lightness.
    /// The low chroma and lightness swatches are scored higher.
    /// Gaussian parameters: chroma = 20, lightness = 40, sigma = 15.
    Muted,
    /// The theme selects the swatches based on the moderate chroma and high lightness.
    /// The moderate chroma and high lightness swatches are scored higher.
    /// Gaussian parameters: chroma = 40, lightness = 75, sigma = 15.
    Light,
    /// The theme selects the swatches based on the low chroma and low lightness.
    /// The low chroma and lightness swatches are scored higher.
    /// Gaussian parameters: chroma = 20, lightness = 25, sigma = 15.
    Dark,
}

impl Theme {
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
        let color = swatch.color();
        let chroma = color.chroma();
        let lightness = color.lightness();
        match self {
            Theme::Basic => swatch.ratio(),
            Theme::Colorful => {
                let score_c = gaussian_score(chroma, T::from_u16(60), T::from_u16(15));
                let score_l = gaussian_score(lightness, T::from_u16(50), T::from_u16(15));
                score_c * score_l
            }
            Theme::Vivid => {
                let score_c = gaussian_score(chroma, T::from_u16(75), T::from_u16(15));
                let score_l = gaussian_score(lightness, T::from_u16(50), T::from_u16(15));
                score_c * score_l
            }
            Theme::Muted => {
                let score_c = gaussian_score(chroma, T::from_u16(20), T::from_u16(15));
                let score_l = gaussian_score(lightness, T::from_u16(40), T::from_u16(15));
                score_c * score_l
            }
            Theme::Light => {
                let score_c = gaussian_score(chroma, T::from_u16(40), T::from_u16(15));
                let score_l = gaussian_score(lightness, T::from_u16(75), T::from_u16(15));
                score_c * score_l
            }
            Theme::Dark => {
                let score_c = gaussian_score(chroma, T::from_u16(20), T::from_u16(15));
                let score_l = gaussian_score(lightness, T::from_u16(25), T::from_u16(15));
                score_c * score_l
            }
        }
    }
}

impl FromStr for Theme {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "basic" => Ok(Theme::Basic),
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

/// Calculates the score based on the Gaussian function.
///
/// # Arguments
/// * `value` - The value to score.
/// * `center` - The center of the Gaussian function.
/// * `sigma` - The standard deviation of the Gaussian function.
///
/// # Returns
/// The score of the value.
#[inline]
fn gaussian_score<T>(value: T, center: T, sigma: T) -> T
where
    T: FloatNumber,
{
    if sigma.is_zero() {
        return T::zero();
    }
    (-(value - center).powi(2) / (T::from_u16(2) * sigma.powi(2))).exp()
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rstest::rstest;

    use super::*;
    use crate::{assert_approx_eq, color::Color};

    #[rstest]
    #[case::black("#000000", 0.2)]
    #[case::gray("#808080", 0.125)]
    #[case::white("#ffffff", 0.5)]
    #[case::red("#ff0000", 0.8)]
    #[case::green("#00ff00", 0.0)]
    #[case::blue("#0000ff", 0.4)]
    fn test_score_basic(#[case] hex: &str, #[case] ratio: f64) {
        // Act
        let color: Color<f64> = Color::from_str(hex).unwrap();
        let swatch = Swatch::new(color, (32, 64), 256, ratio);
        let score = Theme::Basic.score(&swatch);

        // Assert
        assert_eq!(score, ratio);
    }

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
    #[case::basic("basic", Theme::Basic)]
    #[case::colorful("colorful", Theme::Colorful)]
    #[case::vivid("vivid", Theme::Vivid)]
    #[case::muted("muted", Theme::Muted)]
    #[case::light("light", Theme::Light)]
    #[case::dark("dark", Theme::Dark)]
    #[case::basic_upper("BASIC", Theme::Basic)]
    #[case::colorful_upper("COLORFUL", Theme::Colorful)]
    #[case::vivid_upper("VIVID", Theme::Vivid)]
    #[case::muted_upper("MUTED", Theme::Muted)]
    #[case::light_upper("LIGHT", Theme::Light)]
    #[case::dark_upper("DARK", Theme::Dark)]
    #[case::basic_capitalized("Basic", Theme::Basic)]
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

    #[rstest]
    #[case(0.0, 0.0, 1.0, 1.0)]
    #[case(0.0, 1.0, 1.0, 0.606530)]
    #[case(1.0, 0.0, 1.0, 0.606530)]
    #[case(1.0, 1.0, 1.0, 1.0)]
    #[case(0.0, 0.5, 1.0, 0.882496)]
    #[case(0.5, 0.0, 1.0, 0.882496)]
    #[case(16.0, 60.0, 15.0, 0.013538)]
    #[case(64.0, 60.0, 15.0, 0.965069)]
    #[case(104.0, 60.0, 15.0, 0.013538)]
    fn test_gaussian_score(
        #[case] x: f64,
        #[case] mean: f64,
        #[case] std_dev: f64,
        #[case] expected: f64,
    ) {
        // Act
        let actual = gaussian_score(x, mean, std_dev);

        // Assert
        assert_approx_eq!(actual, expected);
    }

    #[test]
    fn test_gaussian_score_zero_sigma() {
        // Act
        let actual = gaussian_score(0.5, 1.0, 0.0);

        // Assert
        assert_eq!(actual, 0.0);
    }
}

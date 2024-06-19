use std::str::FromStr;

use crate::{
    color::Color,
    math::{normalize, FloatNumber},
    Error,
    Swatch,
};

/// The theme representation for scoring the swatches.
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
    /// The theme selects the swatches based on the population.
    /// The high population swatches are preferred.
    #[default]
    Basic,
    /// The theme selects the swatches based on the population and the lightness.
    /// The high population swatches are preferred.
    Colorful,
    /// The theme selects the swatches based on the chroma.
    /// The saturated colors are preferred.
    Vivid,
    /// The theme selects the swatches based on the chroma.
    /// The desaturated colors are preferred.
    Muted,
    /// The theme selects the swatches based on the lightness.
    /// The light colors are preferred.
    Light,
    /// The theme selects the swatches based on the lightness.
    /// The dark colors are preferred.
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
        match self {
            Theme::Basic => score_basic(swatch),
            Theme::Colorful => score_colorful(swatch),
            Theme::Vivid => score_vivid(swatch),
            Theme::Muted => score_muted(swatch),
            Theme::Light => score_light(swatch),
            Theme::Dark => score_dark(swatch),
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

#[inline]
fn score_basic<T>(swatch: &Swatch<T>) -> T
where
    T: FloatNumber,
{
    swatch.ratio()
}

#[inline]
fn score_colorful<T>(swatch: &Swatch<T>) -> T
where
    T: FloatNumber,
{
    let color = swatch.color();
    let chroma = color.chroma();
    let score_c = if chroma <= T::from_u16(36) {
        T::zero()
    } else {
        normalize(chroma, Color::<T>::min_chroma(), Color::<T>::max_chroma())
    };

    let lightness = color.lightness();
    let score_l = if lightness <= T::from_u16(25) || lightness >= T::from_u16(85) {
        T::zero()
    } else {
        normalize(
            lightness,
            Color::<T>::min_lightness(),
            Color::<T>::max_lightness(),
        )
    };
    score_c * score_l
}

#[inline]
fn score_vivid<T>(swatch: &Swatch<T>) -> T
where
    T: FloatNumber,
{
    let color = swatch.color();
    let chroma = color.chroma();
    let score_c = if chroma <= T::from_u32(48) {
        T::zero()
    } else {
        normalize(chroma, Color::<T>::min_chroma(), Color::<T>::max_chroma())
    };

    let score_l = {
        let score = normalize(
            color.lightness(),
            Color::<T>::min_lightness(),
            Color::<T>::max_lightness(),
        );
        T::one() - (score - T::from_f32(0.5)).abs()
    };
    score_c * score_l
}

#[inline]
fn score_muted<T>(swatch: &Swatch<T>) -> T
where
    T: FloatNumber,
{
    let color = swatch.color();
    let chroma = color.chroma();
    let score_c = if chroma <= T::from_u16(60) {
        T::one() - normalize(chroma, Color::<T>::min_chroma(), Color::<T>::max_chroma())
    } else {
        T::zero()
    };

    let score_l = {
        let score = normalize(
            color.lightness(),
            Color::<T>::min_lightness(),
            Color::<T>::max_lightness(),
        );
        T::one() - (score - T::from_f32(0.5)).abs()
    };
    score_c * score_l
}

#[inline]
fn score_light<T>(swatch: &Swatch<T>) -> T
where
    T: FloatNumber,
{
    let color = swatch.color();
    let lightness = color.lightness();
    if lightness <= T::from_u16(60) {
        T::zero()
    } else {
        let score = normalize(
            color.lightness(),
            Color::<T>::min_lightness(),
            Color::<T>::max_lightness(),
        );
        score.powi(2)
    }
}

#[inline]
fn score_dark<T>(swatch: &Swatch<T>) -> T
where
    T: FloatNumber,
{
    let color = swatch.color();
    let lightness = color.lightness();
    if lightness >= T::from_u16(40) {
        T::zero()
    } else {
        let score = normalize(
            color.lightness(),
            Color::<T>::min_lightness(),
            Color::<T>::max_lightness(),
        );
        T::one() - score.powi(2)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rstest::rstest;

    use super::*;

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
    #[case::black("#000000", 0.0)]
    #[case::gray("#808080", 0.0)]
    #[case::white("#ffffff", 0.0)]
    #[case::red("#ff0000", 0.309)]
    #[case::green("#00ff00", 0.0)]
    #[case::blue("#0000ff", 0.240)]
    #[case::yellow("#ffff00", 0.0)]
    #[case::cyan("#00ffff", 0.0)]
    #[case::magenta("#ff00ff", 0.387)]
    #[case::orange("#ff8000", 0.319)]
    #[case::purple("#8000ff", 0.284)]
    #[case::lime("#80ff00", 0.0)]
    fn test_score_colorful(#[case] hex: &str, #[case] expected: f64) {
        // Act
        let color: Color<f64> = Color::from_str(hex).unwrap();
        let swatch = Swatch::new(color, (32, 64), 256, 0.5);
        let score = Theme::Colorful.score(&swatch);

        // Assert
        assert!((score - expected).abs() < 1e-3);
    }

    #[rstest]
    #[case::black("#000000", 0.0)]
    #[case::gray("#808080", 0.0)]
    #[case::white("#ffffff", 0.0)]
    #[case::red("#ff0000", 0.562)]
    #[case::green("#00ff00", 0.414)]
    #[case::blue("#0000ff", 0.612)]
    #[case::yellow("#ffff00", 0.285)]
    #[case::cyan("#00ffff", 0.164)]
    #[case::magenta("#ff00ff", 0.576)]
    #[case::orange("#ff8000", 0.394)]
    #[case::purple("#8000ff", 0.631)]
    #[case::lime("#80ff00", 0.365)]
    fn test_score_vivid(#[case] hex: &str, #[case] expected: f64) {
        // Act
        let color: Color<f64> = Color::from_str(hex).unwrap();
        let swatch = Swatch::new(color, (32, 64), 256, 0.5);
        let score = Theme::Vivid.score(&swatch);

        // Assert
        assert!((score - expected).abs() < 1e-3);
    }

    #[rstest]
    #[case::black("#000000", 0.5)]
    #[case::gray("#808080", 0.964)]
    #[case::white("#ffffff", 0.500)]
    #[case::red("#ff0000", 0.0)]
    #[case::green("#00ff00", 0.0)]
    #[case::blue("#0000ff", 0.0)]
    #[case::yellow("#ffff00", 0.0)]
    #[case::cyan("#00ffff", 0.425)]
    #[case::magenta("#ff00ff", 0.0)]
    #[case::orange("#ff8000", 0.0)]
    #[case::purple("#8000ff", 0.0)]
    #[case::lime("#80ff00", 0.0)]
    fn test_score_muted(#[case] hex: &str, #[case] expected: f64) {
        // Act
        let color: Color<f64> = Color::from_str(hex).unwrap();
        let swatch = Swatch::new(color, (32, 64), 256, 0.5);
        let score = Theme::Muted.score(&swatch);

        // Assert
        assert!((score - expected).abs() < 1e-3);
    }

    #[rstest]
    #[case::black("#000000", 0.0)]
    #[case::gray("#808080", 0.0)]
    #[case::white("#ffffff", 1.0)]
    #[case::red("#ff0000", 0.0)]
    #[case::green("#00ff00", 0.770)]
    #[case::blue("#0000ff", 0.0)]
    #[case::yellow("#ffff00", 0.944)]
    #[case::cyan("#00ffff", 0.830)]
    #[case::magenta("#ff00ff", 0.364)]
    #[case::orange("#ff8000", 0.450)]
    #[case::purple("#8000ff", 0.0)]
    #[case::lime("#80ff00", 0.808)]
    fn test_score_light(#[case] hex: &str, #[case] expected: f64) {
        // Act
        let color: Color<f64> = Color::from_str(hex).unwrap();
        let swatch = Swatch::new(color, (32, 64), 256, 0.5);
        let score = Theme::Light.score(&swatch);

        // Assert
        assert!((score - expected).abs() < 1e-3);
    }

    #[rstest]
    #[case::black("#000000", 1.0)]
    #[case::gray("#808080", 0.0)]
    #[case::white("#ffffff", 0.0)]
    #[case::red("#ff0000", 0.0)]
    #[case::green("#00ff00", 0.0)]
    #[case::blue("#0000ff", 0.896)]
    #[case::yellow("#ffff00", 0.0)]
    #[case::cyan("#00ffff", 0.0)]
    #[case::magenta("#ff00ff", 0.0)]
    #[case::orange("#ff8000", 0.0)]
    #[case::purple("#8000ff", 0.0)]
    #[case::lime("#80ff00", 0.0)]
    fn test_score_dark(#[case] hex: &str, #[case] expected: f64) {
        // Act
        let color: Color<f64> = Color::from_str(hex).unwrap();
        let swatch = Swatch::new(color, (32, 64), 256, 0.5);
        let score = Theme::Dark.score(&swatch);

        // Assert
        assert!((score - expected).abs() < 1e-3);
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
            format!("The theme '{}' is not supported.", str)
        );
    }
}

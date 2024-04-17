mod lab;
mod rgb;
mod white_point;
mod xyz;

use crate::math::FloatNumber;
pub use lab::{xyz_to_lab, Lab};
pub use rgb::RGB;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
pub use white_point::D65;
pub use xyz::{rgb_to_xyz, XYZ};

/// Struct representing a color.
///
/// # Type Parameters
/// * `T` - The floating point type.
///
/// # Examples
/// ```
/// use std::str::FromStr;
/// use auto_palette::Color;
///
/// let color = Color::from_str("#2c7de7").unwrap();
/// assert!(color.is_light());
/// assert_eq!(color.lightness(), 52.917793);
/// assert_eq!(color.chroma(), 61.9814870);
/// assert_eq!(color.hue(), 282.6622);
///```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color<T>
where
    T: FloatNumber,
{
    pub(super) l: T,
    pub(super) a: T,
    pub(super) b: T,
}

impl<T> Color<T>
where
    T: FloatNumber,
{
    /// Creates a new `Color` instance.
    ///
    /// # Arguments
    /// * `l` - The value of l.
    /// * `a` - The value of a.
    /// * `b` - The value of b.
    ///
    /// # Returns
    /// A new `Color` instance.
    #[must_use]
    pub(crate) fn new(l: T, a: T, b: T) -> Self {
        Self { l, a, b }
    }

    /// Returns the minimum value of the chroma component.
    ///
    /// # Returns
    /// The minimum value of the chroma component.
    #[inline]
    #[must_use]
    pub(crate) fn min_chroma() -> T {
        T::zero()
    }

    /// Returns the maximum value of the chroma component.
    ///
    /// # Returns
    /// The maximum value of the chroma component.
    #[inline]
    #[must_use]
    pub(crate) fn max_chroma() -> T {
        T::from_f32(180.0)
    }

    /// Returns the minimum value of the lightness component.
    ///
    /// # Returns
    /// The minimum value of the lightness component.
    #[inline]
    #[must_use]
    pub(crate) fn min_lightness() -> T {
        T::zero()
    }

    /// Returns the maximum value of the lightness component.
    ///
    /// # Returns
    /// The maximum value of the lightness component.
    #[inline]
    #[must_use]
    pub(crate) fn max_lightness() -> T {
        T::from_f32(100.0)
    }

    /// Returns whether this color is light.
    ///
    /// # Returns
    /// `true` if the color is light, otherwise `false`.
    #[must_use]
    pub fn is_light(&self) -> bool {
        self.l > T::from_f32(50.0)
    }

    /// Returns whether this color is dark.
    ///
    /// # Returns
    /// `true` if the color is dark, otherwise `false`.
    #[must_use]
    pub fn is_dark(&self) -> bool {
        !self.is_light()
    }

    /// Returns the lightness of this color.
    ///
    /// # Returns
    /// The lightness of this color.
    #[must_use]
    pub fn lightness(&self) -> T {
        self.l
    }

    /// Returns the chroma of this color.
    ///
    /// # Returns
    /// The chroma of this color.
    #[must_use]
    pub fn chroma(&self) -> T {
        (self.a.powi(2) + self.b.powi(2)).sqrt()
    }

    /// Returns the hue of this color. The hue is the angle of the vector in the a*b* plane.
    ///
    /// # Returns
    /// The hue of this color.
    #[must_use]
    pub fn hue(&self) -> T {
        let hue = self.b.atan2(self.a).to_degrees();
        if hue < T::zero() {
            hue + T::from_f32(360.0)
        } else {
            hue
        }
    }

    /// Converts this color to the RGB color space.
    ///
    /// # Returns
    /// The converted `RGB` color.
    #[must_use]
    pub fn to_rgb(&self) -> RGB {
        let xyz = self.to_xyz();
        RGB::from(&xyz)
    }

    /// Converts this color to the CIE XYZ color space.
    ///
    /// # Returns
    /// The converted `XYZ` color.
    #[must_use]
    pub fn to_xyz(&self) -> XYZ<T> {
        XYZ::from(&self.to_lab())
    }

    /// Converts this color to the CIE L*a*b* color space.
    ///
    /// # Returns
    /// The converted `Lab` color.
    #[must_use]
    pub fn to_lab(&self) -> Lab<T> {
        Lab::new(self.l, self.a, self.b)
    }
}

impl<T> Display for Color<T>
where
    T: FloatNumber,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Color(l: {}, a: {}, b: {})", self.l, self.a, self.b)
    }
}

impl<T> FromStr for Color<T>
where
    T: FloatNumber,
{
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 7 || !s.starts_with('#') {
            return Err("Invalid color format");
        }

        let r = u8::from_str_radix(&s[1..3], 16).map_err(|_| "Invalid hex value")?;
        let g = u8::from_str_radix(&s[3..5], 16).map_err(|_| "Invalid hex value")?;
        let b = u8::from_str_radix(&s[5..7], 16).map_err(|_| "Invalid hex value")?;

        let (x, y, z) = rgb_to_xyz::<T>(r, g, b);
        let (l, a, b) = xyz_to_lab::<T, D65>(x, y, z);
        Ok(Self::new(l, a, b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_new_color() {
        // Act
        let color = Color::new(80.0, 0.0, 0.0);

        // Assert
        assert_eq!(color.l, 80.0);
        assert_eq!(color.a, 0.0);
        assert_eq!(color.b, 0.0);
    }

    #[rstest]
    #[case((0.0, 0.0, 0.0), false)]
    #[case((50.0, 0.0, 0.0), false)]
    #[case((50.1, 0.0, 0.0), true)]
    #[case((80.0, 0.0, 0.0), true)]
    fn test_color_is_light(#[case] input: (f32, f32, f32), #[case] expected: bool) {
        // Arrange
        let color = Color::new(input.0, input.1, input.2);

        // Act
        let is_light = color.is_light();

        // Assert
        assert_eq!(is_light, expected);
    }

    #[rstest]
    #[case((0.0, 0.0, 0.0), true)]
    #[case((50.0, 0.0, 0.0), true)]
    #[case((50.1, 0.0, 0.0), false)]
    #[case((80.0, 0.0, 0.0), false)]
    fn test_color_is_dark(#[case] input: (f32, f32, f32), #[case] expected: bool) {
        // Arrange
        let color = Color::new(input.0, input.1, input.2);

        // Act
        let is_dark = color.is_dark();

        // Assert
        assert_eq!(is_dark, expected);
    }

    #[test]
    fn test_lightness() {
        // Act
        let color = Color::new(91.1120, -48.0806, -14.1521);
        let lightness = color.lightness();

        // Assert
        assert_eq!(lightness, 91.1120);
    }

    #[test]
    fn test_chroma() {
        // Act
        let color: Color<f32> = Color::new(91.1120, -48.0806, -14.1521);
        let chroma = color.chroma();

        // Assert
        assert!((chroma - 50.120_117).abs() < 1e-3);
    }

    #[rstest]
    #[case::black((0.0, 0.0, 0.0), 0.0)]
    #[case::white((100.0, - 0.002_443, 0.011_384), 102.111_946)]
    #[case::red((53.237_144, 80.088_320, 67.199_460), 39.998_900)]
    #[case::green((87.735_535, - 86.183_550, 83.179_924), 136.016_020)]
    #[case::blue((32.300_800, 79.194_260, - 107.868_910), 306.28503)]
    #[case::cyan((91.114_750, - 48.080_950, - 14.142_858), 196.391_080,)]
    #[case::magenta((60.322_700, 98.235_580, - 60.842_370), 328.227_940,)]
    #[case::yellow((97.138_580, - 21.562_368, 94.476_760), 102.856_380)]
    fn test_hue(#[case] input: (f32, f32, f32), #[case] expected: f32) {
        // Act
        let color: Color<f32> = Color::new(input.0, input.1, input.2);
        let hue = color.hue();

        // Assert
        assert!((hue - expected).abs() < 1e-3);
    }

    #[test]
    fn test_to_rgb() {
        // Act
        let color = Color::new(91.1120, -48.0806, -14.1521);
        let rgb = color.to_rgb();

        // Assert
        assert_eq!(rgb.r, 0);
        assert_eq!(rgb.g, 255);
        assert_eq!(rgb.b, 255);
    }

    #[test]
    fn test_to_xyz() {
        // Act
        let color: Color<f32> = Color::new(91.1120, -48.0806, -14.1521);
        let xyz: XYZ<f32> = color.to_xyz();

        // Assert
        assert!((xyz.x - 0.5380).abs() < 1e-3);
        assert!((xyz.y - 0.7873).abs() < 1e-3);
        assert!((xyz.z - 1.0690).abs() < 1e-3);
    }

    #[test]
    fn test_to_lab() {
        // Act
        let color: Color<f32> = Color::new(91.1120, -48.0806, -14.1521);
        let lab = color.to_lab();

        // Assert
        assert_eq!(lab.l, 91.1120);
        assert_eq!(lab.a, -48.0806);
        assert_eq!(lab.b, -14.1521);
    }

    #[test]
    fn test_fmt() {
        // Act & Assert
        let color: Color<f32> = Color::new(91.114_750, -48.080_950, -14.142_8581);
        assert_eq!(
            format!("{}", color),
            "Color(l: 91.11475, a: -48.08095, b: -14.1428585)"
        );
    }

    #[rstest]
    #[case::black("#000000", 0.0, 0.0, 0.0)]
    #[case::white("#FFFFFF", 100.0, - 0.002_443, 0.011_384)]
    #[case::red("#FF0000", 53.237_144, 80.088_320, 67.199_460)]
    #[case::green("#00FF00", 87.735_535, - 86.183_550, 83.179_924)]
    #[case::blue("#0000FF", 32.300_800, 79.194_260, - 107.868_910)]
    #[case::cyan("#00FFFF", 91.114_750, - 48.080_950, - 14.142_858)]
    #[case::magenta("#FF00FF", 60.322_700, 98.235_580, - 60.842_370)]
    #[case::yellow("#FFFF00", 97.138_580, - 21.562_368, 94.476_760)]
    fn test_from_str(#[case] input: &str, #[case] l: f32, #[case] a: f32, #[case] b: f32) {
        // Act
        let color = Color::<f32>::from_str(input).unwrap();

        // Assert
        assert!((color.l - l).abs() < 1e-3);
        assert!((color.a - a).abs() < 1e-3);
        assert!((color.b - b).abs() < 1e-3);
    }

    #[rstest]
    #[case::empty("")]
    #[case::invalid("123456")]
    #[case::invalid_length("#12345")]
    #[case::invalid_prefix("123456")]
    #[case::invalid_hex_red("#GGAA99")]
    #[case::invalid_hex_green("#00GG99")]
    #[case::invalid_hex_blue("#00AAGG")]
    fn test_from_str_error(#[case] input: &str) {
        // Act
        let result = Color::<f32>::from_str(input);

        // Assert
        assert!(result.is_err());
    }
}

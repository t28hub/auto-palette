mod ansi16;
mod ansi256;
mod cmyk;
mod error;
mod hsl;
mod hsv;
mod hue;
mod lab;
mod lchab;
mod lchuv;
mod luv;
mod oklab;
mod oklch;
mod rgb;
mod white_point;
mod xyz;

use std::{
    fmt,
    fmt::{Display, Formatter},
    marker::PhantomData,
    str::FromStr,
};

pub use ansi16::Ansi16;
pub use ansi256::Ansi256;
pub use cmyk::CMYK;
pub use hsl::HSL;
pub use hsv::HSV;
pub use hue::Hue;
pub(crate) use lab::xyz_to_lab;
pub use lab::Lab;
pub use lchab::LCHab;
pub use lchuv::LCHuv;
pub use luv::Luv;
use num_traits::clamp;
pub use oklab::Oklab;
pub use oklch::Oklch;
pub use rgb::RGB;
pub use white_point::*;
pub(crate) use xyz::rgb_to_xyz;
pub use xyz::XYZ;

use crate::{color::error::ColorError, math::FloatNumber};

/// The color representation.
///
/// # Type Parameters
/// * `T` - The floating point type.
/// * `W` - The white point type.
///
/// # Examples
/// ```
/// use std::str::FromStr;
///
/// use auto_palette::color::Color;
///
/// let color: Color<f32> = Color::from_str("#2c7de7").unwrap();
/// assert!(color.is_light());
/// assert!(color.lightness() - 52.917 < 1e-3);
/// assert!(color.chroma() - 61.981 < 1e-3);
/// assert!(color.hue().to_degrees() - 282.662 < 1e-3);
///
/// let rgb = color.to_rgb();
/// assert_eq!(format!("{}", rgb), "RGB(44, 125, 231)");
///
/// let hsl = color.to_hsl();
/// assert_eq!(format!("{}", hsl), "HSL(214.01, 0.80, 0.54)");
///
/// let lab = color.to_lab();
/// assert_eq!(format!("{}", lab), "Lab(52.92, 13.59, -60.47)");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color<T, W = D65>
where
    T: FloatNumber,
{
    pub(super) l: T,
    pub(super) a: T,
    pub(super) b: T,
    _marker: PhantomData<W>,
}

impl<T, W> Color<T, W>
where
    T: FloatNumber,
    W: WhitePoint,
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
        Self {
            l,
            a,
            b,
            _marker: PhantomData,
        }
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

    /// Calculates the delta E76 value between this color and another color.
    ///
    /// # Arguments
    /// * `other` - The other color to compare against.
    ///
    /// # Returns
    /// The delta E76 value, which is a measure of the difference between two colors.
    ///
    /// # Note
    /// This method uses the CIE76 formula, which is a simple Euclidean distance in the L*a*b* color space.
    /// [Color difference CIE76 - Wikipedia](https://en.wikipedia.org/wiki/Color_difference#CIE76)
    #[inline]
    #[must_use]
    pub fn delta_e(&self, other: &Self) -> T {
        let delta_l = self.l - other.l;
        let delta_a = self.a - other.a;
        let delta_b = self.b - other.b;
        (delta_l.powi(2) + delta_a.powi(2) + delta_b.powi(2)).sqrt()
    }

    /// Returns the hue of this color. The hue is the angle of the vector in the a*b* plane.
    ///
    /// # Returns
    /// The hue of this color.
    #[must_use]
    pub fn hue(&self) -> Hue<T> {
        let degrees = self.b.atan2(self.a).to_degrees();
        Hue::from_degrees(degrees)
    }

    /// Converts this color to a hexadecimal string.
    ///
    /// # Returns
    /// The hexadecimal string representation of this color.
    #[must_use]
    pub fn to_hex_string(&self) -> String {
        let RGB { r, g, b } = self.to_rgb();
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }

    /// Converts this color to the RGB color space.
    ///
    /// # Returns
    /// The converted `RGB` color.
    #[must_use]
    pub fn to_rgb(&self) -> RGB {
        RGB::from(&self.to_xyz())
    }

    /// Converts this color to the CMYK color space.
    ///
    /// # Returns
    /// The converted `CMYK` color.
    #[must_use]
    pub fn to_cmyk(&self) -> CMYK<T> {
        CMYK::from(&self.to_rgb())
    }

    /// Converts this color to the HSL color space.
    ///
    /// # Returns
    /// The converted `HSL` color.
    #[must_use]
    pub fn to_hsl(&self) -> HSL<T> {
        HSL::from(&self.to_rgb())
    }

    /// Converts this color to the HSV color space.
    ///
    /// # Returns
    /// The converted `HSV` color.
    #[must_use]
    pub fn to_hsv(&self) -> HSV<T> {
        HSV::from(&self.to_rgb())
    }

    /// Converts this color to the CIE XYZ color space.
    ///
    /// # Returns
    /// The converted `XYZ` color.
    #[must_use]
    pub fn to_xyz(&self) -> XYZ<T> {
        XYZ::from(&self.to_lab())
    }

    /// Converts this color to the CIE L*u*v* color space.
    ///
    /// # Returns
    /// The converted `Luv` color.
    #[must_use]
    pub fn to_luv(&self) -> Luv<T, W> {
        Luv::<T, W>::from(&self.to_xyz())
    }

    /// Converts this color to the CIE LCH(uv) color space.
    ///
    /// # Returns
    /// The converted `LCHuv` color.
    #[must_use]
    pub fn to_lchuv(&self) -> LCHuv<T, W> {
        LCHuv::<T, W>::from(&self.to_luv())
    }

    /// Converts this color to the CIE L*a*b* color space.
    ///
    /// # Returns
    /// The converted `Lab` color.
    #[must_use]
    pub fn to_lab(&self) -> Lab<T, W> {
        Lab::<T, W>::new(self.l, self.a, self.b)
    }

    /// Converts this color to the CIE LCH(ab) color space.
    ///
    /// # Returns
    /// The converted `LCHab` color.
    #[must_use]
    pub fn to_lchab(&self) -> LCHab<T, W> {
        LCHab::<T, W>::from(&self.to_lab())
    }

    /// Converts this color to the CIE Oklab color space.
    ///
    /// # Returns
    /// The converted `Oklab` color.
    #[must_use]
    pub fn to_oklab(&self) -> Oklab<T> {
        Oklab::from(&self.to_xyz())
    }

    /// Converts this color to the CIE Oklch color space.
    ///
    /// # Returns
    /// The converted `Oklch` color.
    #[must_use]
    pub fn to_oklch(&self) -> Oklch<T> {
        Oklch::from(&self.to_oklab())
    }

    /// Converts this color to the 4-bit ANSI 16 color space.
    ///
    /// # Returns
    /// The converted `Ansi16` color.
    #[must_use]
    pub fn to_ansi16(&self) -> Ansi16 {
        Ansi16::from(&self.to_rgb())
    }

    /// Converts this color to the 8-bit ANSI 256 color space.
    ///
    /// # Returns
    /// The converted `Ansi256` color.
    #[must_use]
    pub fn to_ansi256(&self) -> Ansi256 {
        Ansi256::from(&self.to_rgb())
    }

    /// Converts this color to a 32-bit integer representation in RGB.
    ///
    /// # Returns
    /// The converted RGB integer representation.
    #[must_use]
    pub fn to_rgb_int(&self) -> u32 {
        let rgb = self.to_rgb();
        let r = rgb.r as u32;
        let g = rgb.g as u32;
        let b = rgb.b as u32;
        (r << 16) | (g << 8) | b
    }

    /// Converts this color to a 32-bit integer representation in RGBA.
    ///
    /// # Arguments
    /// * `alpha` - The alpha value (0-255).
    ///
    /// # Returns
    /// The converted RGBA integer representation.
    #[must_use]
    pub fn to_rgba_int(&self, alpha: u8) -> u32 {
        let rgb = self.to_rgb();
        let r = rgb.r as u32;
        let g = rgb.g as u32;
        let b = rgb.b as u32;
        (r << 24) | (g << 16) | (b << 8) | alpha as u32
    }

    /// Mixes this color with another color by a given fraction.
    ///
    /// # Arguments
    /// * `other` - The other color to mix with.
    /// * `fraction` - The fraction to mix the two colors (0.0 to 1.0).
    ///
    /// # Returns
    /// A new `Color` instance that is the result of mixing the two colors.
    #[must_use]
    pub fn mix(&self, other: &Self, fraction: T) -> Self {
        let fraction = clamp(fraction, T::zero(), T::one());
        Self {
            l: self.l + fraction * (other.l - self.l),
            a: self.a + fraction * (other.a - self.a),
            b: self.b + fraction * (other.b - self.b),
            _marker: PhantomData,
        }
    }
}

impl<T> Default for Color<T>
where
    T: FloatNumber,
{
    fn default() -> Self {
        Self {
            l: T::zero(),
            a: T::zero(),
            b: T::zero(),
            _marker: PhantomData,
        }
    }
}

impl<T> Display for Color<T>
where
    T: FloatNumber,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Color(l: {:.2}, a: {:.2}, b: {:.2})",
            self.l, self.a, self.b
        )
    }
}

impl<T> From<u32> for Color<T>
where
    T: FloatNumber,
{
    fn from(value: u32) -> Self {
        let r = (value >> 16) & 0xFF;
        let g = (value >> 8) & 0xFF;
        let b = value & 0xFF;
        let (x, y, z) = rgb_to_xyz::<T>(r as u8, g as u8, b as u8);
        let (l, a, b) = xyz_to_lab::<T, D65>(x, y, z);
        Self::new(l, a, b)
    }
}

impl<T> FromStr for Color<T>
where
    T: FloatNumber,
{
    type Err = ColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("#") {
            return Err(ColorError::InvalidHexValue(s.to_string()));
        }

        let (r, g, b) = match s.len() {
            // Handle 3-digit hex color #RGB
            4 => {
                let r = u8::from_str_radix(&s[1..2].repeat(2), 16)
                    .map_err(|_| ColorError::InvalidHexValue(s.to_string()))?;
                let g = u8::from_str_radix(&s[2..3].repeat(2), 16)
                    .map_err(|_| ColorError::InvalidHexValue(s.to_string()))?;
                let b = u8::from_str_radix(&s[3..4].repeat(2), 16)
                    .map_err(|_| ColorError::InvalidHexValue(s.to_string()))?;
                (r, g, b)
            }
            // Handle 4-digit hex color #RGBA
            5 => {
                let r = u8::from_str_radix(&s[1..2].repeat(2), 16)
                    .map_err(|_| ColorError::InvalidHexValue(s.to_string()))?;
                let g = u8::from_str_radix(&s[2..3].repeat(2), 16)
                    .map_err(|_| ColorError::InvalidHexValue(s.to_string()))?;
                let b = u8::from_str_radix(&s[3..4].repeat(2), 16)
                    .map_err(|_| ColorError::InvalidHexValue(s.to_string()))?;
                // Check whether the alpha value is valid
                let _ = u8::from_str_radix(&s[4..5].repeat(2), 16)
                    .map_err(|_| ColorError::InvalidHexValue(s.to_string()))?;
                (r, g, b)
            }
            // Handle 6-digit hex color #RRGGBB
            7 => {
                let r = u8::from_str_radix(&s[1..3], 16)
                    .map_err(|_| ColorError::InvalidHexValue(s.to_string()))?;
                let g = u8::from_str_radix(&s[3..5], 16)
                    .map_err(|_| ColorError::InvalidHexValue(s.to_string()))?;
                let b = u8::from_str_radix(&s[5..7], 16)
                    .map_err(|_| ColorError::InvalidHexValue(s.to_string()))?;
                (r, g, b)
            }
            // Handle 8-digit hex color #RRGGBBAA
            9 => {
                let r = u8::from_str_radix(&s[1..3], 16)
                    .map_err(|_| ColorError::InvalidHexValue(s.to_string()))?;
                let g = u8::from_str_radix(&s[3..5], 16)
                    .map_err(|_| ColorError::InvalidHexValue(s.to_string()))?;
                let b = u8::from_str_radix(&s[5..7], 16)
                    .map_err(|_| ColorError::InvalidHexValue(s.to_string()))?;
                // Check whether the alpha value is valid
                let _ = u8::from_str_radix(&s[7..9], 16)
                    .map_err(|_| ColorError::InvalidHexValue(s.to_string()))?;
                (r, g, b)
            }
            _ => return Err(ColorError::InvalidHexValue(s.to_string())),
        };

        let (x, y, z) = rgb_to_xyz::<T>(r, g, b);
        let (l, a, b) = xyz_to_lab::<T, D65>(x, y, z);
        Ok(Self::new(l, a, b))
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::assert_approx_eq;

    #[test]
    fn test_new() {
        // Act
        let actual: Color<f32> = Color::new(80.0, 0.0, 0.0);

        // Assert
        assert_eq!(actual.l, 80.0);
        assert_eq!(actual.a, 0.0);
        assert_eq!(actual.b, 0.0);
    }

    #[rstest]
    #[case((0.0, 0.0, 0.0), false)]
    #[case((50.0, 0.0, 0.0), false)]
    #[case((50.1, 0.0, 0.0), true)]
    #[case((80.0, 0.0, 0.0), true)]
    fn test_color_is_light(#[case] input: (f32, f32, f32), #[case] expected: bool) {
        // Act
        let color: Color<f32> = Color::new(input.0, input.1, input.2);
        let actual = color.is_light();

        // Assert
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case((0.0, 0.0, 0.0), true)]
    #[case((50.0, 0.0, 0.0), true)]
    #[case((50.1, 0.0, 0.0), false)]
    #[case((80.0, 0.0, 0.0), false)]
    fn test_color_is_dark(#[case] input: (f32, f32, f32), #[case] expected: bool) {
        // Act
        let color: Color<f32> = Color::new(input.0, input.1, input.2);
        let actual = color.is_dark();

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_lightness() {
        // Act
        let color: Color<f32> = Color::new(91.1120, -48.0806, -14.1521);
        let actual = color.lightness();

        // Assert
        assert_approx_eq!(actual, 91.1120);
    }

    #[test]
    fn test_chroma() {
        // Act
        let color: Color<f32> = Color::new(91.1120, -48.0806, -14.1521);
        let actual = color.chroma();

        // Assert
        assert_approx_eq!(actual, 50.120_117);
    }

    #[test]
    fn test_delta_e() {
        // Arrange
        let color1: Color<f32> = Color::new(91.1120, -48.0806, -14.1521);
        let color2: Color<f32> = Color::new(53.237_144, 80.088_320, 67.199_460);

        // Act
        let actual = color1.delta_e(&color2);

        // Assert
        assert_approx_eq!(actual, 156.460388);
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
        let actual = color.hue();

        // Assert
        assert_approx_eq!(actual.to_degrees(), expected, 1e-3);
    }

    #[test]
    fn test_to_hex_string() {
        // Act
        let color: Color<f32> = Color::new(91.1120, -48.0806, -14.1521);
        let actual = color.to_hex_string();

        // Assert
        assert_eq!(actual, "#00FFFF");
    }

    #[test]
    fn test_to_rgb() {
        // Act
        let color: Color<f32> = Color::new(91.1120, -48.0806, -14.1521);
        let actual = color.to_rgb();

        // Assert
        assert_eq!(actual, RGB::new(0, 255, 255));
    }

    #[test]
    fn test_to_cmyk() {
        // Act
        let color: Color<f32> = Color::new(91.1120, -48.0806, -14.1521);
        let actual = color.to_cmyk();

        // Assert
        assert_eq!(actual, CMYK::new(1.0, 0.0, 0.0, 0.0));
    }

    #[test]
    fn test_to_hsl() {
        // Act
        let color: Color<f32> = Color::new(91.1120, -48.0806, -14.1521);
        let actual = color.to_hsl();

        // Assert
        assert_approx_eq!(actual.h.to_degrees(), 180.0, 1e-3);
        assert_approx_eq!(actual.s, 1.0);
        assert_approx_eq!(actual.l, 0.5);
    }

    #[test]
    fn test_to_hsv() {
        // Act
        let color: Color<f32> = Color::new(91.1120, -48.0806, -14.1521);
        let actual = color.to_hsv();

        // Assert
        assert_approx_eq!(actual.h.to_degrees(), 180.0, 1e-3);
        assert_approx_eq!(actual.s, 1.0);
        assert_approx_eq!(actual.v, 1.0);
    }

    #[test]
    fn test_to_xyz() {
        // Act
        let color: Color<f32> = Color::new(91.1120, -48.0806, -14.1521);
        let actual: XYZ<f32> = color.to_xyz();

        // Assert
        assert_approx_eq!(actual.x, 0.5380, 1e-3);
        assert_approx_eq!(actual.y, 0.7873, 1e-3);
        assert_approx_eq!(actual.z, 1.0690, 1e-3);
    }

    #[test]
    fn test_to_luv() {
        // Act
        let color: Color<f32> = Color::new(91.1120, -48.0806, -14.1521);
        let actual = color.to_luv();

        // Assert
        assert_approx_eq!(actual.l, 91.1120);
        assert_approx_eq!(actual.u, -70.480, 1e-3);
        assert_approx_eq!(actual.v, -15.240, 1e-3);
    }

    #[test]
    fn test_to_lchuv() {
        // Act
        let color: Color<f32> = Color::new(91.1120, -48.0806, -14.1521);
        let actual = color.to_lchuv();

        // Assert
        assert_approx_eq!(actual.l, 91.1120);
        assert_approx_eq!(actual.c, 72.109, 1e-3);
        assert_approx_eq!(actual.h.to_degrees(), 192.202, 1e-3);
    }

    #[test]
    fn test_to_lab() {
        // Act
        let color: Color<f32> = Color::new(91.1120, -48.0806, -14.1521);
        let actual = color.to_lab();

        // Assert
        assert_approx_eq!(actual.l, 91.1120);
        assert_approx_eq!(actual.a, -48.0806);
        assert_approx_eq!(actual.b, -14.1521);
    }

    #[test]
    fn test_to_oklab() {
        // Act
        let color: Color<f32> = Color::new(91.1120, -48.0806, -14.1521);
        let actual = color.to_oklab();

        // Assert
        assert_approx_eq!(actual.l, 0.905, 1e-3);
        assert_approx_eq!(actual.a, -0.149, 1e-3);
        assert_approx_eq!(actual.b, -0.040, 1e-3);
    }

    #[test]
    fn test_to_oklch() {
        // Act
        let color: Color<f32> = Color::new(91.1120, -48.0806, -14.1521);
        let actual = color.to_oklch();

        // Assert
        assert_approx_eq!(actual.l, 0.905, 1e-3);
        assert_approx_eq!(actual.c, 0.155, 1e-3);
        assert_approx_eq!(actual.h.to_degrees(), 194.82, 1e-3);
    }

    #[test]
    fn test_to_lchab() {
        // Act
        let color: Color<f32> = Color::new(91.1120, -48.0806, -14.1521);
        let actual = color.to_lchab();

        // Assert
        assert_approx_eq!(actual.l, 91.1120, 1e-3);
        assert_approx_eq!(actual.c, 50.120, 1e-3);
        assert_approx_eq!(actual.h.to_degrees(), 196.401, 1e-3);
    }

    #[test]
    fn test_to_anis16() {
        // Act
        let color: Color<f32> = Color::new(91.1120, -48.0806, -14.1521);
        let actual = color.to_ansi16();

        // Assert
        assert_eq!(actual, Ansi16::bright_cyan());
    }

    #[test]
    fn test_to_ansi256() {
        // Act
        let color: Color<f32> = Color::new(91.1120, -48.0806, -14.1521);
        let actual = color.to_ansi256();

        // Assert
        assert_eq!(actual, Ansi256::new(51));
    }

    #[test]
    fn test_to_rgb_int() {
        // Act
        let color: Color<f32> = Color::new(91.1120, -48.0806, -14.1521);
        let actual = color.to_rgb_int();

        // Assert
        assert_eq!(actual, 0x00ffff);
    }

    #[test]
    fn test_to_rgba_int() {
        // Act
        let color: Color<f32> = Color::new(91.1120, -48.0806, -14.1521);
        let actual = color.to_rgba_int(128);

        // Assert
        assert_eq!(actual, 0x00ffff80);
    }

    #[rstest]
    #[case::mix_zero(0.0, (91.1120, -48.0806, -14.1521))]
    #[case::mix_half(0.5, (73.0004, 18.2257, -5.0432))]
    #[case::mix_full(1.0, (54.8888, 84.5321, 4.0656))]
    #[case::mix_over(1.5, (54.8888, 84.5321, 4.0656))]
    #[case::mix_negative(-0.5, (91.1120, -48.0806, -14.1521))]
    fn test_mix(#[case] fraction: f32, #[case] (l, a, b): (f32, f32, f32)) {
        // Arrange
        let color1: Color<f32> = Color::new(91.1120, -48.0806, -14.1521);
        let color2: Color<f32> = Color::new(54.8888, 84.5321, 4.0656);

        // Act
        let actual = color1.mix(&color2, fraction);

        // Assert
        assert_approx_eq!(actual.l, l, 1e-3);
        assert_approx_eq!(actual.a, a, 1e-3);
        assert_approx_eq!(actual.b, b, 1e-3);
    }

    #[test]
    fn test_default() {
        // Act
        let actual: Color<f64> = Color::default();

        // Assert
        assert_eq!(
            actual,
            Color {
                l: 0.0,
                a: 0.0,
                b: 0.0,
                _marker: PhantomData
            }
        );
    }

    #[test]
    fn test_fmt() {
        // Act & Assert
        let color: Color<f32> = Color::new(91.114_750, -48.080_950, -14.142_8581);
        assert_eq!(
            format!("{}", color),
            "Color(l: 91.11, a: -48.08, b: -14.14)"
        );
    }

    #[test]
    fn test_from_u32() {
        // Act
        let actual: Color<f32> = Color::from(0xff0080);

        // Assert
        assert_approx_eq!(actual.l, 54.8888, 1e-3);
        assert_approx_eq!(actual.a, 84.5321, 1e-3);
        assert_approx_eq!(actual.b, 4.0656, 1e-3);
    }

    #[rstest]
    #[case::black_rgb("#000", 0.0, 0.0, 0.0)]
    #[case::white_rgb("#fff", 100.0, - 0.002_443, 0.011_384)]
    #[case::red_rgb("#f00", 53.237_144, 80.088_320, 67.199_460)]
    #[case::green_rgb("#0f0", 87.735_535, - 86.183_550, 83.179_924)]
    #[case::blue_rgb("#00f", 32.300_800, 79.194_260, - 107.868_910)]
    #[case::cyan_rgb("#0ff", 91.114_750, - 48.080_950, - 14.142_858)]
    #[case::magenta_rgb("#f0f", 60.322_700, 98.235_580, - 60.842_370)]
    #[case::yellow_rgb("#ff0", 97.138_580, - 21.562_368, 94.476_760)]
    #[case::black_rgba("#0000", 0.0, 0.0, 0.0)]
    #[case::white_rgba("#ffff", 100.0, - 0.002_443, 0.011_384)]
    #[case::red_rgba("#f00f", 53.237_144, 80.088_320, 67.199_460)]
    #[case::green_rgba("#0f0f", 87.735_535, - 86.183_550, 83.179_924)]
    #[case::blue_rgba("#00ff", 32.300_800, 79.194_260, - 107.868_910)]
    #[case::cyan_rgba("#0fff", 91.114_750, - 48.080_950, - 14.142_858)]
    #[case::magenta_rgba("#f0ff", 60.322_700, 98.235_580, - 60.842_370)]
    #[case::yellow_rgba("#ff0f", 97.138_580, - 21.562_368, 94.476_760)]
    #[case::black_rrggbb("#000000", 0.0, 0.0, 0.0)]
    #[case::white_rrggbb("#ffffff", 100.0, - 0.002_443, 0.011_384)]
    #[case::red_rrggbb("#ff0000", 53.237_144, 80.088_320, 67.199_460)]
    #[case::green_rrggbb("#00ff00", 87.735_535, - 86.183_550, 83.179_924)]
    #[case::blue_rrggbb("#0000ff", 32.300_800, 79.194_260, - 107.868_910)]
    #[case::cyan_rrggbb("#00ffff", 91.114_750, - 48.080_950, - 14.142_858)]
    #[case::magenta_rrggbb("#ff00ff", 60.322_700, 98.235_580, - 60.842_370)]
    #[case::yellow_rrggbb("#ffff00", 97.138_580, - 21.562_368, 94.476_760)]
    #[case::black_rrggbbaa("#000000ff", 0.0, 0.0, 0.0)]
    #[case::white_rrggbbaa("#ffffffff", 100.0, - 0.002_443, 0.011_384)]
    #[case::red_rrggbbaa("#ff0000ff", 53.237_144, 80.088_320, 67.199_460)]
    #[case::green_rrggbbaa("#00ff00ff", 87.735_535, - 86.183_550, 83.179_924)]
    #[case::blue_rrggbbaa("#0000ffff", 32.300_800, 79.194_260, - 107.868_910)]
    #[case::cyan_rrggbbaa("#00ffffff", 91.114_750, - 48.080_950, - 14.142_858)]
    #[case::magenta_rrggbbaa("#ff00ffff", 60.322_700, 98.235_580, - 60.842_370)]
    #[case::yellow_rrggbbaa("#ffff00ff", 97.138_580, - 21.562_368, 94.476_760)]
    fn test_from_str(#[case] input: &str, #[case] l: f32, #[case] a: f32, #[case] b: f32) {
        // Act
        let actual: Color<f32> = Color::from_str(input).unwrap();

        // Assert
        assert_approx_eq!(actual.l, l, 1e-3);
        assert_approx_eq!(actual.a, a, 1e-3);
        assert_approx_eq!(actual.b, b, 1e-3);
    }

    #[rstest]
    #[case::empty("")]
    #[case::invalid("123456")]
    #[case::invalid_length("#12345")]
    #[case::invalid_prefix("123456")]
    #[case::invalid_rgb_r("#g00")]
    #[case::invalid_rgb_g("#0g0")]
    #[case::invalid_rgb_b("#00g")]
    #[case::invalid_rrggbb_r("#0g0000")]
    #[case::invalid_rrggbb_g("#000g00")]
    #[case::invalid_rrggbb_b("#00000g")]
    #[case::invalid_rrggbbaa_r("#0g000000")]
    #[case::invalid_rrggbbaa_g("#000g0000")]
    #[case::invalid_rrggbbaa_b("#00000g00")]
    #[case::invalid_rrggbbaa_a("#0000000g")]
    fn test_from_str_error(#[case] input: &str) {
        // Act
        let actual = Color::<f32>::from_str(input);

        // Assert
        assert!(actual.is_err());
        assert_eq!(
            actual.unwrap_err(),
            ColorError::InvalidHexValue(input.to_string())
        );
    }
}

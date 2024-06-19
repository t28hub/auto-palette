mod ansi16;
mod ansi256;
mod cmyk;
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
pub use oklab::Oklab;
pub use oklch::Oklch;
pub use rgb::RGB;
pub use white_point::*;
pub(crate) use xyz::rgb_to_xyz;
pub use xyz::XYZ;

use crate::math::FloatNumber;

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
        let xyz = self.to_xyz();
        RGB::from(&xyz)
    }

    /// Converts this color to the CMYK color space.
    ///
    /// # Returns
    /// The converted `CMYK` color.
    #[must_use]
    pub fn to_cmyk(&self) -> CMYK<T> {
        let rgb = self.to_rgb();
        CMYK::from(&rgb)
    }

    /// Converts this color to the HSL color space.
    ///
    /// # Returns
    /// The converted `HSL` color.
    #[must_use]
    pub fn to_hsl(&self) -> HSL<T> {
        let rgb = self.to_rgb();
        HSL::from(&rgb)
    }

    /// Converts this color to the HSV color space.
    ///
    /// # Returns
    /// The converted `HSV` color.
    #[must_use]
    pub fn to_hsv(&self) -> HSV<T> {
        let rgb = self.to_rgb();
        HSV::from(&rgb)
    }

    /// Converts this color to the CIE XYZ color space.
    ///
    /// # Returns
    /// The converted `XYZ` color.
    #[must_use]
    pub fn to_xyz(&self) -> XYZ<T> {
        let lab = self.to_lab();
        XYZ::from(&lab)
    }

    /// Converts this color to the CIE L*u*v* color space.
    ///
    /// # Returns
    /// The converted `Luv` color.
    #[must_use]
    pub fn to_luv(&self) -> Luv<T, W> {
        let xyz = self.to_xyz();
        Luv::<T, W>::from(&xyz)
    }

    /// Converts this color to the CIE LCH(uv) color space.
    ///
    /// # Returns
    /// The converted `LCHuv` color.
    #[must_use]
    pub fn to_lchuv(&self) -> LCHuv<T, W> {
        let luv = self.to_luv();
        LCHuv::<T, W>::from(&luv)
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
        let lab = self.to_lab();
        LCHab::<T, W>::from(&lab)
    }

    /// Converts this color to the CIE Oklab color space.
    ///
    /// # Returns
    /// The converted `Oklab` color.
    #[must_use]
    pub fn to_oklab(&self) -> Oklab<T> {
        let xyz = self.to_xyz();
        Oklab::from(&xyz)
    }

    /// Converts this color to the CIE Oklch color space.
    ///
    /// # Returns
    /// The converted `Oklch` color.
    #[must_use]
    pub fn to_oklch(&self) -> Oklch<T> {
        let oklab = self.to_oklab();
        Oklch::from(&oklab)
    }

    /// Converts this color to the 4-bit ANSI 16 color space.
    ///
    /// # Returns
    /// The converted `Ansi16` color.
    #[must_use]
    pub fn to_ansi16(&self) -> Ansi16 {
        let rgb = self.to_rgb();
        Ansi16::from(&rgb)
    }

    /// Converts this color to the 8-bit ANSI 256 color space.
    ///
    /// # Returns
    /// The converted `Ansi256` color.
    #[must_use]
    pub fn to_ansi256(&self) -> Ansi256 {
        let rgb = self.to_rgb();
        Ansi256::from(&rgb)
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
    use rstest::rstest;

    use super::*;

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
        assert_eq!(actual, 91.1120);
    }

    #[test]
    fn test_chroma() {
        // Act
        let color: Color<f32> = Color::new(91.1120, -48.0806, -14.1521);
        let actual = color.chroma();

        // Assert
        assert!((actual - 50.120_117).abs() < 1e-3);
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
        assert!((actual.to_degrees() - expected).abs() < 1e-3);
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
        assert!((actual.h.to_degrees() - 180.0).abs() < 1e-3);
        assert!((actual.s - 1.0).abs() < 1e-3);
        assert!((actual.l - 0.5).abs() < 1e-3);
    }

    #[test]
    fn test_to_hsv() {
        // Act
        let color: Color<f32> = Color::new(91.1120, -48.0806, -14.1521);
        let actual = color.to_hsv();

        // Assert
        assert!((actual.h.to_degrees() - 180.0).abs() < 1e-3);
        assert!((actual.s - 1.0).abs() < 1e-3);
        assert!((actual.v - 1.0).abs() < 1e-3);
    }

    #[test]
    fn test_to_xyz() {
        // Act
        let color: Color<f32> = Color::new(91.1120, -48.0806, -14.1521);
        let actual: XYZ<f32> = color.to_xyz();

        // Assert
        assert!((actual.x - 0.5380).abs() < 1e-3);
        assert!((actual.y - 0.7873).abs() < 1e-3);
        assert!((actual.z - 1.0690).abs() < 1e-3);
    }

    #[test]
    fn test_to_luv() {
        // Act
        let color: Color<f32> = Color::new(91.1120, -48.0806, -14.1521);
        let actual = color.to_luv();

        // Assert
        assert_eq!(actual.l, 91.1120);
        assert!((actual.u - -70.480).abs() < 1e-3);
        assert!((actual.v - -15.240).abs() < 1e-3);
    }

    #[test]
    fn test_to_lchuv() {
        // Act
        let color: Color<f32> = Color::new(91.1120, -48.0806, -14.1521);
        let actual = color.to_lchuv();

        // Assert
        assert_eq!(actual.l, 91.1120);
        assert!((actual.c - 72.109).abs() < 1e-3);
        assert!((actual.h.to_degrees() - 192.202).abs() < 1e-3);
    }

    #[test]
    fn test_to_lab() {
        // Act
        let color: Color<f32> = Color::new(91.1120, -48.0806, -14.1521);
        let actual = color.to_lab();

        // Assert
        assert_eq!(actual.l, 91.1120);
        assert_eq!(actual.a, -48.0806);
        assert_eq!(actual.b, -14.1521);
    }

    #[test]
    fn test_to_oklab() {
        // Act
        let color: Color<f32> = Color::new(91.1120, -48.0806, -14.1521);
        let actual = color.to_oklab();

        // Assert
        assert!((actual.l - 0.905).abs() < 1e-3);
        assert!((actual.a + 0.149).abs() < 1e-3);
        assert!((actual.b + 0.040).abs() < 1e-3);
    }

    #[test]
    fn test_to_oklch() {
        // Act
        let color: Color<f32> = Color::new(91.1120, -48.0806, -14.1521);
        let actual = color.to_oklch();

        // Assert
        assert!((actual.l - 0.905).abs() < 1e-3);
        assert!((actual.c - 0.155).abs() < 1e-3);
        assert!((actual.h.to_degrees() - 194.82).abs() < 1e-3);
    }

    #[test]
    fn test_to_lchab() {
        // Act
        let color: Color<f32> = Color::new(91.1120, -48.0806, -14.1521);
        let actual = color.to_lchab();

        // Assert
        assert_eq!(actual.l, 91.1120);
        assert!((actual.c - 50.120).abs() < 1e-3);
        assert!((actual.h.to_degrees() - 196.401).abs() < 1e-3);
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
    fn test_fmt() {
        // Act & Assert
        let color: Color<f32> = Color::new(91.114_750, -48.080_950, -14.142_8581);
        assert_eq!(
            format!("{}", color),
            "Color(l: 91.11, a: -48.08, b: -14.14)"
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
        let actual: Color<f32> = Color::from_str(input).unwrap();

        // Assert
        assert!((actual.l - l).abs() < 1e-3);
        assert!((actual.a - a).abs() < 1e-3);
        assert!((actual.b - b).abs() < 1e-3);
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
        let actual = Color::<f32>::from_str(input);

        // Assert
        assert!(actual.is_err());
    }
}

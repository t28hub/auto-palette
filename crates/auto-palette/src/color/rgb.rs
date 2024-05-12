use std::{fmt, fmt::Display};

use crate::{
    color::{hsl::HSL, xyz::XYZ, HSV},
    math::FloatNumber,
};

/// Color represented in the RGB color space.
///
/// # Fields
/// * `r` - The red component.
/// * `g` - The green component.
/// * `b` - The blue component.
///
/// # Examples
/// ```
/// use auto_palette::color::{HSL, HSV, RGB, XYZ};
///
/// let rgb = RGB::new(255, 0, 64);
/// assert_eq!(format!("{}", rgb), "RGB(255, 0, 64)");
///
/// let hsl: HSL<f32> = (&rgb).into();
/// assert_eq!(format!("{}", hsl), "HSL(344.94, 1.00, 0.50)");
///
/// let hsv: HSV<f32> = (&rgb).into();
/// assert_eq!(format!("{}", hsv), "HSV(344.94, 1.00, 1.00)");
///
/// let xyz: XYZ<f32> = (&rgb).into();
/// assert_eq!(format!("{}", xyz), "XYZ(0.42, 0.22, 0.07)");
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    /// Creates a new `RGB` instance.
    ///
    /// # Arguments
    /// * `r` - The red component.
    /// * `g` - The green component.
    /// * `b` - The blue component.
    ///
    /// # Returns
    /// A new `RGB` instance.
    #[must_use]
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Returns the maximum value of the RGB color space.
    ///
    /// # Returns
    /// The maximum value of the RGB color space.
    #[inline]
    #[must_use]
    pub(crate) fn max_value<T>() -> T
    where
        T: FloatNumber,
    {
        T::from_u8(u8::MAX)
    }
}

impl Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RGB({}, {}, {})", self.r, self.g, self.b)
    }
}

impl<T> From<&HSL<T>> for RGB
where
    T: FloatNumber,
{
    fn from(hsl: &HSL<T>) -> Self {
        let hue = hsl.h.value();

        let c = (T::one() - (T::from_u8(2) * hsl.l - T::one()).abs()) * hsl.s;
        let x = (T::one() - (((hue / T::from_f32(60.0)) % T::from_u8(2)) - T::one()).abs()) * c;
        let m = hsl.l - (c / T::from_u8(2));

        let (r, g, b) = if hue < T::from_f32(60.0) {
            (c, x, T::zero())
        } else if hue < T::from_f32(120.0) {
            (x, c, T::zero())
        } else if hue < T::from_f32(180.0) {
            (T::zero(), c, x)
        } else if hue < T::from_f32(240.0) {
            (T::zero(), x, c)
        } else if hue < T::from_f32(300.0) {
            (x, T::zero(), c)
        } else {
            (c, T::zero(), x)
        };

        let max = RGB::max_value::<T>();
        RGB::new(
            ((r + m) * max).round().to_u8_unsafe(),
            ((g + m) * max).round().to_u8_unsafe(),
            ((b + m) * max).round().to_u8_unsafe(),
        )
    }
}

impl<T> From<&HSV<T>> for RGB
where
    T: FloatNumber,
{
    fn from(hsv: &HSV<T>) -> Self {
        let hue = hsv.h.value();

        let c = hsv.v * hsv.s;
        let x = (T::one() - (((hue / T::from_f32(60.0)) % T::from_u8(2)) - T::one()).abs()) * c;
        let m = hsv.v - c;

        let (r, g, b) = if hue < T::from_f32(60.0) {
            (c, x, T::zero())
        } else if hue < T::from_f32(120.0) {
            (x, c, T::zero())
        } else if hue < T::from_f32(180.0) {
            (T::zero(), c, x)
        } else if hue < T::from_f32(240.0) {
            (T::zero(), x, c)
        } else if hue < T::from_f32(300.0) {
            (x, T::zero(), c)
        } else {
            (c, T::zero(), x)
        };

        let max = RGB::max_value::<T>();
        RGB::new(
            ((r + m) * max).round().to_u8_unsafe(),
            ((g + m) * max).round().to_u8_unsafe(),
            ((b + m) * max).round().to_u8_unsafe(),
        )
    }
}

impl<T> From<&XYZ<T>> for RGB
where
    T: FloatNumber,
{
    fn from(xyz: &XYZ<T>) -> Self {
        let (r, g, b) = xyz_to_rgb(xyz.x, xyz.y, xyz.z);
        RGB::new(r, g, b)
    }
}

/// Converts the CIE XYZ color space to the RGB color space.
///
/// # Type Parameters
/// * `T` - The floating point type.
///
/// # Arguments
/// * `x` - The X component of the XYZ color.
/// * `y` - The Y component of the XYZ color.
/// * `z` - The Z component of the XYZ color.
///
/// # Returns
/// The RGB color space representation of the XYZ color. The tuple contains the red, green, and blue components.
#[inline]
#[must_use]
pub fn xyz_to_rgb<T>(x: T, y: T, z: T) -> (u8, u8, u8)
where
    T: FloatNumber,
{
    let f = |t: T| -> T {
        if t > T::from_f32(0.003_130_8) {
            T::from_f32(1.055) * t.powf(T::from_f32(1.0 / 2.4)) - T::from_f32(0.055)
        } else {
            T::from_f32(12.92) * t
        }
    };

    let r = f(T::from_f32(3.240_97) * x - T::from_f32(1.537_383) * y - T::from_f32(0.498_611) * z);
    let g =
        f(-T::from_f32(0.969_244) * x + T::from_f32(1.875_968) * y + T::from_f32(0.041_555) * z);
    let b = f(T::from_f32(0.055_630) * x - T::from_f32(0.203_977) * y + T::from_f32(1.056_972) * z);

    let max = RGB::max_value::<T>();
    (
        (r * max).round().to_u8_unsafe(),
        (g * max).round().to_u8_unsafe(),
        (b * max).round().to_u8_unsafe(),
    )
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_new() {
        // Act
        let actual = RGB::new(255, 0, 64);

        // Assert
        assert_eq!(actual.r, 255);
        assert_eq!(actual.g, 0);
        assert_eq!(actual.b, 64);
    }

    #[test]
    fn test_fmt() {
        // Act
        let rgb = RGB::new(255, 0, 64);
        let actual = format!("{}", rgb);

        // Assert
        assert_eq!(actual, "RGB(255, 0, 64)");
    }

    #[rstest]
    #[case::black((0.0, 0.0, 0.0), (0, 0, 0))]
    #[case::white((0.0, 0.0, 1.0), (255, 255, 255))]
    #[case::red((0.0, 1.0, 0.5), (255, 0, 0))]
    #[case::green((120.0, 1.0, 0.5), (0, 255, 0))]
    #[case::blue((240.0, 1.0, 0.5), (0, 0, 255))]
    #[case::yellow((60.0, 1.0, 0.5), (255, 255, 0))]
    #[case::cyan((180.0, 1.0, 0.5), (0, 255, 255))]
    #[case::magenta((300.0, 1.0, 0.5), (255, 0, 255))]
    fn test_from_hsl(#[case] hsl: (f32, f32, f32), #[case] rgb: (u8, u8, u8)) {
        // Act
        let (h, s, l) = hsl;
        let actual = RGB::from(&HSL::new(h, s, l));

        // Assert
        assert_eq!(actual.r, rgb.0);
        assert_eq!(actual.g, rgb.1);
        assert_eq!(actual.b, rgb.2);
    }

    #[rstest]
    #[case::black((0.0, 0.0, 0.0), (0, 0, 0))]
    #[case::white((0.0, 0.0, 1.0), (255, 255, 255))]
    #[case::red((0.0, 1.0, 1.0), (255, 0, 0))]
    #[case::green((120.0, 1.0, 1.0), (0, 255, 0))]
    #[case::blue((240.0, 1.0, 1.0), (0, 0, 255))]
    #[case::yellow((60.0, 1.0, 1.0), (255, 255, 0))]
    #[case::cyan((180.0, 1.0, 1.0), (0, 255, 255))]
    #[case::magenta((300.0, 1.0, 1.0), (255, 0, 255))]
    fn test_from_hsv(#[case] hsv: (f32, f32, f32), #[case] rgb: (u8, u8, u8)) {
        // Act
        let (h, s, v) = hsv;
        let actual = RGB::from(&HSV::new(h, s, v));

        // Assert
        assert_eq!(actual.r, rgb.0);
        assert_eq!(actual.g, rgb.1);
        assert_eq!(actual.b, rgb.2);
    }

    #[test]
    fn test_from_xyz() {
        let xyz = XYZ::new(0.3576, 0.7152, 0.119);
        let actual = RGB::from(&xyz);

        // Assert
        assert_eq!(actual.r, 0);
        assert_eq!(actual.g, 255);
        assert_eq!(actual.b, 0);
    }

    #[rstest]
    #[case::black((0.0, 0.0, 0.0), (0, 0, 0))]
    #[case::white((0.9505, 1.0000, 1.0886), (255, 255, 255))]
    #[case::red((0.4125, 0.2127, 0.0193), (255, 0, 0))]
    #[case::green((0.3576, 0.7152, 0.1192), (0, 255, 0))]
    #[case::blue((0.1804, 0.0722, 0.9502), (0, 0, 255))]
    #[case::cyan((0.53802, 0.7873, 1.0698), (0, 255, 255))]
    #[case::magenta((0.5928, 0.2848, 0.9699), (255, 0, 255))]
    #[case::yellow((0.7700, 0.9278, 0.1385), (255, 255, 0))]
    fn test_xyz_to_rgb(#[case] xyz: (f32, f32, f32), #[case] rgb: (u8, u8, u8)) {
        // Act
        let (r, g, b) = xyz_to_rgb(xyz.0, xyz.1, xyz.2);

        // Assert
        assert_eq!(r, rgb.0);
        assert_eq!(g, rgb.1);
        assert_eq!(b, rgb.2);
    }
}

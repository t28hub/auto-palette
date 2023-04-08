use crate::color::xyz::XYZ;
use crate::math::number::{Float, Number};
use crate::white_point::WhitePoint;
use std::fmt::{Display, Formatter, Result};

/// Struct representing a color in standard RGB color space.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    /// Creates a new RGB color.
    ///
    /// # Arguments
    /// * `r` - The red component of this color.
    /// * `g` - The green component of this color.
    /// * `b` - The blue component of this color.
    ///
    /// # Returns
    /// A new RGB color.
    #[inline]
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Returns the min value for each component of an RGBA color.
    ///
    /// # Returns
    /// The min value for each component of an RGBA color.
    #[inline]
    #[must_use]
    pub(crate) fn min_value<T: Number>() -> T {
        T::from_u8(u8::MIN)
    }

    /// Returns the max value for each component of an RGBA color.
    ///
    /// # Returns
    /// The max value for each component of an RGBA color.
    #[inline]
    #[must_use]
    pub(crate) fn max_value<T: Number>() -> T {
        T::from_u8(u8::MAX)
    }

    /// Returns the red component of this color.
    ///
    /// # Returns
    /// The red component of this color.
    #[inline]
    #[must_use]
    pub fn r<T: Number>(&self) -> T {
        T::from_u8(self.r)
    }

    /// Returns the green component of this color.
    ///
    /// # Returns
    /// The green component of this color.
    #[inline]
    #[must_use]
    pub fn g<T: Number>(&self) -> T {
        T::from_u8(self.g)
    }

    /// Returns the blue component of this color.
    ///
    /// # Returns
    /// The blue component of this color.
    #[inline]
    #[must_use]
    pub fn b<T: Number>(&self) -> T {
        T::from_u8(self.b)
    }
}

impl Default for Rgb {
    #[must_use]
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Rgba({r}, {g}, {b})", r = self.r, g = self.g, b = self.b,)
    }
}

impl<F, WP> From<&XYZ<F, WP>> for Rgb
where
    F: Float,
    WP: WhitePoint<F>,
{
    #[inline]
    #[must_use]
    fn from(xyz: &XYZ<F, WP>) -> Self {
        let f = |value: F| -> F {
            if value <= F::from_f64(0.0031308) {
                F::from_f64(12.92) * value
            } else {
                F::from_f64(1.055) * value.powf(F::from_f64(1.0 / 2.4)) - F::from_f64(0.055)
            }
        };

        let fr = f(F::from_f64(3.24097) * xyz.x
            - F::from_f64(1.537383) * xyz.y
            - F::from_f64(0.498611) * xyz.z);
        let fg = f(F::from_f64(-0.969244) * xyz.x
            + F::from_f64(1.875968) * xyz.y
            + F::from_f64(0.041555) * xyz.z);
        let fb = f(F::from_f64(0.05563) * xyz.x - F::from_f64(0.203977) * xyz.y
            + F::from_f64(1.056972) * xyz.z);

        let min_value = Rgb::min_value::<F>();
        let max_value = Rgb::max_value::<F>();
        let denormalize = |value: F| {
            let clamped = (value * max_value).clamp(min_value, max_value);
            clamped.round().to_u8().unwrap_or_else(Rgb::min_value)
        };
        Self {
            r: denormalize(fr),
            g: denormalize(fg),
            b: denormalize(fb),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::white_point::D65;
    use rstest::rstest;

    #[test]
    fn test_rgba() {
        let actual = Rgb::new(0, 64, 128);
        assert_eq!(actual.r, 0);
        assert_eq!(actual.g, 64);
        assert_eq!(actual.b, 128);
    }

    #[test]
    fn test_components() {
        let rgb = Rgb::new(0, 64, 128);
        assert_eq!(rgb.r::<f64>(), 0.0);
        assert_eq!(rgb.g::<f64>(), 64.0);
        assert_eq!(rgb.b::<f64>(), 128.0);
    }

    #[test]
    fn test_default() {
        let rgb = Rgb::default();
        assert_eq!(rgb.r, 0);
        assert_eq!(rgb.g, 0);
        assert_eq!(rgb.b, 0);
    }

    #[test]
    fn test_to_string() {
        let rgb = Rgb::new(0, 64, 128);
        assert_eq!(rgb.to_string(), "Rgba(0, 64, 128)");
    }

    #[rstest]
    #[case((0.0000, 0.0000, 0.0000), (0, 0, 0))] // Black
    #[case((0.9505, 1.0000, 1.0890), (255, 255, 255))] // White
    #[case((0.4124, 0.2126, 0.0193), (255, 0, 0))] // Red
    #[case((0.3576, 0.7152, 0.1192), (0, 255, 0))] // Green
    #[case((0.1805, 0.0722, 0.9505), (0, 0, 255))] // Blue
    #[case((0.5381, 0.7874, 1.0697), (0, 255, 255))] // Cyan
    #[case((0.5929, 0.2848, 0.9698), (255, 0, 255))] // Magenta
    #[case((0.7700, 0.9278, 0.1385), (255, 255, 0))] // Yellow
    fn test_from_xyz(#[case] xyz: (f64, f64, f64), #[case] expected: (u8, u8, u8)) {
        let actual = Rgb::from(&XYZ::<_, D65>::new(xyz.0, xyz.1, xyz.2));
        let (r, g, b) = expected;
        assert_eq!(actual.r, r);
        assert_eq!(actual.g, g);
        assert_eq!(actual.b, b);
    }
}

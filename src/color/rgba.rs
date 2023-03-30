use crate::color::xyz::XYZ;
use crate::math::number::{Float, Number};
use std::fmt::{Display, Formatter, Result};

/// Struct representing a color in standard RGB color space.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    /// Creates a new RGBA color.
    ///
    /// # Arguments
    /// * `r` - The red component of this color.
    /// * `g` - The green component of this color.
    /// * `b` - The blue component of this color.
    /// * `a` - The alpha component of this color.
    ///
    /// # Returns
    /// A new RGBA color.
    #[inline]
    #[must_use]
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
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

    /// Returns the alpha component of this color.
    ///
    /// # Returns
    /// The alpha component of this color.
    #[inline]
    #[must_use]
    pub fn a<T: Number>(&self) -> T {
        T::from_u8(self.a)
    }
}

impl Default for Rgba {
    #[must_use]
    fn default() -> Self {
        Self::new(0, 0, 0, 0)
    }
}

impl Display for Rgba {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Rgba({r}, {g}, {b}, {a})",
            r = self.r,
            g = self.g,
            b = self.b,
            a = self.a
        )
    }
}

impl<F> From<&XYZ<F>> for Rgba
where
    F: Float,
{
    #[inline]
    #[must_use]
    fn from(xyz: &XYZ<F>) -> Self {
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

        let min_value = Rgba::min_value::<F>();
        let max_value = Rgba::max_value::<F>();
        let denormalize = |value: F| {
            let clamped = (value * max_value).clamp(min_value, max_value);
            clamped.round().to_u8().unwrap_or_else(Rgba::min_value)
        };
        Self {
            r: denormalize(fr),
            g: denormalize(fg),
            b: denormalize(fb),
            a: Rgba::max_value(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_rgba() {
        let rgba = Rgba::new(0, 64, 128, 255);
        assert_eq!(rgba.r, 0);
        assert_eq!(rgba.g, 64);
        assert_eq!(rgba.b, 128);
        assert_eq!(rgba.a, 255);
    }

    #[test]
    fn test_components() {
        let rgba = Rgba::new(0, 64, 128, 255);
        assert_eq!(rgba.r::<f64>(), 0.0);
        assert_eq!(rgba.g::<f64>(), 64.0);
        assert_eq!(rgba.b::<f64>(), 128.0);
        assert_eq!(rgba.a::<f64>(), 255.0);
    }

    #[test]
    fn test_default() {
        let rgba = Rgba::default();
        assert_eq!(rgba.r, 0);
        assert_eq!(rgba.g, 0);
        assert_eq!(rgba.b, 0);
        assert_eq!(rgba.a, 0);
    }

    #[test]
    fn test_to_string() {
        let rgba = Rgba::new(0, 64, 128, 255);
        assert_eq!(rgba.to_string(), "Rgba(0, 64, 128, 255)");
    }

    #[rstest]
    #[case((0.0000, 0.0000, 0.0000), (0, 0, 0, 255))] // Black
    #[case((0.9505, 1.0000, 1.0890), (255, 255, 255, 255))] // White
    #[case((0.4124, 0.2126, 0.0193), (255, 0, 0, 255))] // Red
    #[case((0.3576, 0.7152, 0.1192), (0, 255, 0, 255))] // Green
    #[case((0.1805, 0.0722, 0.9505), (0, 0, 255, 255))] // Blue
    #[case((0.5381, 0.7874, 1.0697), (0, 255, 255, 255))] // Cyan
    #[case((0.5929, 0.2848, 0.9698), (255, 0, 255, 255))] // Magenta
    #[case((0.7700, 0.9278, 0.1385), (255, 255, 0, 255))] // Yellow
    fn test_from_xyz(#[case] xyz: (f64, f64, f64), #[case] expected: (u8, u8, u8, u8)) {
        let actual = Rgba::from(&XYZ::new(xyz.0, xyz.1, xyz.2));
        let (r, g, b, a) = expected;
        assert_eq!(actual.r, r);
        assert_eq!(actual.g, g);
        assert_eq!(actual.b, b);
        assert_eq!(actual.a, a);
    }
}

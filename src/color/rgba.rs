use crate::color::xyz::XYZ;
use crate::math::number::{Float, Number};
use std::fmt::{Display, Formatter, Result};

/// Color in standard RGB color space.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    const MIN: u8 = u8::MIN;
    const MAX: u8 = u8::MAX;

    /// Returns the min value of RGBA.
    #[inline]
    #[must_use]
    pub fn min_value<T: Number>() -> T {
        T::from_u8(Self::MIN)
    }

    /// Returns the max value of RGBA.
    #[inline]
    #[must_use]
    pub fn max_value<T: Number>() -> T {
        T::from_u8(Self::MAX)
    }

    /// Create a new RGBA color.
    #[inline]
    #[must_use]
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Return the value of red.
    #[inline]
    #[must_use]
    pub fn r<T: Number>(&self) -> T {
        T::from_u8(self.r)
    }

    /// Return the value of green.
    #[inline]
    #[must_use]
    pub fn g<T: Number>(&self) -> T {
        T::from_u8(self.g)
    }

    /// Return the value of blue.
    #[inline]
    #[must_use]
    pub fn b<T: Number>(&self) -> T {
        T::from_u8(self.b)
    }

    /// Return the value of alpha.
    #[inline]
    #[must_use]
    pub fn a<T: Number>(&self) -> T {
        T::from_u8(self.a)
    }

    /// Create a white color.
    #[must_use]
    pub(crate) fn white() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
    }

    /// Create a black color.
    #[must_use]
    pub(crate) fn black() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }

    /// Create a red color.
    #[must_use]
    pub(crate) fn red() -> Self {
        Self {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        }
    }

    /// Create a green color.
    #[must_use]
    pub(crate) fn green() -> Self {
        Self {
            r: 0,
            g: 255,
            b: 0,
            a: 255,
        }
    }

    /// Create a blue color.
    #[must_use]
    pub(crate) fn blue() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 255,
            a: 255,
        }
    }

    /// Create a transparent color.
    #[must_use]
    pub(crate) fn transparent() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }

    #[must_use]
    fn normalize_value<F: Float>(value: F) -> u8 {
        value.to_u8().expect("The value could not be cast to u8")
    }
}

impl Default for Rgba {
    #[must_use]
    fn default() -> Self {
        Rgba::transparent()
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

        let max_value = Rgba::max_value::<F>();
        let r = Self::normalize_value((fr * max_value).round());
        let g = Self::normalize_value((fg * max_value).round());
        let b = Self::normalize_value((fb * max_value).round());
        Self {
            r,
            g,
            b,
            a: Rgba::max_value(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_create_rgba_color() {
        let rgba = Rgba::new(0, 64, 255, 128);
        assert_eq!(rgba.r, 0);
        assert_eq!(rgba.g, 64);
        assert_eq!(rgba.b, 255);
        assert_eq!(rgba.a, 128);
    }

    #[test]
    fn to_string_should_return_string_representation() {
        let rgba = Rgba::new(0, 64, 255, 128);
        assert_eq!(rgba.to_string(), "Rgba(0, 64, 255, 128)");
    }

    #[test]
    fn from_xyz_should_create_rgba_color() {
        let black = XYZ::new(0.0, 0.0, 0.0);
        assert_eq!(Rgba::from(&black), Rgba::black());

        let white = XYZ::new(0.950456, 1.0, 1.0886440);
        assert_eq!(Rgba::from(&white), Rgba::white());

        let red = XYZ::new(0.412391, 0.212639, 0.019331);
        assert_eq!(Rgba::from(&red), Rgba::red());

        let green = XYZ::new(0.357584, 0.715169, 0.119195);
        assert_eq!(Rgba::from(&green), Rgba::green());

        let blue = XYZ::new(0.180481, 0.072192, 0.950532);
        assert_eq!(Rgba::from(&blue), Rgba::blue());
    }

    #[test]
    fn should_return_value_to_be_cast() {
        let rgba = Rgba::new(0, 64, 255, 128);
        assert_eq!(rgba.r::<f64>(), 0.0);
        assert_eq!(rgba.g::<f64>(), 64.0);
        assert_eq!(rgba.b::<f64>(), 255.0);
        assert_eq!(rgba.a::<f64>(), 128.0);
    }
}

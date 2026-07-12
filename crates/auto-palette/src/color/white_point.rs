use std::fmt::Debug;

/// The white point representation.
///
/// See the following for more details:
/// [White point - Wikipedia](https://en.wikipedia.org/wiki/White_point)
pub trait WhitePoint: Copy + Clone + Debug + Default + PartialEq {
    /// Returns the X component of the white point.
    ///
    /// # Returns
    /// The X component of the white point.
    #[must_use]
    fn x() -> f64;

    /// Returns the Y component of the white point.
    ///
    /// # Returns
    /// The Y component of the white point.
    #[must_use]
    fn y() -> f64;

    /// Returns the Z component of the white point.
    ///
    /// # Returns
    /// The Z component of the white point.
    #[must_use]
    fn z() -> f64;
}

/// The D50 white point representation.
///
/// This is commonly used in color science and is the default white point for many color spaces, including CIE L*a*b* and CIE L*C*h*.
///
/// See the following for more details:
/// [Illuminant D50](https://en.wikipedia.org/wiki/Illuminant_D50)
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[allow(dead_code)]
pub struct D50;

#[allow(dead_code)]
impl D50 {
    const X: f64 = 0.964_22;
    const Y: f64 = 1.0;
    const Z: f64 = 0.825_21;
}

impl WhitePoint for D50 {
    #[inline]
    fn x() -> f64 {
        Self::X
    }

    #[inline]
    fn y() -> f64 {
        Self::Y
    }

    #[inline]
    fn z() -> f64 {
        Self::Z
    }
}

/// The D65 white point representation.
///
/// This is commonly used in color science and is the default white point for many color spaces, including sRGB and Adobe RGB.
///
/// See the following for more details:
/// [Illuminant D65](https://en.wikipedia.org/wiki/Illuminant_D65)
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct D65;

impl D65 {
    const X: f64 = 0.950_47;
    const Y: f64 = 1.0;
    const Z: f64 = 1.088_83;
}

impl WhitePoint for D65 {
    #[inline]
    fn x() -> f64 {
        Self::X
    }

    #[inline]
    fn y() -> f64 {
        Self::Y
    }

    #[inline]
    fn z() -> f64 {
        Self::Z
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d50() {
        let x = D50::x();
        assert_eq!(x, D50::X);

        let y = D50::y();
        assert_eq!(y, D50::Y);

        let z = D50::z();
        assert_eq!(z, D50::Z);
    }

    #[test]
    fn test_d65() {
        let x = D65::x();
        assert_eq!(x, D65::X);

        let y = D65::y();
        assert_eq!(y, D65::Y);

        let z = D65::z();
        assert_eq!(z, D65::Z);
    }
}

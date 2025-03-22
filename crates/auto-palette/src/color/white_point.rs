use std::fmt::Debug;

use crate::math::FloatNumber;

/// The white point representation.
///
/// See the following for more details:
/// [White point - Wikipedia](https://en.wikipedia.org/wiki/White_point)
pub trait WhitePoint: Copy + Clone + Debug + Default + PartialEq {
    /// Returns the X component of the white point.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type.
    ///
    /// # Returns
    /// The X component of the white point.
    #[must_use]
    fn x<T>() -> T
    where
        T: FloatNumber;

    /// Returns the Y component of the white point.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type.
    ///
    /// # Returns
    /// The Y component of the white point.
    #[must_use]
    fn y<T>() -> T
    where
        T: FloatNumber;

    /// Returns the Z component of the white point.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type.
    ///
    /// # Returns
    /// The Z component of the white point.
    #[must_use]
    fn z<T>() -> T
    where
        T: FloatNumber;
}

/// The D50 white point representation.
///
/// This is commonly used in color science and is the default white point for many color spaces, including CIE L*a*b* and CIE L*C*h*.
///
/// See the following for more details:
/// [Illuminant D50](https://en.wikipedia.org/wiki/Illuminant_D50)
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct D50;

impl D50 {
    const X: f64 = 0.964_22;
    const Y: f64 = 1.0;
    const Z: f64 = 0.825_21;
}

impl WhitePoint for D50 {
    #[inline]
    fn x<T>() -> T
    where
        T: FloatNumber,
    {
        T::from_f64(Self::X)
    }

    #[inline]
    fn y<T>() -> T
    where
        T: FloatNumber,
    {
        T::from_f64(Self::Y)
    }

    #[inline]
    fn z<T>() -> T
    where
        T: FloatNumber,
    {
        T::from_f64(Self::Z)
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
    fn x<T>() -> T
    where
        T: FloatNumber,
    {
        T::from_f64(Self::X)
    }

    #[inline]
    fn y<T>() -> T
    where
        T: FloatNumber,
    {
        T::from_f64(Self::Y)
    }

    #[inline]
    fn z<T>() -> T
    where
        T: FloatNumber,
    {
        T::from_f64(Self::Z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d50() {
        let x: f64 = D50::x();
        assert_eq!(x, D50::X);

        let y: f64 = D50::y();
        assert_eq!(y, D50::Y);

        let z: f64 = D50::z();
        assert_eq!(z, D50::Z);
    }

    #[test]
    fn test_d65() {
        let x: f64 = D65::x();
        assert_eq!(x, D65::X);

        let y: f64 = D65::y();
        assert_eq!(y, D65::Y);

        let z: f64 = D65::z();
        assert_eq!(z, D65::Z);
    }
}

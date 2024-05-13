use std::fmt::Debug;

use crate::math::FloatNumber;

/// White point representation.
///
/// # References
/// * [White point - Wikipedia](https://en.wikipedia.org/wiki/White_point)
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
/// # References
/// * [Illuminant D50](https://en.wikipedia.org/wiki/Illuminant_D50)
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct D50;

impl WhitePoint for D50 {
    #[inline]
    fn x<T>() -> T
    where
        T: FloatNumber,
    {
        T::from_f32(0.964_22)
    }

    #[inline]
    fn y<T>() -> T
    where
        T: FloatNumber,
    {
        T::from_f32(1.0)
    }

    #[inline]
    fn z<T>() -> T
    where
        T: FloatNumber,
    {
        T::from_f32(0.825_21)
    }
}

/// The D65 white point representation.
///
/// # References
/// * [Illuminant D65](https://en.wikipedia.org/wiki/Illuminant_D65)
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct D65;

impl WhitePoint for D65 {
    #[inline]
    fn x<T>() -> T
    where
        T: FloatNumber,
    {
        T::from_f32(0.950_470)
    }

    #[inline]
    fn y<T>() -> T
    where
        T: FloatNumber,
    {
        T::from_f32(1.0)
    }

    #[inline]
    fn z<T>() -> T
    where
        T: FloatNumber,
    {
        T::from_f32(1.088_83)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d50() {
        let x: f32 = D50::x();
        assert_eq!(x, 0.964_22);

        let y: f32 = D50::y();
        assert_eq!(y, 1.0);

        let z: f32 = D50::z();
        assert_eq!(z, 0.825_21);
    }

    #[test]
    fn test_d65() {
        let x: f32 = D65::x();
        assert_eq!(x, 0.950_470);

        let y: f32 = D65::y();
        assert_eq!(y, 1.0);

        let z: f32 = D65::z();
        assert_eq!(z, 1.088_83);
    }
}

use std::fmt::Debug;

/// White point in the CIE 1931 XYZ color space.
///
/// # References
/// * [White point - Wikipedia](https://en.wikipedia.org/wiki/White_point)
pub trait WhitePoint: Debug + Default + PartialEq {
    /// Returns the X component of the white point.
    ///
    /// # Returns
    /// The X component of the white point.
    #[must_use]
    fn x() -> f32;

    /// Returns the Y component of the white point.
    ///
    /// # Returns
    /// The Y component of the white point.
    #[must_use]
    fn y() -> f32;

    /// Returns the Z component of the white point.
    ///
    /// # Returns
    /// The Z component of the white point.
    #[must_use]
    fn z() -> f32;
}

/// Struct representing CIE standard illuminant D65.
///
/// # References
/// * [Illuminant D65](https://en.wikipedia.org/wiki/Illuminant_D65)
#[derive(Debug, Default, PartialEq)]
pub struct D65;

impl WhitePoint for D65 {
    #[inline]
    fn x() -> f32 {
        0.950_470
    }

    #[inline]
    fn y() -> f32 {
        1.0
    }

    #[inline]
    fn z() -> f32 {
        1.088_83
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d65() {
        assert_eq!(D65::x(), 0.950_470);
        assert_eq!(D65::y(), 1.0);
        assert_eq!(D65::z(), 1.088_83);
    }
}
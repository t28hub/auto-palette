use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use num_traits::Float;
use rand_distr::weighted_alias::AliasableWeight;

/// Trait for floating point numbers.
pub trait FloatNumber:
    Sized
    + Copy
    + Display
    + PartialOrd
    + Float
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + Sum
    + AliasableWeight
{
    /// Creates a new floating point number from a `u8`.
    ///
    /// # Arguments
    /// * `value` - The value to convert.
    ///
    /// # Returns
    /// The floating point number.
    #[must_use]
    fn from_u8(value: u8) -> Self;

    /// Creates a new floating point number from a `u32`.
    ///
    /// # Arguments
    /// * `value` - The value to convert.
    ///
    /// # Returns
    /// The floating point number.
    #[must_use]
    fn from_u32(value: u32) -> Self;

    /// Creates a new floating point number from a `usize`.
    ///
    /// # Arguments
    /// * `value` - The value to convert.
    ///
    /// # Returns
    /// The floating point number.
    #[must_use]
    fn from_usize(value: usize) -> Self;

    /// Creates a new floating point number from a `f32`.
    ///
    /// # Arguments
    /// * `value` - The value to convert.
    ///
    /// # Returns
    /// The floating point number.
    #[must_use]
    fn from_f32(value: f32) -> Self;

    /// Creates a new floating point number from a `f64`.
    ///
    /// # Arguments
    /// * `value` - The value to convert.
    ///
    /// # Returns
    /// The floating point number.
    #[must_use]
    fn from_f64(value: f64) -> Self;

    /// Converts the floating point number to a `u8`.
    ///
    /// # Returns
    /// The `u8` value. The value is truncated.
    #[must_use]
    fn to_u8_unsafe(&self) -> u8;

    /// Converts the floating point number to a `u32`.
    ///
    /// # Returns
    /// The `u32` value. The value is truncated.
    #[must_use]
    fn to_u32_unsafe(&self) -> u32;

    /// Converts the floating point number to a `usize`.
    ///
    /// # Returns
    /// The `usize` value. The value is truncated.
    #[must_use]
    fn to_usize_unsafe(&self) -> usize;
}

impl FloatNumber for f32 {
    #[inline]
    #[must_use]
    fn from_u8(value: u8) -> Self {
        value as f32
    }

    #[inline]
    #[must_use]
    fn from_u32(value: u32) -> Self {
        value as f32
    }

    #[inline]
    #[must_use]
    fn from_usize(value: usize) -> Self {
        value as f32
    }

    #[inline]
    #[must_use]
    fn from_f32(value: f32) -> Self {
        value
    }

    #[inline]
    #[must_use]
    fn from_f64(value: f64) -> Self {
        value as f32
    }

    #[inline]
    #[must_use]
    fn to_u8_unsafe(&self) -> u8 {
        *self as u8
    }

    #[inline]
    #[must_use]
    fn to_u32_unsafe(&self) -> u32 {
        *self as u32
    }

    #[inline]
    #[must_use]
    fn to_usize_unsafe(&self) -> usize {
        *self as usize
    }
}

impl FloatNumber for f64 {
    #[inline]
    #[must_use]
    fn from_u8(value: u8) -> Self {
        value as f64
    }

    #[inline]
    #[must_use]
    fn from_u32(value: u32) -> Self {
        value as f64
    }

    #[inline]
    #[must_use]
    fn from_usize(value: usize) -> Self {
        value as f64
    }

    #[inline]
    #[must_use]
    fn from_f32(value: f32) -> Self {
        value as f64
    }

    #[inline]
    #[must_use]
    fn from_f64(value: f64) -> Self {
        value
    }

    #[inline]
    #[must_use]
    fn to_u8_unsafe(&self) -> u8 {
        *self as u8
    }

    #[inline]
    #[must_use]
    fn to_u32_unsafe(&self) -> u32 {
        *self as u32
    }

    #[inline]
    #[must_use]
    fn to_usize_unsafe(&self) -> usize {
        *self as usize
    }
}

/// Normalizes a value to the range [min, max].
///
/// # Type Parameters
/// * `T` - The floating point type.
///
/// # Arguments
/// * `value` - The value to normalize.
/// * `min` - The minimum value of the range.
/// * `max` - The maximum value of the range.
///
/// # Returns
/// The normalized value.
///
/// # Panics
/// Panics if `min` is greater than or equal to `max`.
#[inline]
#[must_use]
pub fn normalize<T>(value: T, min: T, max: T) -> T
where
    T: FloatNumber,
{
    debug_assert!(min < max, "min must be less than max");
    (value - min) / (max - min)
}

/// Denormalizes a value from the range [min, max].
///
/// # Type Parameters
/// * `T` - The floating point type.
///
/// # Arguments
/// * `value` - The value to denormalize.
/// * `min` - The minimum value of the range.
/// * `max` - The maximum value of the range.
///
/// # Returns
/// The denormalized value.
///
/// # Panics
/// Panics if `min` is greater than or equal to `max`.
#[inline]
#[must_use]
pub fn denormalize<T>(value: T, min: T, max: T) -> T
where
    T: FloatNumber,
{
    debug_assert!(min < max, "min must be less than max");
    value * (max - min) + min
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_float_number_f32() {
        assert_eq!(f32::from_u8(128), 128.0);
        assert_eq!(f32::from_u32(1024), 1024.0);
        assert_eq!(f32::from_usize(4096), 4096.0);
        assert_eq!(f32::from_f32(0.5), 0.5);
        assert_eq!(f32::from_f64(0.125), 0.125);
        assert_eq!(0.125.to_u8_unsafe(), 0);
        assert_eq!(12.5.to_u32_unsafe(), 12);
        assert_eq!(10.24.to_usize_unsafe(), 10);
    }

    #[test]
    fn test_float_number_f64() {
        assert_eq!(f64::from_u8(128), 128.0);
        assert_eq!(f64::from_u32(1024), 1024.0);
        assert_eq!(f64::from_usize(4096), 4096.0);
        assert_eq!(f64::from_f32(0.5), 0.5);
        assert_eq!(f64::from_f64(0.125), 0.125);
        assert_eq!(0.125.to_u8_unsafe(), 0);
        assert_eq!(12.5.to_u32_unsafe(), 12);
        assert_eq!(10.24.to_usize_unsafe(), 10);
    }

    #[rstest]
    #[case(0.0, 0.0)]
    #[case(16.0, 0.125)]
    #[case(64.0, 0.5)]
    #[case(128.0, 1.0)]
    fn test_normalize(#[case] value: f32, #[case] expected: f32) {
        let actual = normalize(value, 0.0, 128.0);
        assert_eq!(actual, expected);
    }

    #[cfg(debug_assertions)]
    #[rstest]
    #[should_panic]
    #[case(128.0, 0.0)]
    #[should_panic]
    #[case(128.0, 128.0)]
    fn test_normalize_panic(#[case] min: f32, #[case] max: f32) {
        let _ = normalize(0.5, min, max);
    }

    #[rstest]
    #[case(0.0, 0.0)]
    #[case(0.125, 16.0)]
    #[case(0.5, 64.0)]
    #[case(1.0, 128.0)]
    fn test_denormalize(#[case] value: f32, #[case] expected: f32) {
        let actual = denormalize(value, 0.0, 128.0);
        assert_eq!(actual, expected);
    }

    #[cfg(debug_assertions)]
    #[rstest]
    #[should_panic]
    #[case(0.1, 0.0)]
    #[should_panic]
    #[case(0.0, 0.0)]
    fn test_denormalize_panic(#[case] min: f32, #[case] max: f32) {
        let _ = denormalize(0.5, min, max);
    }
}

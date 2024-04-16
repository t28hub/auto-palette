use std::fmt::Display;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use num_traits::Float;
use rand_distr::weighted_alias::AliasableWeight;

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
    #[must_use]
    fn from_u8(value: u8) -> Self;

    #[must_use]
    fn from_u32(value: u32) -> Self;

    #[must_use]
    fn from_usize(value: usize) -> Self;

    #[must_use]
    fn from_f32(value: f32) -> Self;

    #[must_use]
    fn from_f64(value: f64) -> Self;

    #[must_use]
    fn to_u8_unsafe(&self) -> u8;

    #[must_use]
    fn to_u32_unsafe(&self) -> u32;

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
    debug_assert!(min <= max, "min must be less than or equal to max");
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
#[inline]
#[must_use]
pub fn denormalize<T>(value: T, min: T, max: T) -> T
where
    T: FloatNumber,
{
    debug_assert!(min <= max, "min must be less than or equal to max");
    value * (max - min) + min
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0.0, 0.0)]
    #[case(16.0, 0.125)]
    #[case(64.0, 0.5)]
    #[case(128.0, 1.0)]
    fn test_normalize(#[case] value: f32, #[case] expected: f32) {
        let actual = normalize(value, 0.0, 128.0);
        assert_eq!(actual, expected);
    }

    #[test]
    #[should_panic]
    #[cfg(debug_assertions)]
    fn test_normalize_panic() {
        let _ = normalize(0.5, 128.0, 0.0);
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

    #[test]
    #[should_panic]
    #[cfg(debug_assertions)]
    fn test_denormalize_panic() {
        let _ = denormalize(0.5, 128.0, 0.0);
    }
}

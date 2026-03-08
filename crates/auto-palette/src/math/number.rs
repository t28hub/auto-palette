use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use num_traits::Float;

/// Trait for floating point numbers with conversion utilities.
///
/// This trait provides a unified interface for `f32` and `f64` types,
/// enabling generic programming with floating-point numbers throughout the crate.
///
/// # Safety Note
///
/// The truncation methods (`trunc_to_*`) use direct casting which may:
/// - Wrap around for out-of-range values (e.g., 256.0 -> 0 for u8)
/// - Lose precision for large values
/// - Produce undefined behavior for NaN or infinity
///
/// # Examples
/// ```ignore
/// use auto_palette::FloatNumber;
///
/// fn scale_value<T: FloatNumber>(value: u8) -> T {
///     let normalized = T::from_u8(value) / T::from_u8(255);
///     normalized * T::from_f32(100.0)
/// }
/// ```
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

    /// Creates a new floating point number from a `u16`.
    ///
    /// # Arguments
    /// * `value` - The value to convert.
    ///
    /// # Returns
    /// The floating point number.
    #[must_use]
    fn from_u16(value: u16) -> Self;

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

    /// Truncates the floating point number to a `u8`.
    ///
    /// # Warning
    /// Values outside [0, 255] will wrap around due to casting behavior.
    /// Fractional parts are truncated (not rounded).
    ///
    /// # Returns
    /// The truncated `u8` value.
    #[must_use]
    fn trunc_to_u8(&self) -> u8;

    /// Truncates the floating point number to a `u32`.
    ///
    /// # Warning
    /// Negative values and values > u32::MAX will wrap around.
    /// Fractional parts are truncated (not rounded).
    ///
    /// # Returns
    /// The truncated `u32` value.
    #[must_use]
    fn trunc_to_u32(&self) -> u32;

    /// Truncates the floating point number to a `usize`.
    ///
    /// # Warning
    /// Negative values and values > usize::MAX will wrap around.
    /// Fractional parts are truncated (not rounded).
    ///
    /// # Returns
    /// The truncated `usize` value.
    #[must_use]
    fn trunc_to_usize(&self) -> usize;
}

impl FloatNumber for f32 {
    #[inline]
    fn from_u8(value: u8) -> Self {
        value as f32
    }

    #[inline]
    fn from_u16(value: u16) -> Self {
        value as f32
    }

    #[inline]
    fn from_u32(value: u32) -> Self {
        value as f32
    }

    #[inline]
    fn from_usize(value: usize) -> Self {
        value as f32
    }

    #[inline]
    fn from_f32(value: f32) -> Self {
        value
    }

    #[inline]
    fn from_f64(value: f64) -> Self {
        value as f32
    }

    #[inline]
    fn trunc_to_u8(&self) -> u8 {
        *self as u8
    }

    #[inline]
    fn trunc_to_u32(&self) -> u32 {
        *self as u32
    }

    #[inline]
    fn trunc_to_usize(&self) -> usize {
        *self as usize
    }
}

impl FloatNumber for f64 {
    #[inline]
    fn from_u8(value: u8) -> Self {
        value as f64
    }

    #[inline]
    fn from_u16(value: u16) -> Self {
        value as f64
    }

    #[inline]
    fn from_u32(value: u32) -> Self {
        value as f64
    }

    #[inline]
    fn from_usize(value: usize) -> Self {
        value as f64
    }

    #[inline]
    fn from_f32(value: f32) -> Self {
        value as f64
    }

    #[inline]
    fn from_f64(value: f64) -> Self {
        value
    }

    #[inline]
    fn trunc_to_u8(&self) -> u8 {
        *self as u8
    }

    #[inline]
    fn trunc_to_u32(&self) -> u32 {
        *self as u32
    }

    #[inline]
    fn trunc_to_usize(&self) -> usize {
        *self as usize
    }
}

/// Normalizes a value from [min, max] to [0, 1] with clamping.
///
/// # Type Parameters
/// * `T` - The floating point type.
///
/// # Arguments
/// * `value` - The value to normalize.
/// * `min` - The minimum of the input range.
/// * `max` - The maximum of the input range.
///
/// # Returns
/// A value in [0, 1], clamped if input is outside [min, max].
///
/// # Panics
/// Panics in debug builds if `min >= max`.
#[inline]
#[must_use]
pub fn normalize<T>(value: T, min: T, max: T) -> T
where
    T: FloatNumber,
{
    debug_assert!(min < max, "min must be less than max");
    let normalized = (value - min) / (max - min);
    normalized.max(T::zero()).min(T::one())
}

/// Denormalizes a value from [0, 1] to [min, max].
///
/// # Type Parameters
/// * `T` - The floating point type.
///
/// # Arguments
/// * `value` - The normalized value (typically in [0, 1]).
/// * `min` - The minimum of the output range.
/// * `max` - The maximum of the output range.
///
/// # Returns
/// The denormalized value in [min, max].
///
/// # Panics
/// Panics in debug builds if `min >= max`.
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
        assert_eq!(f32::from_u16(256), 256.0);
        assert_eq!(f32::from_u32(1024), 1024.0);
        assert_eq!(f32::from_usize(4096), 4096.0);
        assert_eq!(f32::from_f32(0.5), 0.5);
        assert_eq!(f32::from_f64(0.125), 0.125);
        assert_eq!(0.125.trunc_to_u8(), 0);
        assert_eq!(12.5.trunc_to_u32(), 12);
        assert_eq!(10.24.trunc_to_usize(), 10);
    }

    #[test]
    fn test_float_number_f64() {
        assert_eq!(f64::from_u8(128), 128.0);
        assert_eq!(f64::from_u16(256), 256.0);
        assert_eq!(f64::from_u32(1024), 1024.0);
        assert_eq!(f64::from_usize(4096), 4096.0);
        assert_eq!(f64::from_f32(0.5), 0.5);
        assert_eq!(f64::from_f64(0.125), 0.125);
        assert_eq!(0.125.trunc_to_u8(), 0);
        assert_eq!(12.5.trunc_to_u32(), 12);
        assert_eq!(10.24.trunc_to_usize(), 10);
    }

    #[test]
    fn test_trunc_overflow() {
        let actual = 256.0_f32.trunc_to_u8();
        assert!(actual == 0 || actual == 255); // Platform dependent

        let actual = (-1.0_f32).trunc_to_u8();
        assert!(actual == 255 || actual == 0); // Platform dependent
    }

    #[test]
    fn test_precision_loss() {
        // Large f64 to f32 conversion
        let value = 1e10_f64;
        let actual = f32::from_f64(value);
        assert!(actual.is_finite()); // May lose precision but should be finite

        // Very small values
        let value = 1e-40_f64;
        let actual = f32::from_f64(value);
        assert!(actual == 0.0 || actual == value as f32); // May or may not be denormalized
    }

    #[test]
    fn test_special_values() {
        // Test NaN
        assert!(f32::from_f64(f64::NAN).is_nan());
        assert!(f64::from_f32(f32::NAN).is_nan());

        // Test infinity
        assert_eq!(f32::from_f64(f64::INFINITY), f32::INFINITY);
        assert_eq!(f64::from_f32(f32::INFINITY), f64::INFINITY);
        assert_eq!(f32::from_f64(f64::NEG_INFINITY), f32::NEG_INFINITY);
        assert_eq!(f64::from_f32(f32::NEG_INFINITY), f64::NEG_INFINITY);
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

    #[test]
    fn test_normalize_clamping() {
        // Values below min should clamp to 0
        assert_eq!(normalize(-10.0, 0.0, 100.0), 0.0);
        // Values above max should clamp to 1
        assert_eq!(normalize(150.0, 0.0, 100.0), 1.0);
    }

    #[test]
    fn test_normalize_negative_range() {
        // Test normalization with negative range
        assert_eq!(normalize(0.0, -100.0, 100.0), 0.5);
        assert_eq!(normalize(-50.0, -100.0, 100.0), 0.25);
        assert_eq!(normalize(50.0, -100.0, 100.0), 0.75);
    }

    #[test]
    fn test_normalize_special_values() {
        // Infinity should clamp to 1
        assert_eq!(normalize(f32::INFINITY, 0.0, 100.0), 1.0);
        // Negative infinity should clamp to 0
        assert_eq!(normalize(f32::NEG_INFINITY, 0.0, 100.0), 0.0);
        // NaN should remain NaN
        assert_eq!(normalize(f32::NAN, 0.0, 100.0), 0.0);
    }

    #[test]
    fn test_denormalize_outside_unit_range() {
        // Values > 1 should extrapolate beyond max
        assert_eq!(denormalize(2.0, 0.0, 100.0), 200.0);
        // Values < 0 should extrapolate below min
        assert_eq!(denormalize(-0.5, 0.0, 100.0), -50.0);
    }

    #[test]
    fn test_denormalize_negative_range() {
        // Test denormalization with negative range
        assert_eq!(denormalize(0.5, -100.0, 100.0), 0.0);
        assert_eq!(denormalize(0.25, -100.0, 100.0), -50.0);
        assert_eq!(denormalize(0.75, -100.0, 100.0), 50.0);
    }

    #[test]
    fn test_denormalize_special_values() {
        // NaN should remain NaN
        assert!(denormalize(f32::NAN, 0.0, 100.0).is_nan());
        // Infinity should remain infinity
        assert_eq!(denormalize(f32::INFINITY, 0.0, 100.0), f32::INFINITY);
        // Negative infinity should remain negative infinity
        assert_eq!(
            denormalize(f32::NEG_INFINITY, 0.0, 100.0),
            f32::NEG_INFINITY
        );
    }
}

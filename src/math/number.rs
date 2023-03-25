use num_traits::real::Real;
use num_traits::Num;
use std::fmt::Debug;
use std::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};

/// Trait representing an integer number.
pub trait Number:
    Copy
    + Clone
    + Debug
    + Num
    + PartialOrd
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
    + Clamp
{
    /// Creates a value from an u8 number.
    ///
    /// # Arguments
    /// * `n` - The `u8` number to be converted.
    ///
    /// # Returns
    /// A converted value.
    #[must_use]
    fn from_u8(n: u8) -> Self;

    /// Creates a value from an u32 number.
    ///
    /// # Arguments
    /// * `n` - The `u32` number to be converted.
    ///
    /// # Returns
    /// A converted value.
    #[must_use]
    fn from_u32(n: u32) -> Self;

    /// Creates a value from an u64 number.
    ///
    /// # Arguments
    /// * `n` - The `u64` number to be converted.
    ///
    /// # Returns
    /// A converted value.
    #[must_use]
    fn from_u64(n: u64) -> Self;

    /// Creates a value from an usize number.
    ///
    /// # Arguments
    /// * `n` - The `usize` number to be converted.
    ///
    /// # Returns
    /// A converted value.
    #[must_use]
    fn from_usize(n: usize) -> Self;
}

/// Trait representing a float number.
pub trait Float: Number + Real + Normalize {
    /// Creates a value from a f32 number.
    ///
    /// # Arguments
    /// * `n` - The `f32` number to be converted.
    ///
    /// # Returns
    /// A converted value.
    #[must_use]
    fn from_f32(n: f32) -> Self;

    /// Creates a value from a f64 number.
    ///
    /// # Arguments
    /// * `n` - The `f64` number to be converted.
    ///
    /// # Returns
    /// A converted value.
    #[must_use]
    fn from_f64(n: f64) -> Self;
}

/// Trait for clamp operation.
pub trait Clamp {
    /// Clamps value to be within the range [min, max].
    ///
    /// # Arguments
    /// * `min` - The min value in the range.
    /// * `max` - The max value in the range.
    ///
    /// # Returns
    /// The clamped value.
    #[must_use]
    fn clamp(self, min: Self, max: Self) -> Self;
}

/// Trait for normalize and denormalize operations.
pub trait Normalize {
    /// Normalizes this value.
    ///
    /// # Arguments
    /// * `min` - The min value in the range.
    /// * `max` - The max value in the range.
    ///
    /// # Returns
    /// The normalized value.
    fn normalize(self, min: Self, max: Self) -> Self;

    /// Denormalize a value from the range [0, 1] to the original range.
    ///
    /// # Arguments
    /// * `min` - The min value in the original range.
    /// * `max` - The max value in the original range.
    ///
    /// # Returns
    /// The denormalized value.
    fn denormalize(self, min: Self, max: Self) -> Self;
}

macro_rules! impl_number {
    ($number:ty) => {
        impl Number for $number {
            #[inline]
            fn from_u8(n: u8) -> Self {
                n as $number
            }

            #[inline]
            fn from_u32(n: u32) -> Self {
                n as $number
            }

            #[inline]
            fn from_u64(n: u64) -> Self {
                n as $number
            }

            #[inline]
            fn from_usize(n: usize) -> Self {
                n as $number
            }
        }
    };
}

macro_rules! impl_float {
    ($number:ty) => {
        impl Float for $number {
            #[inline]
            fn from_f32(n: f32) -> Self {
                n as $number
            }

            #[inline]
            fn from_f64(n: f64) -> Self {
                n as $number
            }
        }
    };
}

macro_rules! impl_clamp {
    ($number:ty) => {
        impl Clamp for $number {
            #[inline]
            fn clamp(self, min: Self, max: Self) -> Self {
                assert!(min <= max);
                if self < min {
                    min
                } else if self > max {
                    max
                } else {
                    self
                }
            }
        }
    };
}

macro_rules! impl_normalize {
    ($number:ty) => {
        impl Normalize for $number {
            #[inline]
            fn normalize(self, min: Self, max: Self) -> Self {
                assert!(min <= max);
                (self.clamp(min, max) - min) / (max - min)
            }

            #[inline]
            fn denormalize(self, min: Self, max: Self) -> Self {
                assert!(min <= max);
                (self * (max - min) + min).clamp(min, max)
            }
        }
    };
}

impl_number!(u8);
impl_number!(u32);
impl_number!(u64);
impl_number!(f32);
impl_number!(f64);

impl_float!(f32);
impl_float!(f64);

impl_clamp!(u8);
impl_clamp!(u32);
impl_clamp!(u64);
impl_clamp!(f32);
impl_clamp!(f64);

impl_normalize!(f32);
impl_normalize!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_should_return_clamped_value() {
        assert_eq!(-1.125_f64.clamp(-1.0, 1.0), -1.0);
        assert_eq!(-0.125_f64.clamp(-1.0, 1.0), -0.125);
        assert_eq!(0.125_f64.clamp(-1.0, 1.0), 0.125);
        assert_eq!(1.125_f64.clamp(-1.0, 1.0), 1.0);
    }

    #[test]
    fn normalize_should_return_normalized_value() {
        assert_eq!(0.0_f64.normalize(0.0, 128.0), 0.0);
        assert_eq!(32.0_f64.normalize(0.0, 128.0), 0.25);
        assert_eq!(64.0_f64.normalize(0.0, 128.0), 0.50);
        assert_eq!(128.0_f64.normalize(0.0, 128.0), 1.0);
    }

    #[test]
    fn denormalize_should_return_denormalized_value() {
        assert_eq!(0.0_f64.denormalize(0.0, 128.0), 0.0);
        assert_eq!(0.5_f64.denormalize(0.0, 128.0), 64.0);
        assert_eq!(1.0_f64.denormalize(0.0, 128.0), 128.0);
        assert_eq!(2.0_f64.denormalize(0.0, 128.0), 128.0);
    }
}

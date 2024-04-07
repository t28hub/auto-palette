/// Trait for normalizing and denormalizing numbers.
pub trait Normalizable: Sized {
    /// Normalizes the number to the range [min, max].
    ///
    /// # Parameters
    /// * `min` - The minimum value of the range.
    /// * `max` - The maximum value of the range.
    ///
    /// # Returns
    /// The normalized number.
    ///
    /// # Panics
    /// Panics if `min` is greater than or equal to `max`.
    #[must_use]
    fn normalize(&self, min: Self, max: Self) -> Self;

    /// Denormalizes the number from the range [min, max].
    ///
    /// # Parameters
    /// * `min` - The minimum value of the range.
    /// * `max` - The maximum value of the range.
    ///
    /// # Returns
    /// The denormalized number.
    ///
    /// # Panics
    /// Panics if `min` is greater than or equal to `max`.
    #[must_use]
    fn denormalize(&self, min: Self, max: Self) -> Self;
}

impl Normalizable for f32 {
    #[must_use]
    fn normalize(&self, min: Self, max: Self) -> Self {
        debug_assert!(min < max, "min must be less than max");
        (self - min) / (max - min)
    }

    #[must_use]
    fn denormalize(&self, min: Self, max: Self) -> Self {
        debug_assert!(min < max, "min must be less than max");
        (self * (max - min)) + min
    }
}

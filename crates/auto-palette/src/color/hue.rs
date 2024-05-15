use std::fmt::Display;

use crate::math::FloatNumber;

/// Represents a hue value.
///
/// # Type Parameters
/// * `T` - The floating point type.
///
/// # Examples
/// ```
/// use auto_palette::color::Hue;
///
/// let hue = Hue::from_degrees(485.0);
/// assert_eq!(hue.value(), 125.0);
/// assert_eq!(format!("{}", hue), "125.00");
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hue<T>(T)
where
    T: FloatNumber;

impl<T> Hue<T>
where
    T: FloatNumber,
{
    /// Returns the value of this hue.
    ///
    /// # Returns
    /// The value of this hue.
    #[inline]
    pub fn value(self) -> T {
        self.0
    }

    #[must_use]
    pub fn from_degrees(degrees: T) -> Self {
        Self(normalize(degrees))
    }
}

impl<T> Display for Hue<T>
where
    T: FloatNumber,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}", self.0)
    }
}

#[inline]
#[must_use]
fn normalize<T>(value: T) -> T
where
    T: FloatNumber,
{
    let max = T::from_f32(360.0);
    let value = value % max;
    if value.is_sign_negative() {
        value + max
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_degrees() {
        // Act
        let degree = Hue::from_degrees(720.0);

        // Assert
        assert_eq!(degree.value(), 0.0);
    }

    #[test]
    fn test_from_degrees_negative() {
        // Act
        let degree = Hue::from_degrees(-90.0);

        // Assert
        assert_eq!(degree.value(), 270.0);
    }

    #[test]
    fn test_from_degrees_overflow() {
        // Act
        let degree = Hue::from_degrees(360.0);

        // Assert
        assert_eq!(degree.value(), 0.0);
    }

    #[test]
    fn test_fmt() {
        // Act
        let degree = Hue::from_degrees(45.0);
        let actual = format!("{}", degree);

        // Assert
        assert_eq!(actual, "45.00");
    }
}

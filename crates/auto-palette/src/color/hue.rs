use std::fmt::Display;

#[cfg(feature = "wasm")]
use serde::{Serialize, Serializer};

use crate::math::FloatNumber;

/// The hue component of a color.
///
/// # Type Parameters
/// * `T` - The floating point type.
///
/// # Examples
/// ```
/// use std::f64::consts::PI;
///
/// use auto_palette::color::Hue;
///
/// let hue = Hue::from_degrees(240.0);
/// assert_eq!(hue.to_degrees(), 240.0);
/// assert_eq!(hue.to_radians(), 240.0 / 180.0 * PI);
/// assert_eq!(format!("{}", hue), "240.00");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hue<T>(T)
where
    T: FloatNumber;

impl<T> Hue<T>
where
    T: FloatNumber,
{
    /// Returns the value of the hue in degrees.
    ///
    /// # Returns
    /// The value of the hue in degrees.
    #[inline]
    pub fn to_degrees(&self) -> T {
        self.0
    }

    /// Returns the value of the hue in radians.
    ///
    /// # Returns
    /// The value of the hue in radians.
    #[inline]
    pub fn to_radians(&self) -> T {
        self.0.to_radians()
    }

    /// Creates a new `Hue` instance from the given degrees.
    ///
    /// # Arguments
    /// * `degrees` - The degrees of the hue.
    ///
    /// # Returns
    /// A new `Hue` instance.
    #[must_use]
    pub fn from_degrees(degrees: T) -> Self {
        Self(normalize(degrees))
    }

    /// Creates a new `Hue` instance from the given radians.
    ///
    /// # Arguments
    /// * `radians` - The radians of the hue.
    ///
    /// # Returns
    /// A new `Hue` instance.
    #[must_use]
    pub fn from_radians(radians: T) -> Self {
        Self(normalize(radians.to_degrees()))
    }
}

#[cfg(feature = "wasm")]
impl<T> Serialize for Hue<T>
where
    T: FloatNumber + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
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
    use std::f64::consts::PI;

    use rstest::rstest;
    use serde_test::{assert_ser_tokens, Token};

    use super::*;

    #[test]
    fn test_to_degrees() {
        // Act
        let hue = Hue::from_degrees(45.0);
        let actual = hue.to_degrees();

        // Assert
        assert_eq!(actual, 45.0);
    }

    #[test]
    fn test_to_radians() {
        // Act
        let hue = Hue::from_degrees(45.0);
        let actual = hue.to_radians();

        // Assert
        assert_eq!(actual, 45.0 / 180.0 * PI);
    }

    #[rstest]
    #[case(45.0, 45.0)]
    #[case(360.0, 0.0)]
    #[case(720.0, 0.0)]
    #[case(-90.0, 270.0)]
    fn test_from_degrees(#[case] degrees: f64, #[case] expected: f64) {
        // Act
        let actual = Hue::from_degrees(degrees);

        // Assert
        assert_eq!(actual.to_degrees(), expected);
    }

    #[rstest]
    #[case(0.0 * PI, 0.0)]
    #[case(PI, 180.0)]
    #[case(-PI, 180.0)]
    #[case(2.0 * PI, 0.0)]
    fn test_from_radians(#[case] radians: f64, #[case] expected: f64) {
        // Act
        let actual = Hue::from_radians(radians);

        // Assert
        assert_eq!(actual.to_degrees(), expected);
    }

    #[test]
    #[cfg(feature = "wasm")]
    fn test_serialize() {
        // Act
        let hue = Hue::from_degrees(45.0);

        // Assert
        assert_ser_tokens(&hue, &[Token::F64(45.0)]);
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

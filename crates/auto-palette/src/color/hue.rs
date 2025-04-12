use std::fmt::Display;

#[cfg(feature = "wasm")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
#[cfg(feature = "wasm")]
use tsify::Tsify;

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
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub struct Hue<T = f64>(#[cfg_attr(feature = "wasm", tsify(type = "number"))] T)
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

#[cfg(feature = "wasm")]
impl<'de, T> Deserialize<'de> for Hue<T>
where
    T: FloatNumber + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = T::deserialize(deserializer)?;
        Ok(Hue::from_degrees(value))
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

/// Normalizes the given value to the range [0, 360).
///
/// # Arguments
/// * `value` - The value to normalize.
///
/// # Returns
/// The normalized value in the range [0, 360).
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

    #[cfg(feature = "wasm")]
    use indoc::indoc;
    use rstest::rstest;
    #[cfg(feature = "wasm")]
    use serde_test::{assert_de_tokens, assert_ser_tokens, Token};

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
    #[cfg(feature = "wasm")]
    fn test_deserialize() {
        // Act
        let hue = Hue::from_degrees(60.0);

        // Assert
        assert_de_tokens(&hue, &[Token::F64(60.0)]);
    }

    #[test]
    #[cfg(feature = "wasm")]
    fn test_tsify() {
        // Assert
        let expected = indoc! {
            // language=typescript
            "export type Hue<T> = number;"
        };
        assert_eq!(Hue::<f64>::DECL, expected);
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

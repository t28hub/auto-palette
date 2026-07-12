use std::fmt::Display;

use num_traits::clamp;
#[cfg(feature = "wasm")]
use serde::{Deserialize, Serialize};

use crate::color::{Hue, Lab};

/// The LCH(ab) color representation.
///
/// See the following for more details:
/// [CIE LCHab color space - Wikipedia](https://en.wikipedia.org/wiki/CIELAB#Cylindrical_representation_(CIELCh))
///
/// # Fields
/// * `l` - The lightness component.
/// * `c` - The chroma component.
/// * `h` - The hue component.
///
/// # Examples
/// ```
/// use auto_palette::color::{Lab, Lchab};
///
/// let lchab = Lchab::new(54.617, 92.151, 27.756);
/// assert_eq!(format!("{}", lchab), "LCH(ab)(54.62, 92.15, 27.76)");
///
/// let lab: Lab = (&lchab).into();
/// assert_eq!(format!("{}", lab), "Lab(54.62, 81.55, 42.92)");
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "wasm", derive(Serialize, Deserialize))]
pub struct Lchab {
    pub l: f64,
    pub c: f64,
    pub h: Hue,
}

impl Lchab {
    /// Creates a new `Lchab` instance.
    ///
    /// # Arguments
    /// * `l` - The lightness component.
    /// * `c` - The chroma component.
    /// * `h` - The hue component.
    ///
    /// # Returns
    /// A new `Lchab` instance.
    #[must_use]
    pub fn new(l: f64, c: f64, h: f64) -> Self {
        Self {
            l: clamp(l, 0.0, 100.0),
            c: clamp(c, 0.0, 180.0),
            h: Hue::from_degrees(h),
        }
    }
}

impl Display for Lchab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LCH(ab)({:.2}, {:.2}, {:.2})",
            self.l,
            self.c,
            self.h.to_degrees()
        )
    }
}

impl From<&Lab> for Lchab {
    fn from(lab: &Lab) -> Self {
        // This implementation is based on the formulae from the following sources:
        // http://www.brucelindbloom.com/index.html?Eqn_Lab_to_LCH.html
        let l = lab.l;
        let c = (lab.a.powi(2) + lab.b.powi(2)).sqrt();
        let h = lab.b.atan2(lab.a).to_degrees();
        Self::new(l, c, h)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "wasm")]
    use serde_test::{assert_de_tokens, assert_ser_tokens, Token};

    use super::*;
    use crate::assert_approx_eq;

    #[test]
    fn test_new() {
        // Act
        let actual = Lchab::new(54.617, 92.151, 27.756);

        // Assert
        assert_eq!(
            actual,
            Lchab {
                l: 54.617,
                c: 92.151,
                h: Hue::from_degrees(27.756),
            }
        );
    }

    #[test]
    #[cfg(feature = "wasm")]
    fn test_serialize() {
        // Act
        let lchab = Lchab::new(54.617, 92.151, 27.756);

        // Assert
        assert_ser_tokens(
            &lchab,
            &[
                Token::Struct {
                    name: "Lchab",
                    len: 3,
                },
                Token::Str("l"),
                Token::F64(54.617),
                Token::Str("c"),
                Token::F64(92.151),
                Token::Str("h"),
                Token::F64(27.756),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    #[cfg(feature = "wasm")]
    fn test_deserialize() {
        // Act
        let lchab = Lchab::new(54.617, 92.151, 27.756);

        // Assert
        assert_de_tokens(
            &lchab,
            &[
                Token::Struct {
                    name: "Lchab",
                    len: 3,
                },
                Token::Str("l"),
                Token::F64(54.617),
                Token::Str("c"),
                Token::F64(92.151),
                Token::Str("h"),
                Token::F64(27.756),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_fmt() {
        // Act
        let lchab = Lchab::new(54.617, 92.151, 27.756);
        let actual = format!("{}", lchab);

        // Assert
        assert_eq!(actual, "LCH(ab)(54.62, 92.15, 27.76)");
    }

    #[test]
    fn test_from_lab() {
        // Arrange
        let lab = Lab::new(54.617, 81.549, 42.915);
        let actual = Lchab::from(&lab);

        // Assert
        assert_approx_eq!(actual.l, 54.617);
        assert_approx_eq!(actual.c, 92.151704);
        assert_approx_eq!(actual.h.to_degrees(), 27.755500);
    }
}

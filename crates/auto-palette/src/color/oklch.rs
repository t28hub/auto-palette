use std::fmt::Display;

use num_traits::clamp;
#[cfg(feature = "wasm")]
use serde::{Deserialize, Serialize};

use crate::color::{Hue, Oklab};

/// The Oklch color representation.
///
/// See the following for more details:
/// [The Oklab color space](https://bottosson.github.io/posts/oklab/#the-oklab-color-space)
///
/// # Fields
/// * `l` - The lightness component.
/// * `c` - The chroma component.
/// * `h` - The hue component.
///
/// # Examples
/// ```
/// use auto_palette::color::{Oklab, Oklch};
///
/// let oklch = Oklch::new(0.607, 0.121, 166.651);
/// assert_eq!(format!("{}", oklch), "Oklch(0.61, 0.12, 166.65)");
///
/// let oklab: Oklab = (&oklch).into();
/// assert_eq!(format!("{}", oklab), "Oklab(0.61, -0.12, 0.03)");
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "wasm", derive(Serialize, Deserialize))]
pub struct Oklch {
    pub l: f64,
    pub c: f64,
    pub h: Hue,
}

impl Oklch {
    /// Creates a new `Oklch` instance.
    ///
    /// # Arguments
    /// * `l` - The lightness component.
    /// * `c` - The chroma component.
    /// * `h` - The hue component.
    ///
    /// # Returns
    /// A new `Oklch` instance.
    #[must_use]
    pub fn new(l: f64, c: f64, h: f64) -> Self {
        Self {
            l: clamp(l, 0.0, 100.0),
            c: clamp(c, 0.0, 180.0),
            h: Hue::from_degrees(h),
        }
    }
}

impl Display for Oklch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Oklch({:.2}, {:.2}, {:.2})",
            self.l,
            self.c,
            self.h.to_degrees()
        )
    }
}

impl From<&Oklab> for Oklch {
    fn from(oklab: &Oklab) -> Self {
        // This implementation is based on the formulae from the following sources:
        // http://www.brucelindbloom.com/index.html?Eqn_Lab_to_LCH.html
        let l = oklab.l;
        let c = (oklab.a.powi(2) + oklab.b.powi(2)).sqrt();
        let h = oklab.b.atan2(oklab.a).to_degrees();
        Oklch::new(l, c, h)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "wasm")]
    use serde_test::{assert_de_tokens, assert_ser_tokens, Token};

    use super::*;
    use crate::{assert_approx_eq, color::Oklab};

    #[test]
    fn test_new() {
        // Act
        let actual = Oklch::new(0.607, 0.121, 166.651);

        // Assert
        assert_eq!(
            actual,
            Oklch {
                l: 0.607,
                c: 0.121,
                h: Hue::from_degrees(166.651)
            }
        );
    }

    #[test]
    #[cfg(feature = "wasm")]
    fn test_serialize() {
        // Act
        let oklch = Oklch::new(0.607, 0.121, 166.651);

        // Assert
        assert_ser_tokens(
            &oklch,
            &[
                Token::Struct {
                    name: "Oklch",
                    len: 3,
                },
                Token::Str("l"),
                Token::F64(0.607),
                Token::Str("c"),
                Token::F64(0.121),
                Token::Str("h"),
                Token::F64(166.651),
                Token::StructEnd,
            ],
        )
    }

    #[test]
    #[cfg(feature = "wasm")]
    fn test_deserialize() {
        // Act
        let oklch = Oklch::new(0.70, 0.10, 148.0);

        // Assert
        assert_de_tokens(
            &oklch,
            &[
                Token::Struct {
                    name: "Oklch",
                    len: 3,
                },
                Token::Str("l"),
                Token::F64(0.70),
                Token::Str("c"),
                Token::F64(0.10),
                Token::Str("h"),
                Token::F64(148.0),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_fmt() {
        // Act
        let oklch = Oklch::new(0.607, 0.121, 166.651);
        let actual = format!("{}", oklch);

        // Assert
        assert_eq!(actual, "Oklch(0.61, 0.12, 166.65)");
    }

    #[test]
    fn test_from_oklab() {
        // Act
        let oklab = Oklab::new(0.607, -0.118, 0.028);
        let actual = Oklch::from(&oklab);

        // Assert
        assert_approx_eq!(actual.l, 0.607);
        assert_approx_eq!(actual.c, 0.121_276_5);
        assert_approx_eq!(actual.h.to_degrees(), 166.651273);
    }
}

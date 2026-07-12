use std::fmt::Display;

use num_traits::clamp;
#[cfg(feature = "wasm")]
use serde::{Deserialize, Serialize};

use crate::color::{Hue, Luv};

/// The LCH(uv) color representation.
///
/// See the following for more details:
/// [CIE LCHuv color space - Wikipedia](https://en.wikipedia.org/wiki/CIELUV#Cylindrical_representation_(CIELCh))
///
/// # Fields
/// * `l` - The lightness component.
/// * `c` - The chroma component.
/// * `h` - The hue component.
///
/// # Examples
/// ```
/// use auto_palette::color::{Lchuv, Luv};
///
/// let lchuv = Lchuv::new(56.232, 50.875, 154.710);
/// assert_eq!(format!("{}", lchuv), "LCH(uv)(56.23, 50.88, 154.71)");
///
/// let luv: Luv = (&lchuv).into();
/// assert_eq!(format!("{}", luv), "Luv(56.23, -46.00, 21.73)");
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "wasm", derive(Serialize, Deserialize))]
pub struct Lchuv {
    pub l: f64,
    pub c: f64,
    pub h: Hue,
}

impl Lchuv {
    /// Creates a new `Lchuv` instance.
    ///
    /// # Arguments
    /// * `l` - The lightness component.
    /// * `c` - The chroma component.
    /// * `h` - The hue component.
    ///
    /// # Returns
    /// A new `Lchuv` instance.
    #[must_use]
    pub fn new(l: f64, c: f64, h: f64) -> Self {
        Self {
            l: clamp(l, 0.0, 100.0),
            c: clamp(c, 0.0, 180.0),
            h: Hue::from_degrees(h),
        }
    }
}

impl Display for Lchuv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LCH(uv)({:.2}, {:.2}, {:.2})",
            self.l,
            self.c,
            self.h.to_degrees()
        )
    }
}

impl From<&Luv> for Lchuv {
    fn from(luv: &Luv) -> Self {
        // This implementation is based on the formulae from the following sources:
        // http://www.brucelindbloom.com/index.html?Eqn_Luv_to_LCH.html
        let l = luv.l;
        let c = (luv.u * luv.u + luv.v * luv.v).sqrt();
        let h = luv.v.atan2(luv.u).to_degrees();
        Self::new(l, c, h)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "wasm")]
    use serde_test::{assert_de_tokens, assert_ser_tokens, Token};

    use super::*;
    use crate::{assert_approx_eq, color::Luv};

    #[test]
    fn test_new() {
        // Act
        let actual = Lchuv::new(56.232, 50.875, 154.710);

        // Assert
        assert_eq!(
            actual,
            Lchuv {
                l: 56.232,
                c: 50.875,
                h: Hue::from_degrees(154.710),
            }
        );
    }

    #[test]
    #[cfg(feature = "wasm")]
    fn test_serialize() {
        // Act
        let lchuv = Lchuv::new(56.232, 50.875, 154.710);

        // Assert
        assert_ser_tokens(
            &lchuv,
            &[
                Token::Struct {
                    name: "Lchuv",
                    len: 3,
                },
                Token::Str("l"),
                Token::F64(56.232),
                Token::Str("c"),
                Token::F64(50.875),
                Token::Str("h"),
                Token::F64(154.710),
                Token::StructEnd,
            ],
        )
    }

    #[test]
    #[cfg(feature = "wasm")]
    fn test_deserialize() {
        // Act
        let lchuv = Lchuv::new(56.232, 50.875, 154.710);

        // Assert
        assert_de_tokens(
            &lchuv,
            &[
                Token::Struct {
                    name: "Lchuv",
                    len: 3,
                },
                Token::Str("l"),
                Token::F64(56.232),
                Token::Str("c"),
                Token::F64(50.875),
                Token::Str("h"),
                Token::F64(154.710),
                Token::StructEnd,
            ],
        )
    }

    #[test]
    fn test_fmt() {
        // Act
        let lchuv = Lchuv::new(56.232, 50.875, 154.710);
        let actual = format!("{}", lchuv);

        // Assert
        assert_eq!(actual, "LCH(uv)(56.23, 50.88, 154.71)");
    }

    #[test]
    fn test_from_luv() {
        // Act
        let luv = Luv::new(56.232, -45.999, 21.734);
        let actual = Lchuv::from(&luv);

        // Assert
        assert_approx_eq!(actual.l, 56.232);
        assert_approx_eq!(actual.c, 50.875090);
        assert_approx_eq!(actual.h.to_degrees(), 154.709803);
    }
}

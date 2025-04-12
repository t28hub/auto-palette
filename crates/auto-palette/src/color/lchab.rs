use std::{fmt::Display, marker::PhantomData};

use num_traits::clamp;
#[cfg(feature = "wasm")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use tsify::Tsify;

use crate::{
    color::{Hue, Lab, WhitePoint, D65},
    math::FloatNumber,
};

/// The LCHab color representation.
///
/// See the following for more details:
/// [CIE LCHab color space - Wikipedia](https://en.wikipedia.org/wiki/CIELAB#Cylindrical_representation_(CIELCh))
///
/// # Type Parameters
/// * `T` - The floating point type.
/// * `W` - The white point type.
///
/// # Fields
/// * `l` - The lightness component.
/// * `c` - The chroma component.
/// * `h` - The hue component.
///
/// # Examples
/// ```
/// use auto_palette::color::{LCHab, Lab, D65};
///
/// let lchab: LCHab<_> = LCHab::new(54.617, 92.151, 27.756);
/// assert_eq!(format!("{}", lchab), "LCH(ab)(54.62, 92.15, 27.76)");
///
/// let lab: Lab<_> = (&lchab).into();
/// assert_eq!(format!("{}", lab), "Lab(54.62, 81.55, 42.92)");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(Serialize, Deserialize, Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub struct LCHab<T = f64, W = D65>
where
    T: FloatNumber,
    W: WhitePoint,
{
    #[cfg_attr(feature = "wasm", tsify(type = "number"))]
    pub l: T,
    #[cfg_attr(feature = "wasm", tsify(type = "number"))]
    pub c: T,
    #[cfg_attr(feature = "wasm", tsify(type = "number"))]
    pub h: Hue<T>,
    #[cfg_attr(feature = "wasm", serde(skip))]
    _marker: PhantomData<W>,
}

impl<T, W> LCHab<T, W>
where
    T: FloatNumber,
    W: WhitePoint,
{
    /// Creates a new `LCHab` instance.
    ///
    /// # Arguments
    /// * `l` - The lightness component.
    /// * `c` - The chroma component.
    /// * `h` - The hue component.
    ///
    /// # Returns
    /// A new `LCHab` instance.
    #[must_use]
    pub fn new(l: T, c: T, h: T) -> Self {
        Self {
            l: clamp(l, T::zero(), T::from_u32(100)),
            c: clamp(c, T::zero(), T::from_u32(180)),
            h: Hue::from_degrees(h),
            _marker: PhantomData,
        }
    }
}

impl<T, W> Display for LCHab<T, W>
where
    T: FloatNumber,
    W: WhitePoint,
{
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

impl<T, W> From<&Lab<T, W>> for LCHab<T, W>
where
    T: FloatNumber,
    W: WhitePoint,
{
    fn from(lab: &Lab<T, W>) -> Self {
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
    use indoc::indoc;
    #[cfg(feature = "wasm")]
    use serde_test::{assert_de_tokens, assert_ser_tokens, Token};

    use super::*;
    use crate::assert_approx_eq;

    #[test]
    fn test_new() {
        // Act
        let actual = LCHab::<_>::new(54.617, 92.151, 27.756);

        // Assert
        assert_eq!(
            actual,
            LCHab {
                l: 54.617,
                c: 92.151,
                h: Hue::from_degrees(27.756),
                _marker: PhantomData,
            }
        );
    }

    #[test]
    #[cfg(feature = "wasm")]
    fn test_serialize() {
        // Act
        let lchab = LCHab::<f64>::new(54.617, 92.151, 27.756);

        // Assert
        assert_ser_tokens(
            &lchab,
            &[
                Token::Struct {
                    name: "LCHab",
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
        let lchab = LCHab::<f64>::new(54.617, 92.151, 27.756);

        // Assert
        assert_de_tokens(
            &lchab,
            &[
                Token::Struct {
                    name: "LCHab",
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
    fn test_tsify() {
        // Act & Assert
        let expected = indoc! {
            // language=typescript
            "export interface LCHab<T> {
                l: number;
                c: number;
                h: number;
            }"
        };
        assert_eq!(LCHab::<f64>::DECL, expected);
    }

    #[test]
    fn test_fmt() {
        // Act
        let lchab = LCHab::<f32>::new(54.617, 92.151, 27.756);
        let actual = format!("{}", lchab);

        // Assert
        assert_eq!(actual, "LCH(ab)(54.62, 92.15, 27.76)");
    }

    #[test]
    fn test_from_lab() {
        // Arrange
        let lab: Lab<f32> = Lab::new(54.617, 81.549, 42.915);
        let actual = LCHab::from(&lab);

        // Assert
        assert_approx_eq!(actual.l, 54.617);
        assert_approx_eq!(actual.c, 92.151710);
        assert_approx_eq!(actual.h.to_degrees(), 27.755500);
    }
}

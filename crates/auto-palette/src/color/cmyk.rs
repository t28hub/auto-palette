use std::fmt::{Display, Formatter};

use num_traits::clamp;
#[cfg(feature = "wasm")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use tsify::Tsify;

use crate::{color::RGB, FloatNumber};

/// The CMYK color representation.
///
/// See the following for more details:
/// [CMYK color model - Wikipedia](https://en.wikipedia.org/wiki/CMYK_color_model)
///
/// # Type Parameters
/// * `T` - The floating point type.
///
/// # Fields
/// * `c` - The cyan component.
/// * `m` - The magenta component.
/// * `y` - The yellow component.
/// * `k` - The key (black) component.
///
/// # Examples
/// ```
/// use auto_palette::color::{CMYK, RGB};
///
/// let rgb = RGB::new(255, 255, 0);
/// let cmyk = CMYK::<f32>::from(&rgb);
/// assert_eq!(format!("{}", cmyk), "CMYK(0.00, 0.00, 1.00, 0.00)");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(Serialize, Deserialize, Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub struct CMYK<T = f64>
where
    T: FloatNumber,
{
    #[cfg_attr(feature = "wasm", tsify(type = "number"))]
    pub c: T,
    #[cfg_attr(feature = "wasm", tsify(type = "number"))]
    pub m: T,
    #[cfg_attr(feature = "wasm", tsify(type = "number"))]
    pub y: T,
    #[cfg_attr(feature = "wasm", tsify(type = "number"))]
    pub k: T,
}

impl<T> CMYK<T>
where
    T: FloatNumber,
{
    /// Creates a new `CMYK` instance.
    ///
    /// # Arguments
    /// * `c` - The cyan component.
    /// * `m` - The magenta component.
    /// * `y` - The yellow component.
    /// * `k` - The key (black) component.
    ///
    /// # Returns
    /// A new `CMYK` instance.
    #[must_use]
    pub fn new(c: T, m: T, y: T, k: T) -> Self {
        Self {
            c: clamp(c, T::zero(), T::one()),
            m: clamp(m, T::zero(), T::one()),
            y: clamp(y, T::zero(), T::one()),
            k: clamp(k, T::zero(), T::one()),
        }
    }
}

impl<T> Display for CMYK<T>
where
    T: FloatNumber,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CMYK({:.2}, {:.2}, {:.2}, {:.2})",
            self.c, self.m, self.y, self.k
        )
    }
}

impl<T> From<&RGB> for CMYK<T>
where
    T: FloatNumber,
{
    fn from(rgb: &RGB) -> Self {
        let max = RGB::max_value::<T>();
        let r = T::from_u8(rgb.r) / max;
        let g = T::from_u8(rgb.g) / max;
        let b = T::from_u8(rgb.b) / max;

        let k = T::one() - r.max(g).max(b);
        if k.is_one() {
            CMYK::new(T::zero(), T::zero(), T::zero(), k)
        } else {
            let denominator = T::one() - k;
            let c = (T::one() - r - k) / denominator;
            let m = (T::one() - g - k) / denominator;
            let y = (T::one() - b - k) / denominator;
            CMYK::new(c, m, y, k)
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "wasm")]
    use indoc::indoc;
    use rstest::rstest;
    #[cfg(feature = "wasm")]
    use serde_test::{assert_de_tokens, assert_ser_tokens, Token};

    use super::*;

    #[test]
    fn test_new() {
        // Act
        let actual = CMYK::new(1.00, 0.75, 0.50, 0.25);

        // Assert
        assert_eq!(
            actual,
            CMYK {
                c: 1.00,
                m: 0.75,
                y: 0.50,
                k: 0.25,
            }
        );
    }

    #[rstest]
    #[case((-0.01, -0.02, -0.03, -0.04), (0.00, 0.00, 0.00, 0.00))]
    #[case((1.01, 1.02, 1.03, 1.04), (1.00, 1.00, 1.00, 1.00))]
    fn test_new_clamp(#[case] input: (f32, f32, f32, f32), #[case] expected: (f32, f32, f32, f32)) {
        // Act
        let (c, m, y, k) = input;
        let actual = CMYK::new(c, m, y, k);

        // Assert
        assert_eq!(
            actual,
            CMYK {
                c: expected.0,
                m: expected.1,
                y: expected.2,
                k: expected.3,
            }
        );
    }

    #[test]
    #[cfg(feature = "wasm")]
    fn test_serialize() {
        // Act
        let cmyk = CMYK::new(1.00, 0.75, 0.50, 0.25);

        // Assert
        assert_ser_tokens(
            &cmyk,
            &[
                Token::Struct {
                    name: "CMYK",
                    len: 4,
                },
                Token::Str("c"),
                Token::F64(1.00),
                Token::Str("m"),
                Token::F64(0.75),
                Token::Str("y"),
                Token::F64(0.50),
                Token::Str("k"),
                Token::F64(0.25),
                Token::StructEnd,
            ],
        )
    }

    #[test]
    #[cfg(feature = "wasm")]
    fn test_deserialize() {
        // Act
        let cmyk = CMYK::new(0.50, 0.25, 1.00, 0.50);

        // Assert
        assert_de_tokens(
            &cmyk,
            &[
                Token::Struct {
                    name: "CMYK",
                    len: 4,
                },
                Token::Str("c"),
                Token::F64(0.50),
                Token::Str("m"),
                Token::F64(0.25),
                Token::Str("y"),
                Token::F64(1.00),
                Token::Str("k"),
                Token::F64(0.50),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    #[cfg(feature = "wasm")]
    fn test_tsify() {
        // Assert
        let expected = indoc! {
            // language=typescript
            "export interface CMYK<T> {
                c: number;
                m: number;
                y: number;
                k: number;
            }"
        };
        assert_eq!(CMYK::<f64>::DECL, expected);
    }

    #[test]
    fn test_fmt() {
        // Act
        let cmyk = CMYK::new(0.00, 0.00, 1.00, 0.00);
        let actual = format!("{}", cmyk);

        // Assert
        assert_eq!("CMYK(0.00, 0.00, 1.00, 0.00)", actual);
    }

    #[rstest]
    #[case::black(RGB::new(0, 0, 0), CMYK::new(0.00, 0.00, 0.00, 1.00))]
    #[case::white(RGB::new(255, 255, 255), CMYK::new(0.00, 0.00, 0.00, 0.00))]
    #[case::red(RGB::new(255, 0, 0), CMYK::new(0.00, 1.00, 1.00, 0.00))]
    #[case::green(RGB::new(0, 255, 0), CMYK::new(1.00, 0.00, 1.00, 0.00))]
    #[case::blue(RGB::new(0, 0, 255), CMYK::new(1.00, 1.00, 0.00, 0.00))]
    #[case::yellow(RGB::new(255, 255, 0), CMYK::new(0.00, 0.00, 1.00, 0.00))]
    #[case::cyan(RGB::new(0, 255, 255), CMYK::new(1.00, 0.00, 0.00, 0.00))]
    #[case::magenta(RGB::new(255, 0, 255), CMYK::new(0.00, 1.00, 0.00, 0.00))]
    fn test_from_rgb(#[case] rgb: RGB, #[case] expected: CMYK<f32>) {
        // Act
        let actual = CMYK::from(&rgb);

        // Assert
        assert_eq!(actual, expected);
    }
}

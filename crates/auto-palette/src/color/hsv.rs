use std::fmt::Display;

use num_traits::clamp;
#[cfg(feature = "wasm")]
use serde::Serialize;

use crate::{
    color::{hue::Hue, RGB},
    math::FloatNumber,
};

/// The HSV color representation.
///
/// See the following for more details:
/// [HSL and HSV - Wikipedia](https://en.wikipedia.org/wiki/HSL_and_HSV)
///
/// # Type Parameters
/// * `T` - The floating point type.
///
/// # Fields
/// * `h` - The hue component.
/// * `s` - The saturation component.
/// * `v` - The value component.
///
/// # Examples
/// ```
/// use auto_palette::color::{HSV, RGB};
///
/// let rgb = RGB::new(255, 255, 0);
/// let hsv = HSV::<f32>::from(&rgb);
/// assert_eq!(format!("{}", hsv), "HSV(60.00, 1.00, 1.00)");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(Serialize))]
pub struct HSV<T>
where
    T: FloatNumber,
{
    pub h: Hue<T>,
    pub s: T,
    pub v: T,
}

impl<T> HSV<T>
where
    T: FloatNumber,
{
    /// Creates a new `HSV` instance.
    ///
    /// # Arguments
    /// * `h` - The hue component.
    /// * `s` - The saturation component.
    /// * `v` - The value component.
    ///
    /// # Returns
    /// A new `HSV` instance.
    #[must_use]
    pub fn new(h: T, s: T, v: T) -> Self {
        Self {
            h: Hue::from_degrees(h),
            s: clamp(s, T::zero(), T::one()),
            v: clamp(v, T::zero(), T::one()),
        }
    }
}

impl<T> Display for HSV<T>
where
    T: FloatNumber,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HSV({:.2}, {:.2}, {:.2})", self.h, self.s, self.v)
    }
}

impl<T> From<&RGB> for HSV<T>
where
    T: FloatNumber,
{
    fn from(rgb: &RGB) -> Self {
        let max = RGB::max_value::<T>();
        let r = T::from_u8(rgb.r) / max;
        let g = T::from_u8(rgb.g) / max;
        let b = T::from_u8(rgb.b) / max;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);

        let delta = max - min;

        let h = if delta == T::zero() {
            T::zero()
        } else if max == r {
            T::from_f32(60.0) * (((g - b) / delta) % T::from_f32(6.0))
        } else if max == g {
            T::from_f32(60.0) * (((b - r) / delta) + T::from_f32(2.0))
        } else {
            T::from_f32(60.0) * (((r - g) / delta) + T::from_f32(4.0))
        };

        let s = if max.is_zero() {
            T::zero()
        } else {
            delta / max
        };

        let v = max;

        Self::new(h, s, v)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    #[cfg(feature = "wasm")]
    use serde_test::{assert_ser_tokens, Token};

    use super::*;

    #[test]
    fn test_new() {
        // Act
        let actual = HSV::new(60.0, 1.0, 1.0);

        // Assert
        assert_eq!(
            actual,
            HSV {
                h: Hue::from_degrees(60.0),
                s: 1.0,
                v: 1.0
            }
        )
    }

    #[rstest]
    #[case((400.0, 2.0, 2.0), (40.0, 1.0, 1.0))]
    #[case((-40.0, -1.0, -1.0), (320.0, 0.0, 0.0))]
    fn test_new_with_out_of_range_values(
        #[case] input: (f32, f32, f32),
        #[case] expected: (f32, f32, f32),
    ) {
        // Act
        let (h, s, v) = input;
        let actual = HSV::new(h, s, v);

        // Assert
        let (h, s, v) = expected;
        assert_eq!(actual, HSV::new(h, s, v));
    }

    #[test]
    #[cfg(feature = "wasm")]
    fn test_serialize() {
        // Act
        let hsv = HSV::new(60.0, 0.75, 0.5);

        // Assert
        assert_ser_tokens(
            &hsv,
            &[
                Token::Struct {
                    name: "HSV",
                    len: 3,
                },
                Token::Str("h"),
                Token::F64(60.0),
                Token::Str("s"),
                Token::F64(0.75),
                Token::Str("v"),
                Token::F64(0.5),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_fmt() {
        // Act
        let hsv = HSV::new(60.0, 1.0, 1.0);
        let actual = format!("{}", hsv);

        // Assert
        assert_eq!(actual, "HSV(60.00, 1.00, 1.00)");
    }

    #[rstest]
    #[case::black((0, 0, 0), (0.0, 0.0, 0.0))]
    #[case::white((255, 255, 255), (0.0, 0.0, 1.0))]
    #[case::red((255, 0, 0), (0.0, 1.0, 1.0))]
    #[case::green((0, 255, 0), (120.0, 1.0, 1.0))]
    #[case::blue((0, 0, 255), (240.0, 1.0, 1.0))]
    #[case::yellow((255, 255, 0), (60.0, 1.0, 1.0))]
    #[case::cyan((0, 255, 255), (180.0, 1.0, 1.0))]
    #[case::magenta((255, 0, 255), (300.0, 1.0, 1.0))]
    fn test_from_rgb(#[case] rgb: (u8, u8, u8), #[case] hsv: (f32, f32, f32)) {
        // Act
        let rgb = RGB::new(rgb.0, rgb.1, rgb.2);
        let actual = HSV::<f32>::from(&rgb);

        // Assert
        assert_eq!(actual.h.to_degrees(), hsv.0);
        assert_eq!(actual.s, hsv.1);
        assert_eq!(actual.v, hsv.2);
    }
}

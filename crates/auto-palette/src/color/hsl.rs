use std::fmt::Display;

use num_traits::clamp;

use crate::{
    color::{hue::Hue, rgb::RGB},
    math::FloatNumber,
};

/// The HSL color representation.
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
/// * `l` - The lightness component.
///
/// # Examples
/// ```
/// use auto_palette::color::{HSL, RGB};
///
/// let rgb = RGB::new(255, 255, 0);
/// let hsl = HSL::<f32>::from(&rgb);
/// assert_eq!(format!("{}", hsl), "HSL(60.00, 1.00, 0.50)");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct HSL<T>
where
    T: FloatNumber,
{
    pub h: Hue<T>,
    pub s: T,
    pub l: T,
}

impl<T> HSL<T>
where
    T: FloatNumber,
{
    /// Creates a new `HSL` instance.
    ///
    /// # Arguments
    /// * `h` - The hue component.
    /// * `s` - The saturation component.
    /// * `l` - The lightness component.
    ///
    /// # Returns
    /// A new `HSL` instance.
    #[must_use]
    pub fn new(h: T, s: T, l: T) -> Self {
        Self {
            h: Hue::from_degrees(h),
            s: clamp(s, T::zero(), T::one()),
            l: clamp(l, T::zero(), T::one()),
        }
    }
}

impl<T> Display for HSL<T>
where
    T: FloatNumber,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HSL({:.2}, {:.2}, {:.2})", self.h, self.s, self.l)
    }
}

impl<T> From<&RGB> for HSL<T>
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
        let h = if delta.is_zero() {
            T::zero()
        } else if max == r {
            T::from_f32(60.0) * (((g - b) / delta) % T::from_f32(6.0))
        } else if max == g {
            T::from_f32(60.0) * (((b - r) / delta) + T::from_f32(2.0))
        } else {
            T::from_f32(60.0) * (((r - g) / delta) + T::from_f32(4.0))
        };

        let l = (max + min) / T::from_f32(2.0);

        let s = if delta.is_zero() {
            T::zero()
        } else {
            delta / (T::one() - (T::from_f32(2.0) * l - T::one()).abs())
        };
        HSL::new(h, s, l)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_new() {
        // Act
        let actual = HSL::new(60.0, 1.0, 0.5);

        // Assert
        assert_eq!(
            actual,
            HSL {
                h: Hue::from_degrees(60.0),
                s: 1.0,
                l: 0.5,
            }
        );
    }

    #[rstest]
    #[case((-400.0, -1.0, -1.0), (320.0, 0.0, 0.0))]
    #[case((400.0, 2.0, 2.0), (40.0, 1.0, 1.0))]
    fn test_new_with_out_of_range(
        #[case] input: (f32, f32, f32),
        #[case] expected: (f32, f32, f32),
    ) {
        // Act
        let (h, s, l) = input;
        let actual = HSL::new(h, s, l);

        // Assert
        let (h, s, l) = expected;
        assert_eq!(actual, HSL::new(h, s, l));
    }

    #[test]
    fn test_fmt() {
        // Act
        let hsl = HSL::new(60.0, 1.0, 0.5);
        let actual = format!("{}", hsl);

        // Assert
        assert_eq!("HSL(60.00, 1.00, 0.50)", actual);
    }

    #[rstest]
    #[case::black((0, 0, 0), (0.0, 0.0, 0.0))]
    #[case::white((255, 255, 255), (0.0, 0.0, 1.0))]
    #[case::red((255, 0, 0), (0.0, 1.0, 0.5))]
    #[case::green((0, 255, 0), (120.0, 1.0, 0.5))]
    #[case::blue((0, 0, 255), (240.0, 1.0, 0.5))]
    #[case::yellow((255, 255, 0), (60.0, 1.0, 0.5))]
    #[case::cyan((0, 255, 255), (180.0, 1.0, 0.5))]
    #[case::magenta((255, 0, 255), (300.0, 1.0, 0.5))]
    fn test_from_rgb(#[case] rgb: (u8, u8, u8), #[case] expected: (f32, f32, f32)) {
        // Act
        let rgb = RGB::new(rgb.0, rgb.1, rgb.2);
        let actual = HSL::<f32>::from(&rgb);

        // Assert
        assert!((actual.h.to_degrees() - expected.0).abs() < 1e-2);
        assert!((actual.s - expected.1).abs() < 1e-2);
        assert!((actual.l - expected.2).abs() < 1e-2);
    }
}

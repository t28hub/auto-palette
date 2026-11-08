use std::sync::LazyLock;

use super::{AlphaColor, Srgb8};

/// A color in linear sRGB.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct LinearRgb {
    /// The red value, from 0.0 to 1.0.
    pub(crate) r: f64,
    /// The green value, from 0.0 to 1.0.
    pub(crate) g: f64,
    /// The blue value, from 0.0 to 1.0.
    pub(crate) b: f64,
}

impl LinearRgb {
    /// Creates a color in linear sRGB from its red, green, and blue values.
    ///
    /// # Arguments
    /// * `r` - The red value, from 0.0 to 1.0.
    /// * `g` - The green value, from 0.0 to 1.0.
    /// * `b` - The blue value, from 0.0 to 1.0.
    ///
    /// # Returns
    /// A linear sRGB color with the given `r`, `g`, and `b` values.
    #[inline]
    pub(crate) const fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
}

/// Decodes a gamma-encoded 8-bit sRGB color.
impl From<Srgb8> for LinearRgb {
    #[inline]
    fn from(Srgb8 { r, g, b }: Srgb8) -> Self {
        /// The linear values of the 256 gamma-encoded sRGB channel values.
        static SRGB8_TO_LINEAR: LazyLock<[f64; 256]> = LazyLock::new(|| {
            std::array::from_fn(|value| {
                let encoded = value as f64 / 255.0;
                if encoded <= 0.04045 {
                    encoded / 12.92
                } else {
                    ((encoded + 0.055) / 1.055).powf(2.4)
                }
            })
        });

        let table: &[f64; 256] = &SRGB8_TO_LINEAR;
        Self::new(
            table[usize::from(r)],
            table[usize::from(g)],
            table[usize::from(b)],
        )
    }
}

/// Decodes the color to linear sRGB and scales the alpha to 0.0 to 1.0.
impl From<AlphaColor<Srgb8, u8>> for AlphaColor<LinearRgb> {
    #[inline]
    fn from(color: AlphaColor<Srgb8, u8>) -> Self {
        AlphaColor::<Srgb8>::from(color).map_color(LinearRgb::from)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::assert_approx_eq;

    #[rstest]
    #[case::black(0, 0.0)]
    #[case::linear_segment(10, 10.0 / 255.0 / 12.92)]
    #[case::middle(128, 0.215_860_500_2)]
    #[case::white(255, 1.0)]
    fn test_from_srgb8(#[case] value: u8, #[case] expected: f64) {
        // Act
        let actual = LinearRgb::from(Srgb8::new(value, value, value));

        // Assert
        assert_approx_eq!(actual.r, expected);
        assert_approx_eq!(actual.g, expected);
        assert_approx_eq!(actual.b, expected);
    }

    #[rstest]
    #[case::black(0, 0.0)]
    #[case::middle(128, 0.215_860_500_2)]
    #[case::white(255, 1.0)]
    fn test_from_srgb8_with_alpha(#[case] value: u8, #[case] expected: f64) {
        // Arrange
        let color = AlphaColor::new_u8(Srgb8::new(value, value, value), 51);

        // Act
        let actual = AlphaColor::<LinearRgb>::from(color).color();

        // Assert
        assert_approx_eq!(actual.r, expected);
        assert_approx_eq!(actual.g, expected);
        assert_approx_eq!(actual.b, expected);
    }
}

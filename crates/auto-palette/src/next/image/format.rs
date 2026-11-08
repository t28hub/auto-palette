use crate::next::color::{AlphaColor, Srgb8};

/// The pixel layout of 8-bit image data.
///
/// Each channel takes one byte, in the order the name lists them. Color values are
/// gamma-encoded sRGB. Alpha is straight (not premultiplied), the same as the HTML canvas
/// `ImageData`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PixelFormat {
    /// Grayscale, opaque.
    Luma8,
    /// Grayscale and alpha.
    LumaA8,
    /// Red, green, and blue, opaque.
    Rgb8,
    /// Red, green, blue, and alpha.
    Rgba8,
}

impl PixelFormat {
    /// Returns the number of bytes per pixel.
    ///
    /// # Returns
    /// The number of bytes per pixel.
    #[inline]
    #[must_use]
    pub const fn bytes_per_pixel(self) -> usize {
        match self {
            Self::Luma8 => 1,
            Self::LumaA8 => 2,
            Self::Rgb8 => 3,
            Self::Rgba8 => 4,
        }
    }

    /// Reads one pixel of this format as an sRGB color with alpha.
    ///
    /// # Arguments
    /// * `pixel` - The bytes of one pixel.
    ///
    /// # Returns
    /// The color of the pixel. Formats without alpha are opaque.
    #[inline]
    pub(crate) fn to_srgba8(self, pixel: &[u8]) -> AlphaColor<Srgb8, u8> {
        match self {
            Self::Luma8 => AlphaColor::opaque(Srgb8::new(pixel[0], pixel[0], pixel[0])),
            Self::LumaA8 => AlphaColor::new_u8(Srgb8::new(pixel[0], pixel[0], pixel[0]), pixel[1]),
            Self::Rgb8 => AlphaColor::opaque(Srgb8::new(pixel[0], pixel[1], pixel[2])),
            Self::Rgba8 => AlphaColor::new_u8(Srgb8::new(pixel[0], pixel[1], pixel[2]), pixel[3]),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::luma8(PixelFormat::Luma8, 1)]
    #[case::luma_a8(PixelFormat::LumaA8, 2)]
    #[case::rgb8(PixelFormat::Rgb8, 3)]
    #[case::rgba8(PixelFormat::Rgba8, 4)]
    fn test_bytes_per_pixel(#[case] format: PixelFormat, #[case] expected: usize) {
        // Act
        let actual = format.bytes_per_pixel();

        // Assert
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case::luma8(PixelFormat::Luma8, &[64], AlphaColor::opaque(Srgb8::new(64, 64, 64)))]
    #[case::luma_a8(PixelFormat::LumaA8, &[64, 128], AlphaColor::new_u8(Srgb8::new(64, 64, 64), 128))]
    #[case::rgb8(PixelFormat::Rgb8, &[1, 2, 3], AlphaColor::opaque(Srgb8::new(1, 2, 3)))]
    #[case::rgba8(PixelFormat::Rgba8, &[1, 2, 3, 4], AlphaColor::new_u8(Srgb8::new(1, 2, 3), 4))]
    fn test_to_srgba8(
        #[case] format: PixelFormat,
        #[case] pixel: &[u8],
        #[case] expected: AlphaColor<Srgb8, u8>,
    ) {
        // Act
        let actual = format.to_srgba8(pixel);

        // Assert
        assert_eq!(actual, expected);
    }
}

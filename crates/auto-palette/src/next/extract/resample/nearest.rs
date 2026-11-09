use super::Resample;
use crate::next::{
    color::{AlphaColor, LinearRgb},
    extract::nearest::nearest,
    image::{ImageBuf, Size},
    ImageRef,
};

/// Takes the source pixel nearest to the center of each scaled pixel, also known as
/// [nearest-neighbor interpolation](https://en.wikipedia.org/wiki/Nearest-neighbor_interpolation).
///
/// It decodes only the pixels it keeps, so it is faster than area averaging on large images, but
/// it can miss small details.
pub(super) struct NearestNeighbor;

impl Resample for NearestNeighbor {
    fn resample(&self, image: &ImageRef<'_>, size: Size) -> ImageBuf<AlphaColor<LinearRgb>> {
        let source = image.size();
        let columns: Vec<u32> = (0..size.width)
            .map(|x| nearest(x, size.width, source.width))
            .collect();

        let mut pixels = Vec::with_capacity(size.pixel_count());
        for y in 0..size.height {
            let source_y = nearest(y, size.height, source.height);
            for &source_x in &columns {
                pixels.push(AlphaColor::from(image.pixel(source_x, source_y)));
            }
        }
        ImageBuf::new(size, pixels)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::next::{color::Srgb8, PixelFormat};

    /// An opaque gray pixel with the given 8-bit level.
    fn gray(level: u8) -> AlphaColor<LinearRgb> {
        AlphaColor::opaque(LinearRgb::from(Srgb8::new(level, level, level)))
    }

    #[test]
    fn test_resample_same_size() {
        // Arrange
        let pixels = [10, 20, 30];
        let image = ImageRef::new(&pixels, 3, 1, PixelFormat::Luma8).unwrap();

        // Act
        let actual = NearestNeighbor.resample(&image, Size::new(3, 1));

        // Assert
        assert_eq!(actual.size(), Size::new(3, 1));
        assert_eq!(actual.pixels(), [gray(10), gray(20), gray(30)]);
    }

    #[test]
    fn test_resample_picks_centers() {
        // Arrange: 4x4 into 2x2 keeps the pixels at (1, 1), (3, 1), (1, 3), and (3, 3).
        let pixels = [
            0, 0, 0, 0, //
            0, 11, 0, 31, //
            0, 0, 0, 0, //
            0, 13, 0, 33, //
        ];
        let image = ImageRef::new(&pixels, 4, 4, PixelFormat::Luma8).unwrap();

        // Act
        let actual = NearestNeighbor.resample(&image, Size::new(2, 2));

        // Assert
        assert_eq!(actual.pixels(), [gray(11), gray(31), gray(13), gray(33)]);
    }

    #[test]
    fn test_resample_keeps_alpha() {
        // Arrange: no averaging, so a transparent pixel keeps its own color.
        let pixels = [255, 255, 255, 0];
        let image = ImageRef::from_rgba8(&pixels, 1, 1).unwrap();

        // Act
        let actual = NearestNeighbor.resample(&image, Size::new(1, 1));

        // Assert
        assert_eq!(
            actual.pixels(),
            [AlphaColor::transparent(LinearRgb::new(1.0, 1.0, 1.0))]
        );
    }
}

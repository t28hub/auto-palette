use crate::{
    color::{rgb_to_xyz, xyz_to_lab, Lab, D65},
    error::Error,
    image::{ImageData, Pixel, RGBA_CHANNELS},
    math::{normalize, FloatNumber},
    Filter,
};

/// A borrowed view of RGBA pixel data.
///
/// This is the internal representation the extraction pipeline operates on.
/// It allows palette extraction directly from a borrowed `&[u8]` buffer
/// (see [`PaletteBuilder::build_from_raw`](crate::PaletteBuilder::build_from_raw))
/// without copying it into an owned [`ImageData`].
///
/// Invariant: `data.len() == width * height * 4`, checked on construction.
#[derive(Debug, Clone, Copy)]
pub(crate) struct RawImage<'a> {
    width: u32,
    height: u32,
    data: &'a [u8],
}

impl<'a> RawImage<'a> {
    /// Creates a new `RawImage` over the given pixel data.
    ///
    /// # Arguments
    /// * `width` - The width of the image.
    /// * `height` - The height of the image.
    /// * `data` - The pixel data in RGBA order.
    ///
    /// # Returns
    /// A new `RawImage` instance.
    ///
    /// # Errors
    /// Returns [`Error::InvalidImageData`] if the length of the pixel data is
    /// not equal to `width * height * 4`.
    pub(crate) fn new(width: u32, height: u32, data: &'a [u8]) -> Result<Self, Error> {
        let expected_length = (width * height) as usize * RGBA_CHANNELS;
        if data.len() != expected_length {
            return Err(Error::InvalidImageData {
                expected: expected_length,
                actual: data.len(),
            });
        }

        Ok(Self {
            width,
            height,
            data,
        })
    }

    /// Creates a new `RawImage` without validating the invariant.
    ///
    /// The caller must guarantee `data.len() == width * height * 4`
    /// (e.g. [`ImageData`] enforces it on construction).
    pub(crate) fn new_unchecked(width: u32, height: u32, data: &'a [u8]) -> Self {
        debug_assert_eq!(data.len(), (width * height) as usize * RGBA_CHANNELS);
        Self {
            width,
            height,
            data,
        }
    }

    /// Returns the width of the image.
    ///
    /// # Returns
    /// The width of the image.
    #[inline]
    pub(crate) fn width(self) -> u32 {
        self.width
    }

    /// Returns the height of the image.
    ///
    /// # Returns
    /// The height of the image.
    #[inline]
    pub(crate) fn height(self) -> u32 {
        self.height
    }

    /// Returns the area of the image.
    ///
    /// # Returns
    /// The area of the image.
    pub(crate) fn area(self) -> usize {
        self.width as usize * self.height as usize
    }

    /// Downsamples the image with nearest-neighbor sampling so that the
    /// result contains at most `max_pixels` pixels.
    ///
    /// The aspect ratio is approximately preserved. If the image already
    /// contains `max_pixels` pixels or fewer, an owned copy of the image is
    /// returned.
    ///
    /// # Arguments
    /// * `max_pixels` - The maximum number of pixels in the result.
    ///
    /// # Returns
    /// The downsampled image data.
    pub(crate) fn downsample(self, max_pixels: usize) -> ImageData {
        let area = self.area();
        if max_pixels == 0 || area <= max_pixels {
            return ImageData::from_parts(self.width, self.height, self.data.to_vec());
        }

        let stride = ((area as f64 / max_pixels as f64).sqrt().ceil() as u32).max(1);
        let width = self.width.div_ceil(stride);
        let height = self.height.div_ceil(stride);

        let mut data = Vec::with_capacity((width * height) as usize * RGBA_CHANNELS);
        for row in 0..height {
            let src_row = (row * stride) as usize;
            for col in 0..width {
                let src_col = (col * stride) as usize;
                let offset = (src_row * self.width as usize + src_col) * RGBA_CHANNELS;
                data.extend_from_slice(&self.data[offset..offset + RGBA_CHANNELS]);
            }
        }
        ImageData::from_parts(width, height, data)
    }

    /// Returns an iterator over the pixels of the image.
    ///
    /// # Returns
    /// An iterator over the pixels of the image.
    #[allow(dead_code)]
    pub(crate) fn pixels<T>(self) -> impl Iterator<Item = Pixel<T>> + 'a
    where
        T: FloatNumber,
    {
        self.data
            .chunks_exact(RGBA_CHANNELS)
            .enumerate()
            .map(move |(index, chunk)| self.chunk_to_pixel(index, chunk))
    }

    /// Returns an iterator over the pixels of the image together with the result of applying the `filter`.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type.
    /// * `F` - The filter type.
    ///
    /// # Arguments
    /// * `filter` - The filter to apply to the pixels.
    ///
    /// # Returns
    /// An iterator over the pixels of the image and the result of applying the filter.
    pub(crate) fn pixels_with_filter<'b, T, F>(
        self,
        filter: &'b F,
    ) -> impl Iterator<Item = (Pixel<T>, bool)> + 'b
    where
        'a: 'b,
        T: FloatNumber,
        F: Filter,
    {
        self.data
            .chunks_exact(RGBA_CHANNELS)
            .enumerate()
            .map(move |(index, chunk)| {
                (
                    self.chunk_to_pixel::<T>(index, chunk),
                    filter.test(&[chunk[0], chunk[1], chunk[2], chunk[3]]),
                )
            })
    }

    /// Converts a chunk of pixel data to a pixel representation.
    ///
    /// # Type Parameters
    /// * `T` - The floating point type.
    ///
    /// # Arguments
    /// * `index` - The index of the pixel in the image.
    /// * `chunk` - The chunk of pixel data.
    ///
    /// # Returns
    /// The pixel representation of the chunk of pixel data.
    #[inline(always)]
    #[must_use]
    fn chunk_to_pixel<T>(self, index: usize, chunk: &[u8]) -> Pixel<T>
    where
        T: FloatNumber,
    {
        let (x, y, z) = rgb_to_xyz(chunk[0], chunk[1], chunk[2]);
        let (l, a, b) = xyz_to_lab::<D65>(x, y, z);

        let coord_x = T::from_usize((index % self.width as usize) + 1);
        let coord_y = T::from_usize((index / self.width as usize) + 1);

        let width_f = T::from_u32(self.width);
        let height_f = T::from_u32(self.height);

        [
            T::from_f64(Lab::normalize_l(l)),
            T::from_f64(Lab::normalize_a(a)),
            T::from_f64(Lab::normalize_b(b)),
            normalize(coord_x, T::zero(), width_f),
            normalize(coord_y, T::zero(), height_f),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_approx_eq, Rgba};

    #[test]
    fn test_new() {
        // Arrange
        let pixels = [255u8; 2 * 2 * 4];

        // Act
        let actual = RawImage::new(2, 2, &pixels).unwrap();

        // Assert
        assert_eq!(actual.width(), 2);
        assert_eq!(actual.height(), 2);
        assert_eq!(actual.area(), 4);
    }

    #[test]
    fn test_new_unexpected_length() {
        // Arrange
        let pixels = [255, 255, 255, 255];

        // Act
        let actual = RawImage::new(2, 2, &pixels);

        // Assert
        assert!(matches!(
            actual.unwrap_err(),
            Error::InvalidImageData {
                expected: 16,
                actual: 4,
            }
        ));
    }

    #[test]
    fn test_downsample() {
        // Arrange
        let mut pixels = Vec::with_capacity(8 * 8 * 4);
        for index in 0..64u8 {
            pixels.extend_from_slice(&[index, index, index, 255]);
        }
        let raw = RawImage::new(8, 8, &pixels).unwrap();

        // Act
        let actual = raw.downsample(16);

        // Assert
        assert_eq!(actual.width(), 4);
        assert_eq!(actual.height(), 4);
        assert_eq!(actual.area(), 16);
        // Nearest-neighbor sampling picks every 2nd pixel of every 2nd row.
        assert_eq!(actual.data()[0], 0);
        assert_eq!(actual.data()[4], 2);
        assert_eq!(actual.data()[16], 16);
    }

    #[test]
    fn test_downsample_no_op() {
        // Arrange
        let pixels = vec![255; 2 * 2 * 4];
        let raw = RawImage::new(2, 2, &pixels).unwrap();

        // Act
        let actual = raw.downsample(4);

        // Assert
        assert_eq!(actual.width(), 2);
        assert_eq!(actual.height(), 2);
        assert_eq!(actual.data(), pixels.as_slice());
    }

    #[test]
    fn test_pixels() {
        // Arrange
        let pixels = [
            255, 255, 255, 255, // White
            0, 0, 0, 255, // Black
        ];
        let raw = RawImage::new(2, 1, &pixels).unwrap();

        // Act
        let actual: Vec<_> = raw.pixels::<f64>().collect();

        // Assert
        assert_eq!(actual.len(), 2);
        // White pixel: L = 100 normalized to 1.0
        assert_approx_eq!(actual[0][0], 1.0, 1e-3);
        // Black pixel: L = 0 normalized to 0.0
        assert_approx_eq!(actual[1][0], 0.0, 1e-3);
        // 1-indexed normalized coordinates
        assert_approx_eq!(actual[0][3], 0.5);
        assert_approx_eq!(actual[1][3], 1.0);
    }

    #[test]
    fn test_pixels_with_filter() {
        // Arrange
        let pixels = [
            255, 255, 255, 255, // White, opaque
            0, 0, 0, 0, // Black, transparent
        ];
        let raw = RawImage::new(2, 1, &pixels).unwrap();
        let filter = |rgba: &Rgba| rgba[3] != 0;

        // Act
        let actual: Vec<_> = raw.pixels_with_filter::<f64, _>(&filter).collect();

        // Assert
        assert_eq!(actual.len(), 2);
        assert!(actual[0].1);
        assert!(!actual[1].1);
    }
}

use crate::math::Point;

/// The number of channels in an RGBA pixel.
pub const RGBA_CHANNELS: usize = 4;

/// A pixel that contains the RGBA color space values.
pub type RgbaPixel = [u8; RGBA_CHANNELS];

/// The number of channels in a CIELAB pixel + the pixel coordinates.
pub const LABXY_CHANNELS: usize = 5;

/// A pixel that contains the CIELAB color space values and the coordinates of the pixel in the image.
///
/// # Type Parameters
/// * `T` - The floating point type.
pub type Pixel<T> = Point<T, LABXY_CHANNELS>;

use super::{area::AreaAverage, nearest::NearestNeighbor};
use crate::next::{
    color::{AlphaColor, LinearRgb},
    image::{ImageBuf, Size},
    ImageRef,
};

/// An algorithm that scales an image to a given size.
///
/// Implementations read pixels with [`ImageRef::pixel_rows`] or [`ImageRef::pixel`] and convert
/// them into [`AlphaColor<LinearRgb>`], so they only work on linear values.
pub(crate) trait Resample {
    /// Scales `image` to `size`.
    ///
    /// # Arguments
    /// * `image` - The image to scale, with at least one pixel.
    /// * `size` - The size of the scaled image, with at least one pixel.
    ///
    /// # Returns
    /// The scaled image in linear RGB.
    fn resample(&self, image: &ImageRef<'_>, size: Size) -> ImageBuf<AlphaColor<LinearRgb>>;
}

/// The resampling algorithm to use.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) enum Resampling {
    /// Area averaging ([`AreaAverage`]).
    #[default]
    Area,
    /// Nearest neighbor ([`NearestNeighbor`]).
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "selected by the builder, which is not implemented yet"
        )
    )]
    Nearest,
}

impl Resample for Resampling {
    fn resample(&self, image: &ImageRef<'_>, size: Size) -> ImageBuf<AlphaColor<LinearRgb>> {
        match self {
            Self::Area => AreaAverage.resample(image, size),
            Self::Nearest => NearestNeighbor.resample(image, size),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::next::PixelFormat;

    #[rstest]
    #[case::area(Resampling::Area)]
    #[case::nearest(Resampling::Nearest)]
    fn test_resample(#[case] resampling: Resampling) {
        // Arrange
        let pixels = [255; 4 * 4];
        let image = ImageRef::new(&pixels, 4, 4, PixelFormat::Luma8).unwrap();

        // Act
        let actual = resampling.resample(&image, Size::new(2, 2));

        // Assert
        let white = AlphaColor::opaque(LinearRgb::new(1.0, 1.0, 1.0));
        assert_eq!(actual.size(), Size::new(2, 2));
        assert_eq!(actual.pixels(), [white; 4]);
    }
}

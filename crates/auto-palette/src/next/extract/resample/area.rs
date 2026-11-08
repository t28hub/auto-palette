use super::Resample;
use crate::next::{
    color::{AlphaColor, LinearRgb},
    image::{ImageBuf, Size},
    ImageRef,
};

/// Averages the pixels that fall in each scaled pixel, also known as
/// [box sampling](https://en.wikipedia.org/wiki/Image_scaling#Box_sampling).
pub(super) struct AreaAverage;

impl Resample for AreaAverage {
    fn resample(&self, image: &ImageRef<'_>, size: Size) -> ImageBuf<AlphaColor<LinearRgb>> {
        let source = image.size();
        let width = size.width as usize;
        let columns = targets(source.width, size.width);
        let rows = targets(source.height, size.height);
        let mut sums = vec![[0.0; 4]; size.pixel_count()];
        for (row, &target_row) in image.pixel_rows().zip(&rows) {
            let sums = &mut sums[target_row * width..][..width];
            for (pixel, &target_column) in row.zip(&columns) {
                let pixel = AlphaColor::<LinearRgb>::from(pixel);
                let LinearRgb { r, g, b } = pixel.color();
                let alpha = pixel.alpha();
                // Weight the color by alpha so that transparent pixels add no color.
                let sum = &mut sums[target_column];
                sum[0] += r * alpha;
                sum[1] += g * alpha;
                sum[2] += b * alpha;
                sum[3] += alpha;
            }
        }

        let column_counts = counts(&columns, size.width);
        let row_counts = counts(&rows, size.height);
        let mut pixels = Vec::with_capacity(sums.len());
        for (row, row_count) in sums.chunks_exact(width).zip(&row_counts) {
            for (&sum, column_count) in row.iter().zip(&column_counts) {
                pixels.push(average(sum, row_count * column_count));
            }
        }
        ImageBuf::new(size, pixels)
    }
}

/// Maps each source index to the scaled index it falls in.
///
/// # Arguments
/// * `source` - The number of source pixels along one side.
/// * `target` - The number of scaled pixels along the same side.
///
/// # Returns
/// The scaled index of each source index.
fn targets(source: u32, target: u32) -> Vec<usize> {
    (0..u64::from(source))
        .map(|index| (index * u64::from(target) / u64::from(source)) as usize)
        .collect()
}

/// Counts the source pixels that fall in each scaled index.
///
/// # Arguments
/// * `targets` - The scaled index of each source index.
/// * `target` - The number of scaled pixels along the same side.
///
/// # Returns
/// The number of source pixels for each scaled index.
fn counts(targets: &[usize], target: u32) -> Vec<f64> {
    let mut counts = vec![0.0; target as usize];
    targets.iter().for_each(|&index| counts[index] += 1.0);
    counts
}

/// Averages the alpha-weighted sums of the pixels in one scaled pixel.
///
/// # Arguments
/// * `sum` - The sums of `r * alpha`, `g * alpha`, `b * alpha`, and `alpha`.
/// * `count` - The number of source pixels in the scaled pixel.
///
/// # Returns
/// The average color and alpha. If every pixel is transparent, the color is finite but unspecified.
fn average([r, g, b, alpha]: [f64; 4], count: f64) -> AlphaColor<LinearRgb> {
    let color = if alpha > 0.0 {
        LinearRgb::new(r / alpha, g / alpha, b / alpha)
    } else {
        LinearRgb::new(0.0, 0.0, 0.0)
    };
    AlphaColor::new(color, alpha / count).expect("an average of alpha values is an alpha value")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::next::PixelFormat;

    const RED: LinearRgb = LinearRgb::new(1.0, 0.0, 0.0);
    const BLUE: LinearRgb = LinearRgb::new(0.0, 0.0, 1.0);
    const WHITE: LinearRgb = LinearRgb::new(1.0, 1.0, 1.0);
    const GRAY: LinearRgb = LinearRgb::new(0.5, 0.5, 0.5);
    const BLACK: LinearRgb = LinearRgb::new(0.0, 0.0, 0.0);

    #[test]
    fn test_resample_same_size() {
        // Arrange
        let pixels = [
            255, 0, 0, 255, // Red
            0, 0, 255, 255, // Blue
        ];
        let image = ImageRef::from_rgba8(&pixels, 2, 1).unwrap();

        // Act
        let actual = AreaAverage.resample(&image, Size::new(2, 1));

        // Assert
        assert_eq!(actual.size(), Size::new(2, 1));
        assert_eq!(
            actual.pixels(),
            [AlphaColor::opaque(RED), AlphaColor::opaque(BLUE)]
        );
    }

    #[test]
    fn test_resample_averages_in_linear_light() {
        // Arrange: black and white, which average to 0.5 in linear light, not in sRGB.
        let pixels = [0, 255, 255, 0];
        let image = ImageRef::new(&pixels, 2, 2, PixelFormat::Luma8).unwrap();

        // Act
        let actual = AreaAverage.resample(&image, Size::new(1, 1));

        // Assert
        assert_eq!(actual.pixels(), [AlphaColor::opaque(GRAY)]);
    }

    #[test]
    fn test_resample_weights_by_alpha() {
        // Arrange: a transparent pixel adds no color.
        let pixels = [
            255, 0, 0, 255, // Red
            0, 255, 0, 0, // Transparent green
        ];
        let image = ImageRef::from_rgba8(&pixels, 2, 1).unwrap();

        // Act
        let actual = AreaAverage.resample(&image, Size::new(1, 1));

        // Assert
        assert_eq!(actual.pixels(), [AlphaColor::new(RED, 0.5).unwrap()]);
    }

    #[test]
    fn test_resample_transparent() {
        // Arrange
        let pixels = [255, 255, 255, 0];
        let image = ImageRef::from_rgba8(&pixels, 1, 1).unwrap();

        // Act
        let actual = AreaAverage.resample(&image, Size::new(1, 1));

        // Assert: the color is left to the caller, but it must not be NaN.
        let [pixel] = actual.pixels() else {
            panic!("expected one pixel");
        };
        let LinearRgb { r, g, b } = pixel.color();
        assert!(pixel.is_transparent());
        assert!(r.is_finite() && g.is_finite() && b.is_finite());
    }

    #[test]
    fn test_resample_uneven_ratio() {
        // Arrange: 3 columns into 2, so the scaled pixels average 2 and 1 source pixels.
        let pixels = [255, 255, 0];
        let image = ImageRef::new(&pixels, 3, 1, PixelFormat::Luma8).unwrap();

        // Act
        let actual = AreaAverage.resample(&image, Size::new(2, 1));

        // Assert
        assert_eq!(
            actual.pixels(),
            [AlphaColor::opaque(WHITE), AlphaColor::opaque(BLACK)]
        );
    }
}

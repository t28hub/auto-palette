use super::LinearRgb;

/// A color in OKLab.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "read by the segmentation, which is not implemented yet"
    )
)]
pub(crate) struct Oklab {
    /// The perceived lightness, from 0.0 (black) to 1.0 (white).
    pub(crate) l: f64,
    /// The green (negative) to red (positive) axis.
    pub(crate) a: f64,
    /// The blue (negative) to yellow (positive) axis.
    pub(crate) b: f64,
}

impl Oklab {
    /// Creates a color in OKLab from its lightness and two color axes.
    ///
    /// # Arguments
    /// * `l` - The lightness.
    /// * `a` - The green-red value.
    /// * `b` - The blue-yellow value.
    ///
    /// # Returns
    /// An OKLab color with the given `l`, `a`, and `b` values.
    #[inline]
    pub(crate) const fn new(l: f64, a: f64, b: f64) -> Self {
        Self { l, a, b }
    }
}

/// Converts with the matrices from
/// [the original OKLab post](https://bottosson.github.io/posts/oklab/).
impl From<LinearRgb> for Oklab {
    #[inline]
    fn from(LinearRgb { r, g, b }: LinearRgb) -> Self {
        let l = (0.412_221_470_8 * r + 0.536_332_536_3 * g + 0.051_445_992_9 * b).cbrt();
        let m = (0.211_903_498_2 * r + 0.680_699_545_1 * g + 0.107_396_956_6 * b).cbrt();
        let s = (0.088_302_461_9 * r + 0.281_718_837_6 * g + 0.629_978_700_5 * b).cbrt();
        Self::new(
            0.210_454_255_3 * l + 0.793_617_785_0 * m - 0.004_072_046_8 * s,
            1.977_998_495_1 * l - 2.428_592_205_0 * m + 0.450_593_709_9 * s,
            0.025_904_037_1 * l + 0.782_771_766_2 * m - 0.808_675_766_0 * s,
        )
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::assert_approx_eq;

    #[rstest]
    #[case::black(LinearRgb::new(0.0, 0.0, 0.0), Oklab::new(0.0, 0.0, 0.0))]
    #[case::white(LinearRgb::new(1.0, 1.0, 1.0), Oklab::new(1.0, 0.0, 0.0))]
    #[case::red(
        LinearRgb::new(1.0, 0.0, 0.0),
        Oklab::new(0.627_955, 0.224_863, 0.125_846)
    )]
    #[case::green(LinearRgb::new(0.0, 1.0, 0.0), Oklab::new(0.866_440, -0.233_888, 0.179_498))]
    #[case::blue(LinearRgb::new(0.0, 0.0, 1.0), Oklab::new(0.452_014, -0.032_457, -0.311_528))]
    fn test_from_linear_rgb(#[case] rgb: LinearRgb, #[case] expected: Oklab) {
        // Act
        let actual = Oklab::from(rgb);

        // Assert
        assert_approx_eq!(actual.l, expected.l, 1e-5);
        assert_approx_eq!(actual.a, expected.a, 1e-5);
        assert_approx_eq!(actual.b, expected.b, 1e-5);
    }
}

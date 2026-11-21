mod alpha_color;
mod delta_e;
mod linear_rgb;
mod oklab;
mod srgb8;

pub(crate) use self::{
    alpha_color::AlphaColor,
    delta_e::DeltaE,
    linear_rgb::LinearRgb,
    oklab::Oklab,
    srgb8::Srgb8,
};

/// A color, independent of any color space.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Color(Oklab);

impl Color {
    /// Black.
    pub(crate) const BLACK: Self = Self(Oklab::new(0.0, 0.0, 0.0));

    /// Creates a color from OKLab.
    ///
    /// # Arguments
    /// * `oklab` - The color in OKLab.
    ///
    /// # Returns
    /// The color with the given OKLab values.
    #[inline]
    pub(crate) const fn from_oklab(oklab: Oklab) -> Self {
        Self(oklab)
    }

    /// Creates a color from linear sRGB.
    ///
    /// # Arguments
    /// * `rgb` - The color in linear sRGB.
    ///
    /// # Returns
    /// The color converted from `rgb`.
    #[inline]
    pub(crate) fn from_linear_rgb(rgb: LinearRgb) -> Self {
        Self(Oklab::from(rgb))
    }

    /// Converts the color to OKLab.
    ///
    /// # Returns
    /// The color in OKLab.
    #[inline]
    pub(crate) const fn to_oklab(self) -> Oklab {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_oklab_keeps_values() {
        // Arrange
        let oklab = Oklab::new(0.5, 0.1, -0.1);

        // Act
        let actual = Color::from_oklab(oklab);

        // Assert
        assert_eq!([actual.0.l, actual.0.a, actual.0.b], [0.5, 0.1, -0.1]);
    }

    #[test]
    fn test_from_linear_rgb_converts_to_oklab() {
        // Arrange
        let rgb = LinearRgb::new(1.0, 0.0, 0.0);

        // Act
        let actual = Color::from_linear_rgb(rgb);

        // Assert
        let expected = Oklab::from(rgb);
        assert_eq!(
            [actual.0.l, actual.0.a, actual.0.b],
            [expected.l, expected.a, expected.b]
        );
    }

    #[test]
    fn test_to_oklab_keeps_values() {
        // Arrange
        let color = Color(Oklab::new(0.5, 0.1, -0.1));

        // Act
        let actual = color.to_oklab();

        // Assert
        assert_eq!([actual.l, actual.a, actual.b], [0.5, 0.1, -0.1]);
    }
}

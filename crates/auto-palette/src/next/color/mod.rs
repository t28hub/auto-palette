mod alpha_color;
mod linear_rgb;
mod oklab;
mod srgb8;

pub(crate) use self::{alpha_color::AlphaColor, linear_rgb::LinearRgb, oklab::Oklab, srgb8::Srgb8};

/// A color, independent of any color space.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Color(Oklab);

#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "used by the color difference, which is not implemented yet"
    )
)]
impl Color {
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
        // Act
        let actual = Color::from_oklab(Oklab::new(0.5, 0.1, -0.1));

        // Assert
        assert_eq!([actual.0.l, actual.0.a, actual.0.b], [0.5, 0.1, -0.1]);
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

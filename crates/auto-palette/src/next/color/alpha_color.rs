/// A color with a straight (not premultiplied) alpha value.
///
/// The alpha type `A` sets the range: `f64` from 0.0 to 1.0, or `u8` from 0 to 255. The alpha is
/// always in that range, because [`AlphaColor::new`] rejects any other value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct AlphaColor<C, A = f64> {
    /// The color.
    color: C,
    /// The opacity, from [`AlphaValue::TRANSPARENT`] to [`AlphaValue::OPAQUE`].
    alpha: A,
}

/// A type that holds an alpha value.
pub(crate) trait AlphaValue: Copy + PartialOrd {
    /// The value of a fully transparent color.
    const TRANSPARENT: Self;
    /// The value of a fully opaque color.
    const OPAQUE: Self;
}

impl AlphaValue for f64 {
    const TRANSPARENT: Self = 0.0;
    const OPAQUE: Self = 1.0;
}

impl AlphaValue for u8 {
    const TRANSPARENT: Self = 0;
    const OPAQUE: Self = u8::MAX;
}

impl<C, A: AlphaValue> AlphaColor<C, A> {
    /// Creates a color with an alpha value.
    ///
    /// # Arguments
    /// * `color` - The color.
    /// * `alpha` - The opacity, from [`AlphaValue::TRANSPARENT`] to [`AlphaValue::OPAQUE`].
    ///
    /// # Returns
    /// `color` with the opacity `alpha`, or `None` if `alpha` is out of range or is NaN.
    #[inline]
    pub(crate) fn new(color: C, alpha: A) -> Option<Self> {
        (A::TRANSPARENT..=A::OPAQUE)
            .contains(&alpha)
            .then_some(Self { color, alpha })
    }

    /// Creates a fully opaque color.
    ///
    /// # Arguments
    /// * `color` - The color.
    ///
    /// # Returns
    /// `color` with the opacity [`AlphaValue::OPAQUE`].
    #[inline]
    pub(crate) const fn opaque(color: C) -> Self {
        Self {
            color,
            alpha: A::OPAQUE,
        }
    }

    /// Creates a fully transparent color.
    ///
    /// # Arguments
    /// * `color` - The color.
    ///
    /// # Returns
    /// `color` with the opacity [`AlphaValue::TRANSPARENT`].
    #[inline]
    pub(crate) const fn transparent(color: C) -> Self {
        Self {
            color,
            alpha: A::TRANSPARENT,
        }
    }

    /// Returns whether the color is fully transparent.
    ///
    /// # Returns
    /// `true` if the opacity is [`AlphaValue::TRANSPARENT`].
    #[inline]
    pub(crate) fn is_transparent(&self) -> bool {
        self.alpha == A::TRANSPARENT
    }
}

impl<C> AlphaColor<C, u8> {
    /// Creates a color with an 8-bit alpha value, which is always in range.
    ///
    /// # Arguments
    /// * `color` - The color.
    /// * `alpha` - The opacity, from 0 (transparent) to 255 (opaque).
    ///
    /// # Returns
    /// `color` with the opacity `alpha`.
    #[inline]
    pub(crate) const fn new_u8(color: C, alpha: u8) -> Self {
        Self { color, alpha }
    }
}

/// Scales the 8-bit alpha value to 0.0 to 1.0.
impl<C> From<AlphaColor<C, u8>> for AlphaColor<C> {
    #[inline]
    fn from(color: AlphaColor<C, u8>) -> Self {
        Self {
            color: color.color,
            // Any 8-bit value divided by the maximum is from 0.0 to 1.0.
            alpha: f64::from(color.alpha) / f64::from(u8::OPAQUE),
        }
    }
}

impl<C: Copy, A: Copy> AlphaColor<C, A> {
    /// Returns the color.
    ///
    /// # Returns
    /// The color without its alpha value.
    #[inline]
    pub(crate) const fn color(&self) -> C {
        self.color
    }

    /// Returns the alpha value.
    ///
    /// # Returns
    /// The opacity, from [`AlphaValue::TRANSPARENT`] to [`AlphaValue::OPAQUE`].
    #[inline]
    pub(crate) const fn alpha(&self) -> A {
        self.alpha
    }
}

impl<C, A> AlphaColor<C, A> {
    /// Converts the color and keeps the alpha value.
    ///
    /// # Arguments
    /// * `f` - The function that converts the color.
    ///
    /// # Returns
    /// The converted color with the same alpha value.
    #[inline]
    pub(crate) fn map_color<D>(self, f: impl FnOnce(C) -> D) -> AlphaColor<D, A> {
        AlphaColor {
            color: f(self.color),
            alpha: self.alpha,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::transparent(0.0)]
    #[case::partial(0.5)]
    #[case::opaque(1.0)]
    fn test_new(#[case] alpha: f64) {
        // Act
        let actual = AlphaColor::new((), alpha);

        // Assert
        assert_eq!(actual.map(|color| color.alpha()), Some(alpha));
    }

    #[rstest]
    #[case::negative(-0.1)]
    #[case::above_one(1.1)]
    #[case::nan(f64::NAN)]
    fn test_new_out_of_range(#[case] alpha: f64) {
        // Act
        let actual = AlphaColor::new((), alpha);

        // Assert
        assert_eq!(actual, None);
    }

    #[test]
    fn test_new_u8() {
        // Act
        let actual = AlphaColor::new_u8((), 51);

        // Assert
        assert_eq!(actual.alpha(), 51);
    }

    #[rstest]
    #[case::transparent(0, 0.0)]
    #[case::partial(51, 0.2)]
    #[case::opaque(255, 1.0)]
    fn test_from_u8_alpha(#[case] alpha: u8, #[case] expected: f64) {
        // Act
        let actual = AlphaColor::<()>::from(AlphaColor::new_u8((), alpha));

        // Assert
        assert_eq!(actual.alpha(), expected);
    }

    #[test]
    fn test_opaque() {
        // Act
        let actual = AlphaColor::<()>::opaque(());

        // Assert
        assert_eq!(actual.alpha(), 1.0);
    }

    #[test]
    fn test_transparent() {
        // Act
        let actual = AlphaColor::<()>::transparent(());

        // Assert
        assert_eq!(actual.alpha(), 0.0);
    }

    #[rstest]
    #[case::transparent(0, true)]
    #[case::partial(1, false)]
    #[case::opaque(255, false)]
    fn test_is_transparent(#[case] alpha: u8, #[case] expected: bool) {
        // Arrange
        let color = AlphaColor::new_u8((), alpha);

        // Act
        let actual = color.is_transparent();

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_map_color() {
        // Arrange
        let color = AlphaColor::new(2, 0.5).unwrap();

        // Act
        let actual = color.map_color(|value| value * 3);

        // Assert
        assert_eq!(actual.color(), 6);
        assert_eq!(actual.alpha(), 0.5);
    }
}

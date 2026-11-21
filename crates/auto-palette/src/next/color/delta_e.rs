use super::Color;

/// A formula for the difference between two colors.
///
/// From [CSS Color 4](https://www.w3.org/TR/css-color-4/#color-difference-code).
#[derive(Debug, Clone, Copy)]
pub(crate) enum DeltaE {
    /// ΔEOK, the Euclidean distance in OKLab.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "selected by the builder, which is not implemented yet"
        )
    )]
    Ok,
    /// ΔEOKr2, ΔEOK with the lightness through Ottosson's toe and a and b doubled.
    Okr2,
}

impl DeltaE {
    /// Measures the difference between two colors.
    ///
    /// # Arguments
    /// * `reference` - The color to measure from.
    /// * `sample` - The color to measure.
    ///
    /// # Returns
    /// The difference between `reference` and `sample`, 0.0 for the same color.
    #[inline]
    pub(crate) fn measure(self, reference: Color, sample: Color) -> f64 {
        match self {
            Self::Ok => {
                let (reference, sample) = (reference.to_oklab(), sample.to_oklab());
                let delta_l = reference.l - sample.l;
                let delta_a = reference.a - sample.a;
                let delta_b = reference.b - sample.b;
                (delta_l * delta_l + delta_a * delta_a + delta_b * delta_b).sqrt()
            }
            Self::Okr2 => {
                let (reference, sample) = (reference.to_oklab(), sample.to_oklab());
                let delta_l = apply_toe(reference.l) - apply_toe(sample.l);
                let delta_a = 2.0 * (reference.a - sample.a);
                let delta_b = 2.0 * (reference.b - sample.b);
                (delta_l * delta_l + delta_a * delta_a + delta_b * delta_b).sqrt()
            }
        }
    }
}

/// Applies Ottosson's toe, which brings OKLab's lightness close to CIELab's.
///
/// From ["Okhsv and Okhsl"](https://bottosson.github.io/posts/colorpicker/) (Björn Ottosson).
///
/// # Arguments
/// * `l` - The lightness in OKLab, from 0.0 to 1.0.
///
/// # Returns
/// The lightness after the toe, from 0.0 to 1.0.
fn apply_toe(l: f64) -> f64 {
    const K1: f64 = 0.206;
    const K2: f64 = 0.03;
    const K3: f64 = (1.0 + K1) / (1.0 + K2);
    let shifted = K3 * l - K1;
    0.5 * (shifted + (shifted * shifted + 4.0 * K2 * K3 * l).sqrt())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::{assert_approx_eq, next::color::Oklab};

    /// Creates a color from its OKLab values.
    fn color(l: f64, a: f64, b: f64) -> Color {
        Color::from_oklab(Oklab::new(l, a, b))
    }

    #[test]
    fn test_measure_ok() {
        // Arrange
        let reference = color(0.5, 0.0, 0.0);
        let sample = color(0.5, 0.03, 0.04);

        // Act
        let actual = DeltaE::Ok.measure(reference, sample);

        // Assert
        assert_approx_eq!(actual, 0.05);
    }

    #[test]
    fn test_measure_okr2_doubles_chroma_difference() {
        // Arrange
        let reference = color(0.5, 0.0, 0.0);
        let sample = color(0.5, 0.03, 0.04);

        // Act
        let actual = DeltaE::Okr2.measure(reference, sample);

        // Assert
        assert_approx_eq!(actual, 0.1);
    }

    #[test]
    fn test_measure_okr2_shrinks_dark_difference() {
        // Arrange
        let reference = color(0.05, 0.0, 0.0);
        let sample = color(0.1, 0.0, 0.0);

        // Act
        let actual = DeltaE::Okr2.measure(reference, sample);

        // Assert
        assert_approx_eq!(actual, 0.018_553);
    }

    #[rstest]
    #[case::black(0.0, 0.0)]
    #[case::dark(0.05, 0.011_078)]
    #[case::dim(0.1, 0.029_631)]
    #[case::middle(0.5, 0.421_141)]
    #[case::white(1.0, 1.0)]
    fn test_apply_toe(#[case] l: f64, #[case] expected: f64) {
        // Act
        let actual = apply_toe(l);

        // Assert
        assert_approx_eq!(actual, expected);
    }
}

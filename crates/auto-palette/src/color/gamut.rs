use crate::{color::Hue, math::FloatNumber};

/// The gamut representation for the color space.
#[derive(Debug, Clone, PartialEq)]
pub struct Gamut {
    /// The kind of color gamut.
    kind: GamutKind,

    /// The maximum chroma values for each hue (0 to 359 degrees).
    values: [f32; 360],
}

impl Gamut {
    /// Creates a new `Gamut` instance with the specified kind and chroma values.
    ///
    /// # Type Parameters
    /// * `T` - The float number type.
    ///
    /// # Arguments
    /// * `kind` - The kind of color gamut.
    /// * `values` - An array of maximum chroma values for each hue (0 to 359 degrees).
    #[must_use]
    pub fn new(kind: GamutKind, values: [f32; 360]) -> Self {
        Self { kind, values }
    }

    /// Calculates the maximum chroma at a given hue.
    ///
    /// # Type Parameters
    /// * `T` - The float number type.
    ///
    /// # Arguments
    /// * `hue` - The hue to calculate the maximum chroma for.
    ///
    /// # Returns
    /// The maximum chroma at the specified hue within the gamut.
    #[inline]
    #[must_use]
    pub fn max_chroma_at<T>(&self, hue: &Hue<T>) -> T
    where
        T: FloatNumber,
    {
        let degrees = hue.to_degrees().round().trunc_to_usize();
        let index = degrees % self.values.len();
        T::from_f32(self.values[index])
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum GamutKind {
    /// The sRGB color space.
    /// This is the most common color space used in web and digital media.
    /// [sRGB - Wikipedia](https://en.wikipedia.org/wiki/SRGB)
    #[default]
    Srgb = 0,

    /// The Display P3 color space.
    /// This color space is wider than sRGB and is used in modern displays, especially in Apple devices.
    /// [DCI-P3 - Wikipedia](https://en.wikipedia.org/wiki/Display_P3)
    DisplayP3 = 1,

    /// The Adobe RGB color space.
    /// This color space is wider than sRGB and is often used in professional photography and printing.
    /// [Adobe RGB - Wikipedia](https://en.wikipedia.org/wiki/Adobe_RGB_color_space)
    #[allow(unused)]
    AdobeRgb = 2,
}

impl TryFrom<u8> for GamutKind {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GamutKind::Srgb),
            1 => Ok(GamutKind::DisplayP3),
            2 => Ok(GamutKind::AdobeRgb),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::assert_approx_eq;

    // #[rstest]
    // #[case::hue0(0.0, 84.961)]
    // #[case::hue30(30.0, 92.664)]
    // #[case::hue60(60.0, 85.401)]
    // #[case::hue90(90.0, 86.098)]
    // #[case::hue120(120.0, 102.102)]
    // #[case::hue150(150.0, 87.037)]
    // #[case::hue180(180.0, 56.041)]
    // #[case::hue210(210.0, 45.280)]
    // #[case::hue240(240.0, 44.448)]
    // #[case::hue270(270.0, 56.731)]
    // #[case::hue300(300.0, 107.831)]
    // #[case::hue330(330.0, 110.767)]
    // #[case::hue360(360.0, 84.961)]
    // fn test_max_chroma(#[case] degrees: f64, #[case] expected: f64) {
    //     // Act
    //     let hue = Hue::from_degrees(degrees);
    //     let actual = Gamut::default().max_chroma(hue);
    //
    //     // Assert
    //     assert_approx_eq!(actual, expected, 1e-3);
    // }
    //
    // #[rstest]
    // #[case::adobe_rgb(Gamut::AdobeRgb, 123.071)]
    // #[case::display_p3(Gamut::DisplayP3, 129.371)]
    // #[case::srgb(Gamut::Srgb, 121.086)]
    // fn test_max_chroma_gamut(#[case] gamut: Gamut, #[case] expected: f64) {
    //     // Act
    //     let hue = Hue::from_degrees(315.0);
    //     let actual = gamut.max_chroma(hue);
    //
    //     // Assert
    //     assert_approx_eq!(actual, expected, 1e-3);
    // }
    //
    // #[rstest]
    // #[case::lightness0(0.0, 0.0)]
    // #[case::lightness50(50.0, 62.396)]
    // #[case::lightness100(100.0, 0.0)]
    // #[case::lightness101(101.0, 0.0)]
    // fn test_max_chroma_at_lightness(#[case] lightness: f64, #[case] expected: f64) {
    //     // Act
    //     let hue = Hue::from_degrees(120.0);
    //     let actual = Gamut::default().max_chroma_at_lightness(hue, lightness);
    //
    //     // Assert
    //     assert_approx_eq!(actual, expected, 1e-3);
    // }
    //
    // #[rstest]
    // #[case::in_gamut_true(90.0, 56.041, true)]
    // #[case::in_gamut_false(90.0, 100.0, false)]
    // fn test_in_gamut(#[case] lightness: f64, #[case] chroma: f64, #[case] expected: bool) {
    //     // Act
    //     let hue = Hue::from_degrees(180.0);
    //     let actual = Gamut::default().in_gamut(hue, lightness, chroma);
    //
    //     // Assert
    //     assert_eq!(actual, expected);
    // }
    //
    // #[rstest]
    // #[case::adobe_rgb(Gamut::AdobeRgb, (0.715, 0.0, 0.958))]
    // #[case::display_p3(Gamut::DisplayP3, (0.822, 0.033, 0.927))]
    // #[case::srgb(Gamut::Srgb, (1.0, 0.0, 1.0))]
    // fn test_xyz_to_rgb(#[case] gamut: Gamut, #[case] expected: (f64, f64, f64)) {
    //     // Act
    //     let xyz = [0.5928, 0.2848, 0.9699];
    //     let actual = gamut.xyz_to_rgb(xyz);
    //
    //     // Assert
    //     assert_approx_eq!(actual[0], expected.0, 1e-3);
    //     assert_approx_eq!(actual[1], expected.1, 1e-3);
    //     assert_approx_eq!(actual[2], expected.2, 1e-3);
    // }
}

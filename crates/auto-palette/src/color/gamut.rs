use crate::{
    color::{Hue, WhitePoint, D65},
    FloatNumber,
};

/// The gamut representation for the color space.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Gamut {
    /// The Adobe RGB color space.
    /// This color space is wider than sRGB and is often used in professional photography and printing.
    /// [Adobe RGB - Wikipedia](https://en.wikipedia.org/wiki/Adobe_RGB_color_space)
    #[allow(unused)]
    AdobeRgb,

    /// The Display P3 color space.
    /// This color space is wider than sRGB and is used in modern displays, especially in Apple devices.
    /// [DCI-P3 - Wikipedia](https://en.wikipedia.org/wiki/Display_P3)
    #[allow(unused)]
    DisplayP3,

    /// The sRGB color space.
    /// This is the most common color space used in web and digital media.
    /// [sRGB - Wikipedia](https://en.wikipedia.org/wiki/SRGB)
    #[default]
    Srgb,
}

impl Gamut {
    /// The minimum lightness value for the gamut calculations.
    const MIN_LIGHTNESS: u8 = 0;

    /// The maximum lightness value for the gamut calculations.
    const MAX_LIGHTNESS: u8 = 100;

    /// The minimum chroma value for the gamut calculations.
    const MIN_CHROMA: u8 = 0;

    /// The maximum chroma value for the gamut calculations.
    const MAX_CHROMA: u8 = 180;

    /// The epsilon value for the minimum difference in max chroma calculations.
    const EPSILON: f64 = 1e-5;

    /// Calculates the maximum chroma at a given hue within the specified gamut.
    ///
    /// # Type Parameters
    /// * `T` - The float number type.
    ///
    /// # Arguments
    /// * `hue` - The hue to calculate the maximum chroma for.
    /// * `gamut` - The gamut to use for the calculation.
    ///
    /// # Returns
    /// The maximum chroma at the specified hue within the gamut.
    #[inline]
    #[must_use]
    pub fn max_chroma<T>(&self, hue: Hue<T>) -> T
    where
        T: FloatNumber,
    {
        (Self::MIN_LIGHTNESS..=Self::MAX_LIGHTNESS).fold(T::zero(), |max, step| {
            let lightness = T::from_u8(step);
            let chroma = self.max_chroma_at_lightness(hue, lightness);
            if chroma > max {
                chroma
            } else {
                max
            }
        })
    }

    /// Calculates the maximum chroma at a given hue and lightness within the specified gamut.
    ///
    /// # Type Parameters
    /// * `T` - The float number type.
    ///
    /// # Arguments
    /// * `hue` - The hue to calculate the maximum chroma for.
    /// * `lightness` - The lightness to calculate the maximum chroma for.
    ///
    /// # Returns
    /// The maximum chroma at the specified lightness and hue within the gamut.
    #[inline]
    #[must_use]
    pub fn max_chroma_at_lightness<T>(&self, hue: Hue<T>, lightness: T) -> T
    where
        T: FloatNumber,
    {
        let mut min_chroma = T::from_u8(Self::MIN_CHROMA);
        let mut max_chroma = T::from_u8(Self::MAX_CHROMA);
        while max_chroma - min_chroma > T::from_f64(Self::EPSILON) {
            let mid_chroma = (min_chroma + max_chroma) / T::from_f64(2.0);
            if self.in_gamut(hue, lightness, mid_chroma) {
                min_chroma = mid_chroma;
            } else {
                max_chroma = mid_chroma;
            }
        }
        min_chroma
    }

    /// Checks if the given XYZ color is within the gamut of the color space.
    ///
    /// # Type Parameters
    /// * `T` - The float number type.
    ///
    /// # Arguments
    /// * `hue` - The hue of the color.
    /// * `lightness` - The lightness of the color.
    /// * `chroma` - The chroma of the color.
    ///
    /// # Returns
    /// `true` if the color is within the gamut, `false` otherwise.
    #[inline]
    #[must_use]
    fn in_gamut<T>(&self, hue: Hue<T>, lightness: T, chroma: T) -> bool
    where
        T: FloatNumber,
    {
        let radians = hue.to_radians();
        let xyz = lab_to_xyz::<T, D65>([lightness, chroma * radians.cos(), chroma * radians.sin()]);
        self.xyz_to_rgb(xyz)
            .iter()
            .all(|&c| T::zero() <= c && c <= T::one())
    }

    /// Converts the XYZ color to RGB color space based on the gamut.
    ///
    /// # Type Parameters
    /// * `T` - The float number type.
    ///
    /// # Arguments
    /// * `[x, y, z]` - The XYZ color components.
    ///
    /// # Returns
    /// The RGB color components as an array.
    ///
    /// # Notes
    /// The matrix used for conversion is based on the color space conversion formulas.
    /// [Color space conversion RGB-XYZ conversion](https://fujiwaratko.sakura.ne.jp/infosci/colorspace/colorspace2.html)
    #[inline]
    #[must_use]
    fn xyz_to_rgb<T>(&self, [x, y, z]: [T; 3]) -> [T; 3]
    where
        T: FloatNumber,
    {
        let m = match *self {
            Gamut::AdobeRgb => [
                T::from_f32(2.041588),
                T::from_f32(-0.565007),
                T::from_f32(-0.344731),
                T::from_f32(-0.969244),
                T::from_f32(1.875968),
                T::from_f32(0.041555),
                T::from_f32(0.013444),
                T::from_f32(-0.118362),
                T::from_f32(1.015175),
            ],
            Gamut::DisplayP3 => [
                T::from_f32(2.493497),
                T::from_f32(-0.931384),
                T::from_f32(-0.402711),
                T::from_f32(-0.829489),
                T::from_f32(1.762664),
                T::from_f32(0.023625),
                T::from_f32(0.035846),
                T::from_f32(-0.076172),
                T::from_f32(0.956885),
            ],
            Gamut::Srgb => [
                T::from_f32(3.24097),
                T::from_f32(-1.537383),
                T::from_f32(-0.498611),
                T::from_f32(-0.969244),
                T::from_f32(1.875968),
                T::from_f32(0.041555),
                T::from_f32(0.055630),
                T::from_f32(-0.203977),
                T::from_f32(1.056972),
            ],
        };
        [
            m[0] * x + m[1] * y + m[2] * z,
            m[3] * x + m[4] * y + m[5] * z,
            m[6] * x + m[7] * y + m[8] * z,
        ]
    }
}

#[inline]
#[must_use]
fn lab_to_xyz<T, WP>([l, a, b]: [T; 3]) -> [T; 3]
where
    T: FloatNumber,
    WP: WhitePoint,
{
    let epsilon = T::from_f64(6.0 / 29.0);
    let kappa = T::from_f64(108.0 / 841.0); // 3.0 * ((6.0 / 29.0) ^ 2)
    let delta = T::from_f64(4.0 / 29.0);

    let f = |t: T| -> T {
        if t > epsilon {
            t.powi(3)
        } else {
            kappa * (t - delta)
        }
    };

    let l2 = (l + T::from_f64(16.0)) / T::from_f64(116.0);
    let fx = f(a / T::from_f64(500.0) + l2);
    let fy = f(l2);
    let fz = f(l2 - b / T::from_f32(200.0));

    let x = WP::x::<T>() * fx;
    let y = WP::y::<T>() * fy;
    let z = WP::z::<T>() * fz;
    [x, y, z]
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::assert_approx_eq;

    #[test]
    fn test_default() {
        // Act
        let gamut = Gamut::default();

        // Assert
        assert_eq!(gamut, Gamut::Srgb);
    }

    #[rstest]
    #[case::hue0(0.0, 84.961)]
    #[case::hue30(30.0, 92.664)]
    #[case::hue60(60.0, 85.401)]
    #[case::hue90(90.0, 86.098)]
    #[case::hue120(120.0, 102.102)]
    #[case::hue150(150.0, 87.037)]
    #[case::hue180(180.0, 56.041)]
    #[case::hue210(210.0, 45.280)]
    #[case::hue240(240.0, 44.448)]
    #[case::hue270(270.0, 56.731)]
    #[case::hue300(300.0, 107.831)]
    #[case::hue330(330.0, 110.767)]
    #[case::hue360(360.0, 84.961)]
    fn test_max_chroma(#[case] degrees: f64, #[case] expected: f64) {
        // Act
        let hue = Hue::from_degrees(degrees);
        let actual = Gamut::default().max_chroma(hue);

        // Assert
        assert_approx_eq!(actual, expected, 1e-3);
    }

    #[rstest]
    #[case::adobe_rgb(Gamut::AdobeRgb, 123.071)]
    #[case::display_p3(Gamut::DisplayP3, 129.371)]
    #[case::srgb(Gamut::Srgb, 121.086)]
    fn test_max_chroma_gamut(#[case] gamut: Gamut, #[case] expected: f64) {
        // Act
        let hue = Hue::from_degrees(315.0);
        let actual = gamut.max_chroma(hue);

        // Assert
        assert_approx_eq!(actual, expected, 1e-3);
    }

    #[rstest]
    #[case::lightness0(0.0, 0.0)]
    #[case::lightness50(50.0, 62.396)]
    #[case::lightness100(100.0, 0.0)]
    #[case::lightness101(101.0, 0.0)]
    fn test_max_chroma_at_lightness(#[case] lightness: f64, #[case] expected: f64) {
        // Act
        let hue = Hue::from_degrees(120.0);
        let actual = Gamut::default().max_chroma_at_lightness(hue, lightness);

        // Assert
        assert_approx_eq!(actual, expected, 1e-3);
    }

    #[rstest]
    #[case::in_gamut_true(90.0, 56.041, true)]
    #[case::in_gamut_false(90.0, 100.0, false)]
    fn test_in_gamut(#[case] lightness: f64, #[case] chroma: f64, #[case] expected: bool) {
        // Act
        let hue = Hue::from_degrees(180.0);
        let actual = Gamut::default().in_gamut(hue, lightness, chroma);

        // Assert
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case::adobe_rgb(Gamut::AdobeRgb, (0.715, 0.0, 0.958))]
    #[case::display_p3(Gamut::DisplayP3, (0.822, 0.033, 0.927))]
    #[case::srgb(Gamut::Srgb, (1.0, 0.0, 1.0))]
    fn test_xyz_to_rgb(#[case] gamut: Gamut, #[case] expected: (f64, f64, f64)) {
        // Act
        let xyz = [0.5928, 0.2848, 0.9699];
        let actual = gamut.xyz_to_rgb(xyz);

        // Assert
        assert_approx_eq!(actual[0], expected.0, 1e-3);
        assert_approx_eq!(actual[1], expected.1, 1e-3);
        assert_approx_eq!(actual[2], expected.2, 1e-3);
    }
}

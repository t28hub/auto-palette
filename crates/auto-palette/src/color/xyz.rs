use std::fmt::Display;

use num_traits::clamp;
#[cfg(feature = "wasm")]
use serde::{Deserialize, Serialize};

use crate::color::{
    lab::Lab,
    luv::Luv,
    rgb::Rgb,
    white_point::{WhitePoint, D65},
    Oklab,
};

/// The CIE XYZ color representation.
///
/// See the following for more details:
/// [CIE 1931 color space - Wikipedia](https://en.wikipedia.org/wiki/CIE_1931_color_space)
///
/// # Fields
/// * `x` - The X component.
/// * `y` - The Y component.
/// * `z` - The Z component.
///
/// # Examples
/// ```
/// use auto_palette::color::{Lab, Oklab, Rgb, Xyz};
///
/// let rgb = Rgb::new(255, 0, 255);
/// let xyz = Xyz::from(&rgb);
/// assert_eq!(format!("{}", xyz), "XYZ(0.59, 0.28, 0.97)");
///
/// let lab: Lab = (&xyz).into();
/// assert_eq!(format!("{}", lab), "Lab(60.32, 98.24, -60.84)");
///
/// let oklab: Oklab = (&xyz).into();
/// assert_eq!(format!("{}", oklab), "Oklab(0.70, 0.27, -0.17)");
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "wasm", derive(Serialize, Deserialize))]
pub struct Xyz {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Xyz {
    /// Creates a new `Xyz` instance.
    ///
    /// # Arguments
    /// * `x` - The X component.
    /// * `y` - The Y component.
    /// * `z` - The Z component.
    ///
    /// # Returns
    /// A new `Xyz` instance.
    #[must_use]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x: clamp(x, Self::min_x(), Self::max_x()),
            y: clamp(y, Self::min_y(), Self::max_y()),
            z: clamp(z, Self::min_z(), Self::max_z()),
        }
    }

    /// Returns the minimum value of the X component.
    ///
    /// # Returns
    /// The minimum value of the X component.
    #[inline]
    #[must_use]
    pub(crate) fn min_x() -> f64 {
        0.0
    }

    /// Returns the maximum value of the X component.
    ///
    /// # Returns
    /// The maximum value of the X component.
    #[inline]
    #[must_use]
    pub(crate) fn max_x() -> f64 {
        f64::from(0.950_456_f32)
    }

    /// Returns the minimum value of the Y component.
    ///
    /// # Returns
    /// The minimum value of the Y component.
    #[inline]
    #[must_use]
    pub(crate) fn min_y() -> f64 {
        0.0
    }

    /// Returns the maximum value of the Y component.
    ///
    /// # Returns
    /// The maximum value of the Y component.
    #[inline]
    #[must_use]
    pub(crate) fn max_y() -> f64 {
        1.0
    }

    /// Returns the minimum value of the Z component.
    ///
    /// # Returns
    /// The minimum value of the Z component.
    #[inline]
    #[must_use]
    pub(crate) fn min_z() -> f64 {
        0.0
    }

    /// Returns the maximum value of the Z component.
    ///
    /// # Returns
    /// The maximum value of the Z component.
    #[inline]
    #[must_use]
    pub(crate) fn max_z() -> f64 {
        f64::from(1.088_644_f32)
    }
}

impl Display for Xyz {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "XYZ({:.2}, {:.2}, {:.2})", self.x, self.y, self.z)
    }
}

impl From<&Rgb> for Xyz {
    fn from(rgb: &Rgb) -> Self {
        let (x, y, z) = rgb_to_xyz(rgb.r, rgb.g, rgb.b);
        Self::new(x, y, z)
    }
}

impl From<&Lab> for Xyz {
    fn from(lab: &Lab) -> Self {
        let (x, y, z) = lab_to_xyz::<D65>(lab.l, lab.a, lab.b);
        Self::new(x, y, z)
    }
}

impl From<&Luv> for Xyz {
    fn from(luv: &Luv) -> Self {
        // This implementation is based on the algorithm described in the following link:
        // http://www.brucelindbloom.com/index.html?Eqn_XYZ_to_Luv.html
        if luv.l == 0.0 {
            return Xyz::new(0.0, 0.0, 0.0);
        }

        let denominator = D65::x() + 15.0 * D65::y() + 3.0 * D65::z();
        let u0 = 4.0 * D65::x() / denominator;
        let v0 = 9.0 * D65::y() / denominator;

        let y = if luv.l > 8.0 {
            ((luv.l + 16.0) / 116.0).powi(3)
        } else {
            luv.l / 903.296_296
        };

        let a = ((52.0 * luv.l) / (luv.u + 13.0 * luv.l * u0) - 1.0) * f64::from(1.0_f32 / 3.0_f32);
        let b = y * -5.0;
        let c = f64::from(-1.0_f32 / 3.0_f32);
        let d = y * ((39.0 * luv.l) / (luv.v + 13.0 * luv.l * v0) - 5.0);

        let x = (d - b) / (a - c);
        let z = x * a + b;
        Self::new(x, y, z)
    }
}

impl From<&Oklab> for Xyz {
    fn from(oklab: &Oklab) -> Self {
        // The inverse matrix of the conversion matrix M2 from LMS to L*a*b* is multiplied.
        let l_prime =
            0.999_999_998_5 * oklab.l + 0.396_337_792_1 * oklab.a + 0.215_803_758_1 * oklab.b;
        let m_prime =
            1.000_000_008_9 * oklab.l - 0.105_561_342_3 * oklab.a - 0.063_854_174_8 * oklab.b;
        let c_prime =
            1.000_000_054_7 * oklab.l - 0.089_484_182_1 * oklab.a - 1.291_485_537_8 * oklab.b;

        let l = l_prime.powi(3);
        let m = m_prime.powi(3);
        let c = c_prime.powi(3);

        // The inverse matrix of the conversion matrix M1 from XYZ to LMS is multiplied.
        let x = 1.227_013_851_1 * l + -0.557_799_980_7 * m + 0.281_256_149_0 * c;
        let y = -0.040_580_178_4 * l + 1.112_256_869_6 * m + -0.071_676_678_7 * c;
        let z = -0.076_381_284_5 * l + -0.421_481_978_4 * m + 1.586_163_220_4 * c;
        Self::new(x, y, z)
    }
}

/// Converts the RGB color space to the CIE XYZ color space.
///
/// # Arguments
/// * `r` - The red component of the RGB color.
/// * `g` - The green component of the RGB color.
/// * `b` - The blue component of the RGB color.
///
/// # Returns
/// The XYZ color space representation of the RGB color. The tuple contains the X, Y, and Z components.
#[inline(always)]
#[must_use]
pub(crate) fn rgb_to_xyz(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
    // This implementation is based on the algorithm described in the following link:
    // http://www.brucelindbloom.com/index.html?Eqn_RGB_to_XYZ.html
    let f = |t: f64| -> f64 {
        if t <= f64::from(0.04045_f32) {
            t / f64::from(12.92_f32)
        } else {
            ((t + f64::from(0.055_f32)) / f64::from(1.055_f32)).powf(f64::from(2.4_f32))
        }
    };

    let r = f(f64::from(r) / Rgb::max_value());
    let g = f(f64::from(g) / Rgb::max_value());
    let b = f(f64::from(b) / Rgb::max_value());

    let x =
        f64::from(0.412_391_f32) * r + f64::from(0.357_584_f32) * g + f64::from(0.180_481_f32) * b;
    let y =
        f64::from(0.212_639_f32) * r + f64::from(0.715_169_f32) * g + f64::from(0.072_192_f32) * b;
    let z =
        f64::from(0.019_331_f32) * r + f64::from(0.119_195_f32) * g + f64::from(0.950_532_f32) * b;

    (
        clamp(x, Xyz::min_x(), Xyz::max_x()),
        clamp(y, Xyz::min_y(), Xyz::max_y()),
        clamp(z, Xyz::min_z(), Xyz::max_z()),
    )
}

/// Converts the CIE L*a*b* color space to the CIE XYZ color space.
///
/// # Type Parameters
/// * `W` - The white point.
///
/// # Arguments
/// * `l` - The L component of the L*a*b* color.
/// * `a` - The a component of the L*a*b* color.
/// * `b` - The b component of the L*a*b* color.
///
/// # Returns
/// The XYZ color space representation of the L*a*b* color. The tuple contains the X, Y, and Z components.
#[inline(always)]
#[must_use]
pub(crate) fn lab_to_xyz<W>(l: f64, a: f64, b: f64) -> (f64, f64, f64)
where
    W: WhitePoint,
{
    let epsilon = 6.0 / 29.0;
    let kappa = 108.0 / 841.0; // 3.0 * ((6.0 / 29.0) ^ 2)
    let delta = 4.0 / 29.0;

    let f = |t: f64| -> f64 {
        if t > epsilon {
            t.powi(3)
        } else {
            kappa * (t - delta)
        }
    };

    let l2 = (l + 16.0) / 116.0;
    let fx = f(a / 500.0 + l2);
    let fy = f(l2);
    let fz = f(l2 - b / 200.0);

    let x = W::x() * fx;
    let y = W::y() * fy;
    let z = W::z() * fz;
    (
        clamp(x, Xyz::min_x(), Xyz::max_x()),
        clamp(y, Xyz::min_y(), Xyz::max_y()),
        clamp(z, Xyz::min_z(), Xyz::max_z()),
    )
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    #[cfg(feature = "wasm")]
    use serde_test::{assert_de_tokens, assert_ser_tokens, Token};

    use super::*;
    use crate::assert_approx_eq;

    #[test]
    fn test_new() {
        // Act
        let actual = Xyz::new(0.5928, 0.2848, 0.9699);

        // Assert
        assert_eq!(actual.x, 0.5928);
        assert_eq!(actual.y, 0.2848);
        assert_eq!(actual.z, 0.9699);
    }

    #[test]
    #[cfg(feature = "wasm")]
    fn test_serialize() {
        // Act
        let xyz = Xyz::new(0.5928, 0.2848, 0.9699);

        // Assert
        assert_ser_tokens(
            &xyz,
            &[
                Token::Struct {
                    name: "Xyz",
                    len: 3,
                },
                Token::Str("x"),
                Token::F64(0.5928),
                Token::Str("y"),
                Token::F64(0.2848),
                Token::Str("z"),
                Token::F64(0.9699),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    #[cfg(feature = "wasm")]
    fn test_deserialize() {
        // Act
        let xyz = Xyz::new(0.5928, 0.2848, 0.9699);

        // Assert
        assert_de_tokens(
            &xyz,
            &[
                Token::Struct {
                    name: "Xyz",
                    len: 3,
                },
                Token::Str("x"),
                Token::F64(0.5928),
                Token::Str("y"),
                Token::F64(0.2848),
                Token::Str("z"),
                Token::F64(0.9699),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_fmt() {
        // Act
        let xyz = Xyz::new(0.5928, 0.2848, 0.9699);
        let actual = format!("{}", xyz);

        // Assert
        assert_eq!(actual, "XYZ(0.59, 0.28, 0.97)");
    }

    #[test]
    fn test_from_rgb() {
        // Act
        let rgb = Rgb::new(255, 0, 255);
        let actual = Xyz::from(&rgb);

        // Assert
        assert_approx_eq!(actual.x, 0.592872);
        assert_approx_eq!(actual.y, 0.284831);
        assert_approx_eq!(actual.z, 0.969863);
    }

    #[test]
    fn test_from_lab() {
        // Act
        let lab = Lab::new(60.3199, 98.2302, -60.8496);
        let actual = Xyz::from(&lab);

        // Assert
        assert_approx_eq!(actual.x, 0.5928);
        assert_approx_eq!(actual.y, 0.2848);
        assert_approx_eq!(actual.z, 0.9699);
    }

    #[rstest]
    #[case::black((0.0, 0.0, 0.0), (0.0, 0.0, 0.0))]
    #[case::dark_gray((4.5, 0.0, 0.0), (0.005, 0.005, 0.005))]
    #[case::white((100.0, 0.0, 0.0), (0.950, 1.000, 1.089))]
    #[case::red((53.241, 175.015, 37.756), (0.412, 0.213, 0.019))]
    #[case::green((87.735, -83.078, 107.399), (0.358, 0.715, 0.119))]
    #[case::blue((32.297, -9.405, -130.342), (0.180, 0.072, 0.950))]
    #[case::yellow((97.139, 7.706, 106.787), (0.770, 0.928, 0.138))]
    #[case::cyan((91.113, -70.477, -15.202), (0.538, 0.787, 1.070))]
    #[case::magenta((60.324, 84.071, -108.683), (0.593, 0.285, 0.970))]
    fn test_from_luv(#[case] luv: (f64, f64, f64), #[case] expected: (f64, f64, f64)) {
        // Act
        let luv = Luv::new(luv.0, luv.1, luv.2);
        let actual = Xyz::from(&luv);

        // Assert
        assert_approx_eq!(actual.x, expected.0, 1e-3);
        assert_approx_eq!(actual.y, expected.1, 1e-3);
        assert_approx_eq!(actual.z, expected.2, 1e-3);
    }

    #[rstest]
    #[case::black((0.000, 0.0000, 0.0000), (0.000, 0.000, 0.000))]
    #[case::white((1.000, 0.0000, 0.0000), (0.950, 1.000, 1.089))]
    #[case::red((0.628, 0.225, 0.126), (0.412, 0.213, 0.019))]
    #[case::green((0.866, -0.234, 0.180), (0.357, 0.714, 0.117))]
    #[case::blue((0.452, -0.032, -0.312), (0.180, 0.072, 0.952))]
    #[case::yellow((0.968, -0.071, 0.199), (0.770, 0.928, 0.137))]
    #[case::cyan((0.905, -0.149, -0.039), (0.538, 0.786, 1.065))]
    #[case::magenta((0.702, 0.275, -0.169), (0.593, 0.285, 0.970))]
    fn test_from_oklab(#[case] oklab: (f64, f64, f64), #[case] expected: (f64, f64, f64)) {
        // Act
        let oklab = Oklab::new(oklab.0, oklab.1, oklab.2);
        let actual = Xyz::from(&oklab);

        // Assert
        assert_approx_eq!(actual.x, expected.0, 1e-3);
        assert_approx_eq!(actual.y, expected.1, 1e-3);
        assert_approx_eq!(actual.z, expected.2, 1e-3);
    }

    #[rstest]
    #[case::black((0, 0, 0), (0.0, 0.0, 0.0))]
    #[case::white((255, 255, 255), (0.9505, 1.0000, 1.0886))]
    #[case::red((255, 0, 0), (0.4125, 0.2127, 0.0193))]
    #[case::green((0, 255, 0), (0.3576, 0.7152, 0.1192))]
    #[case::blue((0, 0, 255), (0.1804, 0.0722, 0.9502))]
    #[case::cyan((0, 255, 255), (0.53802, 0.7873, 1.0698))]
    #[case::magenta((255, 0, 255), (0.5928, 0.2848, 0.9699))]
    #[case::yellow((255, 255, 0), (0.7700, 0.9278, 0.1385))]
    fn test_rgb_to_xyz(#[case] rgb: (u8, u8, u8), #[case] xyz: (f64, f64, f64)) {
        // Act
        let (x, y, z) = rgb_to_xyz(rgb.0, rgb.1, rgb.2);

        // Assert
        assert_approx_eq!(x, xyz.0, 1e-3);
        assert_approx_eq!(y, xyz.1, 1e-3);
        assert_approx_eq!(z, xyz.2, 1e-3);
    }

    #[rstest]
    #[case::black((0.0, 0.0, 0.0), (0.0, 0.0, 0.0))]
    #[case::white((100.0, 0.0052, 0.0141), (0.9505, 1.0000, 1.0886))]
    #[case::red((53.2437, 80.09315, 67.2388), (0.4125, 0.2127, 0.0193))]
    #[case::green((87.7376, - 86.1846, 83.1813), (0.3576, 0.7152, 0.1192))]
    #[case::blue((32.3026, 79.1436, - 107.8436), (0.1804, 0.0722, 0.9502))]
    #[case::cyan((91.1120, - 48.0806, - 14.1521), (0.53802, 0.7873, 1.0698))]
    #[case::magenta((60.3199, 98.2302, - 60.8496), (0.5928, 0.2848, 0.9699))]
    #[case::yellow((97.1382, - 21.5551, 94.4825), (0.7700, 0.9278, 0.1385))]
    fn test_lab_to_xyz(#[case] lab: (f64, f64, f64), #[case] xyz: (f64, f64, f64)) {
        // Act
        let (x, y, z) = lab_to_xyz::<D65>(lab.0, lab.1, lab.2);

        // Assert
        assert_approx_eq!(x, xyz.0, 1e-3);
        assert_approx_eq!(y, xyz.1, 1e-3);
        assert_approx_eq!(z, xyz.2, 1e-3);
    }
}

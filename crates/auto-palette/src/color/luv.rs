use std::fmt::Display;

use num_traits::clamp;
#[cfg(feature = "wasm")]
use serde::{Deserialize, Serialize};

use crate::color::{
    white_point::{WhitePoint, D65},
    Lchuv,
    Xyz,
};

/// The CIE L*u*v* color representation.
///
/// See the following for more details:
/// [CIELUV - Wikipedia](https://en.wikipedia.org/wiki/CIELUV)
///
/// # Fields
/// * `l` - The L* component.
/// * `u` - The u* component.
/// * `v` - The v* component.
///
/// # Examples
/// ```
/// use auto_palette::color::{Lchuv, Luv};
///
/// let luv = Luv::new(53.64, 165.87, 24.17);
/// assert_eq!(format!("{}", luv), "Luv(53.64, 165.87, 24.17)");
///
/// let lchuv: Lchuv = (&luv).into();
/// assert_eq!(format!("{}", lchuv), "LCH(uv)(53.64, 167.62, 8.29)");
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "wasm", derive(Serialize, Deserialize))]
pub struct Luv {
    pub l: f64,
    pub u: f64,
    pub v: f64,
}

impl Luv {
    /// Creates a new `Luv` instance.
    ///
    /// # Arguments
    /// * `l` - The L* component.
    /// * `u` - The u* component.
    /// * `v` - The v* component.
    ///
    /// # Returns
    /// A new `Luv` instance.
    #[must_use]
    pub fn new(l: f64, u: f64, v: f64) -> Self {
        // The u* component is theoretically unbounded, but in practice is rarely below -134 and rarely above 220.
        // The v* component is theoretically unbounded, but in practice is rarely below -140 and rarely above 122.
        // The values are referenced from the following link:
        // https://docs.opencv.org/2.4/modules/imgproc/doc/miscellaneous_transformations.html?highlight=luv#cvtcolor
        Self {
            l: clamp(l, 0.0, 100.0),
            // The u* component is theoretically unbounded, but in practice is rarely below -134 and rarely above 220.
            u: clamp(u, -134.0, 220.0),
            // The v* component is theoretically unbounded, but in practice is rarely below -140 and rarely above 122.
            v: clamp(v, -140.0, 122.0),
        }
    }
}

impl Display for Luv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Luv({:.2}, {:.2}, {:.2})", self.l, self.u, self.v)
    }
}

impl From<&Xyz> for Luv {
    fn from(xyz: &Xyz) -> Self {
        // This implementation is based on the formulae from the following sources:
        // http://www.brucelindbloom.com/index.html?Eqn_XYZ_to_Luv.html
        let y = xyz.y / D65::y();
        let cie_k = 903.296_296; // 24389 / 27
        let cie_e = 0.008_856; // 216 / 24389

        let l = if y > cie_e {
            116.0 * y.powf(f64::from(1.0_f32 / 3.0_f32)) - 16.0
        } else {
            cie_k * y
        };

        let denominator = xyz.x + 15.0 * xyz.y + 3.0 * xyz.z;
        let u_prime = if denominator == 0.0 {
            0.0
        } else {
            4.0 * xyz.x / denominator
        };
        let v_prime = if denominator == 0.0 {
            0.0
        } else {
            9.0 * xyz.y / denominator
        };

        let denominator = D65::x() + 15.0 * D65::y() + 3.0 * D65::z();
        let u_prime_ref = 4.0 * D65::x() / denominator;
        let v_prime_ref = 9.0 * D65::y() / denominator;

        let u = 13.0 * l * (u_prime - u_prime_ref);
        let v = 13.0 * l * (v_prime - v_prime_ref);
        Luv::new(l, u, v)
    }
}

impl From<&Lchuv> for Luv {
    fn from(lch: &Lchuv) -> Self {
        // This implementation is based on the formulae from the following sources:
        // http://www.brucelindbloom.com/index.html?Eqn_LCH_to_Luv.html
        let l = lch.l;
        let c = lch.c;
        let h = lch.h.to_radians();
        let u = c * h.cos();
        let v = c * h.sin();
        Luv::new(l, u, v)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    #[cfg(feature = "wasm")]
    use serde_test::{assert_de_tokens, assert_ser_tokens, Token};

    use super::*;
    use crate::{assert_approx_eq, color::Rgb};

    #[test]
    fn test_new() {
        // Act
        let actual = Luv::new(53.64, 165.86, 24.17);

        // Assert
        assert_eq!(
            actual,
            Luv {
                l: 53.64,
                u: 165.86,
                v: 24.17,
            }
        );
    }

    #[rstest]
    #[case((100.1, 220.1, 122.1), (100.0, 220.0, 122.0))]
    #[case((-0.1, -134.1, -140.1), (0.0, -134.0, -140.0))]
    fn test_new_with_out_of_bounds_values(
        #[case] input: (f64, f64, f64),
        #[case] expected: (f64, f64, f64),
    ) {
        // Act
        let actual = Luv::new(input.0, input.1, input.2);

        // Assert
        assert_eq!(actual.l, expected.0);
        assert_eq!(actual.u, expected.1);
        assert_eq!(actual.v, expected.2);
    }

    #[test]
    #[cfg(feature = "wasm")]
    fn test_serialize() {
        // Act
        let luv = Luv::new(53.64, 165.86, 24.17);

        // Assert
        assert_ser_tokens(
            &luv,
            &[
                Token::Struct {
                    name: "Luv",
                    len: 3,
                },
                Token::Str("l"),
                Token::F64(53.64),
                Token::Str("u"),
                Token::F64(165.86),
                Token::Str("v"),
                Token::F64(24.17),
                Token::StructEnd,
            ],
        )
    }

    #[test]
    #[cfg(feature = "wasm")]
    fn test_deserialize() {
        // Act
        let luv = Luv::new(66.48, 36.71, 44.73);

        // Assert
        assert_de_tokens(
            &luv,
            &[
                Token::Struct {
                    name: "Luv",
                    len: 3,
                },
                Token::Str("l"),
                Token::F64(66.48),
                Token::Str("u"),
                Token::F64(36.71),
                Token::Str("v"),
                Token::F64(44.73),
                Token::StructEnd,
            ],
        )
    }

    #[test]
    fn test_fmt() {
        // Act
        let luv = Luv::new(53.64, 165.86, 24.17);
        let actual = format!("{}", luv);

        // Assert
        assert_eq!(actual, "Luv(53.64, 165.86, 24.17)");
    }

    #[rstest]
    #[case::black((0, 0, 0), (0.0, 0.0, 0.0))]
    #[case::white((255, 255, 255), (100.0, 0.003, 0.018))]
    #[case::red((255, 0, 0), (53.237, 175.003, 37.753))]
    #[case::green((0, 255, 0), (87.735, -83.078, 107.40))]
    #[case::blue((0, 0, 255), (32.300, -9.406, -130.357))]
    #[case::yellow((255, 255, 0), (97.138, 7.691, 106.787))]
    #[case::cyan((0, 255, 255), (91.114, -70.476, -15.224))]
    #[case::magenta((255, 0, 255), (60.322, 84.048, -108.71))]
    fn test_from_xyz(#[case] rgb: (u8, u8, u8), #[case] expected: (f64, f64, f64)) {
        // Act
        let rgb = Rgb::new(rgb.0, rgb.1, rgb.2);
        let xyz = Xyz::from(&rgb);
        let actual = Luv::from(&xyz);

        // Assert
        assert_approx_eq!(actual.l, expected.0, 1e-3);
        assert_approx_eq!(actual.u, expected.1, 1e-3);
        assert_approx_eq!(actual.v, expected.2, 1e-3);
    }

    #[test]
    fn test_from_lchuv() {
        // Act
        let lchuv = Lchuv::new(53.640, 167.622, 8.291);
        let actual = Luv::from(&lchuv);

        // Assert
        assert_approx_eq!(actual.l, 53.640);
        assert_approx_eq!(actual.u, 165.870091);
        assert_approx_eq!(actual.v, 24.171221);
    }
}

use std::fmt::Display;

use num_traits::clamp;
#[cfg(feature = "wasm")]
use serde::{Deserialize, Serialize};

use crate::{
    color::{white_point::WhitePoint, Lchab, Xyz, D65},
    math::{denormalize, normalize},
};

/// The CIE L*a*b* color representation.
///
/// See the following for more details:
/// [CIELAB color space - Wikipedia](https://en.wikipedia.org/wiki/CIELAB_color_space)
///
/// # Fields
/// * `l` - The L component.
/// * `a` - The a component.
/// * `b` - The b component.
///
/// # Examples
/// ```
/// use auto_palette::color::{Lab, Lchab, Xyz};
///
/// let xyz = Xyz::new(0.3576, 0.7152, 0.1192);
/// let lab = Lab::from(&xyz);
/// assert_eq!(format!("{}", lab), "Lab(87.74, -86.18, 83.18)");
///
/// let lchab: Lchab = (&lab).into();
/// assert_eq!(format!("{}", lchab), "LCH(ab)(87.74, 119.78, 136.02)");
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "wasm", derive(Serialize, Deserialize))]
pub struct Lab {
    pub l: f64,
    pub a: f64,
    pub b: f64,
}

impl Lab {
    /// Creates a new `Lab` instance.
    ///
    /// # Arguments
    /// * `l` - The L component.
    /// * `a` - The a component.
    /// * `b` - The b component.
    ///
    /// # Returns
    /// A new `Lab` instance.
    #[must_use]
    pub fn new(l: f64, a: f64, b: f64) -> Self {
        Self {
            l: clamp(l, Lab::min_l(), Lab::max_l()),
            a: clamp(a, Lab::min_a(), Lab::max_a()),
            b: clamp(b, Lab::min_b(), Lab::max_b()),
        }
    }

    /// Returns the minimum value of the L component.
    ///
    /// # Returns
    /// The minimum value of the L component.
    #[inline]
    #[must_use]
    pub(crate) fn min_l() -> f64 {
        0.0
    }

    /// Returns the maximum value of the L component.
    ///
    /// # Returns
    /// The maximum value of the L component.
    #[inline]
    #[must_use]
    pub(crate) fn max_l() -> f64 {
        100.0
    }

    /// Returns the minimum value of the a component.
    ///
    /// # Returns
    /// The minimum value of the a component.
    #[inline]
    #[must_use]
    pub(crate) fn min_a() -> f64 {
        -128.0
    }

    /// Returns the maximum value of the a component.
    ///
    /// # Returns
    /// The maximum value of the a component.
    #[inline]
    #[must_use]
    pub(crate) fn max_a() -> f64 {
        127.0
    }

    /// Returns the minimum value of the b component.
    ///
    /// # Returns
    /// The minimum value of the b component.
    #[inline]
    #[must_use]
    pub(crate) fn min_b() -> f64 {
        -128.0
    }

    /// Returns the maximum value of the b component.
    ///
    /// # Returns
    /// The maximum value of the b component.
    #[inline]
    #[must_use]
    pub(crate) fn max_b() -> f64 {
        127.0
    }

    /// Normalizes the L component to the range of [0, 1].
    ///
    /// # Arguments
    /// * `l` - The L component to normalize.
    ///
    /// # Returns
    /// The normalized L component in the range of [0, 1].
    #[inline(always)]
    #[must_use]
    pub(crate) fn normalize_l(l: f64) -> f64 {
        normalize(l, Lab::min_l(), Lab::max_l())
    }

    /// Denormalizes the L component from the range of [0, 1] to the original range.
    ///
    /// # Arguments
    /// * `l` - The normalized L component to denormalize.
    ///
    /// # Returns
    /// The denormalized L component in the original range.
    #[inline(always)]
    #[must_use]
    pub(crate) fn denormalize_l(l: f64) -> f64 {
        denormalize(l, Lab::min_l(), Lab::max_l())
    }

    /// Normalizes the a component to the range of [0, 1].
    ///
    /// # Arguments
    /// * `a` - The a component to normalize.
    ///
    /// # Returns
    /// The normalized a component in the range of [0, 1].
    #[inline(always)]
    #[must_use]
    pub(crate) fn normalize_a(a: f64) -> f64 {
        normalize(a, Lab::min_a(), Lab::max_a())
    }

    /// Denormalizes the a component from the range of [0, 1] to the original range.
    ///
    /// # Arguments
    /// * `a` - The normalized a component to denormalize.
    ///
    /// # Returns
    /// The denormalized a component in the original range.
    #[inline(always)]
    #[must_use]
    pub(crate) fn denormalize_a(a: f64) -> f64 {
        denormalize(a, Lab::min_a(), Lab::max_a())
    }

    /// Normalizes the b component to the range of [0, 1].
    ///
    /// # Arguments
    /// * `b` - The b component to normalize.
    ///
    /// # Returns
    /// The normalized b component in the range of [0, 1].
    #[inline(always)]
    #[must_use]
    pub(crate) fn normalize_b(b: f64) -> f64 {
        normalize(b, Lab::min_b(), Lab::max_b())
    }

    /// Denormalizes the b component from the range of [0, 1] to the original range.
    ///
    /// # Arguments
    /// * `b` - The normalized b component to denormalize.
    ///
    /// # Returns
    /// The denormalized b component in the original range.
    #[inline(always)]
    #[must_use]
    pub(crate) fn denormalize_b(b: f64) -> f64 {
        denormalize(b, Lab::min_b(), Lab::max_b())
    }
}

impl Display for Lab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lab({:.2}, {:.2}, {:.2})", self.l, self.a, self.b)
    }
}

impl From<&Xyz> for Lab {
    fn from(xyz: &Xyz) -> Self {
        let (l, a, b) = xyz_to_lab::<D65>(xyz.x, xyz.y, xyz.z);
        Lab::new(l, a, b)
    }
}

impl From<&Lchab> for Lab {
    fn from(lch: &Lchab) -> Self {
        // This implementation is based on the formulae from the following sources:
        // http://www.brucelindbloom.com/index.html?Eqn_Lab_to_LCH.html
        let l = lch.l;
        let c = lch.c;
        let h = lch.h.to_radians();
        let a = c * h.cos();
        let b = c * h.sin();
        Lab::new(l, a, b)
    }
}

/// Converts the CIE XYZ color space to the CIE L*a*b* color space.
///
/// # Type Parameters
/// * `W` - The white point type.
///
/// # Arguments
/// * `x` - The X component of the XYZ color.
/// * `y` - The Y component of the XYZ color.
/// * `z` - The Z component of the XYZ color.
///
/// # Returns
/// The L*a*b* color space representation of the XYZ color. The tuple contains the L, a, and b components.
#[inline(always)]
#[must_use]
pub(crate) fn xyz_to_lab<W>(x: f64, y: f64, z: f64) -> (f64, f64, f64)
where
    W: WhitePoint,
{
    let epsilon = (6.0_f64 / 29.0).powi(3);
    let kappa = 841.0 / 108.0; // ((29.0 / 6.0) ^ 2) / 3.0
    let delta = 4.0 / 29.0;

    let f = |t: f64| -> f64 {
        if t > epsilon {
            t.cbrt()
        } else {
            kappa * t + delta
        }
    };

    let fx = f(x / W::x());
    let fy = f(y / W::y());
    let fz = f(z / W::z());

    let l = 116.0 * fy - 16.0;
    let a = 500.0 * (fx - fy);
    let b = 200.0 * (fy - fz);
    (
        clamp(l, Lab::min_l(), Lab::max_l()),
        clamp(a, Lab::min_a(), Lab::max_a()),
        clamp(b, Lab::min_b(), Lab::max_b()),
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
        let actual = Lab::new(53.2437, 80.09315, 67.2388);

        // Assert
        assert_eq!(actual.l, 53.2437);
        assert_eq!(actual.a, 80.09315);
        assert_eq!(actual.b, 67.2388);
    }

    #[test]
    #[cfg(feature = "wasm")]
    fn test_serialize() {
        // Act
        let lab = Lab::new(53.2437, 80.09315, 67.2388);

        // Assert
        assert_ser_tokens(
            &lab,
            &[
                Token::Struct {
                    name: "Lab",
                    len: 3,
                },
                Token::Str("l"),
                Token::F64(53.2437),
                Token::Str("a"),
                Token::F64(80.09315),
                Token::Str("b"),
                Token::F64(67.2388),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    #[cfg(feature = "wasm")]
    fn test_deserialize() {
        // Act
        let lab = Lab::new(66.48, -29.16, 20.62);

        // Assert
        assert_de_tokens(
            &lab,
            &[
                Token::Struct {
                    name: "Lab",
                    len: 3,
                },
                Token::Str("l"),
                Token::F64(66.48),
                Token::Str("a"),
                Token::F64(-29.16),
                Token::Str("b"),
                Token::F64(20.62),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_fmt() {
        // Act
        let lab = Lab::new(53.2437, 80.09315, 67.2388);
        let actual = format!("{}", lab);

        // Assert
        assert_eq!(actual, "Lab(53.24, 80.09, 67.24)");
    }

    #[test]
    fn test_from_xyz() {
        // Act
        let xyz = Xyz::new(0.3576, 0.7152, 0.1192);
        let actual = Lab::from(&xyz);

        // Assert
        assert_approx_eq!(actual.l, 87.737033);
        assert_approx_eq!(actual.a, -86.184636);
        assert_approx_eq!(actual.b, 83.181164);
    }

    #[test]
    fn test_from_lchab() {
        // Act
        let lchab = Lchab::new(54.617, 92.151, 27.756);
        let actual = Lab::from(&lchab);

        // Assert
        assert_approx_eq!(actual.l, 54.617);
        assert_approx_eq!(actual.a, 81.548002);
        assert_approx_eq!(actual.b, 42.915383);
    }

    #[rstest]
    #[case::black((0.0, 0.0, 0.0), (0.0, 0.0, 0.0))]
    #[case::white((0.9505, 1.0000, 1.0886), (100.0, 0.0052, 0.0141))]
    #[case::red((0.4125, 0.2127, 0.0193), (53.2437, 80.09315, 67.2388))]
    #[case::green((0.3576, 0.7152, 0.1192), (87.7376, - 86.1846, 83.1813))]
    #[case::blue((0.1804, 0.0722, 0.9502), (32.3026, 79.1436, - 107.8436))]
    #[case::cyan((0.53802, 0.7873, 1.0698), (91.1120, - 48.0806, - 14.1521))]
    #[case::magenta((0.5928, 0.2848, 0.9699), (60.3199, 98.2302, - 60.8496))]
    #[case::yellow((0.7700, 0.9278, 0.1385), (97.1382, - 21.5551, 94.4825))]
    fn test_xyz_to_lab(#[case] xyz: (f64, f64, f64), #[case] lab: (f64, f64, f64)) {
        // Act
        let (l, a, b) = xyz_to_lab::<D65>(xyz.0, xyz.1, xyz.2);

        // Assert
        assert_approx_eq!(l, lab.0, 1e-3);
        assert_approx_eq!(a, lab.1, 1e-3);
        assert_approx_eq!(b, lab.2, 1e-3);
    }
}

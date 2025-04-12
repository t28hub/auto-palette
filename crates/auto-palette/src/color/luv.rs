use std::{fmt::Display, marker::PhantomData};

use num_traits::clamp;
#[cfg(feature = "wasm")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use tsify::Tsify;

use crate::{
    color::{white_point::WhitePoint, LCHuv, D65, XYZ},
    math::FloatNumber,
};

/// The CIE L*u*v* color representation.
///
/// See the following for more details:
/// [CIELUV - Wikipedia](https://en.wikipedia.org/wiki/CIELUV)
///
/// # Type Parameters
/// * `T` - The floating point type.
/// * `W` - The white point type.
///
/// # Fields
/// * `l` - The L* component.
/// * `u` - The u* component.
/// * `v` - The v* component.
///
/// # Examples
/// ```
/// use auto_palette::color::{LCHuv, Luv, D65};
///
/// let luv: Luv<_> = Luv::new(53.64, 165.87, 24.17);
/// assert_eq!(format!("{}", luv), "Luv(53.64, 165.87, 24.17)");
///
/// let lchuv: LCHuv<_> = (&luv).into();
/// assert_eq!(format!("{}", lchuv), "LCH(uv)(53.64, 167.62, 8.29)");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(Serialize, Deserialize, Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub struct Luv<T = f64, W = D65>
where
    T: FloatNumber,
    W: WhitePoint,
{
    #[cfg_attr(feature = "wasm", tsify(type = "number"))]
    pub l: T,
    #[cfg_attr(feature = "wasm", tsify(type = "number"))]
    pub u: T,
    #[cfg_attr(feature = "wasm", tsify(type = "number"))]
    pub v: T,
    #[cfg_attr(feature = "wasm", serde(skip))]
    _marker: PhantomData<W>,
}

impl<T, W> Luv<T, W>
where
    T: FloatNumber,
    W: WhitePoint,
{
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
    pub fn new(l: T, u: T, v: T) -> Self {
        // The u* component is theoretically unbounded, but in practice is rarely below -134 and rarely above 220.
        // The v* component is theoretically unbounded, but in practice is rarely below -140 and rarely above 122.
        // The values are referenced from the following link:
        // https://docs.opencv.org/2.4/modules/imgproc/doc/miscellaneous_transformations.html?highlight=luv#cvtcolor
        Self {
            l: clamp(l, T::zero(), T::from_f32(100.0)),
            // The u* component is theoretically unbounded, but in practice is rarely below -134 and rarely above 220.
            u: clamp(u, T::from_f32(-134.0), T::from_f32(220.0)),
            // The v* component is theoretically unbounded, but in practice is rarely below -140 and rarely above 122.
            v: clamp(v, T::from_f32(-140.0), T::from_f32(122.0)),
            _marker: PhantomData,
        }
    }
}

impl<T, W> Display for Luv<T, W>
where
    T: FloatNumber,
    W: WhitePoint,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Luv({:.2}, {:.2}, {:.2})", self.l, self.u, self.v)
    }
}

impl<T, W> From<&XYZ<T>> for Luv<T, W>
where
    T: FloatNumber,
    W: WhitePoint,
{
    fn from(xyz: &XYZ<T>) -> Self {
        // This implementation is based on the formulae from the following sources:
        // http://www.brucelindbloom.com/index.html?Eqn_XYZ_to_Luv.html
        let y = xyz.y / W::y();
        let cie_k = T::from_f64(903.296_296); // 24389 / 27
        let cie_e = T::from_f64(0.008_856); // 216 / 24389

        let l = if y > cie_e {
            T::from_u8(116) * y.powf(T::from_f32(1.0 / 3.0)) - T::from_u8(16)
        } else {
            cie_k * y
        };

        let denominator = xyz.x + T::from_f32(15.0) * xyz.y + T::from_f32(3.0) * xyz.z;
        let u_prime = if denominator.is_zero() {
            T::zero()
        } else {
            T::from_f32(4.0) * xyz.x / denominator
        };
        let v_prime = if denominator.is_zero() {
            T::zero()
        } else {
            T::from_f32(9.0) * xyz.y / denominator
        };

        let denominator =
            W::x::<T>() + T::from_f32(15.0) * W::y::<T>() + T::from_f32(3.0) * W::z::<T>();
        let u_prime_ref = T::from_f32(4.0) * W::x() / denominator;
        let v_prime_ref = T::from_f32(9.0) * W::y() / denominator;

        let u = T::from_f32(13.0) * l * (u_prime - u_prime_ref);
        let v = T::from_f32(13.0) * l * (v_prime - v_prime_ref);
        Luv::new(l, u, v)
    }
}

impl<T, W> From<&LCHuv<T, W>> for Luv<T, W>
where
    T: FloatNumber,
    W: WhitePoint,
{
    fn from(lch: &LCHuv<T, W>) -> Self {
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
    #[cfg(feature = "wasm")]
    use indoc::indoc;
    use rstest::rstest;
    #[cfg(feature = "wasm")]
    use serde_test::{assert_de_tokens, assert_ser_tokens, Token};

    use super::*;
    use crate::{assert_approx_eq, color::RGB};

    #[test]
    fn test_new() {
        // Act
        let actual = Luv::<_>::new(53.64, 165.86, 24.17);

        // Assert
        assert_eq!(
            actual,
            Luv {
                l: 53.64,
                u: 165.86,
                v: 24.17,
                _marker: PhantomData,
            }
        );
    }

    #[rstest]
    #[case((100.1, 220.1, 122.1), (100.0, 220.0, 122.0))]
    #[case((-0.1, -134.1, -140.1), (0.0, -134.0, -140.0))]
    fn test_new_with_out_of_bounds_values(
        #[case] input: (f32, f32, f32),
        #[case] expected: (f32, f32, f32),
    ) {
        // Act
        let actual = Luv::<_>::new(input.0, input.1, input.2);

        // Assert
        assert_eq!(actual.l, expected.0);
        assert_eq!(actual.u, expected.1);
        assert_eq!(actual.v, expected.2);
    }

    #[test]
    #[cfg(feature = "wasm")]
    fn test_serialize() {
        // Act
        let luv = Luv::<_>::new(53.64, 165.86, 24.17);

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
        let luv = Luv::<_>::new(66.48, 36.71, 44.73);

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
    #[cfg(feature = "wasm")]
    fn test_tsify() {
        // Act & Assert
        let expected = indoc! {
            // language=typescript
            "export interface Luv<T> {
                l: number;
                u: number;
                v: number;
            }"
        };
        assert_eq!(Luv::<f64>::DECL, expected);
    }

    #[test]
    fn test_fmt() {
        // Act
        let luv = Luv::<_, D65>::new(53.64, 165.86, 24.17);
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
    fn test_from_xyz(#[case] rgb: (u8, u8, u8), #[case] expected: (f32, f32, f32)) {
        // Act
        let rgb = RGB::new(rgb.0, rgb.1, rgb.2);
        let xyz = XYZ::<f32>::from(&rgb);
        let actual = Luv::<_, D65>::from(&xyz);

        // Assert
        assert_approx_eq!(actual.l, expected.0, 1e-3);
        assert_approx_eq!(actual.u, expected.1, 1e-3);
        assert_approx_eq!(actual.v, expected.2, 1e-3);
    }

    #[test]
    fn test_from_lchuv() {
        // Act
        let lchuv = LCHuv::<f32>::new(53.640, 167.622, 8.291);
        let actual = Luv::from(&lchuv);

        // Assert
        assert_approx_eq!(actual.l, 53.640);
        assert_approx_eq!(actual.u, 165.870086);
        assert_approx_eq!(actual.v, 24.171220);
    }
}

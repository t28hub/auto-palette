use num_traits::clamp;
#[cfg(feature = "wasm")]
use serde::{Deserialize, Serialize};

use crate::{
    color::{oklch::Oklch, XYZ},
    math::FloatNumber,
};

/// The Oklab color representation.
///
/// See the following for more details:
/// [Oklab - A perceptual color space for image processing](https://bottosson.github.io/posts/oklab/)
///
/// # Type Parameters
/// * `T` - The floating point type.
///
/// # Fields
/// * `l` - The lightness component.
/// * `a` - The a component.
/// * `b` - The b component.
///
/// # Examples
/// ```
/// use auto_palette::color::{Oklab, XYZ};
///
/// let oklab: Oklab<f32> = Oklab::new(0.607, -0.118, 0.028);
/// assert_eq!(format!("{}", oklab), "Oklab(0.61, -0.12, 0.03)");
///
/// let xyz: XYZ<_> = (&oklab).into();
/// assert_eq!(format!("{}", xyz), "XYZ(0.15, 0.24, 0.20)");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(Serialize, Deserialize))]
pub struct Oklab<T>
where
    T: FloatNumber,
{
    pub l: T,
    pub a: T,
    pub b: T,
}

impl<T> Oklab<T>
where
    T: FloatNumber,
{
    /// Creates a new `Oklab` instance.
    ///
    /// # Arguments
    /// * `l` - The lightness component.
    /// * `a` - The a component.
    /// * `b` - The b component.
    ///
    /// # Returns
    /// A new `Oklab` instance.
    pub fn new(l: T, a: T, b: T) -> Self {
        Self {
            l: clamp(l, T::zero(), T::one()),
            a,
            b,
        }
    }
}

impl<T> std::fmt::Display for Oklab<T>
where
    T: FloatNumber,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Oklab({:.2}, {:.2}, {:.2})", self.l, self.a, self.b)
    }
}

impl<T> From<&XYZ<T>> for Oklab<T>
where
    T: FloatNumber,
{
    fn from(xyz: &XYZ<T>) -> Self {
        // This implementation is based on the formulae from the following sources:
        // https://bottosson.github.io/posts/oklab/#implementation
        let l = T::from_f64(0.818_933_010_1) * xyz.x + T::from_f64(0.361_866_742_4) * xyz.y
            - T::from_f64(0.128_859_713_7) * xyz.z;
        let m = T::from_f64(0.032_984_543_6) * xyz.x
            + T::from_f64(0.929_311_871_5) * xyz.y
            + T::from_f64(0.036_145_638_7) * xyz.z;
        let s = T::from_f64(0.048_200_301_8) * xyz.x
            + T::from_f64(0.264_366_269_1) * xyz.y
            + T::from_f64(0.633_851_707_0) * xyz.z;

        let l_prime = l.cbrt();
        let m_prime = m.cbrt();
        let s_prime = s.cbrt();

        let l = T::from_f64(0.210_454_255_3) * l_prime + T::from_f64(0.793_617_785_0) * m_prime
            - T::from_f64(0.004_072_046_8) * s_prime;
        let a = T::from_f64(1.977_998_495_1) * l_prime - T::from_f64(2.428_592_205_0) * m_prime
            + T::from_f64(0.450_593_709_9) * s_prime;
        let b = T::from_f64(0.025_904_037_1) * l_prime + T::from_f64(0.782_771_766_2) * m_prime
            - T::from_f64(0.808_675_766_0) * s_prime;
        Self::new(l, a, b)
    }
}

impl<T> From<&Oklch<T>> for Oklab<T>
where
    T: FloatNumber,
{
    fn from(oklch: &Oklch<T>) -> Self {
        let l = oklch.l;
        let c = oklch.c;
        let h = oklch.h.to_radians();
        let a = c * h.cos();
        let b = c * h.sin();
        Self::new(l, a, b)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use serde_test::{assert_tokens, Token};

    use super::*;

    #[test]
    fn test_new() {
        // Act
        let actual: Oklab<f32> = Oklab::new(0.607, -0.118, 0.028);

        // Assert
        assert_eq!(
            actual,
            Oklab {
                l: 0.607,
                a: -0.118,
                b: 0.028
            }
        );
    }

    #[test]
    #[cfg(feature = "wasm")]
    fn test_serialize() {
        // Act
        let oklab = Oklab::new(0.607, -0.118, 0.028);

        // Assert
        assert_tokens(
            &oklab,
            &[
                Token::Struct {
                    name: "Oklab",
                    len: 3,
                },
                Token::Str("l"),
                Token::F64(0.607),
                Token::Str("a"),
                Token::F64(-0.118),
                Token::Str("b"),
                Token::F64(0.028),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_fmt() {
        // Act
        let oklab: Oklab<f32> = Oklab::new(0.607, -0.118, 0.028);
        let actual = format!("{}", oklab);

        // Assert
        assert_eq!(actual, "Oklab(0.61, -0.12, 0.03)");
    }

    #[rstest]
    #[case((0.147, 0.241, 0.198), (0.607, -0.118, 0.028))]
    #[case((0.950, 1.000, 1.089), (1.000, -0.000, 0.000))]
    #[case((1.000, 0.000, 0.000), (0.442, 1.215, -0.019))]
    #[case((0.000, 1.000, 0.000), (0.922, -0.671, 0.263))]
    #[case((0.000, 0.000, 1.000), (0.153, -1.415, -0.449))]
    fn test_from_xyz(#[case] xyz: (f32, f32, f32), #[case] expected: (f32, f32, f32)) {
        // Act
        let xyz: XYZ<f32> = XYZ::new(xyz.0, xyz.1, xyz.2);
        let actual = Oklab::from(&xyz);

        // Assert
        assert!((actual.l - expected.0).abs() < 1e-3);
        assert!((actual.a - expected.1).abs() < 1e-3);
        assert!((actual.b - expected.2).abs() < 1e-3);
    }

    #[test]
    fn test_from_oklch() {
        // Act
        let oklch: Oklch<f64> = Oklch::new(0.607, 0.121, 166.651);
        let actual = Oklab::from(&oklch);

        // Assert
        assert_eq!(actual.l, 0.607);
        assert!((actual.a + 0.117).abs() < 1e-3);
        assert!((actual.b - 0.028).abs() < 1e-3);
    }
}

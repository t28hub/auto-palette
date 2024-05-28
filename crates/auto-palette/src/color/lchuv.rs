use std::{fmt::Display, marker::PhantomData};

use num_traits::clamp;

use crate::{
    color::{Hue, Luv, WhitePoint, D65},
    math::FloatNumber,
};

/// The CIE L*u*v* color representation.
///
/// See the following for more details:
/// [CIE LUV | Cylindrical representation (CIELCh)](https://en.wikipedia.org/wiki/CIELUV#Cylindrical_representation_(CIELCh))
///
/// # Type Parameters
/// * `T` - The floating point type.
/// * `W` - The white point type.
///
/// # Fields
/// * `l` - The lightness component.
/// * `c` - The chroma component.
/// * `h` - The hue component.
///
/// # Examples
/// ```
/// use auto_palette::color::{LCHuv, Luv, D65};
///
/// let lchuv: LCHuv<_> = LCHuv::new(56.232, 50.875, 154.710);
/// assert_eq!(format!("{}", lchuv), "LCH(uv)(56.23, 50.88, 154.71)");
///
/// let luv: Luv<_> = (&lchuv).into();
/// assert_eq!(format!("{}", luv), "Luv(56.23, -46.00, 21.73)");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LCHuv<T, W = D65>
where
    T: FloatNumber,
    W: WhitePoint,
{
    pub l: T,
    pub c: T,
    pub h: Hue<T>,
    _marker: PhantomData<W>,
}

impl<T, W> LCHuv<T, W>
where
    T: FloatNumber,
    W: WhitePoint,
{
    /// Creates a new `LCHuv` instance.
    ///
    /// # Arguments
    /// * `l` - The lightness component.
    /// * `c` - The chroma component.
    /// * `h` - The hue component.
    ///
    /// # Returns
    /// A new `LCHuv` instance.
    #[must_use]
    pub fn new(l: T, c: T, h: T) -> Self {
        Self {
            l: clamp(l, T::zero(), T::from_u32(100)),
            c: clamp(c, T::zero(), T::from_u32(180)),
            h: Hue::from_degrees(h),
            _marker: PhantomData,
        }
    }
}

impl<T, W> Display for LCHuv<T, W>
where
    T: FloatNumber,
    W: WhitePoint,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LCH(uv)({:.2}, {:.2}, {:.2})",
            self.l,
            self.c,
            self.h.to_degrees()
        )
    }
}

impl<T, W> From<&Luv<T, W>> for LCHuv<T, W>
where
    T: FloatNumber,
    W: WhitePoint,
{
    fn from(luv: &Luv<T, W>) -> Self {
        // This implementation is based on the formulae from the following sources:
        // http://www.brucelindbloom.com/index.html?Eqn_Luv_to_LCH.html
        let l = luv.l;
        let c = (luv.u * luv.u + luv.v * luv.v).sqrt();
        let h = luv.v.atan2(luv.u).to_degrees();
        Self::new(l, c, h)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Luv;

    #[test]
    fn test_new() {
        // Act
        let actual: LCHuv<_> = LCHuv::new(56.232, 50.875, 154.710);

        // Assert
        assert_eq!(
            actual,
            LCHuv {
                l: 56.232,
                c: 50.875,
                h: Hue::from_degrees(154.710),
                _marker: PhantomData,
            }
        );
    }

    #[test]
    fn test_fmt() {
        // Act
        let lchuv: LCHuv<_> = LCHuv::new(56.232, 50.875, 154.710);
        let actual = format!("{}", lchuv);

        // Assert
        assert_eq!(actual, "LCH(uv)(56.23, 50.88, 154.71)");
    }

    #[test]
    fn test_from_luv() {
        // Act
        let luv: Luv<f32> = Luv::new(56.232, -45.999, 21.734);
        let actual = LCHuv::from(&luv);

        // Assert
        assert_eq!(actual.l, 56.232);
        assert!((actual.c - 50.875).abs() < 1e-3);
        assert!((actual.h.to_degrees() - 154.710).abs() < 1e-3);
    }
}

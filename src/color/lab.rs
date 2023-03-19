use crate::color::white_point::{WhitePoint, D65};
use crate::color::xyz::XYZ;
use crate::math::number::Float;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

/// Color in CIE L*a*b* color space.
#[derive(Debug, Clone, PartialEq)]
pub struct Lab<F: Float, W: WhitePoint<F> = D65> {
    pub l: F,
    pub a: F,
    pub b: F,
    _w: PhantomData<W>,
}

impl<F, W> Lab<F, W>
where
    F: Float,
    W: WhitePoint<F>,
{
    /// Create a color in CIE L*a*b* color space.
    #[inline]
    #[must_use]
    pub fn new(l: F, a: F, b: F) -> Self {
        Self {
            l: Self::normalize_l(l),
            a: Self::normalize_a(a),
            b: Self::normalize_b(b),
            _w: PhantomData::default(),
        }
    }

    /// Return min value of l.
    #[inline]
    #[must_use]
    pub(crate) fn min_l<T: Float>() -> T {
        T::from_f64(0.0)
    }

    /// Return max value of l.
    #[inline]
    #[must_use]
    pub(crate) fn max_l<T: Float>() -> T {
        T::from_f64(100.0)
    }

    /// Return max value of a.
    #[inline]
    #[must_use]
    pub(crate) fn min_a<T: Float>() -> T {
        T::from_f64(-128.0)
    }

    /// Return max value of a.
    #[inline]
    #[must_use]
    pub(crate) fn max_a<T: Float>() -> T {
        T::from_f64(127.0)
    }

    /// Return max value of b.
    #[inline]
    #[must_use]
    pub(crate) fn min_b<T: Float>() -> T {
        T::from_f64(-128.0)
    }

    /// Return max value of b.
    #[inline]
    #[must_use]
    pub(crate) fn max_b<T: Float>() -> T {
        T::from_f64(127.0)
    }

    #[inline]
    #[must_use]
    fn normalize_l(value: F) -> F {
        value.clamp(Self::min_l(), Self::max_l())
    }

    #[inline]
    #[must_use]
    fn normalize_a(value: F) -> F {
        value.clamp(Self::min_a(), Self::max_a())
    }

    #[inline]
    #[must_use]
    fn normalize_b(value: F) -> F {
        value.clamp(Self::min_b(), Self::max_b())
    }
}

impl<F, W> Display for Lab<F, W>
where
    F: Float + Display,
    W: WhitePoint<F>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lab({l}, {a}, {b})", l = self.l, a = self.a, b = self.b)
    }
}

impl<F, W> From<&XYZ<F, W>> for Lab<F, W>
where
    F: Float,
    W: WhitePoint<F>,
{
    #[inline]
    fn from(xyz: &XYZ<F, W>) -> Self {
        let epsilon = F::from_f64(6.0 / 29.0).powi(3);
        let kappa = F::from_f64(841.0 / 108.0); // ((29.0 / 6.0) ^ 2) / 3.0
        let delta = F::from_f64(4.0 / 29.0);
        let f = |t: F| -> F {
            if t > (epsilon) {
                t.cbrt()
            } else {
                kappa * t + delta
            }
        };

        let fx = f(xyz.x / W::x());
        let fy = f(xyz.y / W::y());
        let fz = f(xyz.z / W::z());

        let l = F::from_f64(116.0) * fy - F::from_f64(16.0);
        let a = F::from_f64(500.0) * (fx - fy);
        let b = F::from_f64(200.0) * (fy - fz);
        Lab::new(l, a, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_close_to;
    use crate::color::rgba::Rgba;

    #[test]
    fn new_should_create_lab_color() {
        let lab: Lab<f64, D65> = Lab::new(53.23, 80.11, 67.22);
        assert_eq!(lab.l, 53.23);
        assert_eq!(lab.a, 80.11);
        assert_eq!(lab.b, 67.22);

        let lab: Lab<f64, D65> = Lab::new(-4.0, -192.0, -192.0);
        assert_eq!(lab.l, 0.0);
        assert_eq!(lab.a, -128.0);
        assert_eq!(lab.b, -128.0);

        let lab: Lab<f64, D65> = Lab::new(108.0, 128.0, 128.0);
        assert_eq!(lab.l, 100.0);
        assert_eq!(lab.a, 127.0);
        assert_eq!(lab.b, 127.0);
    }

    #[test]
    fn to_string_should_return_string_representation() {
        let lab: Lab<f64, D65> = Lab::new(53.23, 80.11, 67.22);
        assert_eq!(lab.to_string(), "Lab(53.23, 80.11, 67.22)");
    }

    #[test]
    fn from_xyz_should_convert_to_lab() {
        let black: XYZ<f64, D65> = XYZ::from(&Rgba::black());
        assert_eq!(Lab::from(&black), Lab::new(0.0, 0.0, 0.0));

        let white: XYZ<f64, D65> = XYZ::from(&Rgba::white());
        let actual = Lab::from(&white);
        assert_close_to!(actual.l, 100.0);
        assert_close_to!(actual.a, 0.0);
        assert_close_to!(actual.b, 0.025);

        let red: XYZ<f64, D65> = XYZ::from(&Rgba::red());
        let actual = Lab::from(&red);
        assert_close_to!(actual.l, 53.237);
        assert_close_to!(actual.a, 80.096);
        assert_close_to!(actual.b, 67.203);

        let green: XYZ<f64, D65> = XYZ::from(&Rgba::green());
        let actual = Lab::from(&green);
        assert_close_to!(actual.l, 87.735);
        assert_close_to!(actual.a, -86.182);
        assert_close_to!(actual.b, 83.186);

        let blue: XYZ<f64, D65> = XYZ::from(&Rgba::blue());
        let actual = Lab::from(&blue);
        assert_close_to!(actual.l, 32.300);
        assert_close_to!(actual.a, 79.195);
        assert_close_to!(actual.b, -107.855);
    }
}

use crate::math::number::Float;

/// Trait representing a white point.
///
/// # Type Parameters
/// * `F` - The floating point type.
///
/// # References
/// * [White point - Wikipedia](https://en.wikipedia.org/wiki/White_point)
pub trait WhitePoint<F>: Clone + Default
where
    F: Float,
{
    /// Returns the value of x.
    ///
    /// # Returns
    /// The value of x.
    #[must_use]
    fn x() -> F;

    /// Returns the value of y.
    ///
    /// # Returns
    /// The value of y.
    #[must_use]
    fn y() -> F;

    /// Returns the value of z.
    ///
    /// # Returns
    /// The value of z.
    #[must_use]
    fn z() -> F;
}

/// Struct representing CIE standard illuminant D65
///
/// # References
/// * [Illuminant D65](https://en.wikipedia.org/wiki/Illuminant_D65)
#[derive(Debug, Clone, Default, PartialEq)]
pub struct D65;

impl<F> WhitePoint<F> for D65
where
    F: Float,
{
    #[inline]
    #[must_use]
    fn x() -> F {
        F::from_f64(0.95046)
    }

    #[inline]
    #[must_use]
    fn y() -> F {
        F::from_f64(1.0)
    }

    #[inline]
    #[must_use]
    fn z() -> F {
        F::from_f64(1.08906)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d65() {
        let x: f64 = D65::x();
        assert_eq!(x, 0.95046_f64);

        let y: f64 = D65::y();
        assert_eq!(y, 1.00000_f64);

        let z: f64 = D65::z();
        assert_eq!(z, 1.08906_f64);
    }
}

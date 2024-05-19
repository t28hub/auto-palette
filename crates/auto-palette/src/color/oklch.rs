use std::fmt::Display;

use num_traits::clamp;

use crate::{
    color::{Hue, Oklab},
    math::FloatNumber,
};

/// The Oklch color representation.
///
/// See the following for more details:
/// [The Oklab color space](https://bottosson.github.io/posts/oklab/#the-oklab-color-space)
///
/// # Type Parameters
/// * `T` - The floating point type.
///
/// # Fields
/// * `l` - The lightness component.
/// * `c` - The chroma component.
/// * `h` - The hue component.
///
/// # Examples
/// ```
/// use auto_palette::color::{Oklab, Oklch};
///
/// let oklch: Oklch<f32> = Oklch::new(0.607, 0.121, 166.651);
/// assert_eq!(format!("{}", oklch), "Oklch(0.61, 0.12, 166.65)");
///
/// let oklab: Oklab<_> = (&oklch).into();
/// assert_eq!(format!("{}", oklab), "Oklab(0.61, -0.12, 0.03)");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Oklch<T>
where
    T: FloatNumber,
{
    pub l: T,
    pub c: T,
    pub h: Hue<T>,
}

impl<T> Oklch<T>
where
    T: FloatNumber,
{
    /// Creates a new `Oklch` instance.
    ///
    /// # Arguments
    /// * `l` - The lightness component.
    /// * `c` - The chroma component.
    /// * `h` - The hue component.
    ///
    /// # Returns
    /// A new `Oklch` instance.
    #[must_use]
    pub fn new(l: T, c: T, h: T) -> Self {
        Self {
            l: clamp(l, T::zero(), T::from_u32(100)),
            c: clamp(c, T::zero(), T::from_u32(180)),
            h: Hue::from_degrees(h),
        }
    }
}

impl<T> Display for Oklch<T>
where
    T: FloatNumber,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Oklch({:.2}, {:.2}, {:.2})",
            self.l,
            self.c,
            self.h.to_degrees()
        )
    }
}

impl<T> From<&Oklab<T>> for Oklch<T>
where
    T: FloatNumber,
{
    fn from(oklab: &Oklab<T>) -> Self {
        // This implementation is based on the formulae from the following sources:
        // http://www.brucelindbloom.com/index.html?Eqn_Lab_to_LCH.html
        let l = oklab.l;
        let c = (oklab.a.powi(2) + oklab.b.powi(2)).sqrt();
        let h = oklab.b.atan2(oklab.a).to_degrees();
        Oklch::new(l, c, h)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Oklab;

    #[test]
    fn test_new() {
        // Act
        let actual = Oklch::new(0.607, 0.121, 166.651);

        // Assert
        assert_eq!(
            actual,
            Oklch {
                l: 0.607,
                c: 0.121,
                h: Hue::from_degrees(166.651)
            }
        );
    }

    #[test]
    fn test_fmt() {
        // Act
        let oklch = Oklch::new(0.607, 0.121, 166.651);
        let actual = format!("{}", oklch);

        // Assert
        assert_eq!(actual, "Oklch(0.61, 0.12, 166.65)");
    }

    #[test]
    fn test_from_oklab() {
        // Act
        let oklab: Oklab<f32> = Oklab::new(0.607, -0.118, 0.028);
        let actual = Oklch::from(&oklab);

        // Assert
        assert_eq!(actual.l, 0.607);
        assert!((actual.c - 0.121).abs() < 1e-3);
        assert!((actual.h.to_degrees() - 166.651).abs() < 1e-3);
    }
}

use crate::math::number::Float;
use num_traits::Zero;
use std::fmt::{Debug, Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Sub, SubAssign};

/// Trait representing a point in n-dimensional space.
///
/// # Type Parameters
/// * `F` - The type of the point's components.
pub trait Point<F: Float>:
    Clone
    + Copy
    + Debug
    + Index<usize, Output = F>
    + Zero
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<F>
    + MulAssign<F>
    + Div<F>
    + DivAssign<F>
{
    /// Returns the dimension of this point.
    ///
    /// # Returns
    /// The dimension of this point.
    fn dimension(&self) -> usize;

    /// Returns the dot product of this point and the given point.
    ///
    /// # Arguments
    /// * `other` - The other point.
    ///
    /// # Returns
    /// The dot product of this point and the given point.
    fn dot(&self, other: &Self) -> F;

    /// Returns a vector representation of this point.
    ///
    /// # Returns
    /// A vector representation of this point.
    fn to_vec(&self) -> Vec<F>;
}

/// Struct representing a point in 2-dimensional space.
///
/// # Type Parameters
/// * `F` - The type of the point's components.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point2<F: Float>(pub F, pub F);

impl<F> Index<usize> for Point2<F>
where
    F: Float,
{
    type Output = F;

    #[inline]
    #[must_use]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            _ => panic!(
                "Index out of bounds: the len is 2 but the index is {}",
                index
            ),
        }
    }
}

/// Struct representing a point in 3-dimensional space.
///
/// # Type Parameters
/// * `F` - The type of the point's components.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point3<F: Float>(pub F, pub F, pub F);

impl<F> Index<usize> for Point3<F>
where
    F: Float,
{
    type Output = F;

    #[inline]
    #[must_use]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!(
                "Index out of bounds: the len is 3 but the index is {}",
                index
            ),
        }
    }
}

/// Struct representing a point in 5-dimensional space.
///
/// # Type Parameters
/// * `F` - The type of the point's components.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point5<F: Float>(pub F, pub F, pub F, pub F, pub F);

impl<F> Index<usize> for Point5<F>
where
    F: Float,
{
    type Output = F;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            4 => &self.4,
            _ => panic!(
                "Index out of bounds: the len is 5 but the index is {}",
                index
            ),
        }
    }
}

macro_rules! impl_point {
  ($Point:ident { $($label:tt: $field:tt),+ }, $size:expr) => {
    impl<F> $Point<F> where F: Float {
        /// Creates a new `Point` instance with the given components.
        ///
        /// # Returns
        /// A new `Point` instance.
        #[inline]
        #[must_use]
        #[allow(unused)]
        pub fn new($($label: F),+) -> Self {
            Self { $($field: $label),+ }
        }
    }

    impl<F> Display for $Point<F> where F: Float + Display {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{}{:?}", stringify!($Point), ($(self.$field),+))
        }
    }

    impl<F> Point<F> for $Point<F> where F: Float {
        #[inline]
        #[must_use]
        fn dimension(&self) -> usize {
           $size
        }

        #[inline]
        #[must_use]
        fn dot(&self, other: &Self) -> F {
            let mut sum = F::zero();
            for i in 0..self.dimension() {
                sum += self[i] * other[i];
            }
            sum
        }

        #[inline]
        #[must_use]
        fn to_vec(&self) -> Vec<F> {
            vec![$(self.$field),+]
        }
    }

    impl<F> Zero for $Point<F> where F: Float {
        #[inline]
        #[must_use]
        fn zero() -> Self {
            Self { $($field: F::zero()),+ }
        }

        #[inline]
        #[must_use]
        fn is_zero(&self) -> bool {
            $(self.$field.is_zero()) &&+
        }
    }

    impl<F> Add for $Point<F> where F: Float {
        type Output = Self;

        #[inline]
        fn add(self, rhs: Self) -> Self::Output {
            Self { $($field: self.$field + rhs.$field),+ }
        }
    }

    impl<F> Sub for $Point<F> where F: Float {
        type Output = Self;

        #[inline]
        fn sub(self, rhs: Self) -> Self::Output {
            Self { $($field: self.$field - rhs.$field),+ }
        }
    }

    impl<F> Mul<F> for $Point<F> where F: Float {
        type Output = Self;

        #[inline]
        fn mul(self, rhs: F) -> Self::Output {
            Self { $($field: self.$field * rhs),+ }
        }
    }

    impl<F> Div<F> for $Point<F> where F: Float {
        type Output = Self;

        #[inline]
        fn div(self, divisor: F) -> Self::Output {
            if divisor.is_zero() {
                panic!("{} cannot be divided by zero", stringify!($Point));
            }
            Self { $($field: self.$field / divisor),+ }
        }
    }

    impl<F> AddAssign<$Point<F>> for $Point<F> where F: Float {
        #[inline]
        fn add_assign(&mut self, rhs: $Point<F>) {
            $(self.$field += rhs.$field);+
        }
    }

    impl<F> SubAssign<$Point<F>> for $Point<F> where F: Float {
        #[inline]
        fn sub_assign(&mut self, rhs: $Point<F>) {
            $(self.$field -= rhs.$field);+
        }
    }

    impl<F> MulAssign<F> for $Point<F> where F: Float {
        #[inline]
        fn mul_assign(&mut self, rhs: F) {
            $(self.$field *= rhs);+
        }
    }

    impl<F> DivAssign<F> for $Point<F> where F: Float {
        #[inline]
        fn div_assign(&mut self, divisor: F) {
            if divisor.is_zero() {
                panic!("{} cannot be divided by zero", stringify!($Point));
            }
            $(self.$field /= divisor);+
        }
    }
  }
}

impl_point!(Point2 { x: 0, y: 1 }, 2);
impl_point!(Point3 { x: 0, y: 1, z: 2 }, 3);
impl_point!(
    Point5 {
        v: 0,
        w: 1,
        x: 2,
        y: 3,
        z: 4
    },
    5
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point2() {
        let point = Point2::new(1.0, 2.0);
        assert_eq!(point.0, 1.0);
        assert_eq!(point.1, 2.0);
    }

    #[test]
    fn test_point3() {
        let point = Point3::new(1.0, 2.0, 3.0);
        assert_eq!(point.0, 1.0);
        assert_eq!(point.1, 2.0);
        assert_eq!(point.2, 3.0);
    }

    #[test]
    fn test_point5() {
        let point = Point5::new(1.0, 2.0, 3.0, 4.0, 5.0);
        assert_eq!(point.0, 1.0);
        assert_eq!(point.1, 2.0);
        assert_eq!(point.2, 3.0);
        assert_eq!(point.3, 4.0);
        assert_eq!(point.4, 5.0);
    }

    #[test]
    fn test_index() {
        let point2 = Point2::new(1.0, 2.0);
        assert_eq!(*point2.index(0), 1.0);
        assert_eq!(*point2.index(1), 2.0);

        let point3 = Point3::new(1.0, 2.0, 3.0);
        assert_eq!(*point3.index(0), 1.0);
        assert_eq!(*point3.index(1), 2.0);
        assert_eq!(*point3.index(2), 3.0);

        let point5 = Point5::new(1.0, 2.0, 3.0, 4.0, 5.0);
        assert_eq!(*point5.index(0), 1.0);
        assert_eq!(*point5.index(1), 2.0);
        assert_eq!(*point5.index(2), 3.0);
        assert_eq!(*point5.index(3), 4.0);
        assert_eq!(*point5.index(4), 5.0);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds: the len is 2 but the index is 2")]
    fn test_point2_index_panic() {
        let point2 = Point2::new(1.0, 2.0);
        let _ = point2.index(2);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds: the len is 3 but the index is 3")]
    fn test_point3_index_panic() {
        let point3 = Point3::new(1.0, 2.0, 3.0);
        let _ = point3.index(3);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds: the len is 5 but the index is 5")]
    fn test_point5_index_panic() {
        let point5 = Point5::new(1.0, 2.0, 3.0, 4.0, 5.0);
        let _ = point5.index(5);
    }

    #[test]
    fn test_dimension() {
        assert_eq!(Point2::new(1.0, 2.0).dimension(), 2);
        assert_eq!(Point3::new(1.0, 2.0, 3.0).dimension(), 3);
        assert_eq!(Point5::new(1.0, 2.0, 3.0, 4.0, 5.0).dimension(), 5);
    }

    #[test]
    fn test_dot() {
        assert_eq!(Point2::new(1.0, 2.0).dot(&Point2::new(3.0, 4.0)), 11.0);
        assert_eq!(
            Point3::new(1.0, 2.0, 3.0).dot(&Point3::new(4.0, 5.0, 6.0)),
            32.0
        );
        assert_eq!(
            Point5::new(1.0, 2.0, 3.0, 4.0, 5.0).dot(&Point5::new(6.0, 7.0, 8.0, 9.0, 10.0)),
            130.0
        );
    }

    #[test]
    fn test_to_vec() {
        assert_eq!(Point2::new(1.0, 2.0).to_vec(), vec![1.0, 2.0]);
        assert_eq!(Point3::new(1.0, 2.0, 3.0).to_vec(), vec![1.0, 2.0, 3.0]);
        assert_eq!(
            Point5::new(1.0, 2.0, 3.0, 4.0, 5.0).to_vec(),
            vec![1.0, 2.0, 3.0, 4.0, 5.0]
        );
    }

    #[test]
    fn test_to_string() {
        assert_eq!(Point2::new(1.0, 2.0).to_string(), "Point2(1.0, 2.0)");
        assert_eq!(
            Point3::new(1.0, 2.0, 3.0).to_string(),
            "Point3(1.0, 2.0, 3.0)"
        );
        assert_eq!(
            Point5::new(1.0, 2.0, 3.0, 4.0, 5.0).to_string(),
            "Point5(1.0, 2.0, 3.0, 4.0, 5.0)"
        );
    }

    #[test]
    fn test_add() {
        let point1 = Point2::new(1.0, 2.0);
        let point2 = Point2::new(2.0, 3.0);
        assert_eq!(point1.add(point2), Point2::new(3.0, 5.0));

        let point1 = Point3::new(1.0, 2.0, 3.0);
        let point2 = Point3::new(2.0, 3.0, 5.0);
        assert_eq!(point1.add(point2), Point3::new(3.0, 5.0, 8.0));

        let point1 = Point5::new(1.0, 2.0, 3.0, 4.0, 5.0);
        let point2 = Point5::new(2.0, 3.0, 5.0, 7.0, 11.0);
        assert_eq!(point1.add(point2), Point5::new(3.0, 5.0, 8.0, 11.0, 16.0));
    }

    #[test]
    fn test_sub() {
        let point1 = Point2::new(1.0, 3.0);
        let point2 = Point2::new(2.0, 2.0);
        assert_eq!(point1.sub(point2), Point2::new(-1.0, 1.0));

        let point1 = Point3::new(3.0, 5.0, 7.0);
        let point2 = Point3::new(1.0, 2.0, 3.0);
        assert_eq!(point1.sub(point2), Point3::new(2.0, 3.0, 4.0));

        let point1 = Point5::new(1.0, 2.0, 3.0, 4.0, 5.0);
        let point2 = Point5::new(2.0, 3.0, 5.0, 7.0, 11.0);
        assert_eq!(
            point1.sub(point2),
            Point5::new(-1.0, -1.0, -2.0, -3.0, -6.0)
        );
    }

    #[test]
    fn test_mul() {
        let point = Point2::new(1.0, 3.0);
        assert_eq!(point.mul(2.0), Point2::new(2.0, 6.0));

        let point = Point3::new(3.0, 5.0, 7.0);
        assert_eq!(point.mul(0.5), Point3::new(1.5, 2.5, 3.5));

        let point = Point5::new(1.0, 2.0, 3.0, 4.0, 5.0);
        assert_eq!(point.mul(2.5), Point5::new(2.5, 5.0, 7.5, 10.0, 12.5));
    }

    #[test]
    fn test_div() {
        let point = Point2::new(1.0, 3.0);
        assert_eq!(point / 2.0, Point2::new(0.5, 1.5));

        let point = Point3::new(3.0, 5.0, 7.0);
        assert_eq!(point.div(0.5), Point3::new(6.0, 10.0, 14.0));

        let point = Point5::new(1.0, 2.0, 3.0, 4.0, 5.0);
        assert_eq!(point.div(2.0), Point5::new(0.5, 1.0, 1.5, 2.0, 2.5));
    }

    #[test]
    #[should_panic(expected = "Point2 cannot be divided by zero")]
    fn test_div_panic() {
        let point = Point2::new(1.0, 3.0);
        let _ = point / 0.0;
    }

    #[test]
    fn test_add_assign() {
        let mut point1 = Point2::new(1.0, 2.0);
        let point2 = Point2::new(2.0, 3.0);
        point1.add_assign(point2);
        assert_eq!(point1, Point2::new(3.0, 5.0));
    }

    #[test]
    fn test_sub_assign() {
        let mut point1 = Point2::new(1.0, 3.0);
        let point2 = Point2::new(2.0, 2.0);
        point1.sub_assign(point2);
        assert_eq!(point1, Point2::new(-1.0, 1.0));
    }

    #[test]
    fn test_mul_assign() {
        let mut point = Point2::new(1.0, 3.0);
        point.mul_assign(2.0);
        assert_eq!(point, Point2::new(2.0, 6.0));
    }

    #[test]
    fn test_div_assign() {
        let mut point = Point2::new(1.0, 3.0);
        point.div_assign(2.0);
        assert_eq!(point, Point2::new(0.5, 1.5));
    }

    #[test]
    #[should_panic(expected = "Point2 cannot be divided by zero")]
    fn test_div_assign_panic() {
        let mut point = Point2::new(1.0, 3.0);
        point /= 0.0;
    }
}

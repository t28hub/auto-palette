use num_traits::Float;

/// Checks if two floating point values are close to each other within a given tolerance.
///
/// # Arguments
/// * `lhs` - The left-hand side value to compare.
/// * `rhs` - The right-hand side value to compare.
/// * `max_diff` - The maximum allowable difference..
///
/// # Returns
/// `true` if the absolute difference between `lhs` and `rhs` is less than `max_diff`
#[inline(always)]
pub fn is_close_to<F: Float>(lhs: F, rhs: F, max_diff: F) -> bool {
    let diff = (lhs - rhs).abs();
    diff < max_diff
}

#[macro_export]
macro_rules! assert_close_to(
    ($lhs: expr, $rhs: expr) => ({
       assert_close_to!($lhs, $rhs, 0.01)
    });
    ($lhs: expr, $rhs: expr, $max_diff: expr) => ({
        if !$crate::macros::is_close_to($lhs, $rhs, $max_diff) {
           panic!("lhs(`{:?}`) is not similar to rhs(`{:?}`)", $lhs, $rhs)
        }
    });
);

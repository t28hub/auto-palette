use num_traits::Float;

/// Check whether the difference between lhs and the rhs is less than the max difference.
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

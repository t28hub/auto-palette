#[macro_export]
macro_rules! assert_close_to(
    ($lhs: expr, $rhs: expr) => ({
       assert_close_to!($lhs, $rhs, 0.01)
    });
    ($lhs: expr, $rhs: expr, $max_diff: expr) => ({
        let diff = ($lhs - $rhs).abs();
        if diff > $max_diff {
           panic!(
               "lhs(`{:?}`) is not similar to rhs(`{:?}`) (diff: `{:?}`)",
               $lhs, $rhs, diff
           )
        }
    });
);

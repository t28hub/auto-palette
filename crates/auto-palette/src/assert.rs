/// This module contains macros for assertions.
/// These macros are used to check if two values are approximately equal.
///
/// # Example
/// ```ignore
/// use auto_palette::assert_approx_eq;
///
/// assert_approx_eq!(1.0, 1.000001);        // This will pass
/// assert_approx_eq!(1.0, 1.0001);          // This will fail
/// assert_approx_eq!(1.0, 1.0001, 0.0002);  // This will pass
/// assert_approx_eq!(1.0, 1.0001, 0.00005); // This will fail
/// ```
#[cfg(test)]
#[macro_export]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr) => {
        let a = $a as f64;
        let b = $b as f64;
        let eps = 1.0e-6;
        let diff = (a - b).abs();
        assert!(
            diff < eps,
            "assertion failed: `|left-right| >= epsilon`\n  left: `{:?}`,\n right: `{:?}`\n  eps: `{:?}`\n  diff: `{:?}`",
            a,
            b,
            eps,
            diff
        );
    };
    ($a:expr, $b:expr, $eps:expr) => {
        let a = $a as f64;
        let b = $b as f64;
        let eps = $eps;
        let diff = (a - b).abs();
        assert!(
            eps > 0.0,
            "epsilon must be positive, but got: `{:?}`",
            eps
        );
        assert!(
            diff < eps,
            "assertion failed: `|left-right| >= epsilon`\n  left: `{:?}`,\n right: `{:?}`\n  eps: `{:?}`\n  diff: `{:?}`",
            a,
            b,
            eps,
            diff
        );
    };
}

#[cfg(test)]
#[cfg_attr(coverage, coverage(off))]
mod test {
    #[test]
    fn test_assert_approx_eq() {
        assert_approx_eq!(1.0, 1.000001);
    }

    #[test]
    #[should_panic(expected = "assertion failed: `|left-right| >= epsilon`")]
    fn test_assert_approx_eq_fail() {
        assert_approx_eq!(1.0, 1.0001);
    }

    #[test]
    fn test_assert_approx_eq_with_eps() {
        assert_approx_eq!(1.0, 1.001, 1.0e-2);
    }

    #[test]
    #[should_panic(expected = "assertion failed: `|left-right| >= epsilon`")]
    fn test_assert_approx_eq_fail_with_eps() {
        assert_approx_eq!(1.0, 1.0001, 0.00005);
    }

    #[test]
    #[should_panic(expected = "epsilon must be positive, but got: `0.0`")]
    fn test_assert_approx_eq_zero_eps() {
        assert_approx_eq!(1.0, 1.0001, 0.0);
    }
}

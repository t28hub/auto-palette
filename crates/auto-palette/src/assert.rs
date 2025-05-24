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

#[macro_export]
macro_rules! assert_color_eq {
    ($left:expr, $right:expr) => {
        let tolerance = 1.0;
        let delta = $left.delta_e(&$right);
        assert!(
            delta < tolerance,
            "assertion failed: `|left-right| >= tolerance`\n  left: `{:?}`,\n right: `{:?}`\n  tolerance: `{:?}`\n  delta: `{:?}`",
            $left,
            $right,
            tolerance,
            delta
        );
    };
    ($left:expr, $right:expr, $tolerance:expr) => {
        let delta = $left.delta_e(&$right);
        assert!(
            $tolerance >= 0.0,
            "tolerance must be greater than or equal to zero, but got: `{:?}`",
            $tolerance
        );
        assert!(
            delta < $tolerance,
            "assertion failed: `|left-right| >= tolerance`\n  left: `{:?}`,\n right: `{:?}`\n  tolerance: `{:?}`\n  delta: `{:?}`",
            $left,
            $right,
            $tolerance,
            delta
        );
    };
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::color::Color;

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

    #[test]
    fn test_assert_color_eq() {
        let color1: Color<f64> = Color::from_str("#eb367f").unwrap();
        let color2: Color<f64> = Color::from_str("#eb367f").unwrap();
        assert_color_eq!(color1, color2);
    }

    #[test]
    fn test_assert_color_eq_with_tolerance() {
        let color1: Color<f64> = Color::from_str("#eb367f").unwrap();
        let color2: Color<f64> = Color::from_str("#ec367f").unwrap();
        assert_color_eq!(color1, color2, 1.0);
    }

    #[test]
    #[should_panic(expected = "assertion failed: `|left-right| >= tolerance`")]
    fn test_assert_color_eq_fail() {
        let color1: Color<f64> = Color::from_str("#eb367f").unwrap();
        let color2: Color<f64> = Color::from_str("#ee367f").unwrap();
        assert_color_eq!(color1, color2);
    }

    #[test]
    #[should_panic(expected = "assertion failed: `|left-right| >= tolerance`")]
    fn test_assert_color_eq_fail_with_tolerance() {
        let color1: Color<f64> = Color::from_str("#eb367f").unwrap();
        let color2: Color<f64> = Color::from_str("#ec367f").unwrap();
        assert_color_eq!(color1, color2, 1e-6);
    }

    #[test]
    #[should_panic(
        expected = "tolerance must be greater than or equal to zero, but got: `-0.0001`"
    )]
    fn test_assert_color_eq_zero_tolerance() {
        let color1: Color<f64> = Color::from_str("#eb367f").unwrap();
        let color2: Color<f64> = Color::from_str("#ec367f").unwrap();
        assert_color_eq!(color1, color2, -1e-4);
    }
}

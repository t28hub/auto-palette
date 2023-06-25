use crate::number::Float;
use ndarray::{ArrayView1, ArrayViewMut1};
use statrs::distribution::{ContinuousCDF, Normal};
use std::cmp::Ordering;

/// Standardizes the given array of values.
///
/// # Arguments
/// * `x` - The array of values to standardize.
///
/// # Type Parameters
/// * `F` - The floating point type.
#[inline]
pub fn standardize<F>(x: &mut ArrayViewMut1<F>)
where
    F: Float,
{
    if x.is_empty() {
        return;
    }

    let n = F::from_usize(x.len());
    let mean = x.sum() / n;
    let variance = x.mapv(|value| (value - mean).powi(2)).sum() / n;
    let std = variance.sqrt();
    x.mapv_inplace(|value| (value - mean) / std);
}

/// Calculates the Anderson-Darling test statistic for the given array of values.
///
/// # Arguments
/// * `x` - The array of values to calculate the statistic for.
///
/// # Returns
/// The Anderson-Darling test statistic for the given array of values.
///
/// # Type Parameters
/// * `F` - The floating point type.
#[inline]
pub fn anderson_darling_test<F>(x: &ArrayView1<F>) -> Option<F>
where
    F: Float,
{
    if x.is_empty() {
        return None;
    }

    let mut sorted_x = x.to_vec();
    sorted_x.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

    let normal = Normal::new(0.0, 1.0).unwrap();
    let p_values: Vec<F> = sorted_x
        .iter()
        .map(|value| {
            let p = normal.cdf(value.to_f64().unwrap_or(0.0));
            F::from_f64(p)
        })
        .collect();

    let n = x.len();
    let n_f = F::from_usize(n);
    let mut sum = F::zero();
    for i in 0..n {
        sum +=
            F::from_usize(2 * i + 1) * (p_values[i].ln() + (F::one() - p_values[n - 1 - i]).ln());
    }
    let a_squared = sum / -n_f - n_f;
    let score = a_squared * (F::one() + F::from_u32(4) / n_f + F::from_u32(25) / n_f.powi(2));
    Some(score)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::{array, Array1};

    #[test]
    fn test_standardize() {
        let mut x = array![1.0, 2.0, 3.0, 4.0];
        standardize(&mut x.view_mut());
        assert_eq!(
            x,
            array![
                -1.3416407864998738,
                -0.4472135954999579,
                0.4472135954999579,
                1.3416407864998738
            ]
        );
    }

    #[test]
    fn test_standardize_empty() {
        let mut x = Array1::from_elem(0, 0.0);
        standardize(&mut x.view_mut());
        assert_eq!(x, array![]);
    }

    #[test]
    fn test_anderson_darling_test() {
        let mut x = array![1.0, 2.0, 3.0, 4.0];
        standardize(&mut x.view_mut());

        let actual = anderson_darling_test(&x.view());
        assert_eq!(actual, Some(0.7150327614247158));
    }

    #[test]
    fn test_anderson_darling_test_empty() {
        let mut x = Array1::from_elem(0, 0.0);
        standardize(&mut x.view_mut());

        let actual = anderson_darling_test(&x.view());
        assert_eq!(actual, None);
    }
}

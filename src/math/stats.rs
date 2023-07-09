use crate::math::number::Float;
use statrs::distribution::{ContinuousCDF, Normal};
use std::cmp::Ordering;

/// Standardizes the given vector of values.
///
/// # Arguments
/// * `x` - The vector of values to standardize.
///
/// # Type Parameters
/// * `F` - The floating point type.
#[inline]
pub fn standardize<F: Float>(x: &mut [F]) {
    if x.is_empty() {
        return;
    }

    let n = F::from_usize(x.len());
    let mean = x.iter().fold(F::zero(), |mut total, &value| {
        total += value;
        total
    }) / n;
    let variance =
        x.iter()
            .map(|&value| (value - mean).powi(2))
            .fold(F::zero(), |mut total, value| {
                total += value;
                total
            })
            / n;
    let sd = variance.sqrt();
    for value in x.iter_mut() {
        *value = (*value - mean) / sd;
    }
}

/// Calculates the Anderson-Darling test statistic for the given vector of values.
///
/// # Arguments
/// * `x` - The vector of values to calculate the statistic for.
///
/// # Returns
/// The Anderson-Darling test statistic for the given vector of values.
///
/// # Type Parameters
/// * `F` - The floating point type.
///
/// # References
/// * [Anderson-Darling Test](https://en.wikipedia.org/wiki/Anderson%E2%80%93Darling_test)
#[inline]
#[must_use]
pub fn anderson_darling_test<F: Float>(x: &[F]) -> Option<F> {
    if x.is_empty() {
        return None;
    }

    let mut sorted_x = x.to_vec();
    sorted_x.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

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

    #[test]
    fn test_standardize() {
        let mut x: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
        standardize(&mut x);
        assert_eq!(
            x,
            vec![
                -1.3416407864998738,
                -0.4472135954999579,
                0.4472135954999579,
                1.3416407864998738
            ]
        );
    }

    #[test]
    fn test_standardize_empty() {
        let mut x: Vec<f64> = vec![];
        standardize(&mut x);
        assert_eq!(x, vec![]);
    }

    #[test]
    fn test_anderson_darling_test() {
        let mut x: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
        standardize(&mut x);

        let actual = anderson_darling_test(&x);
        assert!(actual.is_some());
        assert!(
            actual.unwrap() < 1.0,
            "Anderson-Darling test statistic: {:?}",
            actual
        );
    }

    #[test]
    fn test_anderson_darling_test_empty() {
        let x: Vec<f64> = vec![];
        let actual = anderson_darling_test(&x);
        assert!(actual.is_none());
    }
}

use thiserror::Error;

use crate::FloatNumber;

/// Error type for Gaussian function.
#[derive(Debug, Error)]
pub enum GaussianError<T>
where
    T: FloatNumber,
{
    /// Error when either the center or sigma is invalid.
    #[error("Invalid Parameters: center = {center}, sigma = {sigma}")]
    InvalidParameters { center: T, sigma: T },
}

/// Calculates the value of the Gaussian function at a given point.
///
/// The formula used is:
/// exp(-((x - center)^2) / (2 * sigma^2))
///
/// # Arguments
/// * `x` - The point at which to evaluate the Gaussian function.
/// * `center` - The center of the Gaussian function.
/// * `sigma` - The standard deviation of the Gaussian function.
///
/// # Returns
/// The value of the Gaussian function at the given point.
#[inline(always)]
pub fn gaussian<T>(x: T, center: T, sigma: T) -> Result<T, GaussianError<T>>
where
    T: FloatNumber,
{
    if center.is_nan() {
        return Err(GaussianError::InvalidParameters { center, sigma });
    }
    if sigma.is_zero() {
        return Err(GaussianError::InvalidParameters { center, sigma });
    }
    Ok((-(x - center).powi(2) / (T::from_u16(2) * sigma.powi(2))).exp())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::assert_approx_eq;

    #[rstest]
    #[case(0.0, 0.0, 1.0, 1.0)]
    #[case(0.0, 1.0, 1.0, 0.606530)]
    #[case(1.0, 0.0, 1.0, 0.606530)]
    #[case(1.0, 1.0, 1.0, 1.0)]
    #[case(0.0, 0.5, 1.0, 0.882496)]
    #[case(0.5, 0.0, 1.0, 0.882496)]
    #[case(16.0, 60.0, 15.0, 0.013538)]
    #[case(64.0, 60.0, 15.0, 0.965069)]
    #[case(104.0, 60.0, 15.0, 0.013538)]
    fn test_gaussian_score(
        #[case] x: f64,
        #[case] center: f64,
        #[case] sigma: f64,
        #[case] expected: f64,
    ) {
        // Act
        let actual = gaussian(x, center, sigma).unwrap();

        // Assert
        assert_approx_eq!(actual, expected);
    }

    #[rstest]
    #[case::invalid_sigma(0.0, 0.0, "Invalid Parameters: center = 0, sigma = 0")]
    #[case::invalid_center(f32::NAN, 1.0, "Invalid Parameters: center = NaN, sigma = 1")]
    fn test_gaussian_invalid(#[case] center: f32, #[case] sigma: f32, #[case] expected: &str) {
        // Act
        let actual = gaussian(1.0, center, sigma);

        // Assert
        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err().to_string(), expected);
    }
}

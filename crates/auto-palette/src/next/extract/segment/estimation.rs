use crate::next::{
    color::{AlphaColor, Oklab},
    math::{DistanceMetric, Vector},
};

/// How to choose the color of a segment from its pixels.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Estimation {
    /// Tukey's biweight, which ignores pixels far from the estimate.
    Biweight(Biweight),
}

impl Default for Estimation {
    fn default() -> Self {
        Self::Biweight(Biweight::default())
    }
}

#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "called by the segment statistics, which are not implemented yet"
    )
)]
impl Estimation {
    /// Estimates the color of a segment.
    ///
    /// # Arguments
    /// * `pixels` - The pixels of the segment.
    ///
    /// # Returns
    /// The color of the segment, or `None` if there are no pixels.
    pub(crate) fn estimate(&self, pixels: &[AlphaColor<Oklab>]) -> Option<Oklab> {
        match self {
            Self::Biweight(biweight) => biweight.estimate(pixels),
        }
    }
}

/// Tukey's biweight, starting from the median of each channel.
///
/// From ["The Fitting of Power Series, Meaning Polynomials, Illustrated on Band-Spectroscopic
/// Data"](https://doi.org/10.1080/00401706.1974.10489171) (Beaton and Tukey, Technometrics 1974),
/// here with a fixed color scale and two passes.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Biweight {
    /// The color distance at which a pixel stops counting.
    scale: f64,
}

impl Biweight {
    /// The default for `scale`.
    const DEFAULT_SCALE: f64 = 0.06;

    /// The number of times the estimate is moved toward the pixels near it.
    const PASSES: usize = 2;

    /// Creates Tukey's biweight with the given scale.
    ///
    /// # Arguments
    /// * `scale` - The color distance at which a pixel stops counting.
    ///
    /// # Returns
    /// The biweight, or `None` if `scale` is not positive and finite.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "called by the builder, which is not implemented yet"
        )
    )]
    pub(crate) fn new(scale: f64) -> Option<Self> {
        if !scale.is_finite() || scale <= 0.0 {
            return None;
        }
        Some(Self { scale })
    }

    /// Estimates the color of a segment.
    ///
    /// # Arguments
    /// * `pixels` - The pixels of the segment.
    ///
    /// # Returns
    /// The color of the segment, or `None` if there are no pixels.
    fn estimate(&self, pixels: &[AlphaColor<Oklab>]) -> Option<Oklab> {
        let mut estimate = find_median(pixels)?;
        for _ in 0..Self::PASSES {
            estimate = self.reweight(pixels, estimate);
        }
        Some(estimate)
    }

    /// Moves an estimate to the mean of the pixels, weighted by how close and opaque they are.
    ///
    /// # Arguments
    /// * `pixels` - The pixels of the segment.
    /// * `estimate` - The current estimate.
    ///
    /// # Returns
    /// The weighted mean, or `estimate` if no pixel counts.
    fn reweight(&self, pixels: &[AlphaColor<Oklab>], estimate: Oklab) -> Oklab {
        let target = Vector([estimate.l, estimate.a, estimate.b]);
        let mut weighted_sum = Vector::default();
        let mut total_weight = 0.0;
        for pixel in pixels {
            let color = pixel.color();
            let vector = Vector([color.l, color.a, color.b]);
            let squared_distance = vector.distance(target, DistanceMetric::SquaredEuclidean);
            let squared_ratio = squared_distance / (self.scale * self.scale);
            // A pixel at the scale or farther, such as a blended edge, is an outlier.
            if squared_ratio >= 1.0 {
                continue;
            }
            let weight = pixel.alpha() * (1.0 - squared_ratio).powi(2);
            weighted_sum += vector * weight;
            total_weight += weight;
        }
        if total_weight == 0.0 {
            return estimate;
        }
        let [l, a, b] = (weighted_sum / total_weight).0;
        Oklab::new(l, a, b)
    }
}

impl Default for Biweight {
    fn default() -> Self {
        Self {
            scale: Self::DEFAULT_SCALE,
        }
    }
}

/// Finds the median of each channel, without weights.
///
/// # Arguments
/// * `pixels` - The pixels of the segment.
///
/// # Returns
/// The medians as a color, or `None` if there are no pixels.
fn find_median(pixels: &[AlphaColor<Oklab>]) -> Option<Oklab> {
    if pixels.is_empty() {
        return None;
    }
    let mut l_values = Vec::with_capacity(pixels.len());
    let mut a_values = Vec::with_capacity(pixels.len());
    let mut b_values = Vec::with_capacity(pixels.len());
    for pixel in pixels {
        let color = pixel.color();
        l_values.push(color.l);
        a_values.push(color.a);
        b_values.push(color.b);
    }
    let l = find_middle(&mut l_values);
    let a = find_middle(&mut a_values);
    let b = find_middle(&mut b_values);
    Some(Oklab::new(l, a, b))
}

/// Finds the middle of some values, the upper one of an even number.
///
/// # Arguments
/// * `values` - The values, at least one, reordered in place.
///
/// # Returns
/// The middle value.
fn find_middle(values: &mut [f64]) -> f64 {
    let middle = values.len() / 2;
    let (_, value, _) = values.select_nth_unstable_by(middle, f64::total_cmp);
    *value
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::next::extract::segment::test_helpers::gray;

    #[test]
    fn test_new_accepts_scale() {
        // Act
        let actual = Biweight::new(0.06);

        // Assert
        assert!(actual.is_some());
    }

    #[rstest]
    #[case::zero(0.0)]
    #[case::negative(-0.06)]
    #[case::nan(f64::NAN)]
    #[case::infinity(f64::INFINITY)]
    fn test_new_rejects_scale(#[case] scale: f64) {
        // Act
        let actual = Biweight::new(scale);

        // Assert
        assert!(actual.is_none());
    }

    #[test]
    fn test_estimate_returns_none_without_pixels() {
        // Act
        let actual = Estimation::default().estimate(&[]);

        // Assert
        assert!(actual.is_none());
    }

    #[test]
    fn test_estimate_ignores_outlier() {
        // Arrange: a white pixel mixed into black ones, which would pull a mean to gray.
        let pixels = [0.0, 0.0, 0.0, 1.0].map(gray).map(AlphaColor::opaque);

        // Act
        let actual = Estimation::default().estimate(&pixels).unwrap();

        // Assert
        assert_eq!(actual.l, 0.0);
    }

    #[test]
    fn test_estimate_weights_by_opacity() {
        // Arrange: two close grays, the lighter one mostly transparent.
        let pixels = [
            AlphaColor::opaque(gray(0.0)),
            AlphaColor::new(gray(0.02), 0.25).unwrap(),
        ];

        // Act
        let actual = Estimation::default().estimate(&pixels).unwrap();

        // Assert: nearer the opaque pixel than the midpoint 0.01.
        assert!(actual.l < 0.01);
    }

    #[test]
    fn test_estimate_keeps_median_when_every_pixel_is_far() {
        // Arrange: the channel medians make a color 0.3 away from both pixels.
        let pixels = [
            AlphaColor::opaque(Oklab::new(0.0, 0.3, 0.0)),
            AlphaColor::opaque(Oklab::new(0.3, 0.0, 0.0)),
        ];

        // Act
        let actual = Estimation::default().estimate(&pixels).unwrap();

        // Assert
        assert_eq!([actual.l, actual.a, actual.b], [0.3, 0.3, 0.0]);
    }

    #[test]
    fn test_find_median_takes_upper_middle() {
        // Arrange
        let pixels = [0.3, 0.1].map(gray).map(AlphaColor::opaque);

        // Act
        let actual = find_median(&pixels).unwrap();

        // Assert
        assert_eq!(actual.l, 0.3);
    }

    #[test]
    fn test_find_median_of_each_channel() {
        // Arrange
        let pixels = [
            AlphaColor::opaque(Oklab::new(0.1, 0.0, -0.1)),
            AlphaColor::opaque(Oklab::new(0.2, 0.1, 0.0)),
            AlphaColor::opaque(Oklab::new(0.3, -0.1, 0.1)),
        ];

        // Act
        let actual = find_median(&pixels).unwrap();

        // Assert
        assert_eq!([actual.l, actual.a, actual.b], [0.2, 0.0, 0.0]);
    }

    #[test]
    fn test_reweight_weights_by_closeness() {
        // Arrange: from 0.03, the pixel at 0.04 is nearer than the one at 0.0.
        let pixels = [0.0, 0.04].map(gray).map(AlphaColor::opaque);

        // Act
        let actual = Biweight::default().reweight(&pixels, gray(0.03));

        // Assert: past the midpoint 0.02, toward the nearer pixel.
        assert!(actual.l > 0.02);
    }
}

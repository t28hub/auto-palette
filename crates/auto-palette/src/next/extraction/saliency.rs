use super::segmentation::Segment;
use crate::next::{color::DeltaE, math::DistanceMetric};

/// A method that scores each segment by how much it stands out.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Saliency {
    /// Region contrast ([`RegionContrast`]).
    RegionContrast(RegionContrast),
}

impl Default for Saliency {
    fn default() -> Self {
        Self::RegionContrast(RegionContrast::default())
    }
}

impl Saliency {
    /// Scores each segment by how much it stands out.
    ///
    /// # Arguments
    /// * `segments` - The segments of an image.
    ///
    /// # Returns
    /// The score of each segment, in the same order, from 0.0 to 1.0 for the most salient one, or
    /// 0.0 for every segment if none stands out.
    pub(crate) fn score(&self, segments: &[Segment]) -> Vec<f64> {
        match self {
            Self::RegionContrast(contrast) => contrast.score(segments),
        }
    }
}

/// Region contrast, which scores a segment by its color differences from the other segments.
///
/// From ["Global Contrast Based Salient Region Detection"](https://doi.org/10.1109/TPAMI.2014.2345401)
/// (Cheng et al., TPAMI 2015), eq. 7, without the border prior.
#[derive(Debug, Clone, Copy)]
pub(crate) struct RegionContrast {
    /// How fast the contrast to farther segments fades, σ_s² in the paper.
    spatial_falloff: f64,
    /// How strongly segments away from the image center are lowered.
    center_bias: f64,
    /// The formula for the color difference between two segments.
    delta_e: DeltaE,
}

impl RegionContrast {
    /// The default for `spatial_falloff`, from the paper.
    const DEFAULT_SPATIAL_FALLOFF: f64 = 0.4;

    /// The default for `center_bias`, weaker than the paper's 9.0 so that distinct colors near the
    /// edges still stand out.
    const DEFAULT_CENTER_BIAS: f64 = 3.0;

    /// Scores each segment by how much it stands out.
    ///
    /// # Arguments
    /// * `segments` - The segments of an image.
    ///
    /// # Returns
    /// The score of each segment, in the same order, from 0.0 to 1.0 for the most salient one, or
    /// 0.0 for every segment if none stands out.
    fn score(&self, segments: &[Segment]) -> Vec<f64> {
        let scores: Vec<f64> = segments
            .iter()
            .enumerate()
            .map(|(index, &segment)| {
                let center_weight = (-self.center_bias * segment.radial_distance().powi(2)).exp();
                center_weight * self.sum_contrasts(index, segments)
            })
            .collect();
        scale_to_max(scores)
    }

    /// Sums the contrasts of a segment to every other segment.
    ///
    /// # Arguments
    /// * `index` - The index of the segment in `segments`.
    /// * `segments` - The segments of an image.
    ///
    /// # Returns
    /// The sum of the contrasts to the segments other than the one at `index`.
    fn sum_contrasts(&self, index: usize, segments: &[Segment]) -> f64 {
        let segment = segments[index];
        segments
            .iter()
            .enumerate()
            .filter(|&(other_index, _)| other_index != index)
            .map(|(_, &other)| self.measure_contrast(segment, other))
            .sum()
    }

    /// Measures the contrast of a segment to another, weighted by its size and nearness.
    ///
    /// # Arguments
    /// * `segment` - The segment to score.
    /// * `other` - The other segment.
    ///
    /// # Returns
    /// The color difference, scaled by the size of `other` and the nearness of the centers.
    fn measure_contrast(&self, segment: Segment, other: Segment) -> f64 {
        let squared_distance = segment
            .center()
            .distance(other.center(), DistanceMetric::SquaredEuclidean);
        let spatial_weight = (-squared_distance / self.spatial_falloff).exp();
        let difference = self.delta_e.measure(segment.color(), other.color());
        spatial_weight * other.weight() * difference
    }
}

impl Default for RegionContrast {
    fn default() -> Self {
        Self {
            spatial_falloff: Self::DEFAULT_SPATIAL_FALLOFF,
            center_bias: Self::DEFAULT_CENTER_BIAS,
            delta_e: DeltaE::Okr2,
        }
    }
}

/// Scales scores so that the largest becomes 1.0.
///
/// # Arguments
/// * `scores` - The scores, 0.0 or more each.
///
/// # Returns
/// The scores divided by the largest, or the scores unchanged if the largest is 0.0.
fn scale_to_max(scores: Vec<f64>) -> Vec<f64> {
    let max = scores.iter().copied().fold(0.0, f64::max);
    if max == 0.0 {
        return scores;
    }
    scores.into_iter().map(|score| score / max).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        assert_approx_eq,
        next::{
            color::{Color, Oklab},
            extraction::position::Position,
        },
    };

    /// Creates a gray color.
    fn gray_color(lightness: f64) -> Color {
        Color::from_oklab(Oklab::new(lightness, 0.0, 0.0))
    }

    /// Creates a gray segment at the image center with no radial distance.
    fn gray(lightness: f64, weight: f64) -> Segment {
        Segment::new(gray_color(lightness), Position::new(0.5, 0.5), weight, 0.0)
    }

    #[test]
    fn test_score_scales_to_most_contrasting_segment() {
        // Arrange
        let segments = [gray(0.5, 1.0), gray(0.5, 1.0), gray(1.0, 1.0)];

        // Act
        let actual = Saliency::default().score(&segments);

        // Assert: each gray differs from one segment, the white from two.
        assert_approx_eq!(actual[0], 0.5);
        assert_approx_eq!(actual[1], 0.5);
        assert_approx_eq!(actual[2], 1.0);
    }

    #[test]
    fn test_score_zero_for_same_colors() {
        // Arrange
        let segments = [gray(0.5, 1.0), gray(0.5, 1.0)];

        // Act
        let actual = Saliency::default().score(&segments);

        // Assert
        assert_eq!(actual, [0.0, 0.0]);
    }

    #[test]
    fn test_score_zero_for_single_segment() {
        // Act
        let actual = Saliency::default().score(&[gray(0.5, 1.0)]);

        // Assert
        assert_eq!(actual, [0.0]);
    }

    #[test]
    fn test_score_weights_contrast_by_size_of_other_segment() {
        // Arrange
        let segments = [gray(0.5, 1.0), gray(1.0, 3.0)];

        // Act
        let actual = Saliency::default().score(&segments);

        // Assert: the gray sees a segment three times as large as the white does.
        assert_approx_eq!(actual[0], 1.0);
        assert_approx_eq!(actual[1], 1.0 / 3.0);
    }

    #[test]
    fn test_contrast_fades_with_squared_distance() {
        // Arrange
        let segment = gray(0.5, 1.0);
        let other = Segment::new(gray_color(1.0), Position::new(0.7, 0.5), 1.0, 0.0);

        // Act
        let actual = RegionContrast::default().measure_contrast(segment, other);

        // Assert: the centers are 0.2 apart, so the spatial weight is exp(-0.2² / 0.4).
        let difference = DeltaE::Okr2.measure(segment.color(), other.color());
        assert_approx_eq!(actual, (-0.04_f64 / 0.4).exp() * difference);
    }

    #[test]
    fn test_score_lowers_segments_away_from_center() {
        // Arrange
        let central = gray(0.5, 1.0);
        let off_center = Segment::new(gray_color(1.0), Position::new(0.5, 0.5), 1.0, 0.5);

        // Act
        let actual = Saliency::default().score(&[central, off_center]);

        // Assert: the same contrast, lowered by exp(-3 * 0.5²) for the off-center segment.
        assert_approx_eq!(actual[0], 1.0);
        assert_approx_eq!(actual[1], (-3.0_f64 * 0.25).exp());
    }
}

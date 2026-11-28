use super::{
    clustering::{Agglomerative, ClusteringAlgorithm, Linkage},
    point::Point,
    segmentation::Segment,
};
use crate::next::color::{Color, DeltaE};

/// A method that groups segments of similar colors into candidates.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Grouping {
    /// The formula for the difference between colors.
    delta_e: DeltaE,
    /// The clustering of the colors, embedded so that the difference is the distance.
    clustering: ClusteringAlgorithm,
}

impl Default for Grouping {
    fn default() -> Self {
        let agglomerative = Agglomerative::new(Linkage::Complete, Self::THRESHOLD)
            .expect("the threshold is a positive normal number");
        Self {
            delta_e: DeltaE::Okr2,
            clustering: ClusteringAlgorithm::Agglomerative(agglomerative),
        }
    }
}

impl Grouping {
    /// The ΔEOKr2 at which people stop accepting two colors as the same.
    ///
    /// From the 50:50% acceptability threshold ΔE00 1.8 of
    /// [Paravina et al.](https://doi.org/10.1111/jerd.12149) (J Esthet Restor Dent 2015), about
    /// ΔEOKr2 0.021.
    const THRESHOLD: f64 = 0.02;

    /// Groups segments of similar colors into candidates.
    ///
    /// # Arguments
    /// * `segments` - The segments of an image.
    /// * `scores` - How much each segment stands out, in the same order.
    ///
    /// # Returns
    /// One candidate for each group of segments, in a fixed order.
    ///
    /// # Panics
    /// Panics if the number of `scores` does not match the number of `segments`.
    pub(crate) fn group(&self, segments: &[Segment], scores: &[f64]) -> Candidates {
        assert_eq!(
            scores.len(),
            segments.len(),
            "the number of scores does not match the number of segments"
        );

        let points = self.embed(segments);
        let clusters = self.clustering.cluster(&points);
        let candidates = clusters
            .iter()
            .map(|cluster| {
                let medoid = cluster.find_medoid(&points);
                Candidate::of_members(segments, scores, cluster.members(), medoid)
            })
            .collect();
        Candidates { candidates }
    }

    /// Embeds the color of each segment where the difference is the Euclidean distance.
    ///
    /// # Arguments
    /// * `segments` - The segments of an image.
    ///
    /// # Returns
    /// The embedded color of each segment with its weight, in the same order.
    fn embed(&self, segments: &[Segment]) -> Vec<Point<3>> {
        segments
            .iter()
            .map(|segment| {
                let coordinates = self.delta_e.embed(segment.color());
                Point::of_vector(coordinates, segment.weight())
            })
            .collect()
    }
}

/// The candidates of an image.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "read by the palette selection, which is not implemented yet"
    )
)]
#[derive(Debug)]
pub(crate) struct Candidates {
    /// The candidates, in a fixed order.
    candidates: Vec<Candidate>,
}

/// A color that may go into the palette, with how much of the image it covers.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "read by the palette selection, which is not implemented yet"
    )
)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct Candidate {
    /// The color of the segment that differs least from the others, weighted by their weights.
    color: Color,
    /// The sum of the weights of the segments.
    weight: f64,
    /// How much the segments stand out, averaged by their weights.
    saliency: f64,
}

impl Candidate {
    /// Summarizes the segments of one group.
    ///
    /// # Arguments
    /// * `segments` - The segments of an image.
    /// * `scores` - How much each segment stands out, in the same order.
    /// * `members` - The indices of the segments in the group.
    /// * `medoid` - The index of the segment that differs least from the others.
    ///
    /// # Returns
    /// The candidate of the segments.
    fn of_members(segments: &[Segment], scores: &[f64], members: &[usize], medoid: usize) -> Self {
        let weight: f64 = members
            .iter()
            .map(|&member| segments[member].weight())
            .sum();
        let weighted_saliency: f64 = members
            .iter()
            .map(|&member| segments[member].weight() * scores[member])
            .sum();
        Self {
            color: segments[medoid].color(),
            weight,
            saliency: weighted_saliency / weight,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::{
        assert_approx_eq,
        next::extraction::{
            clustering::SplitMix64,
            position::Position,
            test_helpers::{cluster_naively, color, gray},
        },
    };

    /// Creates a segment of a color with a weight.
    fn segment(color: Color, weight: f64) -> Segment {
        Segment::new(color, Position::new(0.5, 0.5), weight, 0.0)
    }

    /// Creates segments of random colors close enough to merge, with random weights.
    fn random_segments(count: usize) -> Vec<Segment> {
        let mut random = SplitMix64::new(7);
        (0..count)
            .map(|_| {
                let l = 0.5 + 0.05 * random.next_f64();
                let a = 0.025 * random.next_f64() - 0.0125;
                let b = 0.025 * random.next_f64() - 0.0125;
                let weight = 0.5 + random.next_f64();
                segment(color(l, a, b), weight)
            })
            .collect()
    }

    /// Measures the largest ΔEOKr2 between the colors of two clusters.
    fn measure_farthest_naively(colors: &[Color], first: &[usize], second: &[usize]) -> f64 {
        let mut farthest: f64 = 0.0;
        for &member in first {
            for &other in second {
                let difference = DeltaE::Okr2.measure(colors[member], colors[other]);
                farthest = farthest.max(difference);
            }
        }
        farthest
    }

    #[test]
    fn test_default_clusters_like_naive_complete_linkage() {
        // Arrange
        let segments = random_segments(150);
        let grouping = Grouping::default();
        let points = grouping.embed(&segments);

        // Act
        let actual = grouping.clustering.cluster(&points);

        // Assert
        let colors: Vec<Color> = segments.iter().map(|segment| segment.color()).collect();
        let measure =
            |first: &[usize], second: &[usize]| measure_farthest_naively(&colors, first, second);
        let expected = cluster_naively(colors.len(), Grouping::THRESHOLD, measure);
        let members: Vec<Vec<usize>> = actual
            .iter()
            .map(|cluster| cluster.members().to_vec())
            .collect();
        assert_eq!(members, expected);
    }

    #[rstest]
    #[case::under(0.0095, 1)]
    #[case::over(0.0105, 2)]
    fn test_group_merges_colors_up_to_acceptability(#[case] a: f64, #[case] expected: usize) {
        // Arrange: the a difference doubles in ΔEOKr2.
        let segments = [
            segment(color(0.5, 0.0, 0.0), 1.0),
            segment(color(0.5, a, 0.0), 1.0),
        ];

        // Act
        let actual = Grouping::default().group(&segments, &[0.0; 2]);

        // Assert
        assert_eq!(actual.candidates.len(), expected);
    }

    #[test]
    fn test_group_sums_weights() {
        // Arrange
        let segments = [segment(gray(0.5), 1.0), segment(gray(0.505), 3.0)];

        // Act
        let actual = Grouping::default().group(&segments, &[0.0; 2]);

        // Assert
        assert_eq!(actual.candidates[0].weight, 4.0);
    }

    #[test]
    fn test_group_averages_saliency_by_weight() {
        // Arrange
        let segments = [segment(gray(0.5), 1.0), segment(gray(0.505), 3.0)];

        // Act
        let actual = Grouping::default().group(&segments, &[0.2, 0.6]);

        // Assert
        assert_approx_eq!(actual.candidates[0].saliency, 0.5);
    }

    #[test]
    fn test_group_takes_color_of_medoid() {
        // Arrange
        let segments = [
            segment(gray(0.5), 1.0),
            segment(gray(0.505), 1.0),
            segment(gray(0.51), 1.0),
        ];

        // Act
        let actual = Grouping::default().group(&segments, &[0.0; 3]);

        // Assert
        assert_eq!(actual.candidates[0].color.to_oklab().l, 0.505);
    }

    #[test]
    fn test_group_empty() {
        // Act
        let actual = Grouping::default().group(&[], &[]);

        // Assert
        assert!(actual.candidates.is_empty());
    }

    #[test]
    #[should_panic(expected = "the number of scores does not match the number of segments")]
    fn test_group_rejects_scores_of_other_count() {
        // Arrange
        let segments = [segment(gray(0.5), 1.0)];

        // Act
        Grouping::default().group(&segments, &[]);
    }
}

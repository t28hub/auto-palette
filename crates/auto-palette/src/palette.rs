use std::cmp::Reverse;

use crate::{
    algorithm::Algorithm,
    color::{rgb_to_xyz, xyz_to_lab, Color, Lab, D65},
    error::Error,
    image::ImageData,
    math::{
        clustering::{Cluster, ClusteringAlgorithm, DBSCAN},
        denormalize,
        normalize,
        DistanceMetric,
        FloatNumber,
        Point,
        SamplingStrategy,
    },
    theme::Theme,
    Swatch,
};

/// Palette struct representing a collection of swatches.
///
/// # Type Parameters
/// * `T` - The floating point type.
#[derive(Debug, Clone, PartialEq)]
pub struct Palette<T>
where
    T: FloatNumber,
{
    swatches: Vec<Swatch<T>>,
}

impl<T> Palette<T>
where
    T: FloatNumber,
{
    /// Creates a new `Palette` instance.
    ///
    /// # Arguments
    /// * `swatches` - The swatches of the palette.
    ///
    /// # Returns
    /// A new `Palette` instance.
    #[must_use]
    pub fn new(swatches: Vec<Swatch<T>>) -> Self {
        Self { swatches }
    }

    /// Returns the number of swatches in the palette.
    ///
    /// # Returns
    /// The number of swatches in the palette.
    #[must_use]
    pub fn len(&self) -> usize {
        self.swatches.len()
    }

    /// Returns whether the palette is empty.
    ///
    /// # Returns
    /// `true` if the palette is empty, otherwise `false`.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.swatches.is_empty()
    }

    /// Returns the swatches in the palette.
    ///
    /// # Returns
    /// The swatches in the palette.
    #[must_use]
    pub fn swatches(&self) -> &[Swatch<T>] {
        &self.swatches
    }

    /// Finds the swatches in the palette based on the theme.
    ///
    /// # Arguments
    /// * `n` - The number of swatches to find.
    ///
    /// # Returns
    /// The swatches in the palette.
    #[must_use]
    pub fn find_swatches(&self, n: usize) -> Vec<Swatch<T>> {
        let Some(max_population) = self
            .swatches
            .first()
            .map(|swatch| T::from_usize(swatch.population()))
        else {
            return Vec::new();
        };

        let mut colors = Vec::with_capacity(self.swatches.len());
        let mut weights = Vec::with_capacity(self.swatches.len());
        for swatch in &self.swatches {
            let color = swatch.color();
            colors.push([color.l, color.a, color.b]);

            let weight = T::from_usize(swatch.population()) / max_population;
            weights.push(weight);
        }
        self.find_swatches_with_weights(n, colors, weights)
    }

    /// Finds the swatches in the palette based on the theme.
    ///
    /// # Arguments
    /// * `n` - The number of swatches to find.
    /// * `theme` - The theme to use.
    ///
    /// # Returns
    /// The swatches in the palette based on the theme.
    #[must_use]
    pub fn find_swatches_with_theme(&self, n: usize, theme: Theme) -> Vec<Swatch<T>> {
        let mut colors = Vec::with_capacity(self.swatches.len());
        let mut weights = Vec::with_capacity(self.swatches.len());
        for swatch in &self.swatches {
            let color = swatch.color();
            colors.push([color.l, color.a, color.b]);

            let weight = theme.score(color);
            weights.push(weight);
        }
        self.find_swatches_with_weights(n, colors, weights)
    }

    #[must_use]
    fn find_swatches_with_weights(
        &self,
        n: usize,
        colors: Vec<Point<T, 3>>,
        weights: Vec<T>,
    ) -> Vec<Swatch<T>> {
        let sampling = SamplingStrategy::WeightedFarthestPointSampling::<T>(weights);
        sampling
            .sample(&colors, n)
            .iter()
            .map(|&index| self.swatches[index])
            .collect()
    }

    /// Extracts the palette from the image data. The default clustering algorithm is DBSCAN.
    ///
    /// # Arguments
    /// * `image_data` - The image data to extract the palette from.
    ///
    /// # Returns
    /// The extracted palette.
    pub fn extract(image_data: &ImageData) -> Result<Self, Error> {
        Self::extract_with_algorithm(image_data, Algorithm::DBSCAN)
    }

    /// Extracts the palette from the image data with the given algorithm.
    ///
    /// # Arguments
    /// * `image_data` - The image data to extract the palette from.
    /// * `algorithm` - The clustering algorithm to use.
    ///
    /// # Returns
    /// The extracted palette.
    pub fn extract_with_algorithm(
        image_data: &ImageData,
        algorithm: Algorithm,
    ) -> Result<Self, Error> {
        let pixels = image_data.data();
        if pixels.is_empty() {
            return Err(Error::EmptyImageData);
        }

        let width = image_data.width();
        let height = image_data.height();
        let pixel_clusters = cluster_foo(width as usize, height as usize, pixels, algorithm);
        let color_clusters = cluster_foo_bar(&pixel_clusters);

        let mut swatches = convert_to_swatches(
            T::from_u32(width),
            T::from_u32(height),
            &color_clusters,
            &pixel_clusters,
        );
        swatches.sort_by_key(|swatch| Reverse(swatch.population()));
        Ok(Self { swatches })
    }
}

#[must_use]
fn cluster_foo<T>(
    width: usize,
    height: usize,
    data: &[u8],
    algorithm: Algorithm,
) -> Vec<Cluster<T, 5>>
where
    T: FloatNumber,
{
    let width_f = T::from_usize(width);
    let height_f = T::from_usize(height);
    let points = data
        .chunks(4)
        .enumerate()
        .filter_map(|(index, pixel)| {
            // Ignore transparent pixels.
            if pixel[3] == 0 {
                None
            } else {
                let (x, y, z) = rgb_to_xyz::<T>(pixel[0], pixel[1], pixel[2]);
                let (l, a, b) = xyz_to_lab::<T, D65>(x, y, z);
                let x = T::from_usize(index % width);
                let y = T::from_usize(index / width);
                Some([
                    normalize(l, Lab::min_l(), Lab::max_l()),
                    normalize(a, Lab::min_a(), Lab::max_a()),
                    normalize(b, Lab::min_b(), Lab::max_b()),
                    normalize(x, T::zero(), width_f),
                    normalize(y, T::zero(), height_f),
                ])
            }
        })
        .collect::<Vec<_>>();
    algorithm.cluster::<T>(&points)
}

#[must_use]
fn cluster_foo_bar<T>(pixel_clusters: &[Cluster<T, 5>]) -> Vec<Cluster<T, 3>>
where
    T: FloatNumber,
{
    let colors = pixel_clusters
        .iter()
        .map(|cluster| -> Point<T, 3> {
            let centroid = cluster.centroid();
            [
                denormalize(centroid[0], Lab::min_l(), Lab::max_l()),
                denormalize(centroid[1], Lab::min_a(), Lab::max_a()),
                denormalize(centroid[2], Lab::min_b(), Lab::max_b()),
            ]
        })
        .collect::<Vec<_>>();
    let algorithm = DBSCAN::new(1, T::from_f32(2.5), DistanceMetric::Euclidean).unwrap();
    algorithm.fit(&colors)
}

#[must_use]
fn convert_to_swatches<T>(
    width: T,
    height: T,
    color_clusters: &[Cluster<T, 3>],
    pixel_clusters: &[Cluster<T, 5>],
) -> Vec<Swatch<T>>
where
    T: FloatNumber,
{
    color_clusters
        .iter()
        .fold(Vec::new(), |mut acc, color_cluster| {
            let mut best_color = [T::zero(); 3];
            let mut best_position = (0, 0);
            let mut best_population = 0;
            let mut total_population = 0;
            for &member in color_cluster.members() {
                let Some(pixel_cluster) = pixel_clusters.get(member) else {
                    continue;
                };

                if pixel_cluster.is_empty() {
                    continue;
                }

                let fraction = T::from_usize(pixel_cluster.len())
                    / T::from_usize(pixel_cluster.len() + best_population);
                let centroid = pixel_cluster.centroid();
                best_color[0] += fraction * (centroid[0] - best_color[0]);
                best_color[1] += fraction * (centroid[1] - best_color[1]);
                best_color[2] += fraction * (centroid[2] - best_color[2]);

                if fraction >= T::from_f32(0.5) {
                    best_position.0 = denormalize(centroid[3], T::zero(), width).to_u32_unsafe();
                    best_position.1 = denormalize(centroid[4], T::zero(), height).to_u32_unsafe();
                    best_population = pixel_cluster.len();
                }
                total_population += pixel_cluster.len();
            }

            let l = denormalize(best_color[0], Lab::min_l(), Lab::max_l());
            let a = denormalize(best_color[1], Lab::min_a(), Lab::max_a());
            let b = denormalize(best_color[2], Lab::min_b(), Lab::max_b());
            acc.push(Swatch::new(
                Color::new(l, a, b),
                best_position,
                total_population,
            ));
            acc
        })
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rstest::rstest;

    use super::*;

    #[must_use]
    fn sample_swatches<T>() -> Vec<Swatch<T>>
    where
        T: FloatNumber,
    {
        vec![
            Swatch::new(Color::from_str("#FFFFFF").unwrap(), (159, 106), 61228),
            Swatch::new(Color::from_str("#EE334E").unwrap(), (238, 89), 1080),
            Swatch::new(Color::from_str("#0081C8").unwrap(), (82, 88), 1064),
            Swatch::new(Color::from_str("#00A651").unwrap(), (197, 123), 1037),
            Swatch::new(Color::from_str("#000000").unwrap(), (157, 95), 1036),
            Swatch::new(Color::from_str("#FCB131").unwrap(), (119, 123), 1005),
        ]
    }

    #[must_use]
    fn empty_swatches<T>() -> Vec<Swatch<T>>
    where
        T: FloatNumber,
    {
        vec![]
    }

    #[test]
    fn test_new() {
        // Act
        let swatches = vec![
            Swatch::<f32>::new(Color::from_str("#FFFFFF").unwrap(), (5, 10), 256),
            Swatch::<f32>::new(Color::from_str("#C8102E").unwrap(), (15, 20), 128),
            Swatch::<f32>::new(Color::from_str("#012169").unwrap(), (30, 30), 64),
        ];
        let actual = Palette::new(swatches.clone());

        // Assert
        assert!(!actual.is_empty());
        assert_eq!(actual.len(), 3);
        assert_eq!(actual.swatches, swatches);
    }

    #[test]
    fn test_new_empty() {
        // Act
        let swatches = vec![];
        let actual: Palette<f32> = Palette::new(swatches.clone());

        // Assert
        assert!(actual.is_empty());
        assert_eq!(actual.len(), 0);
    }

    #[test]
    #[cfg(feature = "image")]
    fn test_extract() {
        // Act
        let image_data = ImageData::load("../../gfx/olympic_logo.png").unwrap();
        let actual: Palette<f32> = Palette::extract(&image_data).unwrap();

        // Assert
        assert!(!actual.is_empty());
        assert!(actual.len() >= 3);
    }

    #[cfg(feature = "image")]
    #[rstest]
    #[case::kmeans("kmeans")]
    #[case::dbscan("dbscan")]
    #[case::dbscanpp("dbscan++")]
    fn test_extract_with_algorithm(#[case] name: &str) {
        // Act
        let image_data = ImageData::load("../../gfx/olympic_logo.png").unwrap();
        let algorithm = Algorithm::from_str(name).unwrap();
        let actual: Palette<f32> = Palette::extract_with_algorithm(&image_data, algorithm).unwrap();

        // Assert
        assert!(!actual.is_empty());
        assert!(actual.len() >= 5);
    }

    #[test]
    fn test_extract_empty_image_data() {
        // Act
        let data = Vec::<u8>::new();
        let image_data = ImageData::new(0, 0, &data).unwrap();
        let result = Palette::<f32>::extract(&image_data);

        // Assert
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "The image data is empty and cannot be processed."
        );
    }

    #[test]
    fn test_find_swatches() {
        // Arrange
        let swatches = sample_swatches::<f32>();
        let palette = Palette::new(swatches.clone());

        // Act
        let mut actual = palette.find_swatches(3);
        actual.sort_by_key(|swatch| Reverse(swatch.population()));

        // Assert
        assert_eq!(actual.len(), 3);
        assert_eq!(actual[0].color().to_hex_string(), "#FFFFFF");
        assert_eq!(actual[1].color().to_hex_string(), "#EE334E");
        assert_eq!(actual[2].color().to_hex_string(), "#000000");
    }

    #[test]
    fn test_find_swatches_empty() {
        // Arrange
        let swatches = empty_swatches::<f32>();
        let palette = Palette::new(swatches.clone());

        // Act
        let actual = palette.find_swatches(10);

        // Assert
        assert!(actual.is_empty());
    }

    #[rstest]
    #[case::basic(Theme::Basic, vec ! ["#0081C8", "#FCB131"])]
    #[case::vivid(Theme::Vivid, vec ! ["#EE334E", "#00A651"])]
    #[case::muted(Theme::Muted, vec ! ["#FFFFFF", "#000000"])]
    #[case::light(Theme::Light, vec ! ["#FFFFFF", "#FCB131"])]
    #[case::dark(Theme::Dark, vec ! ["#FFFFFF", "#000000"])]
    fn test_find_swatches_with_theme(#[case] theme: Theme, #[case] expected: Vec<&str>) {
        // Arrange
        let swatches = sample_swatches::<f32>();
        let palette = Palette::new(swatches.clone());

        // Act
        let mut actual = palette.find_swatches_with_theme(2, theme);
        actual.sort_by_key(|swatch| Reverse(swatch.population()));

        // Assert
        assert_eq!(actual.len(), 2);
        assert_eq!(actual[0].color().to_hex_string(), expected[0]);
        assert_eq!(actual[1].color().to_hex_string(), expected[1]);
    }
}

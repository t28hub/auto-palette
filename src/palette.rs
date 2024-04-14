use std::cmp::Reverse;

use crate::algorithm::Algorithm;
use crate::color::{rgb_to_xyz, xyz_to_lab, Lab, D65};
use crate::errors::PaletteError;
use crate::image::ImageData;
use crate::math::clustering::{ClusteringAlgorithm, DBSCAN};
use crate::math::{DistanceMetric, Normalizable, Point5D, SamplingStrategy};
use crate::{Color, Swatch};

/// Palette struct that contains a list of swatches.
#[derive(Debug)]
pub struct Palette {
    swatches: Vec<Swatch>,
}

impl Palette {
    /// Creates a new `Palette` instance.
    ///
    /// # Arguments
    /// * `swatches` - The swatches of the palette.
    ///
    /// # Returns
    /// A new `Palette` instance.
    #[must_use]
    pub fn new(swatches: Vec<Swatch>) -> Self {
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

    /// Finds the swatches in the palette.
    ///
    /// # Arguments
    /// * `n` - The number of swatches to find.
    ///
    /// # Returns
    /// The swatches in the palette.
    #[must_use]
    pub fn find_swatches(&self, n: usize) -> Vec<Swatch> {
        let mut colors = Vec::with_capacity(self.swatches.len());
        let mut weights = Vec::with_capacity(self.swatches.len());
        for swatch in &self.swatches {
            let color = swatch.color();
            colors.push([color.l, color.a, color.b]);

            let chroma = color.chroma();
            if chroma < 60.0 {
                weights.push(0.0);
            } else {
                weights.push(chroma / 180.0);
            }
        }
        let sampling =
            SamplingStrategy::WeightedFarthestPointSampling(DistanceMetric::Euclidean, weights);
        let sampled = sampling.sample(&colors, n);
        sampled.iter().map(|&index| self.swatches[index]).collect()
    }

    /// Extracts the palette from the image data. The default clustering algorithm is DBSCAN.
    ///
    /// # Arguments
    /// * `image_data` - The image data to extract the palette from.
    ///
    /// # Returns
    /// The extracted palette.
    pub fn extract(image_data: &ImageData) -> Result<Self, PaletteError> {
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
    ) -> Result<Self, PaletteError> {
        let pixels = image_data.pixels();
        if pixels.is_empty() {
            return Err(PaletteError::EmptyImageData);
        }

        let points = convert_to_pixels(image_data);
        let pixel_clusters = algorithm.cluster(&points);
        let colors: Vec<_> = pixel_clusters
            .iter()
            .map(|cluster| {
                let centroid = cluster.centroid();
                [
                    centroid[0].denormalize(Lab::MIN_L, Lab::MAX_L),
                    centroid[1].denormalize(Lab::MIN_A, Lab::MAX_A),
                    centroid[2].denormalize(Lab::MIN_B, Lab::MAX_B),
                ]
            })
            .collect();

        let algorithm = DBSCAN::new(1, 2.5, DistanceMetric::Euclidean).unwrap();
        let color_clusters = algorithm.fit(&colors);
        let mut swatches = color_clusters
            .iter()
            .fold(Vec::new(), |mut acc, color_cluster| {
                let mut best_color = [0.0, 0.0, 0.0];
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

                    let fraction =
                        pixel_cluster.len() as f32 / (pixel_cluster.len() + best_population) as f32;
                    let centroid = pixel_cluster.centroid();
                    best_color[0] += fraction * (centroid[0] - best_color[0]);
                    best_color[1] += fraction * (centroid[1] - best_color[1]);
                    best_color[2] += fraction * (centroid[2] - best_color[2]);

                    if fraction >= 0.5 {
                        best_position.0 =
                            centroid[3].denormalize(0.0, image_data.width() as f32) as u32;
                        best_position.1 =
                            centroid[4].denormalize(0.0, image_data.height() as f32) as u32;
                        best_population = pixel_cluster.len();
                    }
                    total_population += pixel_cluster.len();
                }

                let l = best_color[0].denormalize(Lab::MIN_L, Lab::MAX_L);
                let a = best_color[1].denormalize(Lab::MIN_A, Lab::MAX_A);
                let b = best_color[2].denormalize(Lab::MIN_B, Lab::MAX_B);
                acc.push(Swatch::new(
                    Color::new(l, a, b),
                    best_position,
                    total_population,
                ));
                acc
            });
        swatches.sort_by_key(|swatch| Reverse(swatch.population()));
        Ok(Self { swatches })
    }
}

#[must_use]
fn convert_to_pixels(image_data: &ImageData) -> Vec<Point5D> {
    let width = image_data.width() as usize;
    let height = image_data.height() as usize;
    image_data
        .pixels()
        .chunks(4)
        .enumerate()
        .filter_map(|(index, pixel)| {
            // Ignore transparent pixels.
            if pixel[3] == 0 {
                None
            } else {
                let (x, y, z) = rgb_to_xyz(pixel[0], pixel[1], pixel[2]);
                let (l, a, b) = xyz_to_lab::<D65>(x, y, z);
                let x = index % width;
                let y = index / width;
                Some([
                    l.normalize(Lab::MIN_L, Lab::MAX_L),
                    a.normalize(Lab::MIN_A, Lab::MAX_A),
                    b.normalize(Lab::MIN_B, Lab::MAX_B),
                    (x as f32).normalize(0.0, width as f32),
                    (y as f32).normalize(0.0, height as f32),
                ])
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_new_palette() {
        // Act
        let swatches = vec![
            Swatch::new(Color::from_str("#FFFFFF").unwrap(), (5, 10), 256),
            Swatch::new(Color::from_str("#C8102E").unwrap(), (15, 20), 128),
            Swatch::new(Color::from_str("#012169").unwrap(), (30, 30), 64),
        ];
        let palette = Palette::new(swatches.clone());

        // Assert
        assert!(!palette.is_empty());
        assert_eq!(palette.len(), 3);
        assert_eq!(palette.swatches, swatches);
    }

    #[test]
    fn test_new_palette_empty() {
        // Act
        let swatches = vec![];
        let palette = Palette::new(swatches.clone());

        // Assert
        assert!(palette.is_empty());
        assert_eq!(palette.len(), 0);
    }

    #[test]
    fn test_extract() {
        // Arrange
        let image_data = ImageData::load("./tests/assets/flag_uk.png").unwrap();

        // Act
        let palette = Palette::extract(&image_data).unwrap();

        // Assert
        assert!(!palette.is_empty());
        assert!(palette.len() >= 3);
    }

    #[test]
    fn test_extract_with_algorithm() {
        // Arrange
        let image_data =
            ImageData::load("./tests/assets/holly-booth-hLZWGXy5akM-unsplash.jpg").unwrap();

        // Act
        let palette = Palette::extract_with_algorithm(&image_data, Algorithm::KMeans).unwrap();

        // Assert
        assert!(!palette.is_empty());
        assert!(palette.len() >= 3);
    }

    #[test]
    fn test_extract_empty_image_data() {
        // Arrange
        let image_data = ImageData::new(0, 0, vec![]).unwrap();

        // Act
        let result = Palette::extract(&image_data);

        // Assert
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PaletteError::EmptyImageData);
    }
}

use crate::color::lab::{from_xyz, to_xyz};
use crate::color::xyz::{from_rgb, to_rgb};
use crate::errors::PaletteError;
use crate::image::ImageData;
use crate::math::clustering::kmeans::Kmeans;
use crate::math::clustering::strategy::InitializationStrategy;
use crate::math::metrics::DistanceMetric;
use crate::math::point::Point3D;
use crate::Swatch;

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
        self.swatches.iter().take(n).copied().collect()
    }

    /// Extracts the palette from the image data.
    ///
    /// # Arguments
    /// * `image_data` - The image data.
    ///
    /// # Returns
    /// The extracted palette.
    pub fn extract(image_data: &ImageData) -> Result<Self, PaletteError> {
        let pixels = image_data.pixels();
        if pixels.is_empty() {
            return Err(PaletteError::EmptyImageData);
        }

        let points: Vec<Point3D> = image_data
            .pixels()
            .chunks(4)
            .filter_map(|pixel| {
                // Ignore transparent pixels.
                if pixel[3] == 0 {
                    None
                } else {
                    let (x, y, z) = from_rgb(pixel[0], pixel[1], pixel[2]);
                    let (l, a, b) = from_xyz(x, y, z);
                    Some([l, a, b])
                }
            })
            .collect();

        let strategy = InitializationStrategy::KmeansPlusPlus(
            rand::thread_rng(),
            DistanceMetric::SquaredEuclidean,
        );
        let clustering = Kmeans::new(16, 10, 1e-3, DistanceMetric::SquaredEuclidean, strategy)
            .map_err(PaletteError::ExtractionError)?;

        let clusters = clustering.fit(&points);
        let swatches = clusters
            .iter()
            .map(|cluster| {
                let centroid = cluster.centroid();
                let (x, y, z) = to_xyz(centroid[0], centroid[1], centroid[2]);
                let rgb = to_rgb(x, y, z);
                Swatch::new(rgb, cluster.len())
            })
            .collect();
        Ok(Self { swatches })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_palette() {
        // Act
        let swatches = vec![
            Swatch::new((255, 255, 255), 256),
            Swatch::new((200, 16, 46), 128),
            Swatch::new((1, 33, 105), 64),
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
        let image_data = ImageData::open("./tests/assets/flag_uk.png").unwrap();

        // Act
        let palette = Palette::extract(&image_data).unwrap();

        // Assert
        assert!(!palette.is_empty());
        assert!(palette.len() >= 3);
    }
}
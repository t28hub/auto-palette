use crate::color::lab::{from_xyz, to_xyz};
use crate::color::xyz::{from_rgb, to_rgb};
use crate::image::ImageData;
use crate::math::clustering::kmeans::Kmeans;
use crate::math::clustering::strategy::InitializationStrategy;
use crate::math::metrics::DistanceMetric;
use crate::math::point::Point3D;
use crate::Swatch;

#[derive(Debug)]
pub struct Palette {
    swatches: Vec<Swatch>,
}

impl Palette {
    pub fn new(swatches: Vec<Swatch>) -> Result<Self, ()> {
        if swatches.is_empty() {
            return Err(());
        }
        Ok(Self { swatches })
    }

    pub fn extract(image_data: &ImageData) -> Result<Self, ()> {
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

        let clustering = Kmeans::new(
            16,
            10,
            1e-3,
            DistanceMetric::SquaredEuclidean,
            InitializationStrategy::KmeansPlusPlus(
                rand::thread_rng(),
                DistanceMetric::SquaredEuclidean,
            ),
        )
        .expect("Failed to create the K-means algorithm.");
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

    pub fn swatches(&self, count: usize) -> Vec<Swatch> {
        self.swatches.iter().take(count).copied().collect()
    }
}

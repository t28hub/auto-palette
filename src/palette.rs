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
                let r = pixel[0];
                let g = pixel[1];
                let b = pixel[2];
                let a = pixel[3];
                if a == 0 {
                    None
                } else {
                    Some([r as f32, g as f32, b as f32])
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
                let color = (centroid[0] as u8, centroid[1] as u8, centroid[2] as u8);
                Swatch::new(color, cluster.len())
            })
            .collect();
        Ok(Self { swatches })
    }

    pub fn swatches(&self, count: usize) -> Vec<Swatch> {
        self.swatches.iter().take(count).cloned().collect()
    }
}

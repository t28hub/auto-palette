use crate::color::lab::Lab;
use crate::color::rgba::Rgba;
use crate::color::white_point::D65;
use crate::color::xyz::XYZ;
use crate::math::clustering::clustering::Clustering;
use crate::math::clustering::dbscan::clustering::DBSCAN;
use crate::math::clustering::dbscan::params::DBSCANParams;
use crate::math::clustering::hierarchical::clustering::HierarchicalClustering;
use crate::math::distance::metric::DistanceMetric;
use crate::math::number::Float;
use crate::math::point::{Point3, Point5};
use crate::swatch::Swatch;
use std::collections::HashMap;

pub struct Palette<F: Float> {
    width: F,
    height: F,
    clustering: DBSCAN<F, Point5<F>>,
}

impl<F> Palette<F>
where
    F: Float,
{
    #[must_use]
    pub fn generate(image_data: &[u8], width: u32, height: u32) -> Palette<F> {
        let width_u = width as usize;
        let width_f = F::from_u32(width);

        let height_u = height as usize;
        let height_f = F::from_u32(height);

        let mut index = 0;
        let mut pixels = Vec::with_capacity(image_data.len() % 4);
        while index < image_data.len() {
            let rgba = Rgba::new(
                image_data[index],
                image_data[index + 1],
                image_data[index + 2],
                image_data[index + 3],
            );
            let xyz: XYZ<F, D65> = XYZ::from(&rgba);
            let lab: Lab<F, D65> = Lab::from(&xyz);

            let x = F::from_usize((index / 4) % width_u);
            let y = F::from_usize((index / 4 / width_u) % height_u);
            pixels.push(Point5::new(
                Self::normalize(lab.l, Lab::<F>::min_l(), Lab::<F>::max_l()),
                Self::normalize(lab.a, Lab::<F>::min_a(), Lab::<F>::max_a()),
                Self::normalize(lab.b, Lab::<F>::min_b(), Lab::<F>::max_b()),
                Self::normalize(x, F::zero(), width_f),
                Self::normalize(y, F::zero(), height_f),
            ));
            index += 4;
        }

        let params = DBSCANParams::new(9, F::from_f64(0.0025), DistanceMetric::SquaredEuclidean);
        let clustering = DBSCAN::fit(&pixels, &params);
        Self {
            width: width_f,
            height: height_f,
            clustering,
        }
    }

    #[must_use]
    pub fn swatches(&self, n: usize) -> Vec<Swatch<F>> {
        let clusters = self.clustering.clusters();
        let centroids: Vec<Point3<F>> = clusters
            .iter()
            .map(|cluster| {
                let centroid = cluster.centroid();
                Point3::new(centroid.0, centroid.1, centroid.2)
            })
            .collect();
        let hierarchical_clustering = HierarchicalClustering::fit(&centroids, |u, v| {
            let point_u = centroids[u];
            let point_v = centroids[v];
            DistanceMetric::SquaredEuclidean.measure(&point_u, &point_v)
        });

        let size = self.width * self.height;
        let mut swatches = HashMap::new();
        for (index, label) in hierarchical_clustering.partition(n).into_iter().enumerate() {
            let mut swatch = swatches.entry(label).or_insert_with(|| Swatch::default());

            if let Some(cluster) = clusters.get(index) {
                let percentage = F::from_usize(cluster.size()) / size;
                if percentage < swatch.percentage {
                    continue;
                }

                let centroid = cluster.centroid();
                let lab = Lab::new(
                    Self::denormalize(centroid[0], Lab::<F>::min_l(), Lab::<F>::max_l()),
                    Self::denormalize(centroid[1], Lab::<F>::min_a(), Lab::<F>::max_a()),
                    Self::denormalize(centroid[2], Lab::<F>::min_b(), Lab::<F>::max_b()),
                );
                let xyz = XYZ::from(&lab);
                let rgba = Rgba::from(&xyz);
                swatch.color = (rgba.r, rgba.g, rgba.b);

                let x = Self::denormalize(centroid[3], F::zero(), self.width);
                let y = Self::denormalize(centroid[4], F::zero(), self.height);
                swatch.position = (
                    x.to_u32().expect("Could not convert x to u32"),
                    y.to_u32().expect("Could not convert y to u32"),
                );
                swatch.percentage = percentage;
            }
        }
        swatches.into_values().collect()
    }

    #[must_use]
    fn normalize(value: F, min: F, max: F) -> F {
        assert!(min < max);
        (value - min) / (max - min)
    }

    #[inline]
    #[must_use]
    fn denormalize(normalized: F, min: F, max: F) -> F {
        assert!(min < max);
        normalized * (max - min) + min
    }
}

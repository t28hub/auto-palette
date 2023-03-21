use crate::color::lab::Lab;
use crate::color::rgba::Rgba;
use crate::color::white_point::D65;
use crate::color::xyz::XYZ;
use crate::math::clustering::algorithm::Algorithm;
use crate::math::clustering::hdbscan::algorithm::HDBSCAN;
use crate::math::clustering::hdbscan::params::Params;
use crate::math::distance::metric::DistanceMetric::SquaredEuclidean;
use crate::math::number::Float;
use crate::math::point::Point5;
use crate::swatch::Swatch;
use std::cmp::Ordering;

pub struct Palette<F: Float> {
    swatches: Vec<Swatch<F>>,
}

impl<F> Palette<F>
where
    F: Float,
{
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

        let mut swatches = Self::extract(&pixels, width_f, height_f);
        swatches.sort_unstable_by(|swatch1, swatch2| {
            swatch1
                .percentage
                .partial_cmp(&swatch2.percentage)
                .unwrap_or(Ordering::Greater)
        });
        Self { swatches }
    }

    pub fn swatches(&self) -> &[Swatch<F>] {
        &self.swatches
    }

    fn extract(pixels: &[Point5<F>], width: F, height: F) -> Vec<Swatch<F>> {
        let params = Params::new(9, 9, SquaredEuclidean);
        let hdbscan = HDBSCAN::fit(&pixels, &params);
        let mut swatches = Vec::with_capacity(hdbscan.clusters().len());
        for cluster in hdbscan.clusters().into_iter() {
            let centroid = cluster.centroid();
            let lab = Lab::new(
                Self::denormalize(centroid[0], Lab::<F>::min_l(), Lab::<F>::max_l()),
                Self::denormalize(centroid[1], Lab::<F>::min_a(), Lab::<F>::max_a()),
                Self::denormalize(centroid[2], Lab::<F>::min_b(), Lab::<F>::max_b()),
            );
            let xyz = XYZ::from(&lab);
            let rgba = Rgba::from(&xyz);
            let color = (rgba.r, rgba.g, rgba.b);

            let x = Self::denormalize(centroid[3], F::zero(), width);
            let y = Self::denormalize(centroid[4], F::zero(), height);
            let position = (
                x.to_u32().expect("Could not convert x to u32"),
                y.to_u32().expect("Could not convert y to u32"),
            );

            let count = cluster.size();
            let percentage = F::from_usize(count) / F::from_usize(pixels.len());
            swatches.push(Swatch {
                color,
                position,
                percentage,
            })
        }
        swatches
    }

    #[inline]
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

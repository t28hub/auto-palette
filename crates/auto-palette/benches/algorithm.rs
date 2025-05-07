use std::str::FromStr;

use auto_palette::{Algorithm, FloatNumber, ImageData, Palette};
use divan::Bencher;

fn main() {
    divan::main();
}

const IMAGE_PATH: &str = "../../gfx/laura-clugston-pwW2iV9TZao-unsplash.jpg";

const ALGORITHMS: &[&str] = &["kmeans", "dbscan", "dbscan++"];

#[divan::bench(types = [f32, f64], args = ALGORITHMS, sample_count = 10)]
fn bench_algorithm<T>(bencher: Bencher, name: &str)
where
    T: FloatNumber,
{
    bencher
        .with_inputs(|| {
            let image_data = ImageData::load(IMAGE_PATH)
                .expect(format!("Failed to load image: {}", IMAGE_PATH).as_str());
            let algorithm = Algorithm::from_str(name)
                .expect(format!("Failed to parse algorithm: {}", name).as_str());
            (image_data, algorithm)
        })
        .bench_refs(|(image_data, algorithm)| {
            Palette::<T>::builder()
                .algorithm(algorithm.clone())
                .build(image_data)
                .expect(format!("Failed to build palette with algorithm: {}", name).as_str());
        });
}

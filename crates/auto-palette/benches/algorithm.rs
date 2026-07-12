use std::str::FromStr;

use auto_palette::{Algorithm, ImageData, Palette};
use divan::Bencher;

fn main() {
    divan::main();
}

const IMAGE_PATH: &str = "../../gfx/laura-clugston-pwW2iV9TZao-unsplash.jpg";

const ALGORITHMS: &[&str] = &["kmeans", "dbscan", "dbscan++", "slic", "snic"];

#[divan::bench(args = ALGORITHMS, sample_count = 10)]
fn bench_algorithm(bencher: Bencher, name: &str) {
    bencher
        .with_inputs(|| {
            let image_data = ImageData::load(IMAGE_PATH)
                .unwrap_or_else(|_| panic!("Failed to load image: {}", IMAGE_PATH));
            let algorithm = Algorithm::from_str(name)
                .unwrap_or_else(|_| panic!("Failed to parse algorithm: {}", name));
            (image_data, algorithm)
        })
        .bench_refs(|(image_data, algorithm)| {
            Palette::builder()
                .algorithm(*algorithm)
                .build(image_data)
                .unwrap_or_else(|_| panic!("Failed to build palette with algorithm: {}", name));
        });
}

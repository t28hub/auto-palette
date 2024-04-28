#![deny(warnings)]

use auto_palette::{Algorithm, ImageData, Palette, Theme};
use std::time::Instant;

/// Extracts a palette from an image file using the DBSCAN algorithm.
fn main() {
    let Ok(image_data) = ImageData::load("tests/assets/holly-booth-hLZWGXy5akM-unsplash.jpg")
    else {
        eprintln!(
            "Failed to load image data from 'tests/assets/holly-booth-hLZWGXy5akM-unsplash.jpg'"
        );
        return;
    };

    let start = Instant::now();
    let Ok(palette) = Palette::<f32>::extract_with_algorithm(&image_data, Algorithm::DBSCAN) else {
        eprintln!("Failed to extract palette with DBSCAN algorithm from image data");
        return;
    };

    let duration = start.elapsed();
    println!(
        "Extracted {} swatches in {}.{:03} seconds",
        palette.len(),
        duration.as_secs(),
        duration.subsec_millis()
    );

    println!("Color\tPosition\tPopulation");
    let swatches = palette.find_swatches_with_theme(6, Theme::Basic);
    for swatch in swatches {
        let color = swatch.color();
        let rgb = color.to_rgb();
        print!("\x1b[38;2;{};{};{}m", rgb.r, rgb.g, rgb.b);
        let (x, y) = swatch.position();
        let population = swatch.population();
        println!(
            "{}\t({:.2}, {:.2})\t{}",
            color.to_hex_string(),
            x,
            y,
            population
        );
        print!("\x1b[0m");
    }
}

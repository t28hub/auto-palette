#![deny(warnings)]

use std::time::Instant;

use auto_palette::{ImageData, Palette};

/// Extracts a palette from an image file.
///
/// The image path can be provided as a command line argument.
/// ```sh
/// cargo run --example image_path --release -- tests/assets/holly-booth-hLZWGXy5akM-unsplash.jpg
/// ```
fn main() {
    // Read the image path from the command line arguments
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        println!("No image path provided, using the default image path");
        "tests/assets/holly-booth-hLZWGXy5akM-unsplash.jpg".into()
    });

    let image_data = ImageData::load(path.clone())
        .map_err(|e| format!("Failed to load the image data from '{}': {}", path, e))?;

    println!(
        "Loaded the image with dimensions {}x{}",
        image_data.width(),
        image_data.height()
    );

    // Extract the palette from the image data
    let start = Instant::now();
    let palette: Palette<f32> = Palette::extract(&image_data)
        .map_err(|e| format!("Failed to extract the palette: {}", e))?;
    let duration = start.elapsed();
    println!(
        "Extracted {} swatch(es) in {}.{:03} seconds",
        palette.len(),
        duration.as_secs(),
        duration.subsec_millis()
    );

    // Find the top 5 swatches in the palette
    let swatches = palette.find_swatches(5);
    println!("#  Color\tPosition\tPopulation");
    for (i, swatch) in swatches.iter().enumerate() {
        print!("{}  ", i + 1);

        let color = swatch.color();
        if color.is_light() {
            print!("\x1b[1;30m");
        } else {
            print!("\x1b[1;37m");
        }
        let rgb = color.to_rgb();
        print!("\x1b[48;2;{};{};{}m", rgb.r, rgb.g, rgb.b);
        print!("{}", color.to_hex_string());
        print!("\x1b[0m");

        println!("\t{:?}\t{}", swatch.position(), swatch.population());
    }
}

//! This example demonstrates how to extract a color palette from an image and find the prominent colors.
//!
//! # Example Usage
//! To run this example, you can use the following command:
//! ```sh
//! cargo run --example simple --release --features='image' -- 'path'
//! ```
//! Replace `'path'` with the path to your image file.
//! If no image path is provided, it will use a default image located in the `gfx` directory.
#![deny(warnings)]

use std::time::Instant;

use anyhow::{Context, Error};
use auto_palette::{ImageData, Palette};

fn main() -> Result<(), Error> {
    // Read the image path from the command line arguments
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        println!("No image path provided, using the default image path");
        "./gfx/laura-clugston-pwW2iV9TZao-unsplash.jpg".into()
    });

    // Load the image data from the file
    let image_data = ImageData::load(&path)
        .with_context(|| format!("Failed to load the image data from the file: {}", path))?;

    // Start the timer to measure the extraction time
    let start = Instant::now();

    // Extract the color palette from the image data
    let palette: Palette<f64> = Palette::extract(&image_data)
        .with_context(|| "Failed to extract the palette from the image data".to_string())?;

    // Measure the duration of the extraction
    let duration = start.elapsed();
    println!(
        "Extracted {} swatch(es) in {}.{:03} seconds",
        palette.len(),
        duration.as_secs(),
        duration.subsec_millis()
    );

    // Find the 5 dominant colors in the palette and print their information
    let swatches = palette
        .find_swatches(5)
        .with_context(|| "Failed to find swatches in the palette".to_string())?;
    println!(
        "{:>2} | {:<7} | {:<12} | {:<10} | {:<6}",
        "#", "Color", "Position", "Population", "Ratio"
    );
    for (i, swatch) in swatches.iter().enumerate() {
        println!(
            "{:>2} | {:<7} | {:>4?} | {:>10} | {:>5.2}",
            i + 1,
            swatch.color().to_hex_string(),
            swatch.position(),
            swatch.population(),
            swatch.ratio()
        );
    }
    Ok(())
}

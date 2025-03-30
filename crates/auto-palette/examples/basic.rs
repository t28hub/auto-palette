#![deny(warnings)]

use anyhow::{Context, Error};
use auto_palette::{ImageData, Palette};

/// Extracts a palette from an image file.
///
/// The image path can be provided as a command line argument as follows:
/// ```sh
/// cargo run --example basic -- 'path/to/image.jpg'
/// ```
fn main() -> Result<(), Error> {
    // Read the image path from the command line arguments
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        println!("No image path provided, using the default image path");
        "./gfx/holly-booth-hLZWGXy5akM-unsplash.jpg".into()
    });

    // Load the image data from the file
    let image_data = ImageData::load(&path)
        .with_context(|| format!("Failed to load the image data from the file: {}", path))?;

    // Extract the color palette from the image data
    let palette: Palette<f32> = Palette::extract(&image_data)
        .with_context(|| "Failed to extract the palette from the image data".to_string())?;
    println!("Extracted {} swatch(es)", palette.len());

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

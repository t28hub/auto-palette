#![deny(warnings)]

use std::{str::FromStr, time::Instant};

use anyhow::{Context, Error};
use auto_palette::{Algorithm, ImageData, Palette};

/// Extracts a palette from an image file using the specified algorithm.
///
/// The algorithm can be provided as a command line argument as follows:
/// ```sh
/// cargo run --example algorithm -- 'dbscan++'
/// ```
fn main() -> Result<(), Error> {
    // Read the algorithm from the command line arguments
    let algorithm = match std::env::args().nth(1) {
        Some(name) => Algorithm::from_str(&name)
            .map_err(|_| println!("Failed to parse the algorithm '{}'", name))
            .unwrap(),
        None => {
            println!("No algorithm provided, using the default algorithm");
            Algorithm::DBSCAN
        }
    };

    // Load the image data from the file
    let image_data = ImageData::load("./gfx/holly-booth-hLZWGXy5akM-unsplash.jpg")
        .with_context(|| "Failed to load the image data from the file")?;

    // Extract the palette from the image data
    let start = Instant::now();
    let palette: Palette<f32> = Palette::extract_with_algorithm(&image_data, algorithm)
        .with_context(|| "Failed to extract the palette from the image data")?;
    let duration = start.elapsed();
    println!(
        "Extracted {} swatch(es) in {}.{:03} seconds",
        palette.len(),
        duration.as_secs(),
        duration.subsec_millis()
    );

    // Find the top 5 swatches in the palette
    let swatches = palette
        .find_swatches(5)
        .with_context(|| "Failed to find swatches in the palette")?;
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

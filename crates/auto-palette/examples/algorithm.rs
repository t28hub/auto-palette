//! This example demonstrates how to use different color extraction algorithms.
//!
//! # Example Usage
//! To run this example, you can use the following command:
//! ```sh
//! cargo run --example algorithm --release --features='image' -- 'algorithm'
//! ```
//! Replace `'algorithm'` with the one of the following algorithms: `dbscan`, `dbscan++`, `kmeans`, `slic`, or `snic`.
//! If no algorithm is provided, it will default to `dbscan`.
#![deny(warnings)]

use std::{str::FromStr, time::Instant};

use anyhow::{Context, Error};
use auto_palette::{Algorithm, ImageData, Palette};

fn main() -> Result<(), Error> {
    // Read the algorithm from the command line arguments
    let algorithm = std::env::args()
        .nth(1)
        .map(|name| Algorithm::from_str(&name).ok())
        .flatten()
        .unwrap_or(Algorithm::DBSCAN);

    // Load the image data from the file
    let image_data = ImageData::load("./gfx/laura-clugston-pwW2iV9TZao-unsplash.jpg")
        .with_context(|| "Failed to load the image data from the file")?;

    // Start the timer to measure the extraction time
    let start = Instant::now();

    // Extract the palette from the image data
    let palette: Palette<f64> = Palette::builder()
        .algorithm(algorithm)
        .build(&image_data)
        .with_context(|| "Failed to extract the palette from the image data")?;

    // Measure the duration of the extraction
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

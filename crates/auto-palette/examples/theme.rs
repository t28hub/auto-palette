//! This example demonstrates how to use different themes to find dominant colors in an image.
//!
//! # Example Usage
//! To run this example, you can use the following command:
//! ```sh
//! cargo run --example theme --release --features='image' -- 'theme'
//! ```
//! Replace `'theme'` with one of the following themes: `colorful`, `vivid`, `muted`, `soft`, `dark`, or `light`.
//! If no theme is provided, it will select the top 5 swatches without a specific theme.
#![deny(warnings)]

use std::{str::FromStr, time::Instant};

use anyhow::{Context, Error};
use auto_palette::{ImageData, Palette, Theme};

fn main() -> Result<(), Error> {
    // Read the theme from the command line arguments
    let theme = std::env::args()
        .nth(1)
        .map(|name| Theme::from_str(&name).ok())
        .flatten();

    // Load the image data from the file
    let image_data = ImageData::load("./gfx/laura-clugston-pwW2iV9TZao-unsplash.jpg")
        .with_context(|| "failed to load the image file".to_string())?;

    // Start the timer to measure the extraction time
    let start = Instant::now();

    // Extract the palette from the image data
    let palette: Palette<f64> = Palette::extract(&image_data)
        .with_context(|| "failed to extract the palette".to_string())?;

    // Measure the duration of the extraction
    let duration = start.elapsed();
    println!(
        "Extracted {} swatch(es) in {}.{:03} seconds",
        palette.len(),
        duration.as_secs(),
        duration.subsec_millis()
    );

    // Find the top 5 swatches in the palette
    let swatches = match theme {
        Some(theme) => palette
            .find_swatches_with_theme(5, theme)
            .with_context(|| format!("failed to find swatches with theme {:?}", theme))?,
        None => palette
            .find_swatches(5)
            .with_context(|| "failed to find swatches in the palette".to_string())?,
    };
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

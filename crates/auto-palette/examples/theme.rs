#![deny(warnings)]

use std::{str::FromStr, time::Instant};

use anyhow::{Context, Error};
use auto_palette::{ImageData, Palette, Theme};

/// Extracts a palette from an image file and finds the dominant colors using the specified theme.
///
/// The theme can be provided as a command line argument as follows:
/// ```sh
/// cargo run --example theme --release -- vivid
/// ```
fn main() -> Result<(), Error> {
    // Read the theme from the command line arguments
    let theme = match std::env::args().nth(1) {
        Some(name) => Theme::from_str(&name)
            .map_err(|_| format!("Failed to parse the them '{}'", name))
            .unwrap(),
        None => {
            println!("No theme provided, using the default theme");
            Theme::Basic
        }
    };

    // Load the image data from the file
    let image_data = ImageData::load("./gfx/holly-booth-hLZWGXy5akM-unsplash.jpg")
        .with_context(|| "failed to load the image file".to_string())?;

    // Extract the palette from the image data
    let start = Instant::now();
    let palette: Palette<f32> = Palette::extract(&image_data)
        .with_context(|| "failed to extract the palette".to_string())?;
    let duration = start.elapsed();
    println!(
        "Extracted {} swatch(es) in {}.{:03} seconds",
        palette.len(),
        duration.as_secs(),
        duration.subsec_millis()
    );

    // Find the top 5 swatches in the palette
    let swatches = palette
        .find_swatches_with_theme(5, theme)
        .with_context(|| format!("failed to find swatches with theme {:?}", theme))?;
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

#![deny(warnings)]

use std::{str::FromStr, time::Instant};

use auto_palette::{Algorithm, ImageData, Palette};

/// Extracts a palette from an image file using the specified algorithm.
///
/// The algorithm can be provided as a command line argument.
/// ```sh
/// cargo run --example algorithm --release -- 'dbscan++'
/// ```
fn main() {
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
    let image_data =
        ImageData::load("./crates/core/tests/assets/holly-booth-hLZWGXy5akM-unsplash.jpg").unwrap();

    // Extract the palette from the image data
    let start = Instant::now();
    let palette: Palette<f32> = Palette::extract_with_algorithm(&image_data, algorithm).unwrap();
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

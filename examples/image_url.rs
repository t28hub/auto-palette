#![deny(warnings)]

use std::time::Instant;

use auto_palette::{ImageData, Palette};

/// Fetches an image from the given URL and extracts the palette from it.
///
/// The URL can be provided as a command line argument.
/// ```sh
/// cargo run --example image_url --release -- https://picsum.photos/id/360/640/360/
/// ```
fn main() {
    // Read the URL from the command line arguments
    let url = std::env::args().nth(1).unwrap_or_else(|| {
        println!("No URL provided, using the default URL");
        "https://picsum.photos/id/360/640/360/".into()
    });

    println!("Fetching the image from '{}'", url);
    let mut response = reqwest::blocking::get(url.clone()).unwrap();
    if response.status() != 200 {
        eprintln!(
            "Received unexpected status code '{}' from '{}'",
            response.status(),
            url
        );
        return;
    }

    let mut buffer = Vec::new();
    response.copy_to(&mut buffer).unwrap();
    let image = image::load_from_memory(&buffer).unwrap();

    // Convert the image to image data
    let image_data = ImageData::try_from(&image).unwrap();
    println!(
        "Loaded the image with dimensions {}x{}",
        image_data.width(),
        image_data.height()
    );

    // Extract the palette from the image data
    let start = Instant::now();
    let palette: Palette<f32> = Palette::extract(&image_data).unwrap();
    let duration = start.elapsed();
    println!(
        "Extracted {} swatches in {}.{:03} seconds",
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

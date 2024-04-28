#![deny(warnings)]

use auto_palette::{ImageData, Palette};

/// Extracts a palette from an image file.
fn main() {
    // Load the image data from the file
    let image_data = ImageData::load("tests/assets/holly-booth-hLZWGXy5akM-unsplash.jpg").unwrap();

    // Extract the color palette from the image data
    let palette: Palette<f32> = Palette::extract(&image_data).unwrap();
    println!("Extracted {} swatches", palette.len());

    // Find the 5 dominant colors in the palette and print their information
    let swatches = palette.find_swatches(5);
    for swatch in swatches {
        println!("Color: {}", swatch.color().to_hex_string());
        println!("Position: {:?}", swatch.position());
        println!("Population: {}", swatch.population());
    }
}

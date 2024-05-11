use std::{process, time::Instant};

use auto_palette::{ImageData, Palette};
use clap::{crate_description, crate_version, Arg, Command};

fn main() {
    let command = Command::new("auto-palette")
        .version(crate_version!())
        .about(crate_description!())
        .arg_required_else_help(true)
        .arg(
            Arg::new("image")
                .value_name("IMAGE")
                .help("Path to the image file.")
                .long_help("Path to the image file. Supported formats: PNG, JPEG, GIF, BMP, ICO, and TIFF.")
                .required(true)
        );

    let matches = command.get_matches();
    let image_path = matches
        .get_one::<String>("image")
        .expect("IMAGE is required");
    let Ok(image_data) = ImageData::load(image_path) else {
        process::exit(1);
    };

    let instant = Instant::now();
    let Ok(palette) = Palette::<f64>::extract(&image_data) else {
        process::exit(1);
    };
    let swatches = palette.find_swatches(5);
    for swatch in swatches {
        println!("{:?}", swatch);
    }
    println!(
        "Extracted {} swatch(es) in {}.{:03} seconds",
        palette.len(),
        instant.elapsed().as_secs(),
        instant.elapsed().subsec_millis()
    );
}

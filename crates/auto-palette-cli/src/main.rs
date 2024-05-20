use std::{process, time::Instant};

use auto_palette::{ImageData, Palette};
use clap::Parser;

use crate::args::Options;

mod args;

fn main() {
    let options = Options::parse();
    let Ok(image_data) = ImageData::load(options.path) else {
        process::exit(1);
    };

    let instant = Instant::now();
    let Ok(palette) = Palette::<f32>::extract_with_algorithm(&image_data, options.algorithm.into())
    else {
        process::exit(1);
    };

    let color_space = options.color;
    let swatches = options.theme.map_or_else(
        || palette.find_swatches(options.count),
        |theme| palette.find_swatches_with_theme(options.count, theme.into()),
    );
    for swatch in swatches {
        println!("{}", color_space.as_string(swatch.color()));
    }
    println!(
        "Extracted {} swatch(es) in {}.{:03} seconds",
        palette.len(),
        instant.elapsed().as_secs(),
        instant.elapsed().subsec_millis()
    );
}

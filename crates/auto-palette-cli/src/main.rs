use std::{process, time::Instant};

use auto_palette::{Algorithm, ImageData, Palette, Theme};
use clap::Parser;
use image::{self, imageops::FilterType};

use crate::{args::Options, context::Context, env::Env};

mod args;
mod color;
mod context;
mod env;
mod output;
mod style;

const MAX_IMAGE_WIDTH: f64 = 360.0;
const MAX_IMAGE_HEIGHT: f64 = 360.0;

// The entry point of the CLI application.
fn main() {
    let context = Context::new(Options::parse(), Env::init());
    let Ok(image) = image::open(&context.args().path) else {
        eprintln!("Failed to open the image file {:?}", context.args().path);
        process::exit(1);
    };

    let image_width = image.width() as f64;
    let image_height = image.height() as f64;
    let scale = f64::min(
        MAX_IMAGE_WIDTH / image_width,
        MAX_IMAGE_HEIGHT / image_height,
    );

    let resized = if context.args().no_resize {
        image
    } else {
        image.resize_exact(
            (image_width * scale) as u32,
            (image_height * scale) as u32,
            FilterType::Lanczos3,
        )
    };

    let Ok(image_data) = ImageData::try_from(&resized) else {
        process::exit(1);
    };

    let instant = Instant::now();
    let algorithm = Algorithm::from(context.args().algorithm);
    let Ok(palette) = Palette::<f32>::extract_with_algorithm(&image_data, algorithm) else {
        process::exit(1);
    };

    let count = context.args().count;
    if count < 1 {
        eprintln!(
            "error: invalid value '{}' for '--count <count>': must be a positive integer",
            count
        );
        process::exit(1);
    }
    let swatches = context.args().theme.map_or_else(
        || palette.find_swatches(context.args().count),
        |option| {
            let theme = Theme::from(option);
            palette.find_swatches_with_theme(context.args().count, theme)
        },
    );
    context.args().output.print(&context, &swatches).unwrap();

    println!(
        "Extracted {} swatch(es) in {}.{:03} seconds",
        palette.len(),
        instant.elapsed().as_secs(),
        instant.elapsed().subsec_millis()
    );
}

use std::{process, time::Instant};

use anyhow::Context;
use auto_palette::{Algorithm, ImageData, Palette, Rgba, Theme};
use clap::Parser;
use clipboard::get_image_from_clipboard;
use image::{self, imageops::FilterType};

use crate::{args::Options, context::Context as CLIContext, env::Env};

mod args;
mod clipboard;
mod color;
mod context;
mod env;
mod output;
mod style;

const MAX_IMAGE_WIDTH: f64 = 360.0;
const MAX_IMAGE_HEIGHT: f64 = 360.0;

// The entry point of the CLI application.
fn main() -> anyhow::Result<()> {
    let context = CLIContext::new(Options::parse(), Env::init());
    let args = context.args();
    let image = match (&args.path, args.clipboard) {
        (None, false) => {
            return Err(anyhow::anyhow!("no input source provided"));
        }
        (Some(_), true) => {
            return Err(anyhow::anyhow!("only one input source can be provided"));
        }
        (None, true) => get_image_from_clipboard()?,
        (Some(path), false) => image::open(path)
            .with_context(|| format!("failed to open the image file {:?}", path))?,
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
    let Ok(palette) = Palette::<f64>::builder()
        .algorithm(algorithm)
        .filter(|pixel: &Rgba| pixel[3] != 0)
        .build(&image_data)
    else {
        process::exit(1);
    };

    let count = context.args().count;
    if count < 1 {
        return Err(anyhow::anyhow!(
            "invalid value '{}' for '--count <count>': must be a positive integer",
            count
        ));
    }
    let swatches = context
        .args()
        .theme
        .map_or_else(
            || palette.find_swatches(context.args().count),
            |option| {
                let theme = Theme::from(option);
                palette.find_swatches_with_theme(context.args().count, theme)
            },
        )
        .with_context(|| {
            format!(
                "failed to find swatches with theme {:?}",
                context.args().theme
            )
        })?;
    context
        .args()
        .output_format
        .print(&context, &swatches)
        .unwrap();

    println!(
        "Extracted {} swatch(es) in {}.{:03} seconds",
        palette.len(),
        instant.elapsed().as_secs(),
        instant.elapsed().subsec_millis()
    );

    Ok(())
}

use std::{process, time::Instant};

use auto_palette::{FloatNumber, ImageData, Palette};
use clap::Parser;
use colored::Colorize;
use image::{self, imageops::FilterType};

use crate::{args::Options, env::Env};

mod args;
mod env;

const MAX_IMAGE_WIDTH: f64 = 360.0;
const MAX_IMAGE_HEIGHT: f64 = 360.0;

#[derive(Debug)]
pub enum Style {
    TrueColor,
    Ansi256,
    NoColor,
}

fn main() {
    let options = Options::parse();
    let Ok(image) = image::open(&options.path) else {
        eprintln!("Failed to open the image file {:?}", options.path);
        process::exit(1);
    };

    let image_width = image.width() as f64;
    let image_height = image.height() as f64;
    let scale = f64::min(
        MAX_IMAGE_WIDTH / image_width,
        MAX_IMAGE_HEIGHT / image_height,
    );

    let resized = if options.no_resize {
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
    let Ok(palette) = Palette::<f32>::extract_with_algorithm(&image_data, options.algorithm.into())
    else {
        process::exit(1);
    };

    let env = Env::new();
    let style = match (env.is_truecolor_enabled(), env.is_color_disabled()) {
        (true, false) => Style::TrueColor,
        (false, false) => Style::Ansi256,
        (false, true) => Style::NoColor,
        (true, true) => Style::NoColor,
    };
    let swatches = options.theme.map_or_else(
        || palette.find_swatches(options.count),
        |theme| palette.find_swatches_with_theme(options.count, theme.into()),
    );
    for swatch in swatches {
        let color_string = options.color.as_string(swatch.color());
        let color = match style {
            Style::TrueColor => {
                let rgb = swatch.color().to_rgb();
                color_string.bold().on_truecolor(rgb.r, rgb.g, rgb.b)
            }
            Style::Ansi256 => {
                let rgb = swatch.color().to_rgb();
                color_string.bold().on_truecolor(rgb.r, rgb.g, rgb.b)
            }
            Style::NoColor => color_string.bold(),
        };

        let (x, y) = swatch.position();
        let unscaled_x = (x as f64 / scale).to_u32_unsafe();
        let unscaled_y = (y as f64 / scale).to_u32_unsafe();
        println!("{} ({}, {})", color, unscaled_x, unscaled_y);
    }
    println!(
        "Extracted {} swatch(es) in {}.{:03} seconds",
        palette.len(),
        instant.elapsed().as_secs(),
        instant.elapsed().subsec_millis()
    );
}

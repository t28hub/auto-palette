use std::{process, time::Instant};

use auto_palette::{ImageData, Palette};
use clap::Parser;
use colored::Colorize;

use crate::{args::Options, env::Env};

mod args;
mod env;

#[derive(Debug)]
pub enum Style {
    TrueColor,
    Ansi256,
    NoColor,
}

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
        println!("{} {:?}", color, swatch.position());
    }
    println!(
        "Extracted {} swatch(es) in {}.{:03} seconds",
        palette.len(),
        instant.elapsed().as_secs(),
        instant.elapsed().subsec_millis()
    );
}

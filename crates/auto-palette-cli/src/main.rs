use std::{process, time::Instant};

use auto_palette::{color::Color, Algorithm, FloatNumber, ImageData, Palette, Theme};
use clap::Parser;
use image::{self, imageops::FilterType};

use crate::{
    args::{Options, OutputFormat},
    color::ColorMode,
    context::Context,
    env::Env,
    output::{Printer, TablePrinter, TextPrinter},
    style::{style, Style},
    table::Table,
};

mod args;
mod color;
mod context;
mod env;
mod output;
mod style;
mod table;

const MAX_IMAGE_WIDTH: f64 = 360.0;
const MAX_IMAGE_HEIGHT: f64 = 360.0;

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

    let swatches = context.args().theme.map_or_else(
        || palette.find_swatches(context.args().count),
        |option| {
            let theme = Theme::from(option);
            palette.find_swatches_with_theme(context.args().count, theme)
        },
    );
    let mut table = Table::new();
    table
        .add_column("Color")
        .add_column("Position")
        .add_column("Population");
    for swatch in swatches {
        let mut row = vec![];
        let color = swatch.color();
        let style = colored(color, context.env());
        println!("{}", style.apply("  "));
        row.push(context.args().color.as_string(color));

        let (x, y) = swatch.position();
        let unscaled_x = (x as f64 / scale) as u32;
        let unscaled_y = (y as f64 / scale) as u32;
        row.push(format!("({}, {})", unscaled_x, unscaled_y));

        let population = swatch.population();
        row.push(format!("{}", population));
        table.add_row(&row);
    }

    match context.args().output {
        OutputFormat::Text => {
            TextPrinter.print(&table, &mut std::io::stdout()).unwrap();
        }
        OutputFormat::Table => {
            TablePrinter.print(&table, &mut std::io::stdout()).unwrap();
        }
    };

    println!(
        "Extracted {} swatch(es) in {}.{:03} seconds",
        palette.len(),
        instant.elapsed().as_secs(),
        instant.elapsed().subsec_millis()
    );
}

#[inline]
#[must_use]
fn colored<T>(color: &Color<T>, env: &Env) -> Style
where
    T: FloatNumber,
{
    if env.no_color.as_deref().is_some() {
        return style()
            .color(ColorMode::NoColor)
            .background(ColorMode::NoColor);
    }

    match env.colorterm.as_deref() {
        Some("truecolor") | Some("24bit") => {
            let rgb = color.to_rgb();
            style().background(ColorMode::TrueColor(rgb))
        }
        Some("8bit") => {
            let ansi256 = color.to_ansi256();
            style().background(ColorMode::Ansi256(ansi256))
        }
        _ => {
            let ansi16 = color.to_ansi16();
            style().background(ColorMode::Ansi16(ansi16))
        }
    }
}

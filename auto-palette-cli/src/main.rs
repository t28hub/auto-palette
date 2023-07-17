use auto_palette::Palette;
use clap::Parser;
use image::imageops::FilterType;
use image::DynamicImage;
use owo_colors::colors::{Black, White};
use owo_colors::{DynColors, OwoColorize, Style};
use std::io::{stderr, Write};
use std::process::exit;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    path: String,

    #[arg(short = 'n', long, default_value = "5")]
    count: usize,
}

#[must_use]
fn resize(image: DynamicImage) -> DynamicImage {
    let width = image.width() as f64;
    let height = image.height() as f64;
    let scale = 256.0 / width.max(height);

    image.resize(
        (width * scale) as u32,
        (height * scale) as u32,
        FilterType::Nearest,
    )
}

fn main() {
    let args = Args::parse();
    let Ok(image) = image::open(&args.path) else {
        writeln!(stderr(), "[ERROR] Could not open an image file: {}", args.path).ok();
        exit(1);
    };

    if args.count == 0 {
        writeln!(
            stderr(),
            "[ERROR] The count must be greater than 0: {}",
            args.count
        )
        .ok();
        exit(1);
    };

    // Improve the performance, resize the image by FilterType::Nearest.
    let resized = resize(image);
    let palette: Palette<f64> = Palette::extract(&resized);
    palette.swatches(args.count).iter().for_each(|swatch| {
        let color = swatch.color();
        let rgb = color.to_rgb();
        let style = if color.is_dark() {
            let color = DynColors::Rgb(rgb.r, rgb.g, rgb.b);
            Style::new().on_color(color).fg::<White>()
        } else {
            let color = DynColors::Rgb(rgb.r, rgb.g, rgb.b);
            Style::new().on_color(color).fg::<Black>()
        };

        let hex_string = format!("Hex: {}", color.to_hex_string());
        println!("{}", hex_string.style(style));

        let rgb_string = format!("RGB: rgb({r}, {g}, {b})", r = rgb.r, g = rgb.g, b = rgb.b);
        println!("{}", rgb_string.style(style));

        let position_string = format!("Position: {:?}", swatch.position());
        println!("{}", position_string.style(style));
    });
    exit(0);
}

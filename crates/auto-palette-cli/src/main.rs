mod cmd;

use std::{process, str::FromStr, time::Instant};

use auto_palette::{Algorithm, ImageData, Palette, Theme};

fn main() {
    let command = cmd::build_command();
    let matches = command.get_matches();
    let image_path = matches
        .get_one::<String>("image")
        .expect("IMAGE is required");
    let Ok(image_data) = ImageData::load(image_path) else {
        process::exit(1);
    };

    let algorithm = matches
        .get_one::<String>("algorithm")
        .expect("algorithm is required");
    let Ok(algorithm) = Algorithm::from_str(algorithm) else {
        eprintln!("Invalid algorithm: {}", algorithm);
        process::exit(1);
    };

    let theme = matches
        .get_one::<String>("theme")
        .expect("theme is required");
    let Ok(theme) = Theme::from_str(theme) else {
        eprintln!("Invalid theme: {}", theme);
        process::exit(1);
    };

    let count = matches
        .get_one::<String>("count")
        .expect("count is required");
    let Ok(count) = count.parse::<usize>() else {
        eprintln!("Invalid count: {}", count);
        process::exit(1);
    };

    let instant = Instant::now();
    let Ok(palette) = Palette::<f32>::extract_with_algorithm(&image_data, algorithm) else {
        process::exit(1);
    };
    let swatches = palette.find_swatches_with_theme(count, theme);
    for swatch in swatches {
        println!("{}", swatch.color().to_hex_string());
    }
    println!(
        "Extracted {} swatch(es) in {}.{:03} seconds",
        palette.len(),
        instant.elapsed().as_secs(),
        instant.elapsed().subsec_millis()
    );
}

use std::{process, str::FromStr, time::Instant};

use auto_palette::{Algorithm, ImageData, Palette, Theme};
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
        )
        .arg(
            Arg::new("algorithm")
                .long("algorithm")
                .short('a')
                .value_name("name")
                .help("Algorithm to use for extracting the palette.")
                .value_parser(["dbscan", "dbscan++", "kmeans"])
                .ignore_case(true)
                .default_value("dbscan")
        )
        .arg(
            Arg::new("theme")
                .long("theme")
                .short('t')
                .value_name("name")
                .help("Theme to use for extracting the palette.")
                .value_parser(["basic", "vivid", "muted", "light", "dark"])
                .ignore_case(true)
                .default_value("basic")
        )
        .arg(
            Arg::new("count")
                .long("count")
                .short('c')
                .value_name("number")
                .help("Number of swatches to extract.")
                .default_value("5")
        );

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
        println!("{:?}", swatch);
    }
    println!(
        "Extracted {} swatch(es) in {}.{:03} seconds",
        palette.len(),
        instant.elapsed().as_secs(),
        instant.elapsed().subsec_millis()
    );
}

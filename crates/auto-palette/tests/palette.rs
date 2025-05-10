use std::{path::Path, str::FromStr};

use auto_palette::{Algorithm, ImageData, Palette, RgbaPixel};
use rstest::rstest;

#[rstest]
#[case::black("../../gfx/colors/black.png", "#000000")]
#[case::gray("../../gfx/colors/gray.png", "#808080")]
#[case::white("../../gfx/colors/white.png", "#FFFFFF")]
#[case::red("../../gfx/colors/red.png", "#FF0000")]
#[case::blue("../../gfx/colors/blue.png", "#0000FF")]
#[case::green("../../gfx/colors/green.png", "#00FF00")]
fn test_extract_single_color<P>(#[case] path: P, #[case] expected: &str)
where
    P: AsRef<Path>,
{
    // Act
    let image_data = ImageData::load(path).unwrap();
    let actual: Result<Palette<f64>, _> = Palette::extract(&image_data);

    // Assert
    assert!(actual.is_ok());

    let palette = actual.unwrap();
    assert_eq!(palette.len(), 1);

    let swatches = palette.swatches();
    assert_eq!(swatches[0].color().to_hex_string(), expected);
}

#[test]
fn test_extract_multiple_colors() {
    // Act
    let image_data = ImageData::load("../../gfx/olympic_logo.png").unwrap();
    let actual: Result<Palette<f64>, _> = Palette::extract(&image_data);

    // Assert
    assert!(actual.is_ok());

    let palette = actual.unwrap();
    assert_eq!(palette.len(), 6);
}

#[rstest]
#[case::kmeans("kmeans")]
#[case::dbscan("dbscan")]
#[case::dbscanpp("dbscan++")]
fn test_builder_with_algorithm(#[case] name: &str) {
    // Arrange
    let image_data = ImageData::load("../../gfx/holly-booth-hLZWGXy5akM-unsplash.jpg").unwrap();
    let algorithm = Algorithm::from_str(name).unwrap();

    // Act
    let actual: Result<Palette<f64>, _> =
        Palette::builder().algorithm(algorithm).build(&image_data);

    // Assert
    assert!(actual.is_ok());

    let palette = actual.unwrap();
    assert!(!palette.is_empty());
    assert!(palette.len() >= 6);
}

#[test]
fn test_builder_with_filter() {
    // Arrange
    let image_data = ImageData::load("../../gfx/holly-booth-hLZWGXy5akM-unsplash.jpg").unwrap();

    // Act
    let actual: Result<Palette<f64>, _> = Palette::builder()
        .filter(|pixel: &RgbaPixel| pixel[3] != 0)
        .build(&image_data);

    // Assert
    assert!(actual.is_ok());

    let palette = actual.unwrap();
    assert!(!palette.is_empty());
    assert!(palette.len() >= 6);
}

#[test]
fn test_builder_with_max_swatches() {
    // Arrange
    let image_data = ImageData::load("../../gfx/holly-booth-hLZWGXy5akM-unsplash.jpg").unwrap();

    // Act
    let actual: Result<Palette<f64>, _> = Palette::builder().max_swatches(16).build(&image_data);

    // Assert
    assert!(actual.is_ok());

    let palette = actual.unwrap();
    assert!(!palette.is_empty());
    assert_eq!(palette.len(), 16);
}

#[test]
fn test_builder_transparent() {
    // Act
    let image_data = ImageData::load("../../gfx/colors/transparent.png").unwrap();
    let actual: Result<Palette<f64>, _> = Palette::builder()
        .filter(|pixel: &RgbaPixel| pixel[3] != 0)
        .build(&image_data);

    // Assert
    assert!(actual.is_err());

    let error = actual.err().unwrap();
    assert_eq!(
        error.to_string(),
        "Image data is empty: no pixels to process"
    );
}

#[rstest]
#[case::dk("../../gfx/flags/dk.png", 2, vec ! ["#C8102E", "#FFFFFF"])]
#[case::uk("../../gfx/flags/uk.png", 3, vec ! ["#C8102E", "#012169", "#FFFFFF"])]
#[case::my("../../gfx/flags/my.png", 4, vec ! ["#FFFFFF", "#FFCC00", "#CC0000", "#000066"])]
#[case::kn("../../gfx/flags/kn.png", 5, vec ! ["#000000", "#009739", "#FFCD00", "#C8102E", "#FFFFFF"])]
#[case::za("../../gfx/flags/za.png", 6, vec ! ["#FFFFFF", "#FFB81C", "#000000", "#E03C31", "#007749", "#001489"])]
fn test_find_swatches<P>(#[case] path: P, #[case] n: usize, #[case] expected: Vec<&str>)
where
    P: AsRef<Path>,
{
    // Act
    let image_data = ImageData::load(path).unwrap();
    let palette: Palette<f64> = Palette::extract(&image_data).unwrap();
    let actual = palette.find_swatches(n);

    // Assert
    assert!(actual.is_ok());

    let swatches = actual.unwrap();
    assert_eq!(swatches.len(), n);

    let colors: Vec<_> = swatches
        .iter()
        .map(|swatch| swatch.color().to_hex_string())
        .collect();
    for expected_color in expected {
        assert!(colors.contains(&expected_color.to_string()));
    }
}

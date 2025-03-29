use std::{path::Path, str::FromStr};

use auto_palette::{Algorithm, ImageData, Palette};
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
    let palette: Palette<f32> = Palette::extract(&image_data).unwrap();

    // Assert
    assert_eq!(palette.len(), 1);

    let swatches = palette.swatches();
    assert_eq!(swatches[0].color().to_hex_string(), expected);
}

#[test]
fn test_extract_empty() {
    // Act
    let image_data = ImageData::load("../../gfx/colors/transparent.png").unwrap();
    let palette: Palette<f32> = Palette::extract(&image_data).unwrap();

    // Assert
    assert_eq!(palette.len(), 0);
}

#[test]
fn test_extract_multiple_colors() {
    // Act
    let image_data = ImageData::load("../../gfx/olympic_logo.png").unwrap();
    let palette: Palette<f32> = Palette::extract(&image_data).unwrap();

    // Assert
    assert_eq!(palette.len(), 6);
}

#[rstest]
#[case::kmeans("kmeans")]
#[case::dbscan("dbscan")]
#[case::dbscanpp("dbscan++")]
fn test_extract_with_algorithm(#[case] name: &str) {
    // Arrange
    let image_data = ImageData::load("../../gfx/holly-booth-hLZWGXy5akM-unsplash.jpg").unwrap();
    let algorithm = Algorithm::from_str(name).unwrap();

    // Act
    let palette: Palette<f32> = Palette::extract_with_algorithm(&image_data, algorithm).unwrap();

    // Assert
    assert!(!palette.is_empty());
    assert!(palette.len() >= 6);
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
    let palette: Palette<f32> = Palette::extract(&image_data).unwrap();
    let swatches = palette.find_swatches(n).unwrap();

    // Assert
    assert!(!palette.is_empty());
    assert_eq!(swatches.len(), n);

    let colors: Vec<_> = swatches
        .iter()
        .map(|swatch| swatch.color().to_hex_string())
        .collect();
    for expected_color in expected {
        assert!(colors.contains(&expected_color.to_string()));
    }
}

#[test]
fn test_find_swatches_with_empty_palette() {
    // Act
    let image_data = ImageData::load("../../gfx/colors/transparent.png").unwrap();
    let palette: Palette<f32> = Palette::extract(&image_data).unwrap();
    let swatches = palette.find_swatches(5).unwrap();

    // Assert
    assert!(palette.is_empty());
    assert!(swatches.is_empty());
}

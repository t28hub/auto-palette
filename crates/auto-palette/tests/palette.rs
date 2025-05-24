use std::{path::Path, str::FromStr};

use auto_palette::{assert_color_eq, color::Color, Algorithm, ImageData, Palette, Rgba, Swatch};
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
    let image_data = ImageData::load(path).expect("Failed to load image data");
    let actual: Result<Palette<f64>, _> = Palette::extract(&image_data);

    // Assert
    assert!(actual.is_ok());

    let palette = actual.expect("Failed to extract palette");
    assert_eq!(palette.len(), 1);

    let swatches = palette.swatches();
    let actual = swatches[0].color();
    let expected = Color::<f64, _>::from_str(expected).expect("Invalid hex color format");
    assert_color_eq!(actual, expected);
}

#[test]
fn test_extract_multiple_colors() {
    // Act
    let image_data =
        ImageData::load("../../gfx/olympic_logo.png").expect("Failed to load image data");
    let actual: Result<Palette<f64>, _> = Palette::extract(&image_data);

    // Assert
    assert!(actual.is_ok());

    let palette = actual.expect("Failed to extract palette");
    assert_eq!(palette.len(), 6);

    let mut swatches = palette
        .find_swatches(6)
        .expect("Failed to find swatches in palette");
    swatches.sort_by(|a, b| {
        b.population()
            .cmp(&a.population())
            .then_with(|| a.color().to_rgb_int().cmp(&b.color().to_rgb_int()))
    });
    assert_eq!(swatches.len(), 6);

    let actual_colors: Vec<_> = swatches.iter().map(Swatch::color).collect();
    let expected_colors: Vec<_> = [
        "#FFFFFF", // White
        "#0081C8", // Blue
        "#EE334E", // Red
        "#000000", // Black
        "#00A651", // Green
        "#FCB131", // Yellow
    ]
    .iter()
    .map(|hex| Color::<f64, _>::from_str(hex).expect("Invalid hex color format"))
    .collect();
    for (actual, expected) in actual_colors.iter().zip(expected_colors.iter()) {
        assert_color_eq!(actual, expected);
    }
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
    let image_data = ImageData::load("../../gfx/laura-clugston-pwW2iV9TZao-unsplash.jpg")
        .expect("Failed to load image data");

    // Act
    let actual: Result<Palette<f64>, _> = Palette::builder()
        .filter(|pixel: &Rgba| pixel[3] != 0)
        .build(&image_data);

    // Assert
    assert!(actual.is_ok());

    let palette = actual.expect("Failed to extract palette");
    assert!(!palette.is_empty());
    assert!(palette.len() >= 128);
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
        .filter(|pixel: &Rgba| pixel[3] != 0)
        .build(&image_data);

    // Assert
    assert!(actual.is_ok());

    let palette = actual.unwrap();
    assert!(palette.is_empty());
    assert_eq!(palette.len(), 0);
}

#[rstest]
#[case::dk("../../gfx/flags/dk.png", 2, vec ! ["#C8102E", "#FFFFFF"])]
#[case::uk("../../gfx/flags/uk.png", 3, vec ! ["#C8102E", "#FFFFFF", "#012169"])]
#[case::my("../../gfx/flags/my.png", 4, vec ! ["#FFFFFF", "#CC0000", "#000066", "#FFCC00"])]
#[case::kn("../../gfx/flags/kn.png", 5, vec ! ["#000000", "#009739","#C8102E",  "#FFCD00", "#FFFFFF"])]
#[case::za("../../gfx/flags/za.png", 6, vec ! ["#007749", "#E03C31", "#001489", "#FFFFFF", "#000000", "#FFB81C"])]
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

    let mut swatches = actual.unwrap();
    // Sort by population descending, then by color value ascending
    swatches.sort_by(|a, b| {
        b.population()
            .cmp(&a.population())
            .then_with(|| a.color().to_rgb_int().cmp(&b.color().to_rgb_int()))
    });
    assert_eq!(swatches.len(), n);

    let actual_colors: Vec<_> = swatches.iter().map(Swatch::color).collect();
    let expected_colors: Vec<_> = expected
        .iter()
        .map(|hex| Color::from_str(hex).expect("Invalid hex color format"))
        .collect();
    for (actual, expected) in actual_colors.iter().zip(expected_colors.iter()) {
        assert_color_eq!(actual, expected);
    }
}

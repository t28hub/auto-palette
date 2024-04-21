use std::path::Path;
use std::str::FromStr;

use rstest::rstest;

use auto_palette::{Algorithm, ImageData, Palette};

#[rstest]
#[case::black("./tests/assets/colors/black.png", "#000000")]
#[case::gray("./tests/assets/colors/gray.png", "#808080")]
#[case::white("./tests/assets/colors/white.png", "#FFFFFF")]
#[case::red("./tests/assets/colors/red.png", "#FF0000")]
#[case::blue("./tests/assets/colors/blue.png", "#0000FF")]
#[case::green("./tests/assets/colors/green.png", "#00FF00")]
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
    let image_data = ImageData::load("./tests/assets/colors/transparent.png").unwrap();
    let palette: Palette<f32> = Palette::extract(&image_data).unwrap();

    // Assert
    assert_eq!(palette.len(), 0);
}

#[test]
fn test_extract_multiple_colors() {
    // Act
    let image_data = ImageData::load("./tests/assets/olympic_rings.png").unwrap();
    let palette: Palette<f32> = Palette::extract(&image_data).unwrap();

    // Assert
    assert_eq!(palette.len(), 6);

    let swatches = palette.swatches();
    assert_eq!(swatches[0].color().to_hex_string(), "#FFFFFF");
    assert_eq!(swatches[1].color().to_hex_string(), "#EE344E");
    assert_eq!(swatches[2].color().to_hex_string(), "#0081C8");
    assert_eq!(swatches[3].color().to_hex_string(), "#01A651");
    assert_eq!(swatches[4].color().to_hex_string(), "#000000");
    assert_eq!(swatches[5].color().to_hex_string(), "#FCB131");
}

#[rstest]
#[case::kmeans("kmeans")]
#[case::dbscan("dbscan")]
#[case::dbscanpp("dbscan++")]
fn test_extract_with_algorithm(#[case] name: &str) {
    // Arrange
    let image_data =
        ImageData::load("./tests/assets/holly-booth-hLZWGXy5akM-unsplash.jpg").unwrap();
    let algorithm = Algorithm::from_str(name).unwrap();

    // Act
    let palette: Palette<f32> = Palette::extract_with_algorithm(&image_data, algorithm).unwrap();

    // Assert
    assert!(!palette.is_empty());
    assert!(palette.len() >= 6);
}

#[rstest]
#[case::dk("./tests/assets/flags/dk.png", 2, vec ! ["#C8102E", "#FFFFFF"])]
#[case::uk("./tests/assets/flags/uk.png", 3, vec ! ["#C8102E", "#012169", "#FFFFFF"])]
#[case::my("./tests/assets/flags/my.png", 4, vec ! ["#FFFFFF", "#FFCC00", "#CC0000", "#000066"])]
#[case::kn("./tests/assets/flags/kn.png", 5, vec ! ["#000000", "#009739", "#FFCD00", "#C8102E", "#FFFFFF"])]
#[case::za("./tests/assets/flags/za.png", 6, vec ! ["#FFFFFF", "#FFB81C", "#000000", "#E03C31", "#007749", "#001489"])]
fn test_find_swatches<P>(#[case] path: P, #[case] n: usize, #[case] expected: Vec<&str>)
where
    P: AsRef<Path>,
{
    // Act
    let image_data = ImageData::load(path).unwrap();
    let palette: Palette<f32> = Palette::extract(&image_data).unwrap();
    let swatches = palette.find_swatches(n);

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
    let image_data = ImageData::load("./tests/assets/colors/transparent.png").unwrap();
    let palette: Palette<f32> = Palette::extract(&image_data).unwrap();
    let swatches = palette.find_swatches(5);

    // Assert
    assert!(palette.is_empty());
    assert!(swatches.is_empty());
}

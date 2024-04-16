use std::path::Path;

use rstest::rstest;

use auto_palette::{Algorithm, ImageData, Palette};

#[rstest]
#[case::np("./tests/assets/flag_np.png", 3)]
#[case::uk("./tests/assets/flag_uk.png", 3)]
#[case::za("./tests/assets/flag_za.png", 6)]
fn test_extract<P>(#[case] path: P, #[case] n: usize)
where
    P: AsRef<Path>,
{
    // Act
    let image_data = ImageData::load(path).unwrap();
    let palette: Palette<f32> = Palette::extract(&image_data).unwrap();

    // Assert
    assert!(!palette.is_empty());
    assert!(palette.len() >= n);
}

#[test]
fn test_extract_with_kmeans() {
    // Arrange
    let image_data =
        ImageData::load("./tests/assets/holly-booth-hLZWGXy5akM-unsplash.jpg").unwrap();

    // Act
    let palette: Palette<f32> =
        Palette::extract_with_algorithm(&image_data, Algorithm::KMeans).unwrap();
    let swatches = palette.find_swatches(8);

    // Assert
    swatches.iter().for_each(|swatch| {
        let rgb = swatch.color().to_rgb();
        let (x, y) = swatch.position();
        let population = swatch.population();
        println!(
            "#{:02X}{:02X}{:02X} - ({}, {}) - {}",
            rgb.r, rgb.g, rgb.b, x, y, population
        );
    });
    assert!(!palette.is_empty());
}

#[test]
fn test_extract_with_dbscan() {
    // Arrange
    let image_data =
        ImageData::load("./tests/assets/holly-booth-hLZWGXy5akM-unsplash.jpg").unwrap();

    // Act
    let palette: Palette<f32> = Palette::extract(&image_data).unwrap();
    let swatches = palette.find_swatches(8);

    // Assert
    swatches.iter().for_each(|swatch| {
        let rgb = swatch.color().to_rgb();
        let (x, y) = swatch.position();
        let population = swatch.population();
        println!(
            "#{:02X}{:02X}{:02X} - ({}, {}) - {}",
            rgb.r, rgb.g, rgb.b, x, y, population
        );
    });
    assert_eq!(swatches.len(), 4);
}

#[test]
fn test_extract_with_dbscanpp() {
    // Arrange
    let image_data =
        ImageData::load("./tests/assets/holly-booth-hLZWGXy5akM-unsplash.jpg").unwrap();

    // Act
    let palette: Palette<f32> =
        Palette::extract_with_algorithm(&image_data, Algorithm::DBSCANpp).unwrap();
    let swatches = palette.find_swatches(8);

    // Assert
    swatches.iter().for_each(|swatch| {
        let rgb = swatch.color().to_rgb();
        let (x, y) = swatch.position();
        let population = swatch.population();
        println!(
            "#{:02X}{:02X}{:02X} - ({}, {}) - {}",
            rgb.r, rgb.g, rgb.b, x, y, population
        );
    });
    assert_eq!(swatches.len(), 8);
}

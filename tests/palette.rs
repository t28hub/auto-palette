use std::path::Path;

use rstest::rstest;

use auto_palette::{ImageData, Palette};

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
    let palette = Palette::extract(&image_data).unwrap();

    // Assert
    assert!(!palette.is_empty());
    assert!(palette.len() >= n);
}

#[test]
fn test_extract_jpg() {
    // Act
    let image_data =
        ImageData::load("./tests/assets/holly-booth-hLZWGXy5akM-unsplash.jpg").unwrap();
    let palette = Palette::extract(&image_data).unwrap();
    let swatches = palette.find_swatches(6);

    // Assert
    swatches.iter().for_each(|swatch| {
        let (r, g, b) = swatch.color();
        let (x, y) = swatch.position();
        let population = swatch.population();
        println!(
            "#{:02X}{:02X}{:02X} - ({}, {}) - {}",
            r, g, b, x, y, population
        );
    });
    assert_eq!(swatches.len(), 6);
}

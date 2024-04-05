use auto_palette::image::ImageData;
use auto_palette::Palette;
use rstest::rstest;
use std::path::Path;

#[rstest]
#[case::np("./tests/assets/flag_np.png", 3)]
#[case::za("./tests/assets/flag_za.png", 6)]
fn test_extract<P>(#[case] path: P, #[case] n: usize)
where
    P: AsRef<Path>,
{
    // Act
    let image_data = ImageData::open(path).unwrap();
    let palette = Palette::extract(&image_data).unwrap();
    let swatches = palette.find_swatches(n);

    // Assert
    swatches.iter().for_each(|swatch| {
        let color = swatch.color();
        let population = swatch.population();
        println!(
            "#{:02X}{:02X}{:02X} - {}",
            color.0, color.1, color.2, population
        );
    });
    assert_eq!(swatches.len(), n);
}

#[test]
fn test_extract_jpg() {
    // Act
    let image_data =
        ImageData::open("./tests/assets/holly-booth-hLZWGXy5akM-unsplash.jpg").unwrap();
    let palette = Palette::extract(&image_data).unwrap();
    let swatches = palette.find_swatches(6);

    // Assert
    swatches.iter().for_each(|swatch| {
        let color = swatch.color();
        let population = swatch.population();
        println!(
            "#{:02X}{:02X}{:02X} - {}",
            color.0, color.1, color.2, population
        );
    });
    assert_eq!(swatches.len(), 6);
}

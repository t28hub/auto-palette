use auto_palette::image::ImageData;
use auto_palette::Palette;

#[test]
fn test_extract() {
    // Act
    let image_data = ImageData::open("./tests/assets/flag_np.png").unwrap();
    let palette = Palette::extract(&image_data).unwrap();
    let swatches = palette.swatches(5);

    // Assert
    swatches.iter().for_each(|swatch| {
        let color = swatch.color();
        let population = swatch.population();
        println!(
            "#{:02X}{:02X}{:02X} - {}",
            color.0, color.1, color.2, population
        );
    });
    assert_eq!(swatches.len(), 5);
}

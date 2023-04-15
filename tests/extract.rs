extern crate image;

use auto_palette::color_trait::Color;
use auto_palette::{Algorithm, Palette, SimpleImageData};
use rstest::rstest;

#[rstest]
#[case::gmeans(Algorithm::Gmeans, ("#040200", "#04168f", "#1f7941", "#f42222", "#f9ecea", "#f6f9f9"))]
#[case::dbscan(Algorithm::DBSCAN, ("#007944", "#f42222", "#00158f", "#000000", "#feffff", "#ffb400"))]
#[case::hdbscan(Algorithm::HDBSCAN, ("#007944", "#f42222", "#00158f", "#030100", "#fcb300", "#ffffff"))]
fn extract_from_image_data(
    #[case] algorithm: Algorithm,
    #[case] expected: (&str, &str, &str, &str, &str, &str),
) {
    let img = image::open("./tests/images/flag_za.png").unwrap();
    let image_data = SimpleImageData::new(img.as_bytes(), img.width(), img.height()).unwrap();

    let palette: Palette<f64> = Palette::extract(&image_data, algorithm);
    let swatches = palette.swatches(6);
    assert_eq!(swatches.len(), 6);
    assert_eq!(swatches[0].color().to_hex_string(), expected.0);
    assert_eq!(swatches[1].color().to_hex_string(), expected.1);
    assert_eq!(swatches[2].color().to_hex_string(), expected.2);
    assert_eq!(swatches[3].color().to_hex_string(), expected.3);
    assert_eq!(swatches[4].color().to_hex_string(), expected.4);
    assert_eq!(swatches[5].color().to_hex_string(), expected.5);
}

#[test]
fn extract_with_gmeans() {
    let img = image::open("./tests/images/photo_aLMeYMZEJvk.png").unwrap();
    let image_data = SimpleImageData::new(img.as_bytes(), img.width(), img.height()).unwrap();

    let palette: Palette<f64> = Palette::extract(&image_data, Algorithm::Gmeans);
    let swatches = palette.swatches(5);
    assert_eq!(swatches.len(), 5);
    assert_eq!(swatches[0].color().to_hex_string(), "#015bd6");
    assert_eq!(swatches[1].color().to_hex_string(), "#523105");
    assert_eq!(swatches[2].color().to_hex_string(), "#d38502");
    assert_eq!(swatches[3].color().to_hex_string(), "#146011");
    assert_eq!(swatches[4].color().to_hex_string(), "#321102");
}

#[test]
fn extract_with_dbscan() {
    let img = image::open("./tests/images/photo_aLMeYMZEJvk.png").unwrap();
    let image_data = SimpleImageData::new(img.as_bytes(), img.width(), img.height()).unwrap();

    let palette: Palette<f64> = Palette::extract(&image_data, Algorithm::DBSCAN);
    let swatches = palette.swatches(5);
    assert_eq!(swatches.len(), 5);
    assert_eq!(swatches[0].color().to_hex_string(), "#1d86e8");
    assert_eq!(swatches[1].color().to_hex_string(), "#116902");
    assert_eq!(swatches[2].color().to_hex_string(), "#5b2a03");
    assert_eq!(swatches[3].color().to_hex_string(), "#f6a500");
    assert_eq!(swatches[4].color().to_hex_string(), "#49974c");
}

#[test]
fn extract_with_hdbscan() {
    let img = image::open("./tests/images/photo_aLMeYMZEJvk.png").unwrap();
    let image_data = SimpleImageData::new(img.as_bytes(), img.width(), img.height()).unwrap();

    let palette: Palette<f64> = Palette::extract(&image_data, Algorithm::HDBSCAN);
    let swatches = palette.swatches(5);
    assert_eq!(swatches.len(), 5);
    assert_eq!(swatches[0].color().to_hex_string(), "#c08a07");
    assert_eq!(swatches[1].color().to_hex_string(), "#256905");
    assert_eq!(swatches[2].color().to_hex_string(), "#1783e7");
    assert_eq!(swatches[3].color().to_hex_string(), "#80c1fb");
    assert_eq!(swatches[4].color().to_hex_string(), "#dde9f6");
}

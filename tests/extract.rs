extern crate image;

use auto_palette::color_trait::Color;
use auto_palette::{Algorithm, Palette, SimpleImageData};

#[test]
fn extract_with_gmeans() {
    let img = image::open("./tests/images/img.png").unwrap();
    let image_data = SimpleImageData::new(img.as_bytes(), img.width(), img.height()).unwrap();

    let palette: Palette<f32> = Palette::extract(&image_data, Algorithm::Gmeans);
    let swatches = palette.swatches(5);
    assert_eq!(swatches.len(), 5);
    assert_eq!(swatches[0].color().to_hex_string(), "#08080c");
    assert_eq!(swatches[1].color().to_hex_string(), "#c46f89");
    assert_eq!(swatches[2].color().to_hex_string(), "#479547");
    assert_eq!(swatches[3].color().to_hex_string(), "#1d898e");
    assert_eq!(swatches[4].color().to_hex_string(), "#ba4791");
}

#[test]
fn extract_with_dbscan() {
    let img = image::open("./tests/images/img.png").unwrap();
    let image_data = SimpleImageData::new(img.as_bytes(), img.width(), img.height()).unwrap();

    let palette: Palette<f32> = Palette::extract(&image_data, Algorithm::DBSCAN);
    let swatches = palette.swatches(5);
    assert_eq!(swatches.len(), 5);
    assert_eq!(swatches[0].color().to_hex_string(), "#010300");
    assert_eq!(swatches[1].color().to_hex_string(), "#1b959a");
    assert_eq!(swatches[2].color().to_hex_string(), "#f8dc05");
    assert_eq!(swatches[3].color().to_hex_string(), "#d17998");
    assert_eq!(swatches[4].color().to_hex_string(), "#94af11");
}

#[test]
fn extract_with_hdbscan() {
    let img = image::open("./tests/images/img.png").unwrap();
    let image_data = SimpleImageData::new(img.as_bytes(), img.width(), img.height()).unwrap();

    let palette: Palette<f32> = Palette::extract(&image_data, Algorithm::HDBSCAN);
    let swatches = palette.swatches(5);
    assert_eq!(swatches.len(), 5);
    assert_eq!(swatches[0].color().to_hex_string(), "#010300");
    assert_eq!(swatches[1].color().to_hex_string(), "#1e9498");
    assert_eq!(swatches[2].color().to_hex_string(), "#f8dc06");
    assert_eq!(swatches[3].color().to_hex_string(), "#44dacf");
    assert_eq!(swatches[4].color().to_hex_string(), "#cf7795");
}

extern crate image;

use auto_palette::{Algorithm, Palette, SimpleImageData};

#[test]
fn extract_with_gmeans() {
    let img = image::open("./tests/images/img.png").unwrap();
    let image_data =
        SimpleImageData::new(img.as_bytes().to_vec(), img.width(), img.height()).unwrap();

    let palette: Palette<f64> = Palette::extract(&image_data, Algorithm::GMEANS);
    let swatches = palette.get_swatches(5);
    assert_eq!(swatches.len(), 5);
}

#[test]
fn extract_with_dbscan() {
    let img = image::open("./tests/images/img.png").unwrap();
    let image_data =
        SimpleImageData::new(img.as_bytes().to_vec(), img.width(), img.height()).unwrap();

    let palette: Palette<f64> = Palette::extract(&image_data, Algorithm::DBSCAN);
    let swatches = palette.get_swatches(5);
    assert_eq!(swatches.len(), 5);
}

#[test]
fn extract_with_hdbscan() {
    let img = image::open("./tests/images/img.png").unwrap();
    let image_data =
        SimpleImageData::new(img.as_bytes().to_vec(), img.width(), img.height()).unwrap();

    let palette: Palette<f64> = Palette::extract(&image_data, Algorithm::HDBSCAN);
    let swatches = palette.get_swatches(5);
    assert_eq!(swatches.len(), 5);
}

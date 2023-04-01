extern crate image;

use auto_palette::{Algorithm, Palette, SimpleImageData};

#[test]
fn extract() {
    let img = image::open("./tests/images/img.png").unwrap();
    let width = img.width();
    let height = img.height();
    let image_data = SimpleImageData::new(img.into_bytes(), width, height).unwrap();
    let palette: Palette<f64> = Palette::extract(&image_data, Algorithm::GMEANS);
    palette
        .get_swatches(6)
        .iter()
        .for_each(|swatch| println!("{:?}", swatch));
}

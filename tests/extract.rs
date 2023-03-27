extern crate image;

use auto_palette::{Algorithm, Palette, SimpleImageData};

#[test]
fn extract() {
    let img = image::open("./tests/images/img.png").unwrap();
    let width = img.width();
    let height = img.height();
    let image_data = SimpleImageData::new(img.into_bytes(), width, height).unwrap();
    let palette: Palette<f64> = Palette::extract(&image_data, Algorithm::DBSCAN);
    palette
        .get_swatches(6)
        .iter()
        // .filter(|swatch| swatch.percentage > 0.01)
        .for_each(|swatch| println!("{:?}", swatch));
}

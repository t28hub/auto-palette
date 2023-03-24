extern crate image;

use auto_palette::*;

#[test]
fn generate() {
    let img = image::open("./tests/images/img_1.png").unwrap();
    let pixels = img.to_rgba8().to_vec();
    let palette: Palette<f64> = Palette::generate(&pixels, img.width(), img.height());
    palette
        .swatches(6)
        .iter()
        .for_each(|swatch| println!("{:?}", swatch));
}

extern crate image;

use auto_palette::*;

#[test]
fn generate() {
    let img = image::open("./tests/images/img.png").unwrap();
    let pixels = img.to_rgba8().to_vec();
    let palette: Palette<f64> = Palette::generate(&pixels, img.width(), img.height());
    palette
        .swatches()
        .iter()
        .filter(|swatch| swatch.percentage > 0.01)
        .for_each(|swatch| println!("{:?}", swatch));
}

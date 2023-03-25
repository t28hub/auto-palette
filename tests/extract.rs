extern crate image;

use crate::test_image::TestImageData;
use auto_palette::*;

mod test_image;

#[test]
fn generate() {
    let img = image::open("./tests/images/img.png").unwrap();
    let image_data = TestImageData::new(img);
    let palette: Palette<f64> = Palette::generate(&image_data);
    palette
        .get_swatches(6)
        .iter()
        .for_each(|swatch| println!("{:?}", swatch));
}

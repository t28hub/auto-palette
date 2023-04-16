extern crate image;

use auto_palette::color_trait::Color;
use auto_palette::{Algorithm, Palette, SimpleImageData};
use rstest::rstest;

#[rstest]
#[case::gr("./tests/images/flag_gr.png", 2, vec!["#ffffff", "#0060b5"])]
#[case::no("./tests/images/flag_no.png", 3, vec!["#cc0028", "#00215f", "#ffffff"])]
#[case::pg("./tests/images/flag_pg.png", 4, vec!["#000000", "#e10017", "#ffcf00", "#ffffff"])]
#[case::sc("./tests/images/flag_sc.png", 5, vec!["#ed000c", "#003e8d", "#007c30", "#ffd72d", "#ffffff"])]
#[case::za("./tests/images/flag_za.png", 6, vec!["#007944", "#f42222", "#00158f", "#000000", "#feffff", "#ffb400"])]
fn extract(#[case] path: &str, #[case] n: usize, #[case] expected: Vec<&str>) {
    let img = image::open(path).unwrap();
    let image_data = SimpleImageData::new(img.width(), img.height(), img.as_bytes()).unwrap();

    let palette: Palette<f64> = Palette::extract(&image_data);
    let swatches = palette.swatches(n);
    assert_eq!(swatches.len(), n);

    let colors = swatches
        .iter()
        .map(|swatch| swatch.color().to_hex_string())
        .collect::<Vec<String>>();
    assert_eq!(colors, expected);
}

#[test]
fn extract_with_gmeans() {
    let img = image::open("./tests/images/photo_aLMeYMZEJvk.png").unwrap();
    let image_data = SimpleImageData::new(img.width(), img.height(), img.as_bytes()).unwrap();

    let palette: Palette<f64> = Palette::extract_with(&image_data, Algorithm::GMeans);
    let swatches = palette.swatches(4);
    assert_eq!(swatches.len(), 4);
    assert_eq!(swatches[0].color().to_hex_string(), "#015bd6");
    assert_eq!(swatches[1].color().to_hex_string(), "#523105");
    assert_eq!(swatches[2].color().to_hex_string(), "#d38502");
    assert_eq!(swatches[3].color().to_hex_string(), "#146011");
}

#[test]
fn extract_with_dbscan() {
    let img = image::open("./tests/images/photo_aLMeYMZEJvk.png").unwrap();
    let image_data = SimpleImageData::new(img.width(), img.height(), img.as_bytes()).unwrap();

    let palette: Palette<f64> = Palette::extract_with(&image_data, Algorithm::DBSCAN);
    let swatches = palette.swatches(4);
    assert_eq!(swatches.len(), 4);
    assert_eq!(swatches[0].color().to_hex_string(), "#1d86e8");
    assert_eq!(swatches[1].color().to_hex_string(), "#116902");
    assert_eq!(swatches[2].color().to_hex_string(), "#5b2a03");
    assert_eq!(swatches[3].color().to_hex_string(), "#f6a500");
}

/// This test is ignored because it takes a long time to run
#[test]
#[ignore]
fn extract_with_hdbscan() {
    let img = image::open("./tests/images/photo_aLMeYMZEJvk.png").unwrap();
    let image_data = SimpleImageData::new(img.width(), img.height(), img.as_bytes()).unwrap();

    let palette: Palette<f64> = Palette::extract_with(&image_data, Algorithm::HDBSCAN);
    let swatches = palette.swatches(4);
    assert_eq!(swatches.len(), 4);
    assert_eq!(swatches[0].color().to_hex_string(), "#c08a07");
    assert_eq!(swatches[1].color().to_hex_string(), "#256905");
    assert_eq!(swatches[2].color().to_hex_string(), "#1783e7");
    assert_eq!(swatches[3].color().to_hex_string(), "#80c1fb");
}

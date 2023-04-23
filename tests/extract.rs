extern crate image;

use auto_palette::color_trait::Color;
use auto_palette::{Algorithm, Palette, SimpleImageData};
use rstest::rstest;

#[rstest]
#[case::gr("./tests/images/flag_gr.png", 2, vec!["#ffffff", "#0060b5"])]
#[case::no("./tests/images/flag_no.png", 3, vec!["#cc0028", "#00215f", "#ffffff"])]
#[case::pg("./tests/images/flag_pg.png", 4, vec!["#000000", "#e10017", "#ffcf00", "#fefefe"])]
#[case::sc("./tests/images/flag_sc.png", 5, vec!["#ed000c", "#003e8d", "#007c30", "#ffd72d", "#ffffff"])]
#[case::za("./tests/images/flag_za.png", 6, vec!["#007944", "#f42222", "#00158f", "#000000", "#ffffff", "#ffb400"])]
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
    let img = image::open("./tests/images/aLMeYMZEJvk.png").unwrap();
    let image_data = SimpleImageData::new(img.width(), img.height(), img.as_bytes()).unwrap();

    let palette: Palette<f64> = Palette::extract_with(&image_data, Algorithm::GMeans);
    let swatches = palette.swatches(5);
    assert_eq!(swatches.len(), 5);
    assert_eq!(swatches[0].color().to_hex_string(), "#f6be04");
    assert_eq!(swatches[1].color().to_hex_string(), "#367e0d");
    assert_eq!(swatches[2].color().to_hex_string(), "#0153d1");
    assert_eq!(swatches[3].color().to_hex_string(), "#7bbdf4");
    assert_eq!(swatches[4].color().to_hex_string(), "#ae7e0b");
}

#[test]
fn extract_with_dbscan() {
    let img = image::open("./tests/images/aLMeYMZEJvk.png").unwrap();
    let image_data = SimpleImageData::new(img.width(), img.height(), img.as_bytes()).unwrap();

    let palette: Palette<f64> = Palette::extract_with(&image_data, Algorithm::DBSCAN);
    let swatches = palette.swatches(5);
    assert_eq!(swatches.len(), 5);
    assert_eq!(swatches[0].color().to_hex_string(), "#1a7de5");
    assert_eq!(swatches[1].color().to_hex_string(), "#116a02");
    assert_eq!(swatches[2].color().to_hex_string(), "#5a2d08");
    assert_eq!(swatches[3].color().to_hex_string(), "#fddb01");
    assert_eq!(swatches[4].color().to_hex_string(), "#f8a902");
}

/// This test is ignored because it takes a long time to run
#[test]
#[ignore]
fn extract_with_hdbscan() {
    let img = image::open("./tests/images/aLMeYMZEJvk.png").unwrap();
    let image_data = SimpleImageData::new(img.width(), img.height(), img.as_bytes()).unwrap();

    let palette: Palette<f64> = Palette::extract_with(&image_data, Algorithm::HDBSCAN);
    let swatches = palette.swatches(4);
    assert_eq!(swatches.len(), 4);
    assert_eq!(swatches[0].color().to_hex_string(), "#3785e5");
    assert_eq!(swatches[1].color().to_hex_string(), "#e8af02");
    assert_eq!(swatches[2].color().to_hex_string(), "#165d06");
    assert_eq!(swatches[3].color().to_hex_string(), "#552503");
}

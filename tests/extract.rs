extern crate image;

use auto_palette::color_trait::Color;
use auto_palette::{Algorithm, Palette, SimpleImageData, Theme};
use rstest::rstest;

#[rstest]
#[case::gr("./tests/images/flag_gr.png", 2, vec ! ["#0060b5", "#ffffff"])]
#[case::no("./tests/images/flag_no.png", 3, vec ! ["#cc0028", "#00215f", "#ffffff"])]
#[case::pg("./tests/images/flag_pg.png", 4, vec ! ["#000000", "#e10017", "#ffcf00", "#ffffff"])]
#[case::sc("./tests/images/flag_sc.png", 5, vec ! ["#ed000c", "#003e8d", "#007c30", "#ffd72d", "#ffffff"])]
#[case::za("./tests/images/flag_za.png", 6, vec ! ["#007944", "#f42222", "#00158f", "#ffffff", "#000000", "#ffb400"])]
fn extract(#[case] path: &str, #[case] n: usize, #[case] expected: Vec<&str>) {
    let img = image::open(path).unwrap();
    let image_data = SimpleImageData::new(img.width(), img.height(), img.as_bytes()).unwrap();

    let palette: Palette<f32> = Palette::extract(&image_data);
    let swatches = palette.dominant_swatches(n);
    assert_eq!(swatches.len(), n);

    let colors: Vec<String> = swatches
        .iter()
        .map(|swatch| swatch.color().to_hex_string())
        .collect();
    assert_eq!(colors, expected);
}

#[test]
fn extract_with_gmeans() {
    let img = image::open("./tests/images/aLMeYMZEJvk.png").unwrap();
    let image_data = SimpleImageData::new(img.width(), img.height(), img.as_bytes()).unwrap();

    let palette: Palette<f32> = Palette::extract_with(&image_data, Algorithm::GMeans);
    let swatches = palette.dominant_swatches(4);
    assert_eq!(swatches.len(), 4);
}

#[test]
fn extract_with_dbscan() {
    let img = image::open("./tests/images/aLMeYMZEJvk.png").unwrap();
    let image_data = SimpleImageData::new(img.width(), img.height(), img.as_bytes()).unwrap();

    let palette: Palette<f32> = Palette::extract_with(&image_data, Algorithm::DBSCAN);
    // let swatches = palette.find_with_theme(5, &Theme::Vivid);
    let swatches = palette.dominant_swatches(5);
    swatches.iter().for_each(|swatch| {
        println!(
            "color: {}, population: {}, position: {:?}",
            swatch.color().to_hex_string(),
            swatch.population(),
            swatch.position(),
        );
    });
    assert_eq!(swatches.len(), 5);
}

/// This test is ignored because it takes a long time to run
#[test]
#[ignore]
fn extract_with_hdbscan() {
    let img = image::open("./tests/images/aLMeYMZEJvk.png").unwrap();
    let image_data = SimpleImageData::new(img.width(), img.height(), img.as_bytes()).unwrap();

    let palette: Palette<f32> = Palette::extract_with(&image_data, Algorithm::HDBSCAN);
    let swatches = palette.dominant_swatches(4);
    assert_eq!(swatches.len(), 4);
}

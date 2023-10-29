extern crate image;

use auto_palette::number::{Float, Fraction};
use auto_palette::{Algorithm, Palette, Swatch, Theme};
use rstest::rstest;
use std::collections::HashSet;

#[rstest]
#[case::gr("./tests/images/flag_gr.png", 2, vec ! ["#0060b5", "#ffffff"])]
#[case::no("./tests/images/flag_no.png", 3, vec ! ["#cc0028", "#00215f", "#ffffff"])]
#[case::pg("./tests/images/flag_pg.png", 4, vec ! ["#000000", "#e10017", "#ffcf00", "#fffffe"])]
#[case::sc("./tests/images/flag_sc.png", 5, vec ! ["#ed000c", "#003e8d", "#007c30", "#ffd72d", "#ffffff"])]
#[case::za("./tests/images/flag_za.png", 6, vec ! ["#007944", "#f42222", "#00158f", "#ffffff", "#000000", "#ffb400"])]
fn extract(#[case] path: &str, #[case] n: usize, #[case] expected: Vec<&str>) {
    let image = image::open(path).unwrap();
    let palette: Palette<f64> = Palette::extract(&image);
    let swatches = palette.swatches(n);
    assert_eq!(swatches.len(), n);

    let actual: HashSet<String> = swatches
        .iter()
        .map(|swatch| swatch.color().to_hex_string())
        .collect();
    let expected: HashSet<String> = expected.into_iter().map(|s| s.to_string()).collect();
    assert_eq!(actual, expected);
}

#[test]
fn extract_with_gmeans() {
    let image = image::open("./tests/images/aLMeYMZEJvk.png").unwrap();
    let palette: Palette<f64> = Palette::extract_with_algorithm(&image, &Algorithm::GMeans);
    let swatches = palette.swatches(5);
    assert_eq!(swatches.len(), 5);
}

#[test]
fn extract_with_dbscan() {
    let image = image::open("./tests/images/aLMeYMZEJvk.png").unwrap();
    let palette: Palette<f64> = Palette::extract_with_algorithm(&image, &Algorithm::DBSCAN);
    let swatches = palette.swatches(5);
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

#[test]
fn swatches_with_theme() {
    let image = image::open("./tests/images/aLMeYMZEJvk.png").unwrap();

    struct CustomTheme;
    impl Theme for CustomTheme {
        #[must_use]
        fn weight<F>(&self, swatch: &Swatch<F>) -> Fraction<F>
        where
            F: Float,
        {
            let color = swatch.color();
            let chroma = color.chroma().normalize(F::zero(), F::from_u32(128));
            let lightness = color.lightness().normalize(F::zero(), F::from_u32(100));
            Fraction::new(chroma * lightness)
        }
    }

    let palette: Palette<f64> = Palette::extract(&image);
    let swatches = palette.swatches_with_theme(5, &CustomTheme);
    assert_eq!(swatches.len(), 5);
}

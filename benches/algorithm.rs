use auto_palette::{Algorithm, Palette, SimpleImageData};
use criterion::{criterion_group, criterion_main, Criterion};

fn gmeans_benchmark(c: &mut Criterion) {
    let img = image::open("./tests/images/aLMeYMZEJvk.png").unwrap();
    let image_data = SimpleImageData::new(img.width(), img.height(), img.as_bytes()).unwrap();

    c.bench_function("extract with Gmeans", |b| {
        b.iter(|| {
            let _: Palette<f64> = Palette::extract_with_algorithm(&image_data, &Algorithm::GMeans);
        })
    });
}

fn dbscan_benchmark(c: &mut Criterion) {
    let img = image::open("./tests/images/aLMeYMZEJvk.png").unwrap();
    let image_data = SimpleImageData::new(img.width(), img.height(), img.as_bytes()).unwrap();

    c.bench_function("extract with DBSCAN", |b| {
        b.iter(|| {
            let _: Palette<f64> = Palette::extract_with_algorithm(&image_data, &Algorithm::DBSCAN);
        })
    });
}

criterion_group!(benches, gmeans_benchmark, dbscan_benchmark);
criterion_main!(benches);

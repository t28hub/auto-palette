use auto_palette::{Algorithm, Palette};
use criterion::{criterion_group, criterion_main, Criterion};

fn gmeans_benchmark(c: &mut Criterion) {
    let image = image::open("./tests/images/aLMeYMZEJvk.png").unwrap();
    c.bench_function("extract with Gmeans", |b| {
        b.iter(|| {
            let _: Palette<f64> = Palette::extract_with_algorithm(&image, &Algorithm::GMeans);
        })
    });
}

fn dbscan_benchmark(c: &mut Criterion) {
    let image = image::open("./tests/images/aLMeYMZEJvk.png").unwrap();
    c.bench_function("extract with DBSCAN", |b| {
        b.iter(|| {
            let _: Palette<f64> = Palette::extract_with_algorithm(&image, &Algorithm::DBSCAN);
        })
    });
}

criterion_group!(benches, gmeans_benchmark, dbscan_benchmark);
criterion_main!(benches);

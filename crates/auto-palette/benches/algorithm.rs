use auto_palette::{Algorithm, Palette};
use criterion::{criterion_group, criterion_main, Criterion};

fn gmeans_benchmark(c: &mut Criterion) {
    let image = image::open("./tests/images/m3hn2Kn5Bns.jpg").unwrap();
    c.bench_function("extract with Gmeans", |b| {
        b.iter(|| {
            let palette: Palette<f64> = Palette::extract_with_algorithm(&image, &Algorithm::GMeans);
            assert_eq!(palette.is_empty(), false);
        })
    });
}

fn dbscan_benchmark(c: &mut Criterion) {
    let image = image::open("./tests/images/m3hn2Kn5Bns.jpg").unwrap();
    c.bench_function("extract with DBSCAN", |b| {
        b.iter(|| {
            let palette: Palette<f64> = Palette::extract_with_algorithm(&image, &Algorithm::DBSCAN);
            assert_eq!(palette.is_empty(), false);
        })
    });
}

criterion_group!(benches, gmeans_benchmark, dbscan_benchmark);
criterion_main!(benches);

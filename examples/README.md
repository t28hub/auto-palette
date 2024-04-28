# Examples

This directory contains examples of how to use the `auto-palette` library.
You can run the examples by executing the following command:

```sh
cargo run --example {example_name}
```

## [Basic Usage](basic.rs)
This example demonstrates how to extract a color palette from an image file using default settings.
```sh
cargo run --example basic
```

## Algorithms
### [DBSCAN Algorithm](dbscan.rs)
This example demonstrates how to extract a color palette from an image file using the `DBSCAN` algorithm.
```sh
cargo run --example dbscan --release
```

### [DBSCAN++ Algorithm](dbscanpp.rs)
This example demonstrates how to extract a color palette from an image file using the `DBSCAN++` algorithm.
```sh
cargo run --example dbscanpp --release
```

### [KMeans Algorithm](kmeans.rs)
This example demonstrates how to extract a color palette from an image file using the `k-means` algorithm.
```sh
cargo run --example kmeans --release
```
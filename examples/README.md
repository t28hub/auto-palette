# Examples

This directory contains examples of how to use the `auto-palette` library.
You can run the examples by executing the following command:

```sh
cargo run --example {example_name} --release
```

## [Basic Usage](basic.rs)

This example demonstrates how to extract a color palette from an image file using default settings.

```sh
cargo run --example basic --release
```

## [Algorithm](algorithm.rs)

This example demonstrates how to extract a color palette from an image file using a specific algorithm.

```sh
cargo run --example algorithm --release -- [algorithm_name]
```

## [Theme](theme.rs)

This example demonstrates how to extract a color palette from an image file and find the dominant swatches using a specific theme.

```sh
cargo run --example theme --release -- [theme_name]
```

## [Image Path](image_path.rs)

This example demonstrates how to extract a color palette from an image file using a specific path.

```sh
cargo run --example image_path --release -- [image_path]
```

## [Image URL](image_url.rs)

This example demonstrates how to extract a color palette from an image file using a specific URL.

```sh
cargo run --example image_url --release -- [image_url]
```




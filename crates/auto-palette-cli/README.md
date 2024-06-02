# `auto-palette-cli`

> A CLI tool to extract color palettes from images using the [`auto-palette`](https://crates.io/crates/auto-palette) crate.

## Usage

```sh
A CLI tool to extract prominent colors from images.

Usage: auto-palette [OPTIONS] <PATH>

Arguments:
  <PATH>  Path to the image file.

Options:
  -a, --algorithm <name>  Algorithm for extracting the color palette. [default: dbscan] [possible values: dbscan, dbscan++, kmeans]
  -t, --theme <name>      Theme for selecting the swatches. [possible values: basic, vivid, muted, light, dark]
  -n, --count <number>    Number of colors to extract. [default: 5]
  -c, --color <name>      Output color format. [default: hex] [possible values: hex, rgb, hsl, hsv, lab, luv, lchab, lchuv, oklab, oklch, xyz]
  -o, --output <name>     Output format. [default: text] [possible values: text, table]
      --no-resize         Disable image resizing before extracting the color palette.
  -h, --help              Print help (see more with '--help')
  -V, --version           Print version
```

### Examples

```sh
$ auto-palette path/to/your_image.jpg -n 6 -c rgb -o table
+---+--------------------+------------+------------+
| # | Color              | Position   | Population |
+---+--------------------+------------+------------+
| 1 | RGB(221, 226, 222) | (104, 96)  |       6778 |
| 2 | RGB(3, 144, 149)   | (114, 201) |       5476 |
| 3 | RGB(23, 37, 36)    | (120, 300) |       4300 |
| 4 | RGB(36, 88, 131)   | (183, 145) |       1348 |
| 5 | RGB(254, 29, 44)   | (183, 190) |        779 |
| 6 | RGB(253, 213, 116) | (25, 158)  |        567 |
+---+--------------------+------------+------------+
```

## Development

```sh
cargo run -- {image_path}
```

## License

Licensed under the MIT license. See [LICENSE](../../LICENSE) for more information.

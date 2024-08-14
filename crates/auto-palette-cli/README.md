# `auto-palette-cli`

> 🎨 A CLI tool to extract color palettes from images using the [`auto-palette`](https://crates.io/crates/auto-palette) crate.

[![Build](https://img.shields.io/github/actions/workflow/status/t28hub/auto-palette/ci.yml?style=flat-square)](https://github.com/t28hub/auto-palette/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/auto-palette-cli?style=flat-square)](https://crates.io/crates/auto-palette-cli)
[![Version](https://img.shields.io/crates/v/auto-palette-cli?style=flat-square)](https://crates.io/crates/auto-palette-cli)

## Features

- Extract prominent color palettes from images.
- Supports multiple color extraction algorithms (`dbscan`, `dbscan++`, `kmeans`). Defaults to `dbscan`.
- Supports multiple color selection themes (`basic`, `colorful`, `vivid`, `muted`, `light`, `dark`). Defaults to `basic`.
- Supports multiple color formats (`hex`, `rgb`, `cmyk`, `hsl`, `hsv`, `lab`, `luv`, `lchab`, `lchuv`, `oklab`, `oklch`, `xyz`). Defaults to `hex`.
- Outputs the color palette in multiple formats (`json`, `text`, `table`). Defaults to `text`.

## Installation

```sh
cargo install auto-palette-cli
```

## Usage

```sh
$ auto-palette --help
🎨 A CLI tool to extract prominent color palettes from images.

Usage: auto-palette [OPTIONS] <PATH>

Arguments:
  <PATH>  Path to the image file.

Options:
  -a, --algorithm <name>  Algorithm for extracting the color palette. [default: dbscan] [possible values: dbscan, dbscan++, kmeans]
  -t, --theme <name>      Theme for selecting the swatches. [possible values: basic, colorful, vivid, muted, light, dark]
  -n, --count <number>    Number of colors to extract. [default: 5]
  -c, --color <name>      Output color format. [default: hex] [possible values: hex, rgb, cmyk, hsl, hsv, lab, luv, lchab, lchuv, oklab, oklch, xyz]
  -o, --output <name>     Output format. [default: text] [possible values: json, text, table]
      --no-resize         Disable image resizing before extracting the color palette.
  -h, --help              Print help (see more with '--help')
  -V, --version           Print version
```

## Examples

### Basic usage

Here is an example of extracting the color palette from an image:

```sh
$ auto-palette path/to/your_image.jpg
   #EB3739 (82, 293) 7751
   #A24F01 (114, 129) 132 
   #FB9C04 (96, 155) 112 
   #D25A6E (58, 228) 20  
   #8DA502 (94, 84) 16  
```

### Advanced usage

Here is an example of extracting the color palette from an image using the `vivid` theme, the `rgb` color format, and the `table` output format:

```sh
$ auto-palette path/to/your_image.jpg -t vivid -n 6 -c rgb -o table
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

### Building

```sh
cargo run -- {image_path}
```

### Testing

```sh
cargo nextest run --tests --all-features --package auto-palette-cli
```

## License

This project is distributed under the MIT license. See the [LICENSE](../../LICENSE) file for more details.

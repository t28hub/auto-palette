# `auto-palette-cli`

> 🎨 A CLI tool to extract color palettes from images using the [`auto-palette`](https://crates.io/crates/auto-palette) crate.

[![Build](https://img.shields.io/github/actions/workflow/status/t28hub/auto-palette/ci.yml?style=flat-square)](https://github.com/t28hub/auto-palette/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/auto-palette-cli?style=flat-square)](https://crates.io/crates/auto-palette-cli)
[![Version](https://img.shields.io/crates/v/auto-palette-cli?style=flat-square)](https://crates.io/crates/auto-palette-cli)

## Features

- Extract prominent color palettes from images.
- Supports multiple color extraction algorithms: `dbscan`, `dbscan++`, `kmeans`, `slic`, and `snic`. (default: `dbscan`).
- Flexible theme selection for color swatches: `colorful`, `vivid`, `muted`, `light`, and `dark`.
- Supports multiple color formats (`hex`, `rgb`, `cmyk`, `hsl`, `hsv`, `lab`, `luv`, `lchab`, `lchuv`, `oklab`, `oklch`, `xyz`). (default: `hex`).
- Multiple output formats: `json`, `text`, and `table`. (default: `text`).
- Clipboard support for instant palette extraction.

## Installation

```sh
cargo install auto-palette-cli
```

## Quick Start

Extract a simple 5 color palette from an image:
```sh
auto-palette path/to/your_image.jpg
```

Extract a 5 color palette from an image using the `vivid` theme and `rgb` color format with `table` output format:
```sh
auto-palette path/to/your_image.jpg -n 5 -a dbscan++ -t vivid -c rgb -o table
```

Extract a color palette from clipboard image:
```sh
auto-palette --clipboard
```

## Usage

```sh
$ auto-palette -h
🎨 CLI tool to extract a prominent color palette from an image.

Usage: auto-palette [OPTIONS] [PATH]

Arguments:
  [PATH]  Path to the image file, or supply --clipboard

Options:
  -a, --algorithm <ALGORITHM>   Extraction algorithm [default: dbscan] [possible values: dbscan, dbscan++, kmeans, slic, snic]
  -t, --theme <THEME>           Swatch theme [possible values: colorful, vivid, muted, light, dark]
  -n, --count <N>               Number of swatches [default: 5]
  -c, --color-space <SPACE>     Output color space [default: hex] [possible values: hex, rgb, cmyk, hsl, hsv, lab, luv, lchab, lchuv, oklab, oklch, xyz]
  -o, --output-format <FORMAT>  Output format [default: text] [possible values: json, text, table]
      --no-resize               Disable image resizing before extracting the color palette.
      --clipboard               Read image from system clipboard instead of a file.
  -h, --help                    Print help (see more with '--help')
  -V, --version                 Print version
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

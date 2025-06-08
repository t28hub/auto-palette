# Examples

This directory contains examples demonstrating how to use the `auto-palette` library.

## Running Examples

To run an example, use the following command, replacing `{example_name}` with the name of the example you want to run:

```sh
cargo run --example {example_name} --release --features='image'
```

## [Basic Usage](simple.rs)
This example demonstrates how to extract a color palette from an image and find the prominent colors.

### Command
```sh
cargo run --example simple --release --features='image' -- [path]
```
Replace `[path]` with the path to your image file. 
If no path is provided, it will use a default image.

### Output Example
```txt
Extracted 164 swatch(es) in 0.536 seconds
 # | Color   | Position     | Population | Ratio 
 1 | #5ECCFD | ( 607,  346) |     109035 |  0.40
 2 | #D80A1D | ( 158,  267) |       6490 |  0.02
 3 | #011424 | (  66,  171) |       4475 |  0.02
 4 | #C1075C | ( 338,  182) |       2482 |  0.01
 5 | #F8DA27 | (  97,   27) |       2152 |  0.01
```

## [Algorithm](algorithm.rs)
This example demonstrates how to use different color extraction algorithms.

### Command
```sh
cargo run --example algorithm --release --features='image' -- [algorithm]
```

Replace `[algorithm]` with one of the following: `dbscan`, `dbscan++`, `kmeans`, `slic`, or `snic`.
If no algorithm is provided, it will use `dbscan` as the default.

### Output Example
```txt
Extracted 114 swatch(es) in 0.222 seconds
 # | Color   | Position     | Population | Ratio 
 1 | #5ECCFD | ( 511,  211) |     108924 |  0.40
 2 | #D50D1B | (  42,  375) |      15128 |  0.06
 3 | #041D31 | (  85,  152) |      14335 |  0.05
 4 | #FDDF22 | (  90,   40) |       1576 |  0.01
 5 | #A5226E | ( 390,  212) |        756 |  0.00
```


## [Theme](theme.rs)

This example demonstrates how to select swatches based on specific themes.

### Command
```sh
cargo run --example theme --release --features='image' -- [theme]
```

Replace `[theme]` with one of the following: `colorful`, `vivid`, `muted`, `light`, or `dark`.
If no theme is provided, it will select the top 5 swatches without a specific theme.

### Output Example
```txt
Extracted 167 swatch(es) in 0.475 seconds
 # | Color   | Position     | Population | Ratio 
 1 | #D00104 | (  23,  347) |       2159 |  0.01
 2 | #C01D76 | ( 342,  232) |        229 |  0.00
 3 | #BC960E | ( 196,   28) |        208 |  0.00
 4 | #D72A44 | ( 199,  363) |        117 |  0.00
 5 | #0A6AAB | ( 367,  121) |         27 |  0.00
```
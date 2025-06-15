# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased](https://github.com/t28hub/auto-palette/compare/v0.9.1...HEAD)
### Added
### Changed
### Deprecated
### Removed
### Fixed
### Security

## [v0.9.1](https://github.com/t28hub/auto-palette/compare/v0.9.0...v0.9.1) - 2025-06-15
### Changed
- Update `ClusteringAlgorithm` interface and refactor DBSCAN implementation for better modularity. (#208)
- Update color merge threshold in `to_swatches` to improve color grouping accuracy. (#209)
- Improve swatches selection logic in `find_swatches` and `find_swatches_with_theme` for better palette extraction. (#212)
- Replace `HashMap` and `HashSet` with `FxHashMap` and `FxHashSet` to enhance performance. (#214)


## [v0.9.0](https://github.com/t28hub/auto-palette/compare/v0.8.2...v0.9.0) - 2025-06-04
### Added
- Implement SLIC (Simple Linear Iterative Clustering) algorithm for color palette extraction. (#176)
- Implement SNIC (Simple Non-Iterative Clustering) algorithm for color palette extraction. (#177)
- Implement superpixel-based image segmentation using DBSCAN algorithm. (#199)
- Implement `FastDbscanSegmentation` using DBSCAN++ clustering algorithm. (#200)

### Changed
- Reduce overhead by moving KMeans implementation to the segmentation module. (#197)
- Optimize `KDTreeSearch` for faster neighbor search. (#198)

### Fixed
- Resolve pixel count mismatch with mask filtering. (#193)

## [v0.8.2](https://github.com/t28hub/auto-palette/compare/v0.8.1...v0.8.2) - 2025-05-17
### Fixed
- Fixed a bug where pixels were incorrectly excluded when the filter returned `true`. Pixels are now excluded only when the filter returns `false`. (#189)

## [v0.8.1](https://github.com/t28hub/auto-palette/compare/v0.8.0...v0.8.1) - 2025-05-08
### Changed
- Improve KMeans initialization to use a grid-based algorithm instead of the KMeans++ algorithm, enhancing performance and clustering stability. (#179)
- 
### Fixed
- Ensure compatibility with Rust stable versions by removing unstable `#![feature]` attributes. (#184)

## [v0.8.0](https://github.com/t28hub/auto-palette/compare/v0.7.0...v0.8.0) - 2025-04-28
### Added
- Add diverse sampling algorithms (Farthest, Weighted Farthest, Diversity) to improve swatch selection. (#149)
- Add theme-based palette presets and update the Rust example. (#151)
- Add WebAssembly support via `auto-palette-wasm` npm package (exposes `Palette`, `Swatch`, `Color`). (#158, #157, #155, #154, #152)
- Add bundled TypeScript typings for `JsColor`, `JsPalette`, and `JsSwatch`. (#165, #166)
- Add `PaletteBuilder` for chainable palette extraction (algorithms, filters, swatch limits). (#170)

### Changed
- Replace all `unwrap`/`expect` with `Result` based handling for safety error handling. (#148, #169)
- Optimize palette internals and unified error handling. (#148)
- Adopt Gaussian scoring for theme scoring, improving accuracy and consistency. (#147)

### Deprecated
- Deprecate `Palette::extract_with_algorithm`, use `Palette::builder` instead. (#170)
- Deprecate `Theme::Basic` theme, use `Palette::find_swatches` instead. (#150, #173)

### Fixed
- Fix generated TypeScript definitions for color structs and added regression tests. (#159)

### Security
- Upgrade `axios` to resolve upstream CVEs. (#156)
- Patch Vite `server.fs.deny` traversal vulnerability. (#168)


## [v0.7.0](https://github.com/t28hub/auto-palette/compare/v0.6.0...v0.7.0) - 2025-03-26
### Added
- Accept input directly from the system clipboard. (#140)

### Changed
- Update `find_swatches_with_theme` to use diversity sampling strategies for swatch selection. (#136)
- Refactor the `color` module to improve performance and conversion accuracy. (#141)
- Exclude default features of the `image` crate to reduce dependencies and speed up builds. (#144)


## [v0.6.0](https://github.com/t28hub/auto-palette/compare/v0.5.0...v0.6.0) - 2025-02-15
### Added
- Methods to convert `Color` to and from `ColorInt` (integer representation). (#130)

### Changed
- Upgrade [`rand`](https://github.com/rust-random/rand/releases/tag/0.9.0) and related dependencies to `0.9.0`. (#129)


## [v0.5.0](https://github.com/t28hub/auto-palette/compare/v0.4.0...v0.5.0) - 2024-08-13
### Changed
- Improve the scoring formula and criteria for more accurate color selection. (#83)


## [v0.4.0](https://github.com/t28hub/auto-palette/compare/v0.3.0...v0.4.0) - 2024-06-07
### Added
- Support conversion to `CMYK`, `ANSI16`, and `ANSI256` color formats. (#55, #56, #57, #71)
- `ratio` field in `Swatch`, indicating each color’s proportion in the image. (#66)
- `Theme::Colorful` for vibrant palette selection. (#66)

### Changed
- Improve color-selection criteria and scoring to better match themes. (#66)


## [v0.3.0](https://github.com/t28hub/auto-palette/compare/v0.2.0...v0.3.0) - 2024-05-19
### Added
- Support for `HSL` and `HSV` color spaces. (#34)
- Support for `CIE L*u*v*` color space. (#35)
- Support for `LCH(ab)` and `LCH(uv)` color spaces. (#41)
- Support for `OkLab` and `OkLCh` color spaces. (#42)
- Enable the `image` feature by default for greater flexibility. (#45)


## [v0.2.0](https://github.com/t28hub/auto-palette/compare/v0.1.0...v0.2.0) - 2024-05-09
### Added
- Optional `image` feature to make the `image` crate dependency opt-in. 
- `wasm` feature for WebAssembly targets.

### Changed
- `ImageData` now stores a reference to pixel data, improving efficiency. 


## [v0.1.1](https://github.com/t28hub/auto-palette/compare/v0.1.0...v0.1.1) - 2024-04-30

### Changed
- Improve default swatch-selection algorithm using swatch population. 

### Fixed
- Resolve compilation errors in the `algorithm`, `image_path`, and `image_url` examples. 


## [v0.1.0](https://github.com/t28hub/auto-palette/releases/tag/v0.1.0) - 2024-04-29
### Added
- Initial release.


[Unreleased]: https://github.com/t28hub/auto-palette/compare/v0.9.0...HEAD
[v0.8.2]: https://github.com/t28hub/auto-palette/compare/v0.8.2...v0.8.3
[v0.8.2]: https://github.com/t28hub/auto-palette/compare/v0.8.1...v0.8.2
[v0.8.1]: https://github.com/t28hub/auto-palette/compare/v0.8.0...v0.8.1
[v0.8.0]: https://github.com/t28hub/auto-palette/compare/v0.7.0...v0.8.0
[v0.7.0]: https://github.com/t28hub/auto-palette/compare/v0.6.0...v0.7.0
[v0.6.0]: https://github.com/t28hub/auto-palette/compare/v0.5.0...v0.6.0
[v0.5.0]: https://github.com/t28hub/auto-palette/compare/v0.4.0...v0.5.0
[v0.4.0]: https://github.com/t28hub/auto-palette/compare/v0.3.0...v0.4.0
[v0.3.0]: https://github.com/t28hub/auto-palette/compare/v0.2.0...v0.3.0
[v0.2.0]: https://github.com/t28hub/auto-palette/compare/v0.1.1...v0.2.0
[v0.1.1]: https://github.com/t28hub/auto-palette/compare/v0.1.0...v0.1.1
[v0.1.0]: https://github.com/t28hub/auto-palette/releases/tag/v0.1.0
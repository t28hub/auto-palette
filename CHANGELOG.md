# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased](https://github.com/t28hub/auto-palette/compare/v0.7.0...HEAD)
### Added
- Add diverse sampling algorithms (Farthest, Weighted Farthest, Diversity) to improve swatch selection. (#149)
- Add predefined **theme-based color-palette presets** and update the Rust example accordingly. (#151)
- Add **WebAssembly support** via the new `auto-palette-wasm` npm package, exposing `Palette`, `Swatch`, and `JsColor` to JavaScript/TypeScript users. (#158, #157, #155, #154, #152)
- Ship ready-to-use **TypeScript type declarations** for `JsColor`, `JsPalette`, and `JsSwatch`. (#166, #165)
- Add `PaletteBuilder` for easy, chainable configuration of palette extraction (algorithms, filters, swatch limits). (#170)

### Changed
- Replaced all `unwrap`/`expect` calls in clustering and palette code with `Result` based handling for greater safety and readability. (#169, #148)
- Optimised palette implementation and consolidated error handling. (#148)
- Adopted a Gaussian function for theme scoring, improving accuracy and consistency. (#147)

### Removed

### Fixed
- Corrected generated TypeScript type definitions for colour structs and added unit tests to guard against regressions. (#159)

### Security
- Patched Vite `server.fs.deny` bypass that allowed invalid request-target traversal. (#168)
- Upgraded `axios` to resolve known vulnerabilities in transitive dependencies. (#156)

### Deprecated


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


[Unreleased]: https://github.com/t28hub/auto-palette/compare/v0.7.0...HEAD
[0.7.0]: https://github.com/t28hub/auto-palette/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/t28hub/auto-palette/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/t28hub/auto-palette/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/t28hub/auto-palette/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/t28hub/auto-palette/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/t28hub/auto-palette/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/t28hub/auto-palette/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/t28hub/auto-palette/releases/tag/v0.1.0
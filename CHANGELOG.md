# Changelog

## [v0.5.0](https://github.com/t28hub/auto-palette/releases/tag/v0.5.0)

* Improve the scoring formula and criteria for better color selection.
* Enhance documentation of `auto-palette-cli`.

## [v0.4.0](https://github.com/t28hub/auto-palette/releases/tag/v0.4.0)

* Introduce support for color format conversion to `CMYK`, `ANSI16`, and `ANSI256`.
* Enhance the `Swatch` struct with the addition of a `ratio` field, representing the proportion of the image that the color occupies.
* Add `Theme::Colorful` for a more vibrant color selection in the palette.
* Improve color selection criteria and scoring for better alignment with themes.

## [v0.3.0](https://github.com/t28hub/auto-palette/releases/tag/v0.3.0)

* Add multiple color spaces, including the following:
  * `HSL`
  * `HSV`
  * `CIE L*u*v*`
  * `LCH(ab)`
  * `LCH(uv)`
  * `Oklch`
  * `Oklab`
* Add the `image` feature to the default features, enhancing the flexibility of library usage.

## [v0.2.0](https://github.com/t28hub/auto-palette/releases/tag/v0.2.0)

* Modified the `ImageData` interface to hold a reference to the image's pixel data, enhancing the efficiency of data handling.
* Introduced the `image` feature, making the dependency on the `image` crate optional. This allows for more flexibility in feature usage.
* Introduced the `wasm` feature, ensuring proper resolution of dependencies for WebAssembly targets. This enhances the library's compatibility with wasm-based applications.

## [v0.1.1](https://github.com/t28hub/auto-palette/releases/tag/v0.1.1)

* Improve the default swatch selection algorithm by using swatch population.
* Fix the compilation error in the `algorithm`, `image_path` and `image_url` examples.

## [v0.1.0](https://github.com/t28hub/auto-palette/releases/tag/v0.1.0)

* Initial release.

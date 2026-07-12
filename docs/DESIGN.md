# auto-palette API Design

This document describes the design of the `auto-palette` public API introduced
in v0.10.0, the reasoning behind it, and the roadmap for delivering the library
to platforms beyond Rust (CLI, WebAssembly, Android, and iOS).

## Goals

1. **Developer-friendly interface** — a small, predictable API that can be
   learned from a single example and used without turbofish annotations or
   builder type gymnastics.
2. **Bounded running time** — every stage of the pipeline is at worst
   `O(n log n)` in the number of processed pixels, and the number of processed
   pixels itself can be bounded with a single builder option.
3. **Idiomatic Rust** — naming per [RFC 430] (acronyms are camel case:
   `Rgb`, `Hsl`, `Dbscan`), `FromStr`/`Display` pairs, a flat `Error` enum
   built on `thiserror`, and accessors that follow the API guidelines.
4. **FFI-ready core** — the public surface contains no generics, no lifetimes,
   no trait objects in return position, and no borrowed buffers, so the same
   API can be projected 1:1 onto WebAssembly (wasm-bindgen), Android/iOS
   (UniFFI), and any C ABI consumer.

[RFC 430]: https://github.com/rust-lang/rfcs/blob/master/text/0430-finalizing-naming-conventions.md

## Public API surface

```text
auto_palette
├── ImageData            owned RGBA buffer (width, height, Vec<u8>)
├── Palette              extracted palette; find_swatches(), find_swatches_with_theme()
├── PaletteBuilder       algorithm() / filter() / max_swatches() / max_pixels() / build()
├── Swatch               color + position + population + ratio
├── Position             { x: u32, y: u32 }
├── Algorithm            KMeans | Dbscan | DbscanPlusPlus | Slic | Snic
├── Theme                Colorful | Vivid | Muted | Light | Dark
├── Filter, AlphaFilter  pixel predicates (closures implement Filter)
├── Error, Result        flat error enum (thiserror)
└── color                Color, Rgb, Hsl, Hsv, Cmyk, Xyz, Lab, Luv,
                         Lchab, Lchuv, Oklab, Oklch, Hue, Ansi16, Ansi256, Gamut
```

Typical usage:

```rust
use auto_palette::{ImageData, Palette};

let image = ImageData::load("photo.jpg")?;
let palette = Palette::extract(&image)?;
for swatch in palette.find_swatches(5) {
    println!("{} at {}", swatch.color().to_hex_string(), swatch.position());
}
```

Customized usage:

```rust
use auto_palette::{Algorithm, ImageData, Palette, Rgba};

let palette = Palette::builder()
    .algorithm(Algorithm::KMeans)
    .filter(|pixel: &Rgba| pixel[3] >= 64)
    .max_pixels(160_000) // bound the running time for large photos
    .build(&image)?;
```

## Key decisions

### 1. Monomorphic public API (`f64` internally)

Previously every public type was generic over `T: FloatNumber`
(`Palette<T>`, `Swatch<T>`, `Color<T, W>` with a white-point type parameter).
In practice every consumer — the CLI, the WebAssembly bindings, the examples,
the README — instantiated `T = f64`, while the generic surface imposed real
costs:

* `Palette::<f64>::extract(...)` turbofish in the very first example a user
  sees;
* type-state builders (`PaletteBuilder<T, CompositeFilter<F1, F2>>`) whose
  types change as options are set;
* **no path to mobile bindings**: UniFFI, JNI, and Swift bridging cannot
  express Rust generics.

The redesign fixes the numeric type to `f64` at the public boundary. The
internal `math` and `segmentation` modules remain generic over `FloatNumber`
(now crate-private), so an `f32` pipeline can still be offered later behind a
feature flag without changing the public API. Because `f64` was the universal
instantiation, all extraction results are bit-identical to the previous
version.

For callers that only hold a borrowed buffer,
`PaletteBuilder::build_from_raw(width, height, &[u8])` extracts a palette
directly from the slice — internally the pipeline runs on a borrowed
`RawImage<'_>` view, so no copy of the pixel data is made. `ImageData` remains
the owned, FFI-facing representation.

The `W: WhitePoint` parameter on `Color`/`Lab`/`Luv` is likewise gone from the
public API; conversions use D65, and the white-point machinery stays internal.

### 2. Owned `ImageData`, no lifetimes

`ImageData<'a>` borrowed its pixel buffer via `Cow<'a, [u8]>`. A lifetime
parameter in the entry-point type is a hard blocker for FFI object handles and
forces lifetime plumbing on every embedding. `ImageData` now owns a
`Vec<u8>`; `ImageData::new(width, height, data)` accepts anything convertible
into `Vec<u8>`. The one extra copy (only when the caller keeps a borrowed
slice) is negligible next to segmentation cost, and it buys `Send + Sync +
'static` handles that can be stored in JS/Kotlin/Swift objects.

### 3. Value-level configuration instead of type-level

`PaletteBuilder` is a plain struct. Filters are stored as a boxed chain
(`Vec<Box<dyn Filter>>` behind the scenes), so adding a filter no longer
changes the builder's type. `Algorithm` is a plain `Copy` enum. This makes the
builder representable in bindings (each setter maps to a method on an FFI
object) and keeps `rustdoc` signatures readable.

### 4. Flat `Error` enum

The previous hierarchy (`Error` → `ExtractionError`/`SelectionError`/
`UnsupportedError`/`ImageError`, each with a private `kind()`) required three
levels of matching to answer "what went wrong". It is now one
`#[non_exhaustive]` enum with `thiserror`-derived messages. Every variant maps
cleanly onto an exception/error type in each binding language.

`Palette::find_swatches` no longer returns `Result`: with a validated palette
its failure paths were unreachable by construction, and infallible signatures
remove error-handling ceremony from the hot path of every consumer.

### 5. Naming (RFC 430)

`RGB` → `Rgb`, `HSL` → `Hsl`, `HSV` → `Hsv`, `CMYK` → `Cmyk`, `XYZ` → `Xyz`,
`LCHab` → `Lchab`, `LCHuv` → `Lchuv`, `Algorithm::DBSCAN` →
`Algorithm::Dbscan`, `DBSCANpp` → `DbscanPlusPlus`, `SLIC` → `Slic`, `SNIC` →
`Snic`. String forms (`"dbscan++"`, `"lchab"`) are unchanged, so CLI flags,
serialized data, and the JS API are unaffected. Deprecated items
(`Theme::Basic`, `Palette::extract_with_algorithm`) are removed.

## Time complexity

`n` = processed pixels, `k` = segments (default 128), `s` = swatches (≤ 256).

| Stage | Algorithm | Complexity |
|---|---|---|
| Pixel conversion | RGBA → CIELAB 5-D points | `O(n)` |
| Segmentation | DBSCAN (KD-tree) | `O(n log n)` expected |
| | DBSCAN++ | `O(n log c)`, `c` = sampled core pixels |
| | K-means (KD-tree per iteration) | `O(iter · n log k)` |
| | SLIC (windowed) | `O(iter · n)` |
| | SNIC (priority queue) | `O(n log n)` |
| Color merge | DBSCAN over `k` segment centers | `O(k log k)` |
| Swatch selection | gated MMR / diversity sampling | `O(s²)` worst case, `s ≤ 256` |

Two properties keep the pipeline predictable:

* **`PaletteBuilder::max_pixels(budget)`** downsamples the image with
  nearest-neighbor sampling before segmentation, making the whole pipeline
  `O(budget)`-bounded regardless of input resolution. Positions are still
  reported in original-image coordinates. This is opt-in to preserve exact
  results for existing users; bindings for interactive platforms (mobile,
  web) are encouraged to set a default budget (the CLI resizes to ≤ 360×360
  for the same reason).
* All neighbor searches use KD-trees or spatial windows — nothing in the
  default path is `O(n²)`. The KD-tree radius search degrades toward `O(n)`
  per query only for pathological `epsilon` values, which the config
  validation keeps out of reach of the default API.

## Multi-platform roadmap

```text
              ┌────────────────────────┐
              │   auto-palette (core)  │  ← no generics / lifetimes,
              │   owned data, Send+Sync│    Send + Sync, flat errors
              └───────────┬────────────┘
      ┌───────────┬───────┴────────┬─────────────┐
      │           │                │             │
auto-palette-cli  auto-palette-wasm  auto-palette-ffi (planned)
   (clap)        (wasm-bindgen)      (UniFFI proc-macros)
                      │                   │
                @auto-palette/wasm   Kotlin (Android AAR)
                    (npm)            Swift (XCFramework / SPM)
```

The planned `auto-palette-ffi` crate exposes the core API via
[UniFFI](https://mozilla.github.io/uniffi-rs/) proc-macros:

* `ImageData`, `Palette`, `Swatch`, `Position` map to UniFFI objects/records
  as-is — this is exactly what the monomorphic, owned-data redesign enables.
* `Algorithm` and `Theme` map to UniFFI enums; `Error` maps to a UniFFI error.
* Pixel filters are exposed as data (alpha threshold, luminance range) rather
  than callbacks, since crossing the FFI boundary per pixel would dominate the
  running time; the `Filter` trait remains a Rust-only extension point.
* Suggested platform defaults: `max_pixels(160_000)` on mobile bindings.

Release packaging: `cargo ndk` for Android (`.aar` with JNI libs per ABI) and
an XCFramework + Swift Package for iOS, built in CI alongside the existing
wasm-pack pipeline.

## Non-goals / future work

* Per-algorithm tuning (epsilon, segment count, iterations) is intentionally
  not exposed yet; the config structs exist internally and can be surfaced
  later as `Algorithm` variants with data or a dedicated config API once the
  requirements are clear.
* An `f32` pipeline (halves memory bandwidth on mobile) can be introduced as a
  cargo feature without public API changes.
* Theme-based selection uses gated Maximal Marginal Relevance
  (`math/sampling/mmr.rs`): a relevance gate keeps every candidate within the
  theme band, and an additive, normalized relevance/diversity criterion with a
  minimum-separation constraint maximizes perceived variety, measured in a
  lightness-downweighted CIELAB space (themes constrain chroma/lightness, so
  hue is the main source of perceived variation).
* SLIC/SNIC currently ignore their `compactness` parameter (the 5-D metric is
  unweighted); fixing this changes extraction results and is deferred to a
  behavior-change release.

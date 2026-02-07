# Codebase Review: auto-palette

Comprehensive code review of the `auto-palette` Rust workspace, covering all three crates:
`auto-palette` (core), `auto-palette-cli`, and `auto-palette-wasm`.

---

## Table of Contents

1. [Critical / High Severity](#1-critical--high-severity)
2. [Medium Severity](#2-medium-severity)
3. [Low Severity](#3-low-severity)
4. [Summary Statistics](#4-summary-statistics)

---

## 1. Critical / High Severity

### 1.1 [BUG] `clamp_to_u8` does not actually clamp — can produce wrong color values

**File:** `crates/auto-palette/src/color/rgb.rs:147-153`

```rust
fn clamp_to_u8<T>(value: T) -> u8
where
    T: FloatNumber,
{
    let max = T::from_u8(RGB::MAX);
    (value * max).round().trunc_to_u8()
}
```

Despite the function name, no clamping occurs. `trunc_to_u8()` performs `*self as u8`, which is
saturating in Rust 1.45+ (NaN → 0, Inf → 255, negative → 0). However, if `value` is slightly
above `1.0` due to floating-point imprecision in color space conversion (e.g., `1.0000001`), then
`value * 255` rounds to `256.0`, and `256.0 as u8` saturates to `255` — which happens to be
correct in that specific case, but values like `1.003` would produce `256.7 → 255` silently
losing precision. More critically, negative values (possible from out-of-gamut LAB→RGB
conversions) would clamp to 0 without any warning.

**Recommendation:** Add explicit clamping before the cast:

```rust
fn clamp_to_u8<T>(value: T) -> u8
where
    T: FloatNumber,
{
    let max = T::from_u8(RGB::MAX);
    (value * max).round().clamp(T::zero(), max).trunc_to_u8()
}
```

---

### 1.2 [BUG] SLIC `compactness` parameter is stored but never used in the algorithm

**File:** `crates/auto-palette/src/image/segmentation/slic/algorithm.rs:31`

The `compactness` field is defined, validated in the builder (`line 323`), and stored in the
struct (`line 334`), but it is **never referenced** in any algorithmic methods (`iterate`,
`find_lowest_gradient_index`, `segment_with_mask`).

In the SLIC paper, compactness `m` is a critical parameter in the combined distance metric:

```
D = sqrt(d_c² + (d_s/S)² × m²)
```

Without it, the algorithm collapses spatial and color distances into an unweighted metric. Users
can set `compactness` via the builder, but it has zero effect on output.

**Recommendation:** Integrate compactness into the distance calculation in `iterate()`, weighting
the spatial distance component relative to the color distance as described in the original paper.

---

### 1.3 [BUG] Oklch lightness and chroma bounds are incorrect

**File:** `crates/auto-palette/src/color/oklch.rs:60-65`

```rust
Self {
    l: clamp(l, T::zero(), T::from_u32(100)),
    c: clamp(c, T::zero(), T::from_u32(180)),
    h: Hue::from_degrees(h),
}
```

Oklch uses lightness in range `[0, 1]` and chroma approximately `[0, 0.5]`. The bounds `[0, 100]`
and `[0, 180]` are appropriate for CIE LCH(ab)/LCH(uv) but not for Oklch. The current code
happens to work because valid Oklch values fall well within these bounds, but the clamping is
semantically incorrect and would not catch actually-invalid values.

**Recommendation:** Change to:

```rust
l: clamp(l, T::zero(), T::one()),
c: clamp(c, T::zero(), T::from_f32(0.5)),
```

---

### 1.4 [BUG] CLI: Silent `process::exit(1)` with no error message (2 occurrences)

**File:** `crates/auto-palette-cli/src/main.rs:56-58` and `66-68`

```rust
let Ok(image_data) = ImageData::try_from(&resized) else {
    process::exit(1);
};
```

Both paths silently exit with code 1, discarding the `Err` variant entirely. The user receives
no feedback about what went wrong.

**Recommendation:** Use `?` or `.with_context()` to propagate errors through the
`anyhow::Result<()>` return type that `main()` already uses:

```rust
let image_data = ImageData::try_from(&resized)
    .map_err(|e| anyhow::anyhow!("Failed to create image data: {e}"))?;
```

---

### 1.5 [BUG] CLI: Timing output to `stdout` corrupts JSON output

**File:** `crates/auto-palette-cli/src/main.rs:99-104`

```rust
println!(
    "Extracted {} swatch(es) in {}.{:03} seconds",
    palette.len(), ...
);
```

When `--output-format json` is used, the timing message is appended after the JSON on stdout,
making the output invalid JSON. Any downstream tool piping the output to `jq` etc. will fail.

**Recommendation:** Use `eprintln!` instead of `println!` for the timing message.

---

### 1.6 [BUG] CLI: `palette.len()` reported instead of `swatches.len()`

**File:** `crates/auto-palette-cli/src/main.rs:101`

The timing message reports `palette.len()` (total internal clusters, e.g. 50) instead of
`swatches.len()` (the user-requested count, e.g. 3 via `--count 3`). This is misleading.

**Recommendation:** Change `palette.len()` to `swatches.len()`.

---

### 1.7 [BUG] No `mask.len() == pixels.len()` validation in segmentation algorithms

**Files:** All 5 `segment_with_mask` implementations:
- `crates/auto-palette/src/image/segmentation/dbscan/algorithm.rs:134`
- `crates/auto-palette/src/image/segmentation/fastdbscan/algorithm.rs:193`
- `crates/auto-palette/src/image/segmentation/kmeans/algorithm.rs:104`
- `crates/auto-palette/src/image/segmentation/slic/algorithm.rs:185`
- `crates/auto-palette/src/image/segmentation/snic/algorithm.rs:108`

All validate `width * height == pixels.len()` but none validates `mask.len() == pixels.len()`.
A mismatched mask causes an index-out-of-bounds panic at runtime.

**Recommendation:** Add `mask.len() == pixels.len()` validation at the start of each
`segment_with_mask` implementation, returning an appropriate error variant.

---

### 1.8 [BUG] `find_swatches_internal` is `pub` but should be `pub(crate)`

**File:** `crates/auto-palette/src/palette.rs:145`

```rust
pub fn find_swatches_internal<S, F1, F2>(
```

This method leaks internal types (`SamplingAlgorithm`, `SamplingError`) into the public API.

**Recommendation:** Change to `pub(crate)`.

---

### 1.9 [BUG] WASM: Silent swatch dropping in `JsPalette::new()`

**File:** `crates/auto-palette-wasm/src/palette.rs:70-87`

`filter_map` silently discards swatches whose position deserialization fails. No indication is
returned to the caller about how many items were dropped. `new()` returns `Result<Self, JsError>`
but never returns `Err` — an empty palette from total deserialization failure is indistinguishable
from "no colors found."

**Recommendation:** Either propagate the first error or log a warning about dropped swatches.

---

### 1.10 [BUG] Division-by-zero risk in Luv-to-XYZ conversion

**File:** `crates/auto-palette/src/color/xyz.rs:186-192`

```rust
let a = ((T::from_f32(52.0) * luv.l) / (luv.u + T::from_f32(13.0) * luv.l * u0) - T::one())
```

When `luv.u + 13.0 * luv.l * u0 == 0` (or the `v` equivalent), division by zero produces
`inf`/`NaN`. The guard at line 171 handles `luv.l == 0` but not all zero-denominator cases.

**Recommendation:** Add a guard for the denominator being zero (or near-zero).

---

## 2. Medium Severity

### 2.1 [CONSISTENCY] Missing `Copy` derive on 11 color types

**Files:** `hsl.rs:33`, `hsv.rs:33`, `lab.rs:37`, `lchab.rs:36`, `lchuv.rs:36`, `luv.rs:36`,
`cmyk.rs:31`, `oklab.rs:33`, `oklch.rs:35`, `ansi16.rs:24`, `ansi256.rs:25`

All these types contain only `Copy` fields (floats, `Hue<T>`, `PhantomData`) but derive only
`Clone`, not `Copy`. In contrast, `Color`, `XYZ`, `RGB`, and `Hue` do derive `Copy`.

**Recommendation:** Add `Copy` to derive lists for all 11 types.

---

### 2.2 [CONSISTENCY] XYZ white point constants don't match D65 definition

**Files:** `crates/auto-palette/src/color/xyz.rs:88,130` vs `white_point.rs:98-100`

`XYZ::max_x()` returns `0.950_456` and `XYZ::max_z()` returns `1.088_644`, but the D65 white
point in `white_point.rs` defines `X=0.950_47` and `Z=1.088_83`. These mismatched constants
can cause valid XYZ colors near the white point to be clamped differently.

**Recommendation:** Unify the constants, using values from `white_point.rs` consistently.

---

### 2.3 [DUPLICATION] `lab_to_xyz` duplicated in `gamut.rs` and `xyz.rs`

**Files:** `crates/auto-palette/src/color/gamut.rs:190-218` and `xyz.rs:289-319`

Nearly identical implementations with subtle differences (return type: array vs tuple, no
clamping in gamut.rs). Risks diverging over time.

**Recommendation:** Extract a single shared implementation and call it from both locations.

---

### 2.4 [PERFORMANCE] `DiversitySampling::sample` has O(k × n log n) cost

**File:** `crates/auto-palette/src/math/sampling/diversity.rs:138`

Every iteration of the sampling loop clones the `similarities` vector, creates a new
`RankedScores` (sorting O(n log n)), and allocates a fresh rank vector. For `k` samples from
`n` points, the total cost is O(k × n log n).

**Recommendation:** Consider incremental rank updates or a different sampling strategy that
avoids re-sorting on every iteration.

---

### 2.5 [PERFORMANCE] `pixels_with_filter` performs LAB conversion even for filtered-out pixels

**File:** `crates/auto-palette/src/image/data.rs:172-189`

Every pixel undergoes expensive RGB→XYZ→LAB conversion (involving `pow`, `cbrt`) before the
filter result is returned. The filter check could be performed on the raw RGBA data first,
skipping the expensive conversion for filtered-out pixels.

**Recommendation:** Restructure to check the filter on RGBA first, only converting to LAB for
pixels that pass the filter:

```rust
// Pseudocode
if !filter.test(&rgba) {
    return (dummy_pixel, false); // or skip entirely
}
let pixel = convert_to_lab(rgba);
return (pixel, true);
```

---

### 2.6 [CORRECTNESS] `normalize()` division by zero in release builds

**File:** `crates/auto-palette/src/math/number.rs:251-258`

```rust
pub fn normalize<T>(value: T, min: T, max: T) -> T {
    debug_assert!(min < max, "min must be less than max");
    let normalized = (value - min) / (max - min);
    normalized.max(T::zero()).min(T::one())
}
```

The `debug_assert!` only fires in debug builds. In release, `min == max` produces NaN, which
is then silently clamped to `0.0`.

**Recommendation:** Use a runtime check or return `T::zero()` when `min == max`.

---

### 2.7 [CORRECTNESS] `Eq`/`Ord` contracts violated for `Neighbor` with NaN distances

**File:** `crates/auto-palette/src/math/neighbors/neighbor.rs:67,82-86`

`Eq` is implemented for `Neighbor<T>` where `T: FloatNumber`, but NaN violates reflexivity.
The `Ord` implementation uses `unwrap_or(Ordering::Equal)` for NaN, but the KD-tree uses
`unwrap_or(Ordering::Less)` — inconsistent NaN handling across the codebase.

**Recommendation:** Either document that NaN distances are invalid (add `debug_assert!` in
release-visible positions) or use a consistent NaN-handling strategy.

---

### 2.8 [CORRECTNESS] DBSCAN `segment_capacity` can be zero

**File:** `crates/auto-palette/src/image/segmentation/dbscan/algorithm.rs:153`

```rust
let segment_capacity = (width * height) / self.segments;
```

If `self.segments > width * height`, this yields 0, and the break condition
`segment.len() >= segment_capacity` (line 206) fires immediately, producing degenerate segments.

**Recommendation:** Add `segment_capacity = segment_capacity.max(1)` or validate at build time
that `segments <= width * height`.

---

### 2.9 [CORRECTNESS] FastDBSCAN accepts `probability = 0.0`

**File:** `crates/auto-palette/src/image/segmentation/fastdbscan/algorithm.rs:305`

The validation `!(T::zero()..=T::one()).contains(&self.probability)` accepts `0.0`. This causes
`1.0 / 0.0 = Inf`, then `Inf as usize = usize::MAX`, then `step_by(usize::MAX)` selects only
1 pixel. The error message says "range (0, 1]" but the code uses `[0, 1]`.

**Recommendation:** Change to `!(T::zero()..T::one()).contains(&self.probability)` to exclude
0.0 at the lower bound, or use `self.probability <= T::zero()` explicitly.

---

### 2.10 [CORRECTNESS] SNIC `Element::cmp` violates total ordering contract

**File:** `crates/auto-palette/src/image/segmentation/snic/algorithm.rs:321-330`

```rust
fn cmp(&self, other: &Self) -> Ordering {
    self.distance.partial_cmp(&other.distance).unwrap_or(Ordering::Less)
}
```

`NaN.cmp(3.0)` returns `Less` AND `3.0.cmp(NaN)` returns `Less` — both are "less than" each
other, violating total ordering. In a `BinaryHeap`, this can break heap invariants.

**Recommendation:** Handle NaN consistently, e.g., NaN is always `Greater` (sorted to end).

---

### 2.11 [CLI] Inconsistent error handling: three different strategies in one function

**File:** `crates/auto-palette-cli/src/main.rs`

`main()` uses `anyhow::Result` with `?` (lines 28, 35, 87-92), `process::exit(1)` (lines 57-58,
67-68), and `.unwrap()` (line 97). This inconsistency means failures are handled differently
depending on the code path.

**Recommendation:** Unify all error handling to use `?` with `anyhow::Context`.

---

### 2.12 [CLI] Double `instant.elapsed()` causing timing inaccuracy

**File:** `crates/auto-palette-cli/src/main.rs:99-104`

```rust
instant.elapsed().as_secs(),
instant.elapsed().subsec_millis()
```

Two separate calls can cross a second boundary, printing e.g., `1.000` when elapsed is `1.999`.

**Recommendation:** Capture once: `let elapsed = instant.elapsed();`

---

### 2.13 [CLI] Resizing can produce zero-dimension images

**File:** `crates/auto-palette-cli/src/main.rs:39-54`

For a very non-square image (e.g., 1×100000), the scale factor can bring one dimension below
1.0, and `as u32` truncates to 0. Also, images smaller than 360×360 are upscaled unnecessarily.

**Recommendation:** Add `.max(1)` to both dimensions after scaling, and skip resizing when
`scale >= 1.0`.

---

### 2.14 [CONSISTENCY] `Theme` implements `FromStr` but not `Display`

**File:** `crates/auto-palette/src/theme.rs`

`Algorithm` correctly implements both `FromStr` and `Display`, but `Theme` only implements
`FromStr`. This breaks the Rust convention that these traits should be round-trippable.

**Recommendation:** Add `Display` implementation for `Theme`.

---

### 2.15 [API] No `From<ImageError> for Error` conversion

**Files:** `crates/auto-palette/src/error.rs`, `crates/auto-palette/src/image/error.rs`

The two error hierarchies have no conversion path. Users cannot use `?` to compose
`ImageData::load()` (→ `ImageError`) with `Palette::extract()` (→ `Error`).

**Recommendation:** Add a `From<ImageError>` implementation for `Error`, or unify the error types.

---

### 2.16 [API] `ImageData::new` — overflow in length calculation

**File:** `crates/auto-palette/src/image/data.rs:61`

```rust
let expected_length = (width * height) as usize * RGBA_CHANNELS;
```

`width * height` can overflow `u32`. Should use `(width as usize) * (height as usize)`.

**Recommendation:** Perform multiplication in `usize` space or use `checked_mul`.

---

### 2.17 [PERFORMANCE] `expand_cluster` can add duplicate neighbors to the queue

**File:** `crates/auto-palette/src/math/clustering/dbscan.rs:99-119`

When expanding a cluster, secondary neighbors are added to the queue even if already present.
For dense datasets, this causes significant memory bloat.

**Recommendation:** Track visited points in a `HashSet` to avoid duplicate queue entries.

---

### 2.18 [WASM] Unnecessary serialization round-trip for swatch positions

**File:** `crates/auto-palette-wasm/src/palette.rs:73-78`

`swatch.position()` serializes `JsPosition` to `JsValue`, then `JsPalette::new()` immediately
deserializes it back to `JsPosition`. This is a pointless round-trip through JS serialization.

**Recommendation:** Access the position field directly via a `pub(crate)` accessor.

---

### 2.19 [WASM] Unchecked `JsValue`-to-`JsString` cast

**File:** `crates/auto-palette-wasm/src/types.rs:36-37`

```rust
let value: JsValue = name.into();
let string: JsString = value.into();
```

The `.into()` is an unchecked cast. Non-string values passed from JS produce undefined behavior.

**Recommendation:** Use `value.dyn_into::<JsString>()` for a safe checked cast.

---

### 2.20 [CLI] `env::set_var` is unsafe since Rust 1.83

**File:** `crates/auto-palette-cli/src/env.rs:31-46`

With MSRV 1.86.0, `std::env::set_var` requires `unsafe` blocks. Tests using this with
`cargo nextest` (parallel execution) create a potential data race.

**Recommendation:** Use `unsafe` blocks with appropriate safety comments, or restructure tests
to avoid mutating environment variables (e.g., use a test helper that spawns a child process).

---

## 3. Low Severity

### 3.1 [DOC] `max_x()` documented as "minimum value" in XYZ

**File:** `crates/auto-palette/src/color/xyz.rs:82-85`

The doc comment for `max_x()` says "Returns the minimum value of the X component."

**Recommendation:** Fix doc comment to say "maximum."

---

### 3.2 [DOC] `Color` doc example missing `.abs()` in assertions

**File:** `crates/auto-palette/src/color/mod.rs:63-64`

```rust
/// assert!(color.lightness() - 52.917 < 1e-3);
```

Should use `(color.lightness() - 52.917).abs() < 1e-3` to properly test both directions.

---

### 3.3 [DOC] `find_swatches` doc says "based on the theme" but takes no theme parameter

**File:** `crates/auto-palette/src/palette.rs:99`

Copy-paste error from `find_swatches_with_theme`.

**Recommendation:** Update to describe population-based scoring.

---

### 3.4 [DOC] `FloatNumber` doc says "undefined behavior" for `as` casts

**File:** `crates/auto-palette/src/math/number.rs:19`

Since Rust 1.45, `as` casts from float to integer are saturating, not UB. The doc should say
"saturating."

---

### 3.5 [DOC] `SnicError` doc references nonexistent type parameter `T`

**File:** `crates/auto-palette/src/image/segmentation/snic/error.rs:7-8`

Copy-paste error from other error types.

---

### 3.6 [DOC] DBSCAN variable naming: `xy` should be `ny`

**File:** `crates/auto-palette/src/image/segmentation/dbscan/algorithm.rs:164`

```rust
let (nx, xy) = Self::index_to_coords(neighbor_index, width);
```

The second variable represents the neighbor's y-coordinate but is named `xy`.

---

### 3.7 [STYLE] `#[must_use]` missing on `Oklab::new`

**File:** `crates/auto-palette/src/color/oklab.rs:57`

Every other color struct's `new` method has `#[must_use]`, but `Oklab::new` does not.

---

### 3.8 [STYLE] Floating-point equality in HSV but `is_zero()` in HSL

**File:** `crates/auto-palette/src/color/hsv.rs:92`

HSV uses `delta == T::zero()` while HSL uses `delta.is_zero()`. Inconsistent style.

---

### 3.9 [STYLE] `Color::from_str` silently ignores alpha in 4/8-digit hex

**File:** `crates/auto-palette/src/color/mod.rs:404-452`

Alpha is parsed and validated but silently discarded. No documentation about this behavior.

---

### 3.10 [STYLE] Inconsistent `Display` for `Lab` — not parameterized by white point

**File:** `crates/auto-palette/src/color/lab.rs:232-239`

`Display` is implemented for `Lab<T>` (D65 only), but `LCHab<T, W>`, `LCHuv<T, W>`, `Luv<T, W>`
implement `Display` for all white points.

---

### 3.11 [STYLE] Inconsistent Builder pattern: FastDBSCAN uses `build(&self)`, all others use `build(self)`

**Files:** FastDBSCAN builder `build(&self)` vs all others `build(self)`

---

### 3.12 [STYLE] `fold` instead of `unzip` in algorithm.rs

**File:** `crates/auto-palette/src/algorithm.rs:233-243`

The manual `fold` for collecting pixels and mask could be replaced with `.unzip()`.

---

### 3.13 [DEAD CODE] `FarthestSampling`, `LinearSearch`, multiple builder methods

**Files:**
- `crates/auto-palette/src/math/sampling/farthest.rs` (entirely `#[allow(dead_code)]`)
- `crates/auto-palette/src/math/neighbors/linear.rs:39` (`#[allow(dead_code)]`)
- Various `generator()` builder methods in slic, snic, kmeans

**Recommendation:** Move to `#[cfg(test)]` or remove if not intended for future use.

---

### 3.14 [DEAD CODE] `D50` white point, `Gamut::AdobeRgb`, `Gamut::DisplayP3`

**Files:** `white_point.rs:53-86`, `gamut.rs:12,19`

Defined but unused in production code.

---

### 3.15 [DEAD CODE] `RankedScores::scores` field stored but never read

**File:** `crates/auto-palette/src/math/sampling/diversity.rs:171`

The `scores` vector is stored but only `ranks` is ever accessed (except via `len()`).

---

### 3.16 [WASM] Redundant `'static` lifetime on `const` string slices

**Files:** `color.rs:9`, `palette.rs:13`, `position.rs:5`, `swatch.rs:8`, `types.rs:9`

```rust
const TYPE_DEFINITION: &'static str = ...;
```

`'static` is implied for `const` items.

---

### 3.17 [WASM] Repetitive error-mapping boilerplate in color conversions

**File:** `crates/auto-palette-wasm/src/color.rs:349-462`

14 color conversion methods follow the identical pattern. Could be extracted into a helper.

---

### 3.18 [WASM] No `toString()` or `fromRGB()` on Color class

**File:** `crates/auto-palette-wasm/src/color.rs`

JS consumers cannot use template literals with Color, and must pack RGB into an integer manually.

---

### 3.19 [WASM] No iterable protocol or index access on Palette

**File:** `crates/auto-palette-wasm/src/palette.rs`

JS consumers cannot use `for...of`, spread syntax, or `Array.from()` on Palette objects. No
`get(index)` method exists for raw swatch access.

---

### 3.20 [NUMERICAL] Incremental mean drift in `SegmentMetadata::insert`

**File:** `crates/auto-palette/src/image/segmentation/segment.rs:109-112`

Running mean formula `new_mean = (old_mean × (n-1) + new_value) / n` accumulates floating-point
error over many insertions. Welford's algorithm would be more stable.

---

### 3.21 [TEST] CLI: Significant test coverage gaps

**File:** `crates/auto-palette-cli/`

Missing coverage for:
- All output printers (`JsonPrinter`, `TextPrinter`, `TablePrinter`)
- `ColorSpace::fmt` (12 match arms)
- `--output-format json` and `--output-format text` integration tests
- Empty swatch list behavior
- `--count` with non-numeric or negative values

---

### 3.22 [TEST] Algorithm `Display` tests missing SLIC and SNIC

**File:** `crates/auto-palette/src/algorithm.rs:287-297`

`test_fmt` only covers `KMeans`, `DBSCAN`, and `DBSCANpp`.

---

### 3.23 [STYLE] FastDBSCAN `expand_segment` redundant condition

**File:** `crates/auto-palette/src/image/segmentation/fastdbscan/algorithm.rs:134-136`

```rust
if labels[neighbor_index] != Self::LABEL_UNLABELED
    || labels[neighbor_index] == Self::LABEL_IGNORED
```

The second condition is logically redundant (subsumed by the first).

---

## 4. Summary Statistics

| Severity | Count | Categories |
|----------|-------|------------|
| Critical / High | 10 | 4 bugs, 2 CLI bugs, 2 validation gaps, 1 API leak, 1 WASM issue |
| Medium | 20 | 5 correctness, 4 performance, 4 consistency, 3 API design, 2 CLI, 2 WASM |
| Low | 23 | 7 documentation, 5 dead code, 4 style, 3 WASM, 2 tests, 2 numerical |

### Top Recommendations by Impact

1. **Fix `clamp_to_u8`** — Real bug that can produce incorrect RGB values from out-of-gamut colors
2. **Implement SLIC compactness** — Core algorithm parameter has zero effect on output
3. **Fix Oklch bounds** — Semantically incorrect clamping ranges
4. **Unify CLI error handling** — Silent exits provide no user feedback
5. **Add mask length validation** — Missing validation causes panics instead of errors
6. **Add `Copy` to color types** — Low-effort improvement with broad ergonomic benefits
7. **Optimize `pixels_with_filter`** — Skip expensive LAB conversion for filtered-out pixels
8. **Fix timing output** — Use `eprintln!` and capture elapsed once
9. **Unify XYZ/D65 constants** — Mismatched constants in two files
10. **Add `Display` for `Theme`** — Missing trait impl breaks Rust conventions

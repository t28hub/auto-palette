//! build.rs - Build script for generating lookup tables of maximum chroma values.
//!
//! This script computes binary lookup tables for maximum chroma values across hues
//! for supported color gamuts (sRGB, Display P3, Rec. 2020). The generated files are
//! output to the directory specified by the OUT_DIR environment variable.
//!
//! Each lookup table file includes a header with metadata (version, gamut type, body size,
//! and checksum hash) to ensure data integrity.
use std::{env, fs, path::PathBuf};

use bytemuck::{Pod, Zeroable};

/// Entry point for the build script.
/// This function generates lookup tables for maximum chroma values across hues
/// for each supported color gamut (sRGB, Display P3, Rec. 2020). The generated
/// binary files are written to the directory specified by the OUT_DIR environment variable.
///
/// This documentation aims to provide sufficient context for open source contributors.
fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR environment variable not set"));

    for gamut in &[Gamut::SRGB, Gamut::DISPLAY_P3, Gamut::REC_2020] {
        let chroma_values = compute_max_chroma_values(*gamut);
        let lookup_table = ChromaLookupTable::from_slice(*gamut, &chroma_values);
        let bytes = lookup_table.as_bytes();

        let bin_path = out_dir.join(format!("{}_table.bin", gamut.name));
        fs::write(&bin_path, bytes).unwrap_or_else(|err| {
            panic!("Failed to write to file {}: {}", bin_path.display(), err);
        });

        println!(
            "Generated file for {} gamut at {} ({} KB)",
            gamut.name,
            bin_path.display(),
            bytes.len() / 1024
        );
    }
}

/// The minimum lightness value for the gamut calculations.
const MIN_LIGHTNESS: usize = 0;

/// The maximum lightness value for the gamut calculations.
const MAX_LIGHTNESS: usize = 100;

/// The minimum chroma value for the gamut calculations.
const MIN_CHROMA: f32 = 0.0;

/// The maximum chroma value for the gamut calculations.
const MAX_CHROMA: f32 = 180.0;

/// The maximum number of hue degrees (0 to 359).
const MAX_HUE_DEGREES: usize = 360;

/// Computes the maximum chroma values for each hue angle within the specified color gamut.
///
/// # Arguments
/// * `gamut` - The target color gamut (sRGB, Display P3, Rec. 2020, etc.)
///
/// # Returns
/// A vector containing maximum chroma values for each hue angle (0-359 degrees)
#[must_use]
fn compute_max_chroma_values(gamut: Gamut) -> Vec<f32> {
    let mut values = vec![0.0; MAX_HUE_DEGREES];
    for (index, value) in values.iter_mut().enumerate() {
        let hue_degrees = index as f32;
        let mut max_chroma = 0.0;
        for lightness in MIN_LIGHTNESS..=MAX_LIGHTNESS {
            let chroma = max_chroma_at_lightness(&gamut, hue_degrees, lightness as f32);
            if chroma > max_chroma {
                max_chroma = chroma;
            }
        }
        *value = max_chroma;
    }
    values
}

/// Computes the maximum chroma for a given hue and lightness within the specified color gamut.
///
/// # Arguments
/// * `gamut` - The target color gamut (sRGB, Display P3, Rec. 2020, etc.)
/// * `hue_degrees` - The hue angle in degrees (0-359).
///
/// # Returns
/// The maximum chroma value for the specified hue and lightness within the gamut.
#[inline]
#[must_use]
fn max_chroma_at_lightness(gamut: &Gamut, hue_degrees: f32, lightness: f32) -> f32 {
    let hue_radians = hue_degrees.to_radians();
    let mut min_chroma = MIN_CHROMA;
    let mut max_chroma = MAX_CHROMA;
    while max_chroma - min_chroma > 1e-3 {
        let mid_chroma = (min_chroma + max_chroma) * 0.5;
        if gamut.in_gamut([lightness, mid_chroma, hue_radians]) {
            min_chroma = mid_chroma;
        } else {
            max_chroma = mid_chroma;
        }
    }
    min_chroma
}

/// A lookup table that stores the maximum chroma values per hue.
///
/// This structure consists of two parts:
/// - A header that holds metadata about the table file (version, gamut type, body size, data hash, etc.).
/// - A fixed-size body array containing precomputed maximum chroma values for each hue at a specific lightness level.
///
/// It is designed for use in applications involving color gamut calculations and color space conversions.
#[derive(Debug, Clone, Copy, PartialEq, Zeroable)]
#[repr(C)]
struct ChromaLookupTable {
    /// The header contains metadata about the table file such as version, gamut type, body size, and data hash.
    header: Header,

    /// The body contains the max chroma values for each hue at a specific lightness.
    /// The length of the body array must be fixed (e.g., 128, 256, 512, 1024, etc.)
    /// due to the `bytemuck::Pod` trait requirement for fixed-size arrays to ensure a consistent memory layout.
    body: [f32; 512],
}

unsafe impl Pod for ChromaLookupTable {}

impl ChromaLookupTable {
    /// Creates a new `ChromaLookupTable` using a specified gamut and a slice of maximum chroma values.
    ///
    /// # Arguments
    /// * `gamut` - The color gamut for which the lookup table is built.
    /// * `data` - A slice of f32 values representing the maximum chroma for each hue.
    ///
    /// # Returns
    /// A new `ChromaLookupTable` instance containing the header information and precomputed chroma values.
    #[must_use]
    fn from_slice(gamut: Gamut, data: &[f32]) -> Self {
        let mut body = [0.0; 512];
        body[..data.len()].copy_from_slice(data);

        let bytes: &[u8] = bytemuck::cast_slice(&body);
        let header = Header::new(gamut.kind, bytes);

        Self { header, body }
    }

    /// Returns the byte slice representation of the `ChromaLookupTable`.
    ///
    /// # Returns
    /// A byte slice of the table, suitable for file output or binary manipulation.
    #[must_use]
    fn as_bytes(&self) -> &[u8] {
        bytemuck::bytes_of(self)
    }
}

/// Metadata header for the chroma lookup table file.
#[derive(Debug, Clone, Copy, PartialEq, Zeroable)]
#[repr(C)]
struct Header {
    /// The file format version.
    version: u8,

    /// The identifier for the gamut type.
    kind: u8,

    /// The size of the table body in bytes.
    size: u32,

    /// The checksum hash of the table body for data integrity verification.
    checksum: u32,

    /// The reserved field for future use.
    reserved: [u8; 8],
}

unsafe impl Pod for Header {}

impl Header {
    /// The current file format version.
    const VERSION: u8 = 1;

    /// Creates a new `Header` instance with the specified gamut kind and body bytes.
    ///
    /// # Arguments
    /// * `kind` - The color gamut kind (sRGB, Display P3, Rec. 2020).
    /// * `bytes` - A byte slice representing the body of the table.
    ///
    /// # Returns
    /// A new `Header` instance containing the version, kind, size, hash, and reserved fields.
    #[must_use]
    fn new(kind: GamutKind, bytes: &[u8]) -> Self {
        Self {
            version: Self::VERSION,
            kind: kind as u8,
            size: bytes.len() as u32,
            checksum: crc32fast::hash(bytes),
            reserved: [0; 8],
        }
    }
}

/// Supported color gamuts with their transformation matrices.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum GamutKind {
    /// The sRGB color space.
    /// This is the most common color space used in web and digital media.
    /// [sRGB - Wikipedia](https://en.wikipedia.org/wiki/SRGB)
    Srgb = 0,

    /// The Display P3 color space.
    /// This color space is wider than sRGB and is used in modern displays, especially in Apple devices.
    /// [DCI-P3 - Wikipedia](https://en.wikipedia.org/wiki/DCI-P3)
    DisplayP3 = 1,

    /// The Rec. 2020 color space.
    /// This color space is wider than both sRGB and Display P3 and is used in high-definition video and cinema.
    /// [Rec. 2020 - Wikipedia](https://en.wikipedia.org/wiki/Rec._2020)
    Rec2020 = 2,
}

/// Color gamut definitions for color space conversions.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Gamut {
    /// The identifier for the gamut type.
    kind: GamutKind,

    /// The name of the color gamut.
    name: &'static str,

    /// The transformation matrix used for converting from XYZ to RGB.
    matrix: [[f32; 3]; 3],
}

impl Gamut {
    /// The D65 white point used for color space conversions.
    /// [D65 values - Wikipedia](https://en.wikipedia.org/wiki/Standard_illuminant#D65_values)
    const D65: [f32; 3] = [0.95047, 1.0, 1.08883];

    /// The sRGB color space gamut.
    const SRGB: Self = Gamut {
        kind: GamutKind::Srgb,
        name: "srgb",
        matrix: [
            [3.240_454_2, -1.537_138_5, -0.498_531_4],
            [-0.969_266, 1.876_010_8, 0.041_556_0],
            [0.055_643_4, -0.204_025_9, 1.057_225_2],
        ],
    };

    /// The Display P3 color space gamut.
    const DISPLAY_P3: Self = Gamut {
        kind: GamutKind::DisplayP3,
        name: "displayp3",
        matrix: [
            [2.493_396_3, -0.931_345_9, -0.402_694_5],
            [-0.829_486_8, 1.762_659_7, 0.023_624_6],
            [0.035_850_7, -0.076_182_7, 0.957_014],
        ],
    };

    /// The Rec. 2020 color space gamut.
    const REC_2020: Self = Gamut {
        kind: GamutKind::Rec2020,
        name: "rec2020",
        matrix: [
            [1.716_663_4, -0.355_673_3, -0.253_368_09],
            [-0.666_673_84, 1.616_455_7, 0.015_768_3],
            [0.017_642_48, -0.042_776_98, 0.942_243_3],
        ],
    };

    /// Checks whether a given Lch color is within the gamut.
    ///
    /// # Arguments
    /// * `[l, c, h]` - The Lch color components.
    ///
    /// # Returns
    /// `true` if the color is within the gamut, `false` otherwise.
    ///
    /// # Notes
    /// The `h` component is expected to be in radians.
    #[inline(always)]
    #[must_use]
    fn in_gamut(&self, [l, c, h]: [f32; 3]) -> bool {
        let lab = [l, c * h.cos(), c * h.sin()];
        let xyz = self.lab_to_xyz(lab);
        let rgb = self.xyz_to_rgb(xyz);
        rgb.iter().all(|&c| (0.0..=1.0).contains(&c))
    }

    /// Converts a Lab color to XYZ using the gamut's transformation matrix.
    ///
    /// # Arguments
    /// * `[l, a, b]` - The Lab color components.
    ///
    /// # Returns
    /// The XYZ color components `[x, y, z]` as an array of three f32 values.
    #[inline(always)]
    #[must_use]
    fn lab_to_xyz(&self, [l, a, b]: [f32; 3]) -> [f32; 3] {
        const EPSILON: f32 = 6.0 / 29.0;
        const KAPPA: f32 = 108.0 / 841.0;
        const DELTA: f32 = 4.0 / 29.0;

        let fy = (l + 16.0) / 116.0;
        let fx = fy + a / 500.0;
        let fz = fy - b / 200.0;

        let f_inv = |v: f32| {
            if v > EPSILON {
                v.powi(3)
            } else {
                KAPPA * (v - DELTA)
            }
        };

        [
            f_inv(fx) * Self::D65[0],
            f_inv(fy) * Self::D65[1],
            f_inv(fz) * Self::D65[2],
        ]
    }

    /// Converts an XYZ color to RGB using the gamut's transformation matrix.
    ///
    /// # Arguments
    /// * `[x, y, z]` - The XYZ color components.
    ///
    /// # Returns
    /// The RGB color components `[r, g, b]` as an array of three f32 values.
    #[inline(always)]
    #[must_use]
    fn xyz_to_rgb(&self, [x, y, z]: [f32; 3]) -> [f32; 3] {
        [
            self.matrix[0][0] * x + self.matrix[0][1] * y + self.matrix[0][2] * z,
            self.matrix[1][0] * x + self.matrix[1][1] * y + self.matrix[1][2] * z,
            self.matrix[2][0] * x + self.matrix[2][1] * y + self.matrix[2][2] * z,
        ]
    }
}

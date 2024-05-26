use std::path::PathBuf;

use auto_palette::{color::Color, Algorithm, FloatNumber, Theme};
use clap::{crate_authors, crate_description, crate_version, Parser, ValueEnum, ValueHint};

/// The command line options for the `auto-palette` command.
#[derive(Debug, Parser)]
#[command(
    name = "auto-palette",
    bin_name = "auto-palette",
    about = crate_description!(),
    author = crate_authors!(),
    version = crate_version!(),
    arg_required_else_help = true,
)]
pub struct Options {
    #[arg(
        value_name = "IMAGE",
        help = "Path to the image file.",
        long_help = "Path to the image file. Supported formats include PNG, JPEG, GIF, BMP, ICO, and TIFF.",
        required = true,
        value_hint = ValueHint::FilePath,
    )]
    pub path: PathBuf,

    #[arg(
        long,
        short = 'a',
        value_name = "name",
        value_enum,
        help = "Algorithm for extracting the color palette.",
        default_value_t = AlgorithmOption::default(),
        ignore_case = true,
    )]
    pub algorithm: AlgorithmOption,

    #[arg(
        long,
        short = 't',
        value_name = "name",
        value_enum,
        help = "Theme for selecting the swatches from the extracted color palette",
        ignore_case = true
    )]
    pub theme: Option<ThemeOption>,

    #[arg(
        long,
        short = 'n',
        value_name = "number",
        help = "Number of prominent colors to select from the extracted color palette.",
        default_value = "5"
    )]
    pub count: usize,

    #[arg(
        long,
        short = 'c',
        value_name = "name",
        value_enum,
        help = "Output color space for the extracted colors",
        default_value_t = ColorOption::default(),
        ignore_case = true,
    )]
    pub color: ColorOption,

    #[arg(
        long,
        help = "Disable image resizing before extracting the color palette.",
        long_help = "Disable image resizing before extracting the color palette. This potentially improve the accuracy of the results by preserving the original image resolution."
    )]
    pub no_resize: bool,
}

/// The algorithm options for extracting the color palette from the image.
#[derive(Debug, Default, Copy, Clone, ValueEnum)]
pub enum AlgorithmOption {
    #[default]
    #[clap(
        name = "dbscan",
        help = "High accuracy but slower speed. Ideal for precision over performance."
    )]
    Dbscan,
    #[clap(
        name = "dbscan++",
        help = "A balanced algorithm with faster speed and good accuracy. Ideal for a balance between precision and performance."
    )]
    DbscanPlusPlus,
    #[clap(
        name = "kmeans",
        help = "Fast speed but potentially less accurate. Ideal for performance over precision."
    )]
    KMeans,
}

impl From<AlgorithmOption> for Algorithm {
    fn from(option: AlgorithmOption) -> Self {
        match option {
            AlgorithmOption::Dbscan => Algorithm::DBSCAN,
            AlgorithmOption::DbscanPlusPlus => Algorithm::DBSCANpp,
            AlgorithmOption::KMeans => Algorithm::KMeans,
        }
    }
}

/// The theme options for selecting the swatches from the extracted color palette.
#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum ThemeOption {
    #[clap(
        name = "basic",
        help = "A general-purpose theme suitable for a wide range of images."
    )]
    Basic,
    #[clap(name = "vivid", help = "Prioritize saturated colors.")]
    Vivid,
    #[clap(name = "muted", help = "Prioritize desaturated colors.")]
    Muted,
    #[clap(name = "light", help = "Prioritize light colors.")]
    Light,
    #[clap(name = "dark", help = "Prioritize dark colors.")]
    Dark,
}

impl From<ThemeOption> for Theme {
    fn from(option: ThemeOption) -> Self {
        match option {
            ThemeOption::Basic => Theme::Basic,
            ThemeOption::Vivid => Theme::Vivid,
            ThemeOption::Muted => Theme::Muted,
            ThemeOption::Light => Theme::Light,
            ThemeOption::Dark => Theme::Dark,
        }
    }
}

/// The color space options for the extracted colors.
#[derive(Debug, Default, Copy, Clone, ValueEnum)]
pub enum ColorOption {
    #[default]
    #[clap(name = "hex", help = "Hexadecimal color representation")]
    Hex,
    #[clap(name = "rgb", help = "RGB color space")]
    Rgb,
    #[clap(name = "hsl", help = "HSL color space")]
    Hsl,
    #[clap(name = "hsv", help = "HSV color space")]
    Hsv,
    #[clap(name = "lab", help = "CIE L*a*b* color space")]
    Lab,
    #[clap(name = "luv", help = "CIE L*u*v* color space")]
    Luv,
    #[clap(name = "lchab", help = "LCH(ab) color space")]
    LCHab,
    #[clap(name = "lchuv", help = "LCH(uv) color space")]
    LCHuv,
    #[clap(name = "oklab", help = "Oklab color space")]
    Oklab,
    #[clap(name = "oklch", help = "Oklch color space")]
    Oklch,
    #[clap(name = "xyz", help = "CIE XYZ color space")]
    Xyz,
}

impl ColorOption {
    /// Returns the string representation of the color space for the given color.
    ///
    /// # Arguments
    /// * `color` - The color to convert.
    ///
    /// # Returns
    /// The string representation of the color space.
    #[must_use]
    pub fn as_string<T>(&self, color: &Color<T>) -> String
    where
        T: FloatNumber,
    {
        match self {
            Self::Hex => color.to_hex_string(),
            Self::Rgb => color.to_rgb().to_string(),
            Self::Hsl => color.to_hsl().to_string(),
            Self::Hsv => color.to_hsv().to_string(),
            Self::Lab => color.to_lab().to_string(),
            Self::Luv => color.to_luv().to_string(),
            Self::LCHab => color.to_lchab().to_string(),
            Self::LCHuv => color.to_lchuv().to_string(),
            Self::Oklab => color.to_oklab().to_string(),
            Self::Oklch => color.to_oklch().to_string(),
            Self::Xyz => color.to_xyz().to_string(),
        }
    }
}

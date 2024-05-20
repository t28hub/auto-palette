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
        long_help = "Path to the image file. Supported formats: PNG, JPEG, GIF, BMP, ICO, and TIFF.",
        required = true,
        value_hint = ValueHint::FilePath,
    )]
    pub path: PathBuf,

    #[arg(
        long,
        short = 'a',
        value_name = "name",
        value_enum,
        help = "Algorithm for extracting the palette.",
        default_value_t = AlgorithmArg::default(),
        ignore_case = true,
    )]
    pub algorithm: AlgorithmArg,

    #[arg(
        long,
        short = 't',
        value_name = "name",
        value_enum,
        help = "Theme for selecting the swatches.",
        ignore_case = true
    )]
    pub theme: Option<ThemeArg>,

    #[arg(
        long,
        short = 'n',
        value_name = "number",
        help = "Number of swatches to extract.",
        default_value = "5"
    )]
    pub count: usize,

    #[arg(
        long,
        short = 'c',
        value_name = "name",
        value_enum,
        help = "Output color space for the swatches.",
        default_value_t = ColorArg::default(),
        ignore_case = true,
    )]
    pub color: ColorArg,
}

#[derive(Debug, Default, Copy, Clone, ValueEnum)]
pub enum AlgorithmArg {
    #[default]
    #[clap(name = "dbscan")]
    Dbscan,
    #[clap(name = "dbscan++")]
    DbscanPlusPlus,
    #[clap(name = "kmeans")]
    KMeans,
}

impl From<AlgorithmArg> for Algorithm {
    fn from(arg: AlgorithmArg) -> Self {
        match arg {
            AlgorithmArg::Dbscan => Algorithm::DBSCAN,
            AlgorithmArg::DbscanPlusPlus => Algorithm::DBSCANpp,
            AlgorithmArg::KMeans => Algorithm::KMeans,
        }
    }
}

/// The theme options supported by the `auto-palette` command.
#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum ThemeArg {
    Basic,
    Vivid,
    Muted,
    Light,
    Dark,
}

impl From<ThemeArg> for Theme {
    fn from(arg: ThemeArg) -> Self {
        match arg {
            ThemeArg::Basic => Theme::Basic,
            ThemeArg::Vivid => Theme::Vivid,
            ThemeArg::Muted => Theme::Muted,
            ThemeArg::Light => Theme::Light,
            ThemeArg::Dark => Theme::Dark,
        }
    }
}

/// The color options supported by the `auto-palette` command.
#[derive(Debug, Default, Copy, Clone, ValueEnum)]
pub enum ColorArg {
    #[default]
    Hex,
    Rgb,
    Hsl,
    Hsv,
    Lab,
    Luv,
    #[clap(name = "lchab")]
    LCHab,
    #[clap(name = "lchuv")]
    LCHuv,
    Oklab,
    Oklch,
    Xyz,
}

impl ColorArg {
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

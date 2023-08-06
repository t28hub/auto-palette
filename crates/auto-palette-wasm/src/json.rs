use serde::{Deserialize, Serialize};

/// Struct representing a 2-dimensional position.
#[derive(Serialize, Deserialize)]
pub struct PositionJson {
    pub x: u32,
    pub y: u32,
}

/// Struct representing a color in RGB format.
#[derive(Serialize, Deserialize)]
pub struct RGBJson {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// Struct representing a color in CIE L*a*b* format.
#[derive(Serialize, Deserialize)]
pub struct LabJson {
    pub l: f64,
    pub a: f64,
    pub b: f64,
}

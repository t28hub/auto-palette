mod json;
mod printer;
mod table;
mod text;

use auto_palette::{FloatNumber, Swatch};
pub use json::JsonPrinter;
pub use printer::Printer;
pub use table::TablePrinter;
pub use text::TextPrinter;

use crate::args::ColorSpace;

/// Measures the maximum display widths of the color, position, and population
/// columns for the given swatches.
///
/// # Arguments
/// * `swatches` - The swatches to measure.
/// * `color_space` - The color space used to format each color.
///
/// # Returns
/// The maximum widths of the color, position, and population columns.
#[must_use]
fn measure_swatch_widths<T>(swatches: &[Swatch<T>], color_space: ColorSpace) -> [usize; 3]
where
    T: FloatNumber,
{
    swatches.iter().fold([0, 0, 0], |acc, swatch| {
        let color_width = color_space.fmt(swatch.color()).len();

        let (x, y) = swatch.position();
        let position_width = format!("({x}, {y})").len();

        let population_width = swatch.population().to_string().len();
        [
            acc[0].max(color_width),
            acc[1].max(position_width),
            acc[2].max(population_width),
        ]
    })
}

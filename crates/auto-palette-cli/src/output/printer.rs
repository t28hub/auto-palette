use std::io::{Error, Write};

use auto_palette::Swatch;

/// The trait for printing the swatches.
pub trait Printer {
    /// Prints the swatches with given output writer.
    ///
    /// # Type Parameters
    /// * `W` - The type of the output writer.
    ///
    /// # Arguments
    /// * `swatches` - The swatches to print.
    /// * `output` - The output writer.
    ///
    /// # Returns
    /// The result of the operation.
    fn print<W>(&self, swatches: &[Swatch], output: &mut W) -> Result<(), Error>
    where
        W: Write;
}

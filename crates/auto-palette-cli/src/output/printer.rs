use std::io::{Error, Write};

use auto_palette::{FloatNumber, Swatch};

/// The trait for printing the swatches.
pub trait Printer {
    /// Prints the swatches with given output writer.
    ///
    /// # Type Parameters
    /// * `T` - The type of the float number.
    /// * `W` - The type of the output writer.
    ///
    /// # Arguments
    /// * `swatches` - The swatches to print.
    /// * `output` - The output writer.
    ///
    /// # Returns
    /// The result of the operation.
    fn print<T, W>(&self, swatches: &[Swatch<T>], output: &mut W) -> Result<(), Error>
    where
        T: FloatNumber,
        W: Write;
}

use std::io::{BufWriter, Error, Write};

use auto_palette::{FloatNumber, Swatch};

use crate::{context::Context, output::Printer};

const HEADINGS: [&str; 4] = ["#", "Color", "Position", "Population"];

/// The table printer for printing the swatches.
///
/// This printer prints the swatches in the table format.
#[derive(Debug)]
pub struct TablePrinter<'a> {
    context: &'a Context,
}

impl<'a> TablePrinter<'a> {
    /// Creates a new `TablePrinter` instance.
    ///
    /// # Arguments
    /// * `context` - The context of the application.
    ///
    /// # Returns
    /// A new `TablePrinter` instance.
    pub fn new(context: &'a Context) -> Self {
        Self { context }
    }
}

impl Printer for TablePrinter<'_> {
    fn print<T, W>(&self, swatches: &[Swatch<T>], output: &mut W) -> Result<(), Error>
    where
        T: FloatNumber,
        W: Write,
    {
        let mut writer = BufWriter::new(output);

        let color_format = self.context.args().color_space;
        let initial_widths = [
            HEADINGS[0].len(),
            HEADINGS[1].len(),
            HEADINGS[2].len(),
            HEADINGS[3].len(),
        ];
        let widths = swatches
            .iter()
            .enumerate()
            .fold(initial_widths, |acc, (i, swatch)| {
                let number_width = (i + 1).to_string().len();
                let color_width = color_format.fmt(swatch.color()).len();

                let (x, y) = swatch.position();
                let position_width = format!("({}, {})", x, y).len();

                let population_width = swatch.population().to_string().len();
                [
                    acc[0].max(number_width),
                    acc[1].max(color_width),
                    acc[2].max(position_width),
                    acc[3].max(population_width),
                ]
            });

        // Write the header.
        write_horizontal_separator(&mut writer, &widths, 1)?;
        for (i, heading) in HEADINGS.iter().enumerate() {
            write!(writer, "| {:<width$} ", heading, width = widths[i])?;
        }
        writeln!(writer, "|")?;
        write_horizontal_separator(&mut writer, &widths, 1)?;

        // Write the swatches.
        for (i, swatch) in swatches.iter().enumerate() {
            write!(writer, "| {:>width$} ", i + 1, width = widths[0])?;

            let color = color_format.fmt(swatch.color());
            write!(writer, "| {:<width$} ", color, width = widths[1])?;

            let position = format!("{:?}", swatch.position());
            write!(writer, "| {:<width$} ", position, width = widths[2])?;

            let population = swatch.population();
            write!(writer, "| {:>width$} ", population, width = widths[3])?;
            writeln!(writer, "|")?;
        }

        write_horizontal_separator(&mut writer, &widths, 1)?;
        writer.flush()
    }
}

#[inline]
fn write_horizontal_separator<W>(
    writer: &mut W,
    widths: &[usize],
    padding: usize,
) -> Result<(), Error>
where
    W: Write,
{
    for width in widths.iter() {
        write!(writer, "+")?;
        write!(writer, "{:-<width$}", "", width = width + (padding * 2))?;
    }
    writeln!(writer, "+")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_horizontal_separator() {
        // Arrange
        let mut buffer = Vec::new();
        let widths = [2, 4, 8];

        // Act
        write_horizontal_separator(&mut buffer, &widths, 1).unwrap();
        let actual = String::from_utf8(buffer).unwrap();

        // Assert
        assert_eq!(actual, "+----+------+----------+\n");
    }
}

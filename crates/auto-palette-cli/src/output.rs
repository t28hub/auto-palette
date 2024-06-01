use std::io::{BufWriter, Error, Write};

use crate::table::Table;

/// The trait for printing the swatches.
pub trait Printer {
    /// Prints the table.
    ///
    /// # Arguments
    /// * `table` - The table to print.
    /// * `out` - The output writer.
    ///
    /// # Returns
    /// The result of the printing.
    fn print<W>(&self, table: &Table, out: &mut W) -> Result<(), Error>
    where
        W: Write;
}

/// The printer for printing the swatches in the text format.
#[derive(Debug)]
pub struct TextPrinter;

impl Printer for TextPrinter {
    fn print<W>(&self, table: &Table, out: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        let columns = table.columns();
        if columns.is_empty() {
            return Ok(());
        }

        let mut writer = BufWriter::new(out);
        let count = columns[0].rows().len();
        for i in 0..count {
            for column in columns {
                write!(
                    writer,
                    "{:width$} ",
                    column.rows()[i],
                    width = column.width()
                )?;
            }
            writeln!(writer)?;
        }
        writer.flush()
    }
}

/// The printer for printing the swatches in the table format.
#[derive(Debug)]
pub struct TablePrinter;

impl Printer for TablePrinter {
    fn print<W>(&self, table: &Table, out: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        let columns = table.columns();
        if columns.is_empty() {
            return Ok(());
        }

        let mut writer = BufWriter::new(out);
        for column in columns {
            write!(writer, "+")?;
            for _ in 0..column.width() + 2 {
                write!(writer, "-")?;
            }
        }
        writeln!(writer, "+")?;

        for column in columns {
            write!(
                writer,
                "| {:width$} ",
                column.name(),
                width = column.width()
            )?;
        }
        writeln!(writer, "|")?;

        for column in columns {
            write!(writer, "+")?;
            for _ in 0..column.width() + 2 {
                write!(writer, "-")?;
            }
        }
        writeln!(writer, "+")?;

        let count = columns[0].rows().len();
        for i in 0..count {
            for column in columns {
                write!(
                    writer,
                    "| {:width$} ",
                    column.rows()[i],
                    width = column.width()
                )?;
            }
            writeln!(writer, "|")?;
        }

        for column in columns {
            write!(writer, "+")?;
            for _ in 0..column.width() + 2 {
                write!(writer, "-")?;
            }
        }
        writeln!(writer, "+")?;
        writer.flush()
    }
}

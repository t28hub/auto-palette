use std::io::{BufWriter, Error, Write};

use auto_palette::{color::Color, FloatNumber, Swatch};

use crate::{color::ColorMode, context::Context, output::Printer, style::style};

/// The text printer for printing the swatches.
///
/// This printer prints the swatches in the text format.
#[derive(Debug)]
pub struct TextPrinter<'a> {
    context: &'a Context,
}

impl<'a> TextPrinter<'a> {
    /// Creates a new `TextPrinter` instance.
    ///
    /// # Arguments
    /// * `context` - The context of the application.
    ///
    /// # Returns
    /// A new `TextPrinter` instance.
    pub fn new(context: &'a Context) -> Self {
        Self { context }
    }

    #[inline]
    #[must_use]
    fn swatch_to_text<T>(&self, swatch: &Swatch<T>, widths: &[usize]) -> String
    where
        T: FloatNumber,
    {
        let color_mode = self.color_mode(swatch.color());
        let sample_str = if color_mode == ColorMode::NoColor {
            "".to_string()
        } else {
            let styled = style().background(color_mode).apply("  ");
            format!("{} ", styled)
        };

        let color_format = self.context.args().color;
        let color = color_format.fmt(swatch.color());
        let color_str = format!("{:<width$}", color, width = widths[0]);

        let (x, y) = swatch.position();
        let position_str = format!("({}, {})", x, y);

        let population = swatch.population();
        let population_str = format!("{:<width$}", population, width = widths[2]);

        format!(
            "{}{} {} {}",
            sample_str, color_str, position_str, population_str
        )
    }

    #[inline]
    #[must_use]
    fn color_mode<T>(&self, color: &Color<T>) -> ColorMode
    where
        T: FloatNumber,
    {
        let env = self.context.env();
        if env.no_color.as_deref().is_some() {
            return ColorMode::NoColor;
        }

        match env.colorterm.as_deref() {
            Some("truecolor") | Some("24bit") => {
                let rgb = color.to_rgb();
                ColorMode::TrueColor(rgb)
            }
            Some("8bit") => {
                let ansi256 = color.to_ansi256();
                ColorMode::Ansi256(ansi256)
            }
            _ => {
                let ansi16 = color.to_ansi16();
                ColorMode::Ansi16(ansi16)
            }
        }
    }
}

impl<'a> Printer for TextPrinter<'a> {
    fn print<T, W>(&self, swatches: &[Swatch<T>], output: &mut W) -> Result<(), Error>
    where
        T: FloatNumber,
        W: Write,
    {
        let mut writer = BufWriter::new(output);

        let color_format = self.context.args().color;
        let widths = swatches.iter().fold([0, 0, 0], |acc, swatch| {
            let color_width = color_format.fmt(swatch.color()).len();

            let (x, y) = swatch.position();
            let position_width = format!("({}, {})", x, y).len();

            let population_width = swatch.population().to_string().len();
            [
                acc[0].max(color_width),
                acc[1].max(position_width),
                acc[2].max(population_width),
            ]
        });

        for swatch in swatches {
            let text = self.swatch_to_text(swatch, &widths);
            writeln!(writer, "{}", text)?;
        }
        writer.flush()
    }
}

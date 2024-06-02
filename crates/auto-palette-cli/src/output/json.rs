use std::io::{BufWriter, Error, Write};

use auto_palette::{FloatNumber, Swatch};
use serde_json::{Map, Value};

use crate::{context::Context, output::Printer};

const KEY_SWATCHES: &str = "swatches";
const KEY_COLOR: &str = "color";
const KEY_POSITION: &str = "position";
const KEY_POSITION_X: &str = "x";
const KEY_POSITION_Y: &str = "y";
const KEY_POPULATION: &str = "population";

/// The JSON printer for printing the swatches.
#[derive(Debug)]
pub struct JsonPrinter<'a> {
    context: &'a Context,
}

impl<'a> JsonPrinter<'a> {
    /// Creates a new `JsonPrinter` instance.
    ///
    /// # Arguments
    /// * `context` - The context of the application.
    ///
    /// # Returns
    /// A new `JsonPrinter` instance.
    pub fn new(context: &'a Context) -> Self {
        Self { context }
    }

    #[inline]
    #[must_use]
    fn swatch_to_json<T>(&self, swatch: &Swatch<T>) -> Value
    where
        T: FloatNumber,
    {
        let mut swatch_map = Map::with_capacity(3);

        let color_format = self.context.args().color;
        let color = color_format.fmt(swatch.color());
        swatch_map.insert(KEY_COLOR.into(), Value::String(color));

        swatch_map.insert(KEY_POSITION.into(), {
            let mut position_map = Map::with_capacity(2);
            let (x, y) = swatch.position();
            position_map.insert(KEY_POSITION_X.into(), Value::Number(x.into()));
            position_map.insert(KEY_POSITION_Y.into(), Value::Number(y.into()));
            Value::Object(position_map)
        });

        let population = swatch.population();
        swatch_map.insert(KEY_POPULATION.into(), Value::Number(population.into()));
        Value::Object(swatch_map)
    }
}

impl<'a> Printer for JsonPrinter<'a> {
    fn print<T, W>(&self, swatches: &[Swatch<T>], output: &mut W) -> Result<(), Error>
    where
        T: FloatNumber,
        W: Write,
    {
        let swatch_array = Value::Array(
            swatches
                .iter()
                .map(|swatch| self.swatch_to_json(swatch))
                .collect::<Vec<_>>(),
        );

        let root_object = Value::Object({
            let mut result_map = Map::with_capacity(1);
            result_map.insert(KEY_SWATCHES.into(), swatch_array);
            result_map
        });

        let mut writer = BufWriter::new(output);
        let json_str = serde_json::to_string_pretty(&root_object)?;
        writeln!(writer, "{}", json_str)?;
        writer.flush()?;
        Ok(())
    }
}

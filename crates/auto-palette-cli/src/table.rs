use std::fmt::Display;

/// The column representation for the table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Column {
    name: String,
    rows: Vec<String>,
    width: usize,
}

impl Column {
    /// Creates a new `Column` instance with the given heading.
    ///
    /// # Arguments
    /// * `name` - The name of the column.
    ///
    /// # Returns
    /// A new `Column` instance.
    pub fn new<T>(name: T) -> Self
    where
        T: AsRef<str>,
    {
        let name = name.as_ref();
        Self {
            name: name.to_string(),
            rows: vec![],
            width: name.len(),
        }
    }

    /// Returns the width of the column.
    ///
    /// # Returns
    /// The width of the column.
    #[inline]
    #[must_use]
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the name of the column.
    ///
    /// # Returns
    /// The name of the column.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the rows of the column.
    ///
    /// # Returns
    /// The rows of the column.
    #[inline]
    #[must_use]
    pub fn rows(&self) -> &[String] {
        &self.rows
    }

    /// Adds a row to the column.
    ///
    /// # Type Parameters
    /// * `T` - The type of the row.
    ///
    /// # Arguments
    /// * `row` - The row to add.
    pub fn add_row<T>(&mut self, row: T) -> &mut Self
    where
        T: AsRef<str> + Display,
    {
        let string = format!("{}", row);
        self.width = self.width.max(string.len());
        self.rows.push(string);
        self
    }
}

/// The table representation.
#[derive(Debug, PartialEq, Eq)]
pub struct Table {
    columns: Vec<Column>,
}

impl Table {
    /// Creates a new `Table` instance.
    ///
    /// # Returns
    /// A new `Table` instance.
    pub fn new() -> Self {
        Self { columns: vec![] }
    }

    /// Returns the columns of the table.
    ///
    /// # Returns
    /// The columns of the table.
    #[must_use]
    pub fn columns(&self) -> &[Column] {
        &self.columns
    }

    /// Adds a column to the table with the given name.
    ///
    /// # Arguments
    /// * `name` - The name of the column.
    pub fn add_column<T>(&mut self, name: T) -> &mut Self
    where
        T: AsRef<str>,
    {
        self.columns.push(Column::new(name));
        self
    }

    /// Adds a row to the table.
    ///
    /// # Type Parameters
    /// * `T` - The type of the row.
    ///
    /// # Arguments
    /// * `row` - The row to add.
    pub fn add_row<T>(&mut self, row: &[T])
    where
        T: AsRef<str> + Display,
    {
        for (column, value) in self.columns.iter_mut().zip(row.iter()) {
            column.add_row(value);
        }
    }
}

use crate::{Column, FromRowError};

/// A row from a query result
pub trait Row<'row>: 'row {
    /// An error that can occur while converting a row
    type Error: FromRowError;

    /// The column type produced for each column
    type Column<'column>: Column<'column, Error = Self::Error>
    where
        'row: 'column;

    /// Gets the next column of the row
    fn next<'column>(&'column mut self) -> Result<Option<Self::Column<'column>>, Self::Error>;
}

use crate::{Column, FromRowError};

/// A row from a query result
pub trait Row<'a>: Iterator<Item = Result<Self::Column, Self::Error>> {
    /// An error that can occur while converting a row
    type Error: FromRowError;

    /// The column type produced for each column
    type Column: Column<'a, Error = Self::Error>;
}

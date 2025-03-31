use crate::{FromColumn, FromRowError, Row};

/// A data structure which can built from a row from a query result
pub trait FromRow: Sized {
    /// Converts a row to this data structure
    fn from_row<'row, R: Row<'row>>(row: R) -> Result<Self, R::Error>;
}

impl<T: FromColumn> FromRow for T {
    fn from_row<'row, R: Row<'row>>(mut row: R) -> Result<Self, R::Error> {
        T::from_column(row.next()?.ok_or(R::Error::missing_column(""))?)
    }
}

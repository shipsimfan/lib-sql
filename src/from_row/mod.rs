use crate::Row;

mod from_column;

/// A data structure which can built from a row from a query result
pub trait FromRow: Sized {
    /// Converts a row to this data structure
    fn from_row<R: Row>(row: R) -> Result<Self, R::Error>;
}

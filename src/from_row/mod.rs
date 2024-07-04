use crate::Row;

/// A data structure which can built from a row from a query result
pub trait FromRow: Sized {
    /// An error that can occur while converting a row
    type Error;

    /// Converts a row to this data structure
    fn from_row<R: Row>(row: R) -> Result<Self, R::Error<Self::Error>>;
}

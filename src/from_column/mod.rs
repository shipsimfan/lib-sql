use crate::Column;

/// A type which can be created from a [`Column`]
pub trait FromColumn: Sized {
    /// An error that can occur while trying to convert a column to this value
    type Error;

    /// Attempt to convert `column` into `Self`
    fn from_column<'a, C: Column<'a>>(column: C) -> Result<Self, C::Error<Self::Error>>;
}

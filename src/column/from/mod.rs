use crate::Column;

mod blob;
mod null;
mod numeric;
mod string;

/// A type which can be created from a [`Column`]
pub trait FromColumn: Sized {
    /// Attempt to convert `column` into `Self`
    fn from_column<'column, C: Column<'column>>(column: C) -> Result<Self, C::Error>;
}

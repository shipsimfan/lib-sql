use crate::{Column, FromColumnError, FromRowError};

mod blob;
mod null;
mod numeric;
mod option;
mod string;

/// A type which can be created from a [`Column`]
pub trait FromColumn: Sized {
    /// Attempt to convert `column` into `Self`
    fn from_column<'column, C: Column<'column>>(column: C) -> Result<Self, C::Error>;

    /// Unwrap this value from an [`Option`]
    fn unwrap<'column, E: FromRowError>(
        this: Option<Self>,
        column: &'static str,
    ) -> Result<Self, E> {
        this.ok_or_else(|| E::missing_column(column))
    }

    /// Validate the either the length or value of this element
    #[allow(unused_variables)]
    fn validate<E: FromColumnError>(&self, min: Option<f64>, max: Option<f64>) -> Result<(), E> {
        Ok(())
    }
}

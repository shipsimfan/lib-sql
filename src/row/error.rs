use crate::FromColumnError;
use std::{error::Error, fmt::Display};

/// An error that can occur while converting a row
pub trait FromRowError: Error {
    /// An error that can occur while converting a column
    type FromColumnError: FromColumnError;

    /// A column is missing in the row
    fn missing_column(column: &'static str) -> Self;

    /// A column has an invalid value
    fn invalid_value(column: &'static str, error: Self::FromColumnError) -> Self;

    /// Creates a message displaying a custom error
    fn custom<D: Display>(message: D) -> Self;
}

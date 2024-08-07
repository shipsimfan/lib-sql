use std::{error::Error, fmt::Display};

/// An error that can occur while converting a row
pub trait FromRowError: Error {
    /// A column is missing in the row
    fn missing_column(column: &'static str) -> Self;

    /// Creates a message displaying a custom error
    fn custom<D: Display>(message: D) -> Self;
}

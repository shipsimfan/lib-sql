use std::{error::Error, fmt::Display};

/// An error that can occur while converting a column
pub trait FromColumnError: Error {
    /// Creates a message displaying a custom error
    fn custom<D: Display>(message: D) -> Self;
}

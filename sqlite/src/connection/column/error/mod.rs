mod display;
mod from_column_error;

/// An that can occur while converting a row
#[derive(Debug, Clone)]
pub struct SQLite3FromColumnError {
    /// The contained message describing the error
    message: String,
}

impl std::error::Error for SQLite3FromColumnError {}

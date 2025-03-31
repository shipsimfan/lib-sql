mod display;
mod new;

/// An error that can occur while executing SQL
#[derive(Debug)]
pub struct SQLite3ExecuteError {
    /// The message describing the error
    message: String,
}

impl std::error::Error for SQLite3ExecuteError {}

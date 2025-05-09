use crate::SQLite3FromColumnError;
use sqlite3::SQLiteError;

mod display;
mod from;
mod from_row_error;

/// An error that can occur while executing or preparing SQL
#[derive(Debug)]
pub enum SQLite3Error {
    /// An error reported from the database
    Database(SQLiteError),

    /// A column was missing from a query result
    MissingColumn(&'static str),

    /// A column has an invalid value
    InvalidValue(&'static str, SQLite3FromColumnError),

    /// A custom error reported while converting a query result
    Custom(String),
}

impl std::error::Error for SQLite3Error {}

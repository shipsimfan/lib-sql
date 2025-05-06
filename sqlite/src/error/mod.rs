use crate::SQLite3FromRowError;
use sqlite3::SQLiteError;

mod display;
mod from;

/// An error that can occur while executing or preparing SQL
#[derive(Debug)]
pub enum SQLite3Error {
    /// An error occurred while executing the query
    Execute(String),

    /// An error occurred while converting the query results
    FromRow(SQLite3FromRowError),

    /// An error occurred while preparing a statement
    Prepare(SQLiteError),
}

impl std::error::Error for SQLite3Error {}

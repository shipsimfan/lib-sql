use sqlite3::SQLiteError;

mod display;
mod from;
mod from_row_error;

/// An error with the database
#[derive(Debug, Clone)]
pub enum SQLite3FromRowError {
    /// An error reported from the database
    Database(SQLiteError),

    /// An column was missing from a query result
    MissingColumn(&'static str),

    /// A custom error reported while converting a query result
    Custom(String),
}

impl std::error::Error for SQLite3FromRowError {}

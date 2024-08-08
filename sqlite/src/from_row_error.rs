use sqlite3::SQLiteError;

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

impl sql::FromRowError for SQLite3FromRowError {
    fn missing_column(column: &'static str) -> Self {
        SQLite3FromRowError::MissingColumn(column)
    }

    fn custom<D: std::fmt::Display>(message: D) -> Self {
        SQLite3FromRowError::Custom(message.to_string())
    }
}

impl std::error::Error for SQLite3FromRowError {}

impl std::fmt::Display for SQLite3FromRowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SQLite3FromRowError::Database(error) => error.fmt(f),
            SQLite3FromRowError::MissingColumn(name) => write!(f, "missing column \"{}\"", name),
            SQLite3FromRowError::Custom(message) => f.write_str(&message),
        }
    }
}

impl From<SQLiteError> for SQLite3FromRowError {
    fn from(error: SQLiteError) -> Self {
        SQLite3FromRowError::Database(error)
    }
}

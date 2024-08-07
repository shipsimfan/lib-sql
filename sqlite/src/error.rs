use sqlite3::SQLiteError;

/// An error with the database
#[derive(Debug, Clone)]
pub enum SQLiteFromRowError {
    /// An error reported from the database
    Database(SQLiteError),

    /// An column was missing from a query result
    MissingColumn(&'static str),

    /// A custom error reported while converting a query result
    Custom(String),
}

impl sql::FromRowError for SQLiteFromRowError {
    fn missing_column(column: &'static str) -> Self {
        SQLiteFromRowError::MissingColumn(column)
    }

    fn custom<D: std::fmt::Display>(message: D) -> Self {
        SQLiteFromRowError::Custom(message.to_string())
    }
}

impl std::error::Error for SQLiteFromRowError {}

impl std::fmt::Display for SQLiteFromRowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SQLiteFromRowError::Database(error) => error.fmt(f),
            SQLiteFromRowError::MissingColumn(name) => write!(f, "missing column \"{}\"", name),
            SQLiteFromRowError::Custom(message) => f.write_str(&message),
        }
    }
}

impl From<SQLiteError> for SQLiteFromRowError {
    fn from(error: SQLiteError) -> Self {
        SQLiteFromRowError::Database(error)
    }
}

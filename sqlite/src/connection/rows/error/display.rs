use crate::SQLite3FromRowError;

impl std::fmt::Display for SQLite3FromRowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SQLite3FromRowError::Database(error) => error.fmt(f),
            SQLite3FromRowError::MissingColumn(name) => write!(f, "missing column \"{}\"", name),
            SQLite3FromRowError::InvalidValue(column, error) => {
                write!(f, "invalid value for column \"{}\" - {}", column, error)
            }
            SQLite3FromRowError::Custom(message) => message.fmt(f),
        }
    }
}

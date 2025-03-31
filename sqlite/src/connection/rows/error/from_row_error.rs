use crate::SQLite3FromRowError;
use sql::FromRowError;

impl FromRowError for SQLite3FromRowError {
    fn missing_column(column: &'static str) -> Self {
        SQLite3FromRowError::MissingColumn(column)
    }

    fn custom<D: std::fmt::Display>(message: D) -> Self {
        SQLite3FromRowError::Custom(message.to_string())
    }
}

use crate::{SQLite3FromColumnError, SQLite3FromRowError};
use sql::FromRowError;

impl FromRowError for SQLite3FromRowError {
    type FromColumnError = SQLite3FromColumnError;

    fn missing_column(column: &'static str) -> Self {
        SQLite3FromRowError::MissingColumn(column)
    }

    fn invalid_value(column: &'static str, error: Self::FromColumnError) -> Self {
        SQLite3FromRowError::InvalidValue(column, error)
    }

    fn custom<D: std::fmt::Display>(message: D) -> Self {
        SQLite3FromRowError::Custom(message.to_string())
    }
}

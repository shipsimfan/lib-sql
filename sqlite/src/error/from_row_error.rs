use crate::{SQLite3Error, SQLite3FromColumnError};
use sql::FromRowError;

impl FromRowError for SQLite3Error {
    type FromColumnError = SQLite3FromColumnError;

    fn missing_column(column: &'static str) -> Self {
        SQLite3Error::MissingColumn(column)
    }

    fn invalid_value(column: &'static str, error: Self::FromColumnError) -> Self {
        SQLite3Error::InvalidValue(column, error)
    }

    fn custom<D: std::fmt::Display>(message: D) -> Self {
        SQLite3Error::Custom(message.to_string())
    }
}

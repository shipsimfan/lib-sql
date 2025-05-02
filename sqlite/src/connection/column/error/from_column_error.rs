use crate::SQLite3FromColumnError;
use sql::FromColumnError;

impl FromColumnError for SQLite3FromColumnError {
    fn custom<D: std::fmt::Display>(message: D) -> Self {
        SQLite3FromColumnError {
            message: message.to_string(),
        }
    }
}

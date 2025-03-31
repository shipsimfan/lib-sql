use crate::SQLite3ExecuteError;

impl SQLite3ExecuteError {
    /// Creates a new [`SQLite3ExecuteError`]
    pub(crate) fn new(message: String) -> Self {
        SQLite3ExecuteError { message }
    }
}

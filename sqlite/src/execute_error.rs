/// An error that can occur while executing SQL
#[derive(Debug)]
pub struct SQLite3ExecuteError(String);

impl SQLite3ExecuteError {
    /// Creates a new [`SQLite3ExecuteError`]
    pub(crate) fn new(message: String) -> Self {
        SQLite3ExecuteError(message)
    }
}

impl std::error::Error for SQLite3ExecuteError {}

impl std::fmt::Display for SQLite3ExecuteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

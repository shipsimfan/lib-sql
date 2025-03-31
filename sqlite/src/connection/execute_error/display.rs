use crate::SQLite3ExecuteError;

impl std::fmt::Display for SQLite3ExecuteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(f)
    }
}

use crate::SQLite3FromColumnError;

impl std::fmt::Display for SQLite3FromColumnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(f)
    }
}

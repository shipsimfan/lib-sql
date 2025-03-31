use crate::SQLite3FromRowError;
use sqlite3::SQLiteError;

impl From<SQLiteError> for SQLite3FromRowError {
    fn from(error: SQLiteError) -> Self {
        SQLite3FromRowError::Database(error)
    }
}

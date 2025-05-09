use crate::SQLite3Error;
use sqlite3::SQLiteError;

impl From<String> for SQLite3Error {
    fn from(error: String) -> Self {
        SQLite3Error::Custom(error)
    }
}

impl From<SQLiteError> for SQLite3Error {
    fn from(error: SQLiteError) -> Self {
        SQLite3Error::Database(error)
    }
}

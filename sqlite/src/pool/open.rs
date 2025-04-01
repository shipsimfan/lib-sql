use crate::{SQLite3Connection, SQLite3ConnectionPool};
use sqlite3::SQLiteError;
use std::{path::Path, sync::Mutex};

impl SQLite3ConnectionPool {
    /// Attempts to open the database at `path`
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, SQLiteError> {
        SQLite3Connection::open(path).map(|connection| SQLite3ConnectionPool {
            connection: Mutex::new(connection),
        })
    }
}

use crate::{SQLite3Connection, SQLite3ConnectionPool, SQLite3Error};
use std::{path::Path, sync::Mutex};

impl SQLite3ConnectionPool {
    /// Attempts to open the database at `path`
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, SQLite3Error> {
        SQLite3Connection::open(path).map(|connection| SQLite3ConnectionPool {
            connection: Mutex::new(connection),
        })
    }
}

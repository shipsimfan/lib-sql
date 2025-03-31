use crate::SQLite3Connection;
use sqlite3::{
    sqlite3_close, sqlite3_open_v2, try_sqlite3, SQLiteError, SQLITE_OPEN_CREATE,
    SQLITE_OPEN_NOMUTEX, SQLITE_OPEN_READWRITE,
};
use std::{path::Path, ptr::null_mut};

impl SQLite3Connection {
    /// Attempts to open the database at `path`
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, SQLiteError> {
        let mut path = path.as_ref().as_os_str().as_encoded_bytes().to_vec();
        path.push(0);

        let mut handle = null_mut();
        try_sqlite3!(sqlite3_open_v2(
            path.as_ptr().cast(),
            &mut handle,
            SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE | SQLITE_OPEN_NOMUTEX,
            null_mut()
        ))
        .map(|_| SQLite3Connection { handle })
        .map_err(|error| {
            try_sqlite3!(sqlite3_close(handle)).unwrap();
            error
        })
    }
}

use crate::SQLite3Connection;
use sqlite3::{sqlite3_close, try_sqlite3};

impl Drop for SQLite3Connection {
    fn drop(&mut self) {
        try_sqlite3!(sqlite3_close(self.handle)).unwrap();
    }
}

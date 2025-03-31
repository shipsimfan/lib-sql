use crate::SQLite3Statement;
use sqlite3::{sqlite3_finalize, try_sqlite3};

impl<'statement> Drop for SQLite3Statement<'statement> {
    fn drop(&mut self) {
        if self.finalize {
            try_sqlite3!(sqlite3_finalize(self.handle)).unwrap();
        }
    }
}

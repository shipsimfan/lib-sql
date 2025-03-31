use crate::SQLite3Rows;
use sql::FromRow;
use sqlite3::{sqlite3_finalize, try_sqlite3};

impl<'statement, T: FromRow> Drop for SQLite3Rows<'statement, T> {
    fn drop(&mut self) {
        try_sqlite3!(sqlite3_finalize(self.statement.handle)).unwrap();
    }
}

use crate::SQLite3Connection;
use sqlite3::{sqlite3_finalize, try_sqlite3, SQLite3Stmt};

/// A prepared SQL statement for an [`SQLite3Connection`]
pub struct SQLite3Statement<'a> {
    handle: *mut SQLite3Stmt,
    _conn: &'a SQLite3Connection,
}

impl<'a> SQLite3Statement<'a> {
    /// Creates a new [`SQLite3Statement`]
    pub(crate) fn new(handle: *mut SQLite3Stmt, conn: &'a SQLite3Connection) -> Self {
        SQLite3Statement {
            handle,
            _conn: conn,
        }
    }
}

impl<'a> sql::Statement<'a> for SQLite3Statement<'a> {}

impl<'a> Drop for SQLite3Statement<'a> {
    fn drop(&mut self) {
        try_sqlite3!(sqlite3_finalize(self.handle)).unwrap();
    }
}

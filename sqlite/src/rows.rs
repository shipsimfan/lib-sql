use crate::{SQLite3Connection, SQLite3Row, SQLiteFromRowError};
use sql::FromRow;
use sqlite3::{
    sqlite3_finalize, sqlite3_step, try_sqlite3, SQLite3Stmt, SQLiteError, SQLITE_DONE, SQLITE_ROW,
};
use std::marker::PhantomData;

/// An iterator over a set of rows returned as the result of a query to an SQLite3 database
pub struct SQLite3Rows<'a, T: FromRow> {
    handle: *mut SQLite3Stmt,
    _conn: &'a SQLite3Connection,
    _output: PhantomData<T>,
}

impl<'a, T: FromRow> SQLite3Rows<'a, T> {
    /// Creates a new [`SQLite3Rows`] iterator
    pub(crate) fn new(handle: *mut SQLite3Stmt, conn: &'a SQLite3Connection) -> Self {
        SQLite3Rows {
            handle,
            _conn: conn,
            _output: PhantomData,
        }
    }
}

impl<'a, T: FromRow> Iterator for SQLite3Rows<'a, T> {
    type Item = Result<T, SQLiteFromRowError>;

    fn next(&mut self) -> Option<Self::Item> {
        match unsafe { sqlite3_step(self.handle) } {
            SQLITE_DONE => return None,
            SQLITE_ROW => {}
            error => return Some(Err(SQLiteError::new(error).into())),
        }

        Some(T::from_row(SQLite3Row::new(self.handle, self)))
    }
}

impl<'a, T: FromRow> Drop for SQLite3Rows<'a, T> {
    fn drop(&mut self) {
        try_sqlite3!(sqlite3_finalize(self.handle)).unwrap();
    }
}

unsafe impl<'a, T: FromRow> Send for SQLite3Rows<'a, T> {}

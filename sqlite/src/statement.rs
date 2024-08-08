use crate::{SQLite3Connection, SQLite3Rows, SQLite3FromRowError};
use sql::FromRow;
use sqlite3::{
    sqlite3_bind_blob, sqlite3_bind_double, sqlite3_bind_int64, sqlite3_bind_null,
    sqlite3_bind_text, sqlite3_finalize, try_sqlite3, SQLite3Stmt, SQLiteError,
};

/// A prepared SQL statement for an [`SQLite3Connection`]
pub struct SQLite3Statement<'a> {
    handle: *mut SQLite3Stmt,
    conn: &'a SQLite3Connection,
    finalize: bool,
}

impl<'a> SQLite3Statement<'a> {
    /// Creates a new [`SQLite3Statement`]
    pub(crate) fn new(handle: *mut SQLite3Stmt, conn: &'a SQLite3Connection) -> Self {
        SQLite3Statement {
            handle,
            conn,
            finalize: true,
        }
    }
}

impl<'a> sql::Statement<'a> for SQLite3Statement<'a> {
    type BindError = SQLiteError;

    type GetRowError = SQLite3FromRowError;

    fn rows<T: FromRow>(
        mut self,
    ) -> Result<impl Iterator<Item = Result<T, Self::GetRowError>>, Self::GetRowError> {
        self.finalize = false;

        Ok(SQLite3Rows::new(self.handle, self.conn))
    }

    fn bind_u64(&mut self, idx: usize, val: u64) -> Result<(), Self::BindError> {
        self.bind_i64(idx, val as _)
    }

    fn bind_i64(&mut self, idx: usize, val: i64) -> Result<(), Self::BindError> {
        try_sqlite3!(sqlite3_bind_int64(self.handle, idx as _, val)).map(|_| ())
    }

    fn bind_f64(&mut self, idx: usize, val: f64) -> Result<(), Self::BindError> {
        try_sqlite3!(sqlite3_bind_double(self.handle, idx as _, val)).map(|_| ())
    }

    fn bind_str(&mut self, idx: usize, s: &'a str) -> Result<(), Self::BindError> {
        try_sqlite3!(sqlite3_bind_text(
            self.handle,
            idx as _,
            s.as_ptr().cast(),
            s.len() as _,
            None
        ))
        .map(|_| ())
    }

    fn bind_blob(&mut self, idx: usize, b: &'a [u8]) -> Result<(), Self::BindError> {
        try_sqlite3!(sqlite3_bind_blob(
            self.handle,
            idx as _,
            b.as_ptr().cast(),
            b.len() as _,
            None
        ))
        .map(|_| ())
    }

    fn bind_null(&mut self, idx: usize) -> Result<(), Self::BindError> {
        try_sqlite3!(sqlite3_bind_null(self.handle, idx as _)).map(|_| ())
    }
}

impl<'a> Drop for SQLite3Statement<'a> {
    fn drop(&mut self) {
        if self.finalize {
            try_sqlite3!(sqlite3_finalize(self.handle)).unwrap();
        }
    }
}

unsafe impl<'a> Send for SQLite3Statement<'a> {}

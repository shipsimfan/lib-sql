use crate::{SQLite3FromRowError, SQLite3Rows, SQLite3Statement};
use sql::{FromRow, Statement};
use sqlite3::{
    sqlite3_bind_blob, sqlite3_bind_double, sqlite3_bind_int64, sqlite3_bind_null,
    sqlite3_bind_text, sqlite3_step, try_sqlite3, SQLiteError, SQLITE_DONE, SQLITE_OK, SQLITE_ROW,
};

impl<'statement> Statement<'statement> for SQLite3Statement<'statement> {
    type BindError = SQLiteError;

    type GetRowError = SQLite3FromRowError;

    fn rows<T: FromRow>(
        mut self,
    ) -> Result<impl Iterator<Item = Result<T, Self::GetRowError>>, Self::GetRowError> {
        self.finalize = false;

        Ok(SQLite3Rows::new(self))
    }

    fn execute(self) -> Result<(), Self::GetRowError> {
        let result = match unsafe { sqlite3_step(self.handle) } {
            SQLITE_DONE | SQLITE_ROW | SQLITE_OK => Ok(()),
            error => Err(SQLite3FromRowError::Database(SQLiteError::new(error))),
        };
        result
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

    fn bind_str(&mut self, idx: usize, s: &'statement str) -> Result<(), Self::BindError> {
        try_sqlite3!(sqlite3_bind_text(
            self.handle,
            idx as _,
            s.as_ptr().cast(),
            s.len() as _,
            None
        ))
        .map(|_| ())
    }

    fn bind_blob(&mut self, idx: usize, b: &'statement [u8]) -> Result<(), Self::BindError> {
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

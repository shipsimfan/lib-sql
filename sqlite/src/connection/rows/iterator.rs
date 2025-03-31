use crate::{SQLite3FromRowError, SQLite3Row, SQLite3Rows};
use sql::FromRow;
use sqlite3::{sqlite3_step, SQLiteError, SQLITE_DONE, SQLITE_ROW};

impl<'statement, T: FromRow> Iterator for SQLite3Rows<'statement, T> {
    type Item = Result<T, SQLite3FromRowError>;

    fn next(&mut self) -> Option<Self::Item> {
        match unsafe { sqlite3_step(self.statement.handle) } {
            SQLITE_DONE => return None,
            SQLITE_ROW => {}
            error => return Some(Err(SQLiteError::new(error).into())),
        }

        Some(T::from_row(SQLite3Row::new(&mut self.statement)))
    }
}

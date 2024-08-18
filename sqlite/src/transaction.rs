use crate::{connection, SQLite3ExecuteError, SQLite3Statement};
use sql::Transaction;
use sqlite3::{SQLite3, SQLiteError};
use std::sync::MutexGuard;

/// A set of sql statements which will be rolled back automatically if not comitted
pub struct SQLite3Transaction<'a> {
    handle: MutexGuard<'a, *mut SQLite3>,
    comitted: bool,
}

impl<'a> SQLite3Transaction<'a> {
    /// Creates a new [`SQLite3Transaction`]
    pub(crate) fn new(handle: MutexGuard<'a, *mut SQLite3>) -> Result<Self, SQLite3ExecuteError> {
        connection::execute(*handle, "BEGIN;")?;

        Ok(SQLite3Transaction {
            handle,
            comitted: false,
        })
    }
}

impl<'a> Transaction<'a> for SQLite3Transaction<'a> {
    type Statement<'b> = SQLite3Statement<'b> where Self: 'b;

    type ExecuteError = SQLite3ExecuteError;

    type PrepareError = SQLiteError;

    fn execute(&self, sql: &str) -> Result<(), Self::ExecuteError> {
        connection::execute(*self.handle, sql)
    }

    fn prepare<'b>(&'b self, sql: &str) -> Result<Self::Statement<'b>, Self::PrepareError> {
        connection::prepare(*self.handle, None, sql)
    }

    fn commit(mut self) -> Result<(), Self::ExecuteError> {
        connection::execute(*self.handle, "COMMIT;")?;
        self.comitted = true;
        Ok(())
    }
}

impl<'a> Drop for SQLite3Transaction<'a> {
    fn drop(&mut self) {
        connection::execute(*self.handle, "ROLLBACK;").unwrap();
    }
}

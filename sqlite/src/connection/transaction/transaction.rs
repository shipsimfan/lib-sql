use crate::{SQLite3ExecuteError, SQLite3Statement, SQLite3Transaction};
use sql::{Connection, Transaction};
use sqlite3::{sqlite3_last_insert_rowid, SQLiteError};

impl<'transaction> Transaction<'transaction> for SQLite3Transaction<'transaction> {
    type Statement<'statement>
        = SQLite3Statement<'statement>
    where
        'transaction: 'statement;

    type ExecuteError = SQLite3ExecuteError;

    type PrepareError = SQLiteError;

    fn execute(&mut self, sql: &str) -> Result<(), Self::ExecuteError> {
        self.connection.execute(sql)
    }

    fn prepare<'statement>(
        &'statement mut self,
        sql: &str,
    ) -> Result<Self::Statement<'statement>, Self::PrepareError> {
        self.connection.prepare(sql)
    }

    fn last_insert_id(&mut self) -> Option<usize> {
        let id = unsafe { sqlite3_last_insert_rowid(self.connection.handle) };
        if id <= 0 {
            None
        } else {
            Some(id as _)
        }
    }

    fn commit(mut self) -> Result<(), Self::ExecuteError> {
        self.connection.execute("END;")?;
        self.comitted = true;
        Ok(())
    }
}

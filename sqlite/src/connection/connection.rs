use crate::{SQLite3Connection, SQLite3ExecuteError, SQLite3Statement, SQLite3Transaction};
use sql::Connection;
use sqlite3::SQLiteError;

impl Connection for SQLite3Connection {
    type Statement<'statement> = SQLite3Statement<'statement>;

    type Transaction<'transaction> = SQLite3Transaction<'transaction>;

    type ExecuteError = SQLite3ExecuteError;

    type PrepareError = SQLiteError;

    fn execute(&mut self, sql: &str) -> Result<(), SQLite3ExecuteError> {
        self.do_execute(sql)
    }

    fn prepare<'statement>(
        &'statement mut self,
        sql: &str,
    ) -> Result<Self::Statement<'statement>, Self::PrepareError> {
        self.do_prepare(sql)
    }

    fn begin_trasaction<'transaction>(
        &'transaction mut self,
    ) -> Result<Self::Transaction<'transaction>, Self::ExecuteError> {
        SQLite3Transaction::new(self)
    }
}

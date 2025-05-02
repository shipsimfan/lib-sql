use crate::{SQLite3Connection, SQLite3ExecuteError, SQLite3Statement};
use sql::SqlContext;
use sqlite3::SQLiteError;

impl<'context> SqlContext<'context> for SQLite3Connection {
    type Statement<'statement>
        = SQLite3Statement<'statement>
    where
        'context: 'statement;

    type ExecuteError = SQLite3ExecuteError;

    type PrepareError = SQLiteError;

    fn execute(&mut self, sql: &str) -> Result<(), SQLite3ExecuteError> {
        self.do_execute(sql)
    }

    fn prepare<'statement>(
        &'statement mut self,
        sql: &str,
    ) -> Result<Self::Statement<'statement>, Self::PrepareError>
    where
        'context: 'statement,
    {
        self.do_prepare(sql)
    }
}

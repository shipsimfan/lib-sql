use crate::{SQLite3ExecuteError, SQLite3Statement, SQLite3Transaction};
use sql::SqlContext;
use sqlite3::SQLiteError;

impl<'context> SqlContext<'context> for SQLite3Transaction<'context> {
    type Statement<'statement>
        = SQLite3Statement<'statement>
    where
        'context: 'statement;

    type ExecuteError = SQLite3ExecuteError;

    type PrepareError = SQLiteError;

    fn execute(&mut self, sql: &str) -> Result<(), Self::ExecuteError> {
        self.connection.execute(sql)
    }

    fn prepare<'statement>(
        &'statement mut self,
        sql: &str,
    ) -> Result<Self::Statement<'statement>, Self::PrepareError>
    where
        'context: 'statement,
    {
        self.connection.prepare(sql)
    }
}

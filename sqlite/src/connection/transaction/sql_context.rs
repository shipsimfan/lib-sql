use crate::{SQLite3Error, SQLite3Statement, SQLite3Transaction};
use sql::SqlContext;

impl<'context> SqlContext<'context> for SQLite3Transaction<'context> {
    type Statement<'statement>
        = SQLite3Statement<'statement>
    where
        'context: 'statement;

    type Error = SQLite3Error;

    fn execute(&mut self, sql: &str) -> Result<(), Self::Error> {
        self.connection.execute(sql)
    }

    fn prepare<'statement>(
        &'statement mut self,
        sql: &str,
    ) -> Result<Self::Statement<'statement>, Self::Error>
    where
        'context: 'statement,
    {
        self.connection.prepare(sql)
    }
}

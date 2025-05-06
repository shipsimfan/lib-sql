use crate::{SQLite3Connection, SQLite3Error, SQLite3Statement};
use sql::SqlContext;

impl<'context> SqlContext<'context> for SQLite3Connection {
    type Statement<'statement>
        = SQLite3Statement<'statement>
    where
        'context: 'statement;

    type Error = SQLite3Error;

    fn execute(&mut self, sql: &str) -> Result<(), SQLite3Error> {
        self.do_execute(sql)
    }

    fn prepare<'statement>(
        &'statement mut self,
        sql: &str,
    ) -> Result<Self::Statement<'statement>, Self::Error>
    where
        'context: 'statement,
    {
        self.do_prepare(sql)
    }
}

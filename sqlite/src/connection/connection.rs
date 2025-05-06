use crate::{SQLite3Connection, SQLite3Transaction};
use sql::Connection;

impl<'connection> Connection<'connection> for SQLite3Connection {
    type Transaction<'transaction>
        = SQLite3Transaction<'transaction>
    where
        'connection: 'transaction;

    fn begin_trasaction<'transaction>(
        &'transaction mut self,
    ) -> Result<Self::Transaction<'transaction>, Self::Error>
    where
        'connection: 'transaction,
    {
        SQLite3Transaction::new(self)
    }
}

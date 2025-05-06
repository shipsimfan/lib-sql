use crate::{SQLite3Connection, SQLite3Error, SQLite3Transaction};
use sql::SqlContext;

impl<'transaction> SQLite3Transaction<'transaction> {
    /// Creates a new [`SQLite3Transaction`]
    pub(crate) fn new(
        connection: &'transaction mut SQLite3Connection,
    ) -> Result<Self, SQLite3Error> {
        connection.execute("BEGIN;")?;

        Ok(SQLite3Transaction {
            connection,
            comitted: false,
        })
    }
}

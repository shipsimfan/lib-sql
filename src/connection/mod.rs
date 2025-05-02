use crate::{SqlContext, Transaction};

mod sync;

/// Represents a connection to a database
pub trait Connection<'connection>: SqlContext<'connection> {
    /// A transaction which rolls back automatically if not comitted
    type Transaction<'transaction>: Transaction<'transaction>
    where
        'connection: 'transaction;

    /// Start a transaction
    fn begin_trasaction<'transaction>(
        &'transaction mut self,
    ) -> Result<Self::Transaction<'transaction>, Self::ExecuteError>
    where
        'connection: 'transaction;
}

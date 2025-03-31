use crate::{Statement, Transaction};

/// Represents a connection to a database
pub trait Connection: 'static {
    /// A prepared SQL statement
    type Statement<'statement>: Statement<'statement>;

    /// A transaction which rolls back automatically if not comitted
    type Transaction<'transaction>: Transaction<'transaction>;

    /// An error that can occur while executing some SQL
    type ExecuteError: std::error::Error;

    /// An error that can occur while preparing an SQL statement
    type PrepareError: std::error::Error;

    /// Runs an block of SQL code
    fn execute(&mut self, sql: &str) -> Result<(), Self::ExecuteError>;

    /// Prepares an SQL statement for binding and running
    fn prepare<'statement>(
        &'statement mut self,
        sql: &str,
    ) -> Result<Self::Statement<'statement>, Self::PrepareError>;

    /// Start a transaction
    fn begin_trasaction<'transaction>(
        &'transaction mut self,
    ) -> Result<Self::Transaction<'transaction>, Self::ExecuteError>;
}

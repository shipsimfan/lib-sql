use crate::{Statement, Transaction};

/// Represents a connection to a database
pub trait Connection {
    /// A prepared SQL statement
    type Statement<'a>: Statement<'a>
    where
        Self: 'a;

    /// A transaction which rolls back automatically if not comitted
    type Transaction<'a>: Transaction<'a>
    where
        Self: 'a;

    /// An error that can occur while executing some SQL
    type ExecuteError: std::error::Error;

    /// An error that can occur while preparing an SQL statement
    type PrepareError: std::error::Error;

    /// Runs an block of SQL code
    fn execute(&self, sql: &str) -> Result<(), Self::ExecuteError>;

    /// Prepares an SQL statement for binding and running
    fn prepare<'a>(&'a self, sql: &str) -> Result<Self::Statement<'a>, Self::PrepareError>;

    /// Start a transaction
    fn begin_trasaction<'a>(&'a self) -> Result<Self::Transaction<'a>, Self::ExecuteError>;
}

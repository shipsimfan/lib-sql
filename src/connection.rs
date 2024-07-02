use crate::Statement;

/// Represents a connection to a database
pub trait Connection: 'static {
    /// A prepared SQL statement
    type Statement<'a>: Statement<'a>;

    /// An error that can occur while executing some SQL
    type ExecuteError;

    /// An error that can occur while preparing an SQL statement
    type PrepareError;

    /// Runs an block of SQL code
    fn execute(&self, sql: &str) -> Result<(), Self::ExecuteError>;

    /// Prepares an SQL statement for binding and running
    fn prepare<'a>(&'a self, sql: &str) -> Result<Self::Statement<'a>, Self::PrepareError>;
}

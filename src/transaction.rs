use crate::Statement;

/// A set of sql statements which is rolled back automatically if not committed
pub trait Transaction<'a> {
    /// A prepared SQL statement
    type Statement<'b>: Statement<'b>
    where
        Self: 'b;

    /// An error that can occur while executing some SQL
    type ExecuteError: std::error::Error;

    /// An error that can occur while preparing an SQL statement
    type PrepareError: std::error::Error;

    /// Runs an block of SQL code
    fn execute(&self, sql: &str) -> Result<(), Self::ExecuteError>;

    /// Prepares an SQL statement for binding and running
    fn prepare<'b>(&'b self, sql: &str) -> Result<Self::Statement<'b>, Self::PrepareError>;

    /// Commits the effects of this transaction
    fn commit(self) -> Result<(), Self::ExecuteError>;
}

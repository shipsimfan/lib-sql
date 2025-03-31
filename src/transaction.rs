use crate::Statement;

/// A set of sql statements which is rolled back automatically if not committed
pub trait Transaction<'transaction>: 'transaction {
    /// A prepared SQL statement
    type Statement<'statement>: Statement<'statement>
    where
        'transaction: 'statement;

    /// An error that can occur while executing some SQL
    type ExecuteError: std::error::Error;

    /// An error that can occur while preparing an SQL statement
    type PrepareError: std::error::Error;

    /// Runs an block of SQL code
    fn execute(&mut self, sql: &str) -> Result<(), Self::ExecuteError>;

    /// Prepares an SQL statement for binding and running
    fn prepare<'b>(&'b mut self, sql: &str) -> Result<Self::Statement<'b>, Self::PrepareError>;

    /// Gets the last inserted id, if there is one
    fn last_insert_id(&mut self) -> Option<usize>;

    /// Commits the effects of this transaction
    fn commit(self) -> Result<(), Self::ExecuteError>;
}

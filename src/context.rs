use crate::Statement;

/// A context in which SQL can be executed
pub trait SqlContext<'context>: 'context {
    /// A prepared SQL statement
    type Statement<'statement>: Statement<'statement>
    where
        'context: 'statement;

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
    ) -> Result<Self::Statement<'statement>, Self::PrepareError>
    where
        'context: 'statement;
}

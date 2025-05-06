use crate::Statement;

/// A context in which SQL can be executed
pub trait SqlContext<'context>: 'context {
    /// A prepared SQL statement
    type Statement<'statement>: Statement<'statement, Error = Self::Error>
    where
        'context: 'statement;

    /// An error that can occur while executing or preparing some SQL
    type Error: std::error::Error;

    /// Runs an block of SQL code
    fn execute(&mut self, sql: &str) -> Result<(), Self::Error>;

    /// Prepares an SQL statement for binding and running
    fn prepare<'statement>(
        &'statement mut self,
        sql: &str,
    ) -> Result<Self::Statement<'statement>, Self::Error>
    where
        'context: 'statement;
}

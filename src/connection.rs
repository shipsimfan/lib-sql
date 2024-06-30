/// Represents a connection to a database
pub trait Connection {
    /// An error that can occur while executing some SQL
    type ExecuteError;

    /// Runs an block of SQL code
    fn execute(&self, sql: &str) -> Result<(), Self::ExecuteError>;
}

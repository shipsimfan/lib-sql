use crate::SqlContext;

/// A set of sql statements which is rolled back automatically if not committed
pub trait Transaction<'transaction>: SqlContext<'transaction> {
    /// Gets the last inserted id, if there is one
    fn last_insert_id(&mut self) -> Option<usize>;

    /// Commits the effects of this transaction
    fn commit(self) -> Result<(), Self::ExecuteError>;
}

use crate::SQLite3Connection;

mod drop;
mod new;
mod sql_context;
mod transaction;

/// A set of sql statements which will be rolled back automatically if not comitted
pub struct SQLite3Transaction<'transaction> {
    /// The handle to the parent connection this transction is on
    connection: &'transaction mut SQLite3Connection,

    /// Has the transaction been comitted?
    comitted: bool,
}

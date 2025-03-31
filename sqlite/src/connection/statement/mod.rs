use crate::SQLite3Connection;
use sqlite3::SQLite3Stmt;

mod drop;
mod new;
mod statement;

/// A prepared SQL statement for an [`SQLite3Connection`]
pub struct SQLite3Statement<'statement> {
    /// The handle to the statement
    pub(crate) handle: *mut SQLite3Stmt,

    /// The connection this statement comes from
    #[allow(unused)]
    conn: &'statement mut SQLite3Connection,

    /// Is finalizing needed?
    finalize: bool,
}

unsafe impl<'statement> Send for SQLite3Statement<'statement> {}

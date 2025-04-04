use crate::{SQLite3Connection, SQLite3Statement};
use sqlite3::SQLite3Stmt;

impl<'statement> SQLite3Statement<'statement> {
    /// Creates a new [`SQLite3Statement`]
    pub(crate) fn new(handle: *mut SQLite3Stmt, conn: &'statement mut SQLite3Connection) -> Self {
        SQLite3Statement { handle, conn }
    }
}

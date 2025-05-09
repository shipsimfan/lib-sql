use crate::{SQLite3Connection, SQLite3Error, SQLite3Statement};
use sqlite3::{sqlite3_prepare_v2, try_sqlite3};
use std::ptr::null_mut;

impl SQLite3Connection {
    /// Prepares an [`SQLite3Statement`] using `sql` on `handle`, assuming `handle` is locked
    pub(crate) fn do_prepare<'statement>(
        &'statement mut self,
        sql: &str,
    ) -> Result<SQLite3Statement<'statement>, SQLite3Error> {
        let sql = format!("{}\0", sql);

        let mut stmt_handle = null_mut();
        try_sqlite3!(sqlite3_prepare_v2(
            self.handle,
            sql.as_ptr().cast(),
            sql.len() as _,
            &mut stmt_handle,
            null_mut()
        ))
        .map(|_| SQLite3Statement::new(stmt_handle, self))
        .map_err(SQLite3Error::Database)
    }
}

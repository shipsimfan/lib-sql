use crate::SQLite3Transaction;
use sql::{SqlContext, Transaction};
use sqlite3::sqlite3_last_insert_rowid;

impl<'transaction> Transaction<'transaction> for SQLite3Transaction<'transaction> {
    fn last_insert_id(&mut self) -> Option<usize> {
        let id = unsafe { sqlite3_last_insert_rowid(self.connection.handle) };
        if id <= 0 {
            None
        } else {
            Some(id as _)
        }
    }

    fn commit(mut self) -> Result<(), Self::Error> {
        self.connection.execute("END;")?;
        self.comitted = true;
        Ok(())
    }
}

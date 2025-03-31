use crate::SQLite3Transaction;
use sql::Connection;

impl<'transaction> Drop for SQLite3Transaction<'transaction> {
    fn drop(&mut self) {
        if !self.comitted {
            self.connection.execute("ROLLBACK;").unwrap();
        }
    }
}

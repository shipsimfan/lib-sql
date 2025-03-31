use crate::{SQLite3Connection, SQLite3ExecuteError};
use sqlite3::{sqlite3_exec, sqlite3_free, try_sqlite3};
use std::{ffi::CStr, ptr::null_mut};

impl SQLite3Connection {
    /// Executes `sql` on `handle`
    pub(crate) fn do_execute(&mut self, sql: &str) -> Result<(), SQLite3ExecuteError> {
        let sql = format!("{}\0", sql);

        let mut errmsg_ptr = null_mut();
        let result = try_sqlite3!(sqlite3_exec(
            self.handle,
            sql.as_ptr().cast(),
            None,
            null_mut(),
            &mut errmsg_ptr
        ));

        if result.is_ok() {
            return Ok(());
        }

        if errmsg_ptr == null_mut() {
            return Err(SQLite3ExecuteError::new(result.unwrap_err().to_string()));
        }

        let errmsg = unsafe { CStr::from_ptr(errmsg_ptr) }
            .to_string_lossy()
            .to_string();

        unsafe { sqlite3_free(errmsg_ptr.cast()) };

        Err(SQLite3ExecuteError::new(errmsg))
    }
}

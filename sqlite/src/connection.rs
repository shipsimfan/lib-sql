use crate::SQLite3Statement;
use sqlite3::{
    sqlite3_close, sqlite3_exec, sqlite3_free, sqlite3_open_v2, sqlite3_prepare_v2, try_sqlite3,
    SQLite3, SQLiteError, SQLITE_OPEN_CREATE, SQLITE_OPEN_FULLMUTEX, SQLITE_OPEN_READWRITE,
};
use std::{ffi::CStr, path::Path, ptr::null_mut};

/// A connection an SQLite3 database
pub struct SQLite3Connection {
    handle: *mut SQLite3,
}

impl SQLite3Connection {
    /// Attempts to open the database at `path`
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, SQLiteError> {
        let mut path = path.as_ref().as_os_str().as_encoded_bytes().to_vec();
        path.push(0);

        let mut handle = null_mut();
        try_sqlite3!(sqlite3_open_v2(
            path.as_ptr().cast(),
            &mut handle,
            SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE | SQLITE_OPEN_FULLMUTEX,
            null_mut()
        ))
        .map(|_| SQLite3Connection { handle })
        .map_err(|error| {
            try_sqlite3!(sqlite3_close(handle)).unwrap();
            error
        })
    }
}

impl sql::Connection for SQLite3Connection {
    type Statement<'a> = SQLite3Statement<'a>;

    type ExecuteError = String;

    type PrepareError = SQLiteError;

    fn execute(&self, sql: &str) -> Result<(), String> {
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
            return Err(result.unwrap_err().to_string());
        }

        let errmsg = unsafe { CStr::from_ptr(errmsg_ptr) }
            .to_string_lossy()
            .to_string();
        unsafe { sqlite3_free(errmsg_ptr.cast()) };
        Err(errmsg)
    }

    fn prepare<'a>(&'a self, sql: &str) -> Result<Self::Statement<'a>, Self::PrepareError> {
        let mut handle = null_mut();
        try_sqlite3!(sqlite3_prepare_v2(
            self.handle,
            sql.as_ptr().cast(),
            sql.len() as _,
            &mut handle,
            null_mut()
        ))
        .map(|_| SQLite3Statement::new(handle, self))
    }
}

impl Drop for SQLite3Connection {
    fn drop(&mut self) {
        try_sqlite3!(sqlite3_close(self.handle)).unwrap();
    }
}

unsafe impl Send for SQLite3Connection {}
unsafe impl Sync for SQLite3Connection {}

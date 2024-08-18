use crate::{SQLite3ExecuteError, SQLite3Statement};
use sqlite3::{
    sqlite3_close, sqlite3_exec, sqlite3_free, sqlite3_open_v2, sqlite3_prepare_v2, try_sqlite3,
    SQLite3, SQLiteError, SQLITE_OPEN_CREATE, SQLITE_OPEN_NOMUTEX, SQLITE_OPEN_READWRITE,
};
use std::{
    ffi::CStr,
    path::Path,
    ptr::null_mut,
    sync::{Mutex, MutexGuard},
};

/// A connection an SQLite3 database
pub struct SQLite3Connection {
    handle: Mutex<*mut SQLite3>,
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
            SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE | SQLITE_OPEN_NOMUTEX,
            null_mut()
        ))
        .map(|_| SQLite3Connection {
            handle: Mutex::new(handle),
        })
        .map_err(|error| {
            try_sqlite3!(sqlite3_close(handle)).unwrap();
            error
        })
    }

    /// Locks the underlying connection
    pub(crate) fn lock(&self) -> MutexGuard<*mut SQLite3> {
        self.handle.lock().unwrap()
    }
}

impl sql::Connection for SQLite3Connection {
    type Statement<'a> = SQLite3Statement<'a>;

    type ExecuteError = SQLite3ExecuteError;

    type PrepareError = SQLiteError;

    fn execute(&self, sql: &str) -> Result<(), SQLite3ExecuteError> {
        let sql = format!("{}\0", sql);

        let mut errmsg_ptr = null_mut();
        let handle = self.lock();
        let result = try_sqlite3!(sqlite3_exec(
            *handle,
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

    fn prepare<'a>(&'a self, sql: &str) -> Result<Self::Statement<'a>, Self::PrepareError> {
        let sql = format!("{}\0", sql);

        let mut stmt_handle = null_mut();
        let conn_handle = self.lock();
        try_sqlite3!(sqlite3_prepare_v2(
            *conn_handle,
            sql.as_ptr().cast(),
            sql.len() as _,
            &mut stmt_handle,
            null_mut()
        ))
        .map(|_| SQLite3Statement::new(stmt_handle, self))
    }
}

impl Drop for SQLite3Connection {
    fn drop(&mut self) {
        let handle = self.handle.lock().unwrap();
        try_sqlite3!(sqlite3_close(*handle)).unwrap();
    }
}

unsafe impl Send for SQLite3Connection {}
unsafe impl Sync for SQLite3Connection {}

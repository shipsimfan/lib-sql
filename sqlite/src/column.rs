use crate::SQLiteFromRowError;
use sqlite3::{
    sqlite3_column_blob, sqlite3_column_bytes, sqlite3_column_double, sqlite3_column_int64,
    sqlite3_column_text, SQLite3Stmt,
};
use std::{marker::PhantomData, ptr::null};

/// A column of a result returned by an query to an SQLite3 database
pub struct SQLite3Column<'a> {
    handle: *mut SQLite3Stmt,
    index: usize,
    _lifetime: PhantomData<&'a mut ()>,
}

impl<'a> SQLite3Column<'a> {
    /// Creates a new [`SQLite3Column`]
    pub(crate) fn new(handle: *mut SQLite3Stmt, index: usize) -> Self {
        SQLite3Column {
            handle,
            index,
            _lifetime: PhantomData,
        }
    }
}

impl<'a> sql::Column<'a> for SQLite3Column<'a> {
    type Error = SQLiteFromRowError;

    fn name(&self) -> Result<String, Self::Error> {
        Ok(String::new())
    }

    fn into_blob(self) -> Result<&'a [u8], Self::Error> {
        let ptr = unsafe { sqlite3_column_blob(self.handle, self.index as _) };
        let len = unsafe { sqlite3_column_bytes(self.handle, self.index as _) };

        return Ok(if ptr == null() || len <= 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(ptr as _, len as _) }
        });
    }

    fn into_str(self) -> Result<&'a str, Self::Error> {
        let ptr = unsafe { sqlite3_column_text(self.handle, self.index as _) };
        let len = unsafe { sqlite3_column_bytes(self.handle, self.index as _) };

        return Ok(if ptr == null() || len <= 0 {
            ""
        } else {
            unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr as _, len as _)) }
        });
    }

    fn into_u64(self) -> Result<u64, Self::Error> {
        self.into_i64().map(|val| val as _)
    }

    fn into_i64(self) -> Result<i64, Self::Error> {
        Ok(unsafe { sqlite3_column_int64(self.handle, self.index as _) })
    }

    fn into_f64(self) -> Result<f64, Self::Error> {
        Ok(unsafe { sqlite3_column_double(self.handle, self.index as _) })
    }
}

unsafe impl<'a> Send for SQLite3Column<'a> {}

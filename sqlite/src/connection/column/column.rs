use crate::{SQLite3Column, SQLite3FromRowError};
use sqlite3::{
    sqlite3_column_blob, sqlite3_column_bytes, sqlite3_column_double, sqlite3_column_int64,
    sqlite3_column_name, sqlite3_column_text,
};
use std::{ffi::CStr, ptr::null};

impl<'column, 'statement> sql::Column<'column> for SQLite3Column<'column, 'statement> {
    type Error = SQLite3FromRowError;

    fn name(&self) -> Result<String, Self::Error> {
        let ptr = unsafe { sqlite3_column_name(self.statement.handle, self.index as _) };
        if ptr == null() {
            return Ok(String::new());
        }

        Ok(unsafe { CStr::from_ptr(ptr) }.to_string_lossy().to_string())
    }

    fn into_blob(self) -> Result<&'column [u8], Self::Error> {
        let ptr = unsafe { sqlite3_column_blob(self.statement.handle, self.index as _) };
        let len = unsafe { sqlite3_column_bytes(self.statement.handle, self.index as _) };

        return Ok(if ptr == null() || len <= 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(ptr as _, len as _) }
        });
    }

    fn into_str(self) -> Result<&'column str, Self::Error> {
        let ptr = unsafe { sqlite3_column_text(self.statement.handle, self.index as _) };
        let len = unsafe { sqlite3_column_bytes(self.statement.handle, self.index as _) };

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
        Ok(unsafe { sqlite3_column_int64(self.statement.handle, self.index as _) })
    }

    fn into_f64(self) -> Result<f64, Self::Error> {
        Ok(unsafe { sqlite3_column_double(self.statement.handle, self.index as _) })
    }
}

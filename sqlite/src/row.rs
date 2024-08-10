use crate::{SQLite3Column, SQLite3FromRowError, SQLite3Rows};
use sql::FromRow;
use sqlite3::{sqlite3_column_count, SQLite3Stmt};
use std::marker::PhantomData;

/// A row returned as the result of a query to an SQLite3 database
pub struct SQLite3Row<'a> {
    handle: *mut SQLite3Stmt,
    total: usize,
    current: usize,

    _lifetime: PhantomData<&'a mut ()>,
}

impl<'a> SQLite3Row<'a> {
    pub(crate) fn new<T: FromRow>(
        handle: *mut SQLite3Stmt,
        #[allow(unused_variables)] parent: &'a mut SQLite3Rows<T>,
    ) -> Self {
        let total = unsafe { sqlite3_column_count(handle) } as usize;

        SQLite3Row {
            handle,
            total,
            current: 0,
            _lifetime: PhantomData,
        }
    }
}

impl<'a> sql::Row<'a> for SQLite3Row<'a> {
    type Error = SQLite3FromRowError;

    type Column = SQLite3Column<'a>;
}

impl<'a> Iterator for SQLite3Row<'a> {
    type Item = Result<SQLite3Column<'a>, SQLite3FromRowError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.total == self.current {
            return None;
        }

        let column = SQLite3Column::new(self.handle, self.current);
        self.current += 1;
        Some(Ok(column))
    }
}

unsafe impl<'a> Send for SQLite3Row<'a> {}

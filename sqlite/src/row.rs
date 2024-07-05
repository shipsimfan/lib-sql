use crate::{SQLite3Column, SQLite3Rows};
use sql::FromRow;
use sqlite3::{SQLite3Stmt, SQLiteError};
use std::marker::PhantomData;

/// A row returned as the result of a query to an SQLite3 database
pub struct SQLite3Row<'a> {
    handle: *mut SQLite3Stmt,
    _lifetime: PhantomData<&'a mut ()>,
}

impl<'a> SQLite3Row<'a> {
    pub(crate) fn new<T: FromRow>(
        handle: *mut SQLite3Stmt,
        #[allow(unused_variables)] parent: &'a mut SQLite3Rows<T>,
    ) -> Self {
        SQLite3Row {
            handle,
            _lifetime: PhantomData,
        }
    }
}

impl<'a> sql::Row for SQLite3Row<'a> {
    type Error = SQLiteError;

    fn get<T: sql::FromColumn>(&mut self, index: usize) -> Result<T, Self::Error> {
        T::from_column(SQLite3Column::new(self.handle, index, self))
    }
}

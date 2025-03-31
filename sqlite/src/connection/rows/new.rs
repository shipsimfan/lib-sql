use crate::{SQLite3Rows, SQLite3Statement};
use sql::FromRow;
use std::marker::PhantomData;

impl<'statement, T: FromRow> SQLite3Rows<'statement, T> {
    /// Creates a new [`SQLite3Rows`] iterator
    pub(crate) fn new(statement: SQLite3Statement<'statement>) -> Self {
        SQLite3Rows {
            statement,
            _output: PhantomData,
        }
    }
}

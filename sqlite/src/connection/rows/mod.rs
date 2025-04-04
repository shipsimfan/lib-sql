use crate::SQLite3Statement;
use sql::FromRow;
use std::marker::PhantomData;

mod error;
mod row;

mod iterator;
mod new;

pub use error::SQLite3FromRowError;
pub use row::SQLite3Row;

/// An iterator over a set of rows returned as the result of a query to an SQLite3 database
pub struct SQLite3Rows<'statement, T: FromRow> {
    /// The statement these rows come from
    statement: SQLite3Statement<'statement>,

    /// A marker for the output type
    _output: PhantomData<T>,
}

unsafe impl<'statement, T: FromRow> Send for SQLite3Rows<'statement, T> {}

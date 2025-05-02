use crate::SQLite3Statement;

mod error;

mod column;
mod new;

pub use error::SQLite3FromColumnError;

/// A column of a result returned by an query to an SQLite3 database
pub struct SQLite3Column<'column, 'statement> {
    /// The handle to statement this column comes from
    statement: &'column mut SQLite3Statement<'statement>,

    /// The index of this column
    index: usize,
}

unsafe impl<'column, 'statement> Send for SQLite3Column<'column, 'statement> {}

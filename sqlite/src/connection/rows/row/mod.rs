use crate::SQLite3Statement;

mod new;
mod row;

/// A row returned as the result of a query to an SQLite3 database
pub struct SQLite3Row<'row, 'statement> {
    /// The parent statement this row comes from
    statement: &'row mut SQLite3Statement<'statement>,

    /// The total number of rows in the result
    total: usize,

    /// The current index of the result
    current: usize,
}

unsafe impl<'row, 'statement> Send for SQLite3Row<'row, 'statement> {}

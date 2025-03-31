use crate::{SQLite3Column, SQLite3Statement};

impl<'column, 'statement> SQLite3Column<'column, 'statement> {
    /// Creates a new [`SQLite3Column`]
    pub(crate) fn new(statement: &'column mut SQLite3Statement<'statement>, index: usize) -> Self {
        SQLite3Column { statement, index }
    }
}

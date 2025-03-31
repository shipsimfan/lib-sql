use crate::{SQLite3Row, SQLite3Statement};
use sqlite3::sqlite3_column_count;

impl<'row, 'statement> SQLite3Row<'row, 'statement> {
    /// Creates a new [`SQLite3Row`]
    pub(crate) fn new(statement: &'row mut SQLite3Statement<'statement>) -> Self {
        let total = unsafe { sqlite3_column_count(statement.handle) } as usize;

        SQLite3Row {
            statement,
            total,
            current: 0,
        }
    }
}

use crate::{SQLite3Column, SQLite3Error, SQLite3Row};

impl<'row, 'statement> sql::Row<'row> for SQLite3Row<'row, 'statement> {
    type Error = SQLite3Error;

    type Column<'column>
        = SQLite3Column<'column, 'statement>
    where
        'row: 'column;

    fn next<'column>(&'column mut self) -> Result<Option<Self::Column<'column>>, Self::Error> {
        if self.total == self.current {
            return Ok(None);
        }

        let column = SQLite3Column::new(self.statement, self.current);
        self.current += 1;
        Ok(Some(column))
    }
}

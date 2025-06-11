use crate::FromColumn;

impl FromColumn for bool {
    fn from_column<'column, C: crate::Column<'column>>(column: C) -> Result<Self, C::Error> {
        Ok(column.into_usize()? != 0)
    }
}

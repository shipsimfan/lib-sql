use crate::{Column, FromColumn};

impl FromColumn for String {
    fn from_column<'column, C: Column<'column>>(column: C) -> Result<Self, C::Error> {
        column.into_str().map(|str| str.to_string())
    }
}

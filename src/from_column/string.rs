use crate::{Column, FromColumn};

impl FromColumn for String {
    fn from_column<'a, C: Column<'a>>(column: C) -> Result<Self, C::Error> {
        column.into_str().map(|str| str.to_string())
    }
}

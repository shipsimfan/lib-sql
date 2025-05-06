use crate::{FromColumn, Nullable};

impl<T: FromColumn> FromColumn for Nullable<T> {
    fn from_column<'column, C: crate::Column<'column>>(column: C) -> Result<Self, C::Error> {
        if column.is_null()? {
            return Ok(Nullable::Null);
        }

        T::from_column(column).map(Nullable::NonNull)
    }
}

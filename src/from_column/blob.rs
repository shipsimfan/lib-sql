use crate::{Column, FromColumn};

impl FromColumn for Vec<u8> {
    fn from_column<'a, C: Column<'a>>(column: C) -> Result<Self, C::Error> {
        column.into_blob().map(|blob| blob.to_vec())
    }
}

impl FromColumn for Box<[u8]> {
    fn from_column<'a, C: Column<'a>>(column: C) -> Result<Self, C::Error> {
        column
            .into_blob()
            .map(|blob| blob.to_vec().into_boxed_slice())
    }
}

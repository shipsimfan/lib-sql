use crate::{FromColumn, FromRow, Row};

impl<T: FromColumn> FromRow for T {
    fn from_row<R: Row>(mut row: R) -> Result<Self, R::Error> {
        row.get(0)
    }
}

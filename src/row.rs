use crate::FromColumn;

/// A row from a query result
pub trait Row {
    /// An error that can occur while converting a row
    type Error;

    /// Gets the column at index `index` as type `T`
    fn get<T: FromColumn>(&mut self, index: usize) -> Result<T, Self::Error>;
}

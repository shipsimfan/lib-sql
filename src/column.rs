/// A column of a row returned from an SQL query
pub trait Column<'a> {
    /// An error that occur while trying to convert this column to a specific type
    type Error<E>: From<E>;
}

/// A column of a row returned from an SQL query
pub trait Column<'a>: Sized {
    /// An error that occur while trying to convert this column to a specific type
    type Error;

    /// Converts this column into a blob
    fn into_blob(self) -> Result<&'a [u8], Self::Error>;

    /// Converts this column into a string
    fn into_str(self) -> Result<&'a str, Self::Error>;

    /// Converts this column into a [`u8`]
    fn into_u8(self) -> Result<u8, Self::Error> {
        self.into_u64().map(|val| val as _)
    }

    /// Converts this column into a [`u16`]
    fn into_u16(self) -> Result<u16, Self::Error> {
        self.into_u64().map(|val| val as _)
    }

    /// Converts this column into a [`u32`]
    fn into_u32(self) -> Result<u32, Self::Error> {
        self.into_u64().map(|val| val as _)
    }

    /// Converts this column into a [`u64`]
    fn into_u64(self) -> Result<u64, Self::Error>;

    /// Converts this column into a [`usize`]
    fn into_usize(self) -> Result<usize, Self::Error> {
        self.into_u64().map(|val| val as _)
    }

    /// Converts this column into a [`i8`]
    fn into_i8(self) -> Result<i8, Self::Error> {
        self.into_i64().map(|val| val as _)
    }

    /// Converts this column into a [`i16`]
    fn into_i16(self) -> Result<i16, Self::Error> {
        self.into_i64().map(|val| val as _)
    }

    /// Converts this column into a [`i32`]
    fn into_i32(self) -> Result<i32, Self::Error> {
        self.into_i64().map(|val| val as _)
    }

    /// Converts this column into a [`i64`]
    fn into_i64(self) -> Result<i64, Self::Error>;

    /// Converts this column into a [`isize`]
    fn into_isize(self) -> Result<isize, Self::Error> {
        self.into_i64().map(|val| val as _)
    }

    /// Converts this column into a [`f32`]
    fn into_f32(self) -> Result<f32, Self::Error> {
        self.into_f64().map(|val| val as _)
    }

    /// Converts this column into a [`f64`]
    fn into_f64(self) -> Result<f64, Self::Error>;
}

use crate::{Bind, FromRow};

/// A prepared SQL statement
pub trait Statement<'statement>: Sized {
    /// An error that can occur while binding a value
    type Error: std::error::Error;

    /// Execute the query and get the result rows
    fn rows<T: FromRow>(self) -> Result<impl Iterator<Item = Result<T, Self::Error>>, Self::Error>;

    /// Executes the query
    fn execute(self) -> Result<(), Self::Error>;

    /// Binds `val` to the parameter at index `idx`
    fn bind<T: Bind>(&mut self, idx: usize, val: &'statement T) -> Result<(), Self::Error> {
        val.bind(idx, self)
    }

    /// Binds `val` to the parameter at index `idx`
    fn bind_u8(&mut self, idx: usize, val: u8) -> Result<(), Self::Error> {
        self.bind_u64(idx, val as _)
    }

    /// Binds `val` to the parameter at index `idx`
    fn bind_u16(&mut self, idx: usize, val: u16) -> Result<(), Self::Error> {
        self.bind_u64(idx, val as _)
    }

    /// Binds `val` to the parameter at index `idx`
    fn bind_u32(&mut self, idx: usize, val: u32) -> Result<(), Self::Error> {
        self.bind_u64(idx, val as _)
    }

    /// Binds `val` to the parameter at index `idx`
    fn bind_u64(&mut self, idx: usize, val: u64) -> Result<(), Self::Error>;

    /// Binds `val` to the parameter at index `idx`
    fn bind_usize(&mut self, idx: usize, val: usize) -> Result<(), Self::Error> {
        self.bind_u64(idx, val as _)
    }

    /// Binds `val` to the parameter at index `idx`
    fn bind_i8(&mut self, idx: usize, val: i8) -> Result<(), Self::Error> {
        self.bind_i64(idx, val as _)
    }

    /// Binds `val` to the parameter at index `idx`
    fn bind_i16(&mut self, idx: usize, val: i16) -> Result<(), Self::Error> {
        self.bind_i64(idx, val as _)
    }

    /// Binds `val` to the parameter at index `idx`
    fn bind_i32(&mut self, idx: usize, val: i32) -> Result<(), Self::Error> {
        self.bind_i64(idx, val as _)
    }

    /// Binds `val` to the parameter at index `idx`
    fn bind_i64(&mut self, idx: usize, val: i64) -> Result<(), Self::Error>;

    /// Binds `val` to the parameter at index `idx`
    fn bind_isize(&mut self, idx: usize, val: isize) -> Result<(), Self::Error> {
        self.bind_i64(idx, val as _)
    }

    /// Binds `val` to the parameter at index `idx`
    fn bind_f32(&mut self, idx: usize, val: f32) -> Result<(), Self::Error> {
        self.bind_f64(idx, val as _)
    }

    /// Binds `val` to the parameter at index `idx`
    fn bind_f64(&mut self, idx: usize, val: f64) -> Result<(), Self::Error>;

    /// Binds `s` to the parameter at index `idx`
    fn bind_str(&mut self, idx: usize, s: &'statement str) -> Result<(), Self::Error>;

    /// Binds `b` to the parameter at index `idx`
    fn bind_blob(&mut self, idx: usize, b: &'statement [u8]) -> Result<(), Self::Error>;

    /// Binds NULL to the parameter at index `idx`
    fn bind_null(&mut self, idx: usize) -> Result<(), Self::Error>;
}

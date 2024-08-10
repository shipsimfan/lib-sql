use crate::{Bind, FromRow};

/// A prepared SQL statement
pub trait Statement<'a>: Sized {
    /// An error that can occur while binding a value
    type BindError: std::error::Error;

    /// An error that can occur while getting a row
    type GetRowError: std::error::Error;

    /// Execute the query and get the result rows
    fn rows<T: FromRow>(
        self,
    ) -> Result<impl Iterator<Item = Result<T, Self::GetRowError>>, Self::GetRowError>;

    /// Executes the query
    fn execute(self) -> Result<(), Self::GetRowError> {
        self.rows::<usize>().map(|_| ())
    }

    /// Binds `val` to the parameter at index `idx`
    fn bind<T: Bind>(&mut self, idx: usize, val: &'a T) -> Result<(), Self::BindError> {
        val.bind(idx, self)
    }

    /// Binds `val` to the parameter at index `idx`
    fn bind_u8(&mut self, idx: usize, val: u8) -> Result<(), Self::BindError> {
        self.bind_u64(idx, val as _)
    }

    /// Binds `val` to the parameter at index `idx`
    fn bind_u16(&mut self, idx: usize, val: u16) -> Result<(), Self::BindError> {
        self.bind_u64(idx, val as _)
    }

    /// Binds `val` to the parameter at index `idx`
    fn bind_u32(&mut self, idx: usize, val: u32) -> Result<(), Self::BindError> {
        self.bind_u64(idx, val as _)
    }

    /// Binds `val` to the parameter at index `idx`
    fn bind_u64(&mut self, idx: usize, val: u64) -> Result<(), Self::BindError>;

    /// Binds `val` to the parameter at index `idx`
    fn bind_usize(&mut self, idx: usize, val: usize) -> Result<(), Self::BindError> {
        self.bind_u64(idx, val as _)
    }

    /// Binds `val` to the parameter at index `idx`
    fn bind_i8(&mut self, idx: usize, val: i8) -> Result<(), Self::BindError> {
        self.bind_i64(idx, val as _)
    }

    /// Binds `val` to the parameter at index `idx`
    fn bind_i16(&mut self, idx: usize, val: i16) -> Result<(), Self::BindError> {
        self.bind_i64(idx, val as _)
    }

    /// Binds `val` to the parameter at index `idx`
    fn bind_i32(&mut self, idx: usize, val: i32) -> Result<(), Self::BindError> {
        self.bind_i64(idx, val as _)
    }

    /// Binds `val` to the parameter at index `idx`
    fn bind_i64(&mut self, idx: usize, val: i64) -> Result<(), Self::BindError>;

    /// Binds `val` to the parameter at index `idx`
    fn bind_isize(&mut self, idx: usize, val: isize) -> Result<(), Self::BindError> {
        self.bind_i64(idx, val as _)
    }

    /// Binds `val` to the parameter at index `idx`
    fn bind_f32(&mut self, idx: usize, val: f32) -> Result<(), Self::BindError> {
        self.bind_f64(idx, val as _)
    }

    /// Binds `val` to the parameter at index `idx`
    fn bind_f64(&mut self, idx: usize, val: f64) -> Result<(), Self::BindError>;

    /// Binds `s` to the parameter at index `idx`
    fn bind_str(&mut self, idx: usize, s: &'a str) -> Result<(), Self::BindError>;

    /// Binds `b` to the parameter at index `idx`
    fn bind_blob(&mut self, idx: usize, b: &'a [u8]) -> Result<(), Self::BindError>;

    /// Binds NULL to the parameter at index `idx`
    fn bind_null(&mut self, idx: usize) -> Result<(), Self::BindError>;
}

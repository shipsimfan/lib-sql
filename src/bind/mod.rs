use crate::Statement;

mod blob;
mod null;
mod numeric;
mod string;

/// A data structure which can be bound to as an SQL statement parameter
pub trait Bind {
    /// Bind this structure to `statement` at `idx`
    fn bind<'statement, S: Statement<'statement>>(
        &'statement self,
        idx: usize,
        statement: &mut S,
    ) -> Result<(), S::BindError>;
}
